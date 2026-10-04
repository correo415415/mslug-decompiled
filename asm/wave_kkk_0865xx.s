| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave KKK — region $0865BE..$088A56
|  Región: $0865BE..$088A56  (9,334 B, 79 entradas, 6 huecos)
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
|  TaskHandler_0865be  @ $0865BE  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0865be, "ax", @progbits
        .global TaskHandler_0865be
TaskHandler_0865be:
        move.b  #0x0,0x10e39e.l                 | +000
        lea     0xffff.w,a0                     | +008
        move.l  a0,0x4c(a6)                     | +00c
        jsr     0x283ca.l                       | +010
        jmp     0x518.l                         | +016

| ----------------------------------------------------------------------------
|  TaskHandler_0865da  @ $0865DA  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0865da, "ax", @progbits
        .global TaskHandler_0865da
TaskHandler_0865da:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0865dc  @ $0865DC  (504 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0865dc, "ax", @progbits
        .global TaskHandler_0865dc
TaskHandler_0865dc:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x0,0x20(a6)                   | +00a
        jsr     TaskHandler_0884a6(pc)          | +010
        lea     0x2edcb0.l,a1                   | +014
        move.l  a1,0x78(a6)                     | +01a
        lea     0xedc8c.l,a1                    | +01e
        move.l  a1,0x7c(a6)                     | +024
        lea     TaskHandler_086e4a(pc),a1       | +028
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd22.l                       | +032
        addi.w  #0x50,0x22(a0)                  | +038
        addi.w  #0x10,0x24(a0)                  | +03e
        bra.w   .L086738                        | +044
        movea.l 0x3c(a6),a1                     | +048
        jsr     0x2942a.l                       | +04c
        move.b  #0x1,0x20(a6)                   | +052
        jsr     TaskHandler_0885b4(pc)          | +058
        lea     0x2edcba.l,a1                   | +05c
        move.l  a1,0x78(a6)                     | +062
        lea     0xedcd2.l,a1                    | +066
        move.l  a1,0x7c(a6)                     | +06c
        lea     TaskHandler_086e4a__L086e74(pc),a1 | +070
        jsr     0x4ae.l                         | +074
        jsr     0x5dd22.l                       | +07a
        addi.w  #0x50,0x22(a0)                  | +080
        bra.w   .L086738                        | +086
        movea.l 0x3c(a6),a1                     | +08a
        jsr     0x2942a.l                       | +08e
        move.b  #0x2,0x20(a6)                   | +094
        jsr     TaskHandler_0886c2(pc)          | +09a
        lea     0x2edcfe.l,a1                   | +09e
        move.l  a1,0x78(a6)                     | +0a4
        lea     0xedd2a.l,a1                    | +0a8
        move.l  a1,0x7c(a6)                     | +0ae
        lea     TaskHandler_086e4a__L086e9e(pc),a1 | +0b2
        jsr     0x4ae.l                         | +0b6
        jsr     0x5dd22.l                       | +0bc
        addi.w  #0x50,0x22(a0)                  | +0c2
        addi.w  #0x10,0x24(a0)                  | +0c8
        bra.w   .L086738                        | +0ce
        movea.l 0x3c(a6),a1                     | +0d2
        jsr     0x2942a.l                       | +0d6
        move.b  #0x3,0x20(a6)                   | +0dc
        jsr     TaskHandler_0887d0(pc)          | +0e2
        lea     0x2edd0c.l,a1                   | +0e6
        move.l  a1,0x78(a6)                     | +0ec
        lea     0xedda6.l,a1                    | +0f0
        move.l  a1,0x7c(a6)                     | +0f6
        lea     TaskHandler_086e4a__L086ec8(pc),a1 | +0fa
        jsr     0x4ae.l                         | +0fe
        jsr     0x5dd22.l                       | +104
        addi.w  #0x50,0x22(a0)                  | +10a
        bra.w   .L086738                        | +110
        movea.l 0x3c(a6),a1                     | +114
        jsr     0x2942a.l                       | +118
        move.b  #0x4,0x20(a6)                   | +11e
        jsr     TaskHandler_08891a(pc)          | +124
        lea     0x2edd20.l,a1                   | +128
        move.l  a1,0x78(a6)                     | +12e
        lea     0xede22.l,a1                    | +132
        move.l  a1,0x7c(a6)                     | +138
        lea     TaskHandler_086e4a__L086ef2(pc),a1 | +13c
        jsr     0x4ae.l                         | +140
        jsr     0x5dd22.l                       | +146
        addi.w  #0x50,0x22(a0)                  | +14c
        addi.w  #0x10,0x24(a0)                  | +152
        bra.w   .L086738                        | +158
.L086738:
        move.w  #0xc0,0x70(a6)                  | +15c
        move.w  #0x140,0x66(a6)                 | +162
        move.b  #0x0,0x21(a6)                   | +168
        lea     0x2ec41e.l,a0                   | +16e
        move.l  a0,0x48(a6)                     | +174
        lea     .L08675a(pc),a1                 | +178
        move.l  a1,(a6)                         | +17c
.L08675a:
        cmpi.w  #0xffc0,0x22(a6)                | +17e
        bgt.w   .L08676c                        | +184
        lea     0xffff.w,a0                     | +188
        move.l  a0,0x48(a6)                     | +18c
.L08676c:
        jsr     0x2783a.l                       | +190
        jsr     0x2870a.l                       | +196
        bcc.w   .L08678e                        | +19c
        lea     0x5e766.l,a0                    | +1a0
        jsr     0x5e770.l                       | +1a6
        bclr    #0x3,0x13(a6)                   | +1ac
.L08678e:
        cmpi.w  #0xc0,0x22(a6)                  | +1b2
        blt.w   .L08679e                        | +1b8
        move.w  #0x140,0x66(a6)                 | +1bc
.L08679e:
        cmpi.w  #0x0,0x66(a6)                   | +1c2
        bgt.w   .L0867bc                        | +1c8
        lea     0xffff.w,a0                     | +1cc
        move.l  a0,0x48(a6)                     | +1d0
        move.b  #0xff,0x21(a6)                  | +1d4
        lea     TaskHandler_0867d4(pc),a1       | +1da
        move.l  a1,(a6)                         | +1de
.L0867bc:
        jsr     0x4fa70.l                       | +1e0
        bcc.w   .L0867d2                        | +1e6
        move.b  #0xff,0x21(a6)                  | +1ea
        jmp     0x518.l                         | +1f0
.L0867d2:
        rts                                     | +1f6

| ----------------------------------------------------------------------------
|  TaskHandler_0867d4  @ $0867D4  (120 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0867d4, "ax", @progbits
        .global TaskHandler_0867d4
TaskHandler_0867d4:
        jsr     0x2783a.l                       | +000
        move.w  #0x1028,d0                      | +006
        jsr     0x2352.l                        | +00a
        lea     0x2ecf68.l,a1                   | +010
        jsr     0x77c7e.l                       | +016
        lea     0x2ecf7a.l,a1                   | +01c
        jsr     0x77c7e.l                       | +022
        lea     0x2ed04a.l,a1                   | +028
        jsr     0x77c7e.l                       | +02e
        jsr     TaskHandler_088a28(pc)          | +034
        movea.l 0x78(a6),a1                     | +038
        jsr     0x43fac.l                       | +03c
        jsr     0x434dc.l                       | +042
        lea     0x2ecc88.l,a0                   | +048
        move.l  a0,0x4c(a6)                     | +04e
        jsr     0x283ca.l                       | +052
        jsr     0x283ca.l                       | +058
        jsr     0x283d8.l                       | +05e
        movea.l 0x7c(a6),a1                     | +064
        move.w  #0x82,d0                        | +068
        move.b  #0x0,0x82(a6)                   | +06c
        jsr     0x4429e.l                       | +072

| ----------------------------------------------------------------------------
|  TaskHandler_086854  @ $086854  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086854, "ax", @progbits
        .global TaskHandler_086854
TaskHandler_086854:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x4c(a6)                     | +004
        jsr     0x283ca.l                       | +008
        jmp     0x518.l                         | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_086868  @ $086868  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086868, "ax", @progbits
        .global TaskHandler_086868
TaskHandler_086868:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_08686a  @ $08686A  (314 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08686a, "ax", @progbits
        .global TaskHandler_08686a
TaskHandler_08686a:
        move.w  #0xe3,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0x0,0x21(a6)                   | +00a
        bra.w   .L0868e2                        | +010
        move.w  #0xe4,d1                        | +014
        jsr     0x236e.l                        | +018
        move.b  #0x30,0x21(a6)                  | +01e
        bra.w   .L0868e2                        | +024
        .global TaskHandler_08686a__L086892
TaskHandler_08686a__L086892:
        move.w  #0xe3,d1                        | +028
        jsr     0x236e.l                        | +02c
        move.b  #0x60,0x21(a6)                  | +032
        bra.w   .L0868e2                        | +038
        .global TaskHandler_08686a__L0868a6
TaskHandler_08686a__L0868a6:
        move.w  #0xe3,d1                        | +03c
        jsr     0x236e.l                        | +040
        move.b  #0x1,0x21(a6)                   | +046
        bra.w   .L0868e2                        | +04c
        .global TaskHandler_08686a__L0868ba
TaskHandler_08686a__L0868ba:
        move.w  #0xe4,d1                        | +050
        jsr     0x236e.l                        | +054
        move.b  #0x31,0x21(a6)                  | +05a
        bra.w   .L0868e2                        | +060
        move.w  #0xe3,d1                        | +064
        jsr     0x236e.l                        | +068
        move.b  #0x61,0x21(a6)                  | +06e
        bra.w   .L0868e2                        | +074
.L0868e2:
        move.b  #0xff,0x32(a6)                  | +078
        move.b  #0xff,0x33(a6)                  | +07e
        move.b  #0x0,0x3a(a6)                   | +084
        move.w  #0x28,d0                        | +08a
        move.w  d0,0x66(a6)                     | +08e
        move.w  d0,0x80(a6)                     | +092
        move.w  #0x8000,0x38(a6)                | +096
        jsr     0x267e2.l                       | +09c
        jsr     0x27cee.l                       | +0a2
        clr.l   d0                              | +0a8
        move.b  0x21(a6),d0                     | +0aa
        asr.l   #0x4,d0                         | +0ae
        movea.l #0x2eb2c0,a0                    | +0b0
        lsl.w   #0x2,d0                         | +0b6
        movea.l (a0,d0.w),a0                    | +0b8
        cmpa.l  #0xffffffff,a0                  | +0bc
        beq.w   .L086936                        | +0c2
        jsr     0x28cd4.l                       | +0c6
.L086936:
        move.w  0x72(a6),d0                     | +0cc
        move.b  d0,0x44(a6)                     | +0d0
        lea     .L086944(pc),a1                 | +0d4
        move.l  a1,(a6)                         | +0d8
.L086944:
        movea.l 0xc(a6),a0                      | +0da
        cmpi.b  #0xff,0x21(a0)                  | +0de
        beq.w   .L086990                        | +0e4
        jsr     0x2783a.l                       | +0e8
        jsr     0x28d70.l                       | +0ee
        jsr     0x2870a.l                       | +0f4
        bcc.w   .L08697c                        | +0fa
        move.w  #0x108d,d0                      | +0fe
        jsr     0x2352.l                        | +102
        bclr    #0x3,0x13(a6)                   | +108
        jsr     TaskHandler_08848c(pc)          | +10e
.L08697c:
        cmpi.w  #0x1e,0x66(a6)                  | +112
        bgt.w   .L0869a2                        | +118
        lea     TaskHandler_0869a4(pc),a1       | +11c
        move.l  a1,(a6)                         | +120
        bra.w   .L0869a2                        | +122
.L086990:
        lea     0x2edd48.l,a1                   | +126
        jsr     0x43fac.l                       | +12c
        jmp     0x518.l                         | +132
.L0869a2:
        rts                                     | +138

| ----------------------------------------------------------------------------
|  TaskHandler_0869a4  @ $0869A4  (140 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0869a4, "ax", @progbits
        .global TaskHandler_0869a4
TaskHandler_0869a4:
        clr.l   d0                              | +000
        move.b  0x21(a6),d0                     | +002
        asr.l   #0x4,d0                         | +006
        addq.l  #0x1,d0                         | +008
        movea.l #0x2eb2c0,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L0869ca                        | +01c
        jsr     0x28cd4.l                       | +020
.L0869ca:
        lea     .L0869d0(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L0869d0:
        movea.l 0xc(a6),a0                      | +02c
        cmpi.b  #0xff,0x21(a0)                  | +030
        beq.w   .L086a1c                        | +036
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        jsr     0x2870a.l                       | +046
        bcc.w   .L086a08                        | +04c
        move.w  #0x108d,d0                      | +050
        jsr     0x2352.l                        | +054
        bclr    #0x3,0x13(a6)                   | +05a
        jsr     TaskHandler_08848c(pc)          | +060
.L086a08:
        cmpi.w  #0x1a,0x66(a6)                  | +064
        bgt.w   .L086a2e                        | +06a
        lea     TaskHandler_086a30(pc),a1       | +06e
        move.l  a1,(a6)                         | +072
        bra.w   .L086a2e                        | +074
.L086a1c:
        lea     0x2edd48.l,a1                   | +078
        jsr     0x43fac.l                       | +07e
        jmp     0x518.l                         | +084
.L086a2e:
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  TaskHandler_086a30  @ $086A30  (140 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086a30, "ax", @progbits
        .global TaskHandler_086a30
TaskHandler_086a30:
        clr.l   d0                              | +000
        move.b  0x21(a6),d0                     | +002
        asr.l   #0x4,d0                         | +006
        addq.l  #0x2,d0                         | +008
        movea.l #0x2eb2c0,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L086a56                        | +01c
        jsr     0x28cd4.l                       | +020
.L086a56:
        lea     .L086a5c(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L086a5c:
        movea.l 0xc(a6),a0                      | +02c
        cmpi.b  #0xff,0x21(a0)                  | +030
        beq.w   .L086aa8                        | +036
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        jsr     0x2870a.l                       | +046
        bcc.w   .L086a94                        | +04c
        move.w  #0x108d,d0                      | +050
        jsr     0x2352.l                        | +054
        bclr    #0x3,0x13(a6)                   | +05a
        jsr     TaskHandler_08848c(pc)          | +060
.L086a94:
        cmpi.w  #0xd,0x66(a6)                   | +064
        bgt.w   .L086aba                        | +06a
        lea     TaskHandler_086abc(pc),a1       | +06e
        move.l  a1,(a6)                         | +072
        bra.w   .L086aba                        | +074
.L086aa8:
        lea     0x2edd48.l,a1                   | +078
        jsr     0x43fac.l                       | +07e
        jmp     0x518.l                         | +084
.L086aba:
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  TaskHandler_086abc  @ $086ABC  (232 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086abc, "ax", @progbits
        .global TaskHandler_086abc
TaskHandler_086abc:
        lea     .L086ac2(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L086ac2:
        movea.l 0xc(a6),a0                      | +006
        cmpi.b  #0xff,0x21(a0)                  | +00a
        beq.w   .L086b90                        | +010
        jsr     0x2783a.l                       | +014
        jsr     0x28d70.l                       | +01a
        jsr     0x2870a.l                       | +020
        bcc.w   .L086afa                        | +026
        move.w  #0x108d,d0                      | +02a
        jsr     0x2352.l                        | +02e
        bclr    #0x3,0x13(a6)                   | +034
        jsr     TaskHandler_08848c(pc)          | +03a
.L086afa:
        jsr     0x28758.l                       | +03e
        bcc.w   .L086ba2                        | +044
        bclr    #0x0,0x13(a6)                   | +048
        lea     0xffff.w,a0                     | +04e
        move.l  a0,0x48(a6)                     | +052
        btst    #0x0,0x21(a6)                   | +056
        beq.w   .L086b26                        | +05c
        lea     TaskHandler_086ba4(pc),a1       | +060
        move.l  a1,(a6)                         | +064
        bra.w   .L086ba2                        | +066
.L086b26:
        move.l  #0x100,d0                       | +06a
        jsr     0x51a28.l                       | +070
        move.w  #0x102f,d0                      | +076
        jsr     0x2352.l                        | +07a
        lea     0x78066.l,a1                    | +080
        jsr     0x4ae.l                         | +086
        jsr     0x5dd22.l                       | +08c
        addi.w  #0x10,0x24(a0)                  | +092
        movea.l 0x74(a6),a2                     | +098
        jsr     0x5022a.l                       | +09c
        lea     0x2ed05c.l,a1                   | +0a2
        jsr     0x77c7e.l                       | +0a8
        addi.w  #0x10,0x24(a0)                  | +0ae
        movea.l 0x7c(a6),a1                     | +0b4
        move.l  a1,d0                           | +0b8
        cmpi.l  #0xffffffff,d0                  | +0ba
        beq.w   .L086b90                        | +0c0
        move.w  #0x82,d0                        | +0c4
        move.b  #0x0,0x82(a6)                   | +0c8
        jsr     0x4429e.l                       | +0ce
.L086b90:
        lea     0x2edd48.l,a1                   | +0d4
        jsr     0x43fac.l                       | +0da
        jmp     0x518.l                         | +0e0
.L086ba2:
        rts                                     | +0e6

| ----------------------------------------------------------------------------
|  TaskHandler_086ba4  @ $086BA4  (190 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086ba4, "ax", @progbits
        .global TaskHandler_086ba4
TaskHandler_086ba4:
        lea     TaskHandler_086c62(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        lea     .L086bba(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L086bba:
        movea.l 0xc(a6),a0                      | +016
        cmpi.b  #0xff,0x21(a0)                  | +01a
        beq.w   .L086c4e                        | +020
        jsr     0x2783a.l                       | +024
        jsr     0x28d70.l                       | +02a
        move.b  0x21(a6),d0                     | +030
        andi.b  #0xf,d0                         | +034
        cmpi.b  #0xf,d0                         | +038
        bne.w   .L086c60                        | +03c
        move.l  #0x100,d0                       | +040
        jsr     0x51a28.l                       | +046
        move.w  #0x102c,d0                      | +04c
        jsr     0x2352.l                        | +050
        lea     0x78066.l,a1                    | +056
        jsr     0x4ae.l                         | +05c
        jsr     0x5dd22.l                       | +062
        addi.w  #0x10,0x24(a0)                  | +068
        movea.l 0x74(a6),a2                     | +06e
        jsr     0x5022a.l                       | +072
        lea     0x2ed05c.l,a1                   | +078
        jsr     0x77c7e.l                       | +07e
        addi.w  #0x10,0x24(a0)                  | +084
        movea.l 0x7c(a6),a1                     | +08a
        move.l  a1,d0                           | +08e
        cmpi.l  #0xffffffff,d0                  | +090
        beq.w   .L086c4e                        | +096
        move.w  #0x82,d0                        | +09a
        move.b  #0x0,0x82(a6)                   | +09e
        jsr     0x4429e.l                       | +0a4
.L086c4e:
        lea     0x2edd48.l,a1                   | +0aa
        jsr     0x43fac.l                       | +0b0
        jmp     0x518.l                         | +0b6
.L086c60:
        rts                                     | +0bc

| ----------------------------------------------------------------------------
|  TaskHandler_086c62  @ $086C62  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086c62, "ax", @progbits
        .global TaskHandler_086c62
TaskHandler_086c62:
        move.w  #0x6c,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x0,0x38(a6)                   | +01c
        addi.w  #0x8,0x24(a6)                   | +022
        lea     0x2eb2e4.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L086c9c(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L086c9c:
        movea.l 0xc(a6),a0                      | +03a
        movea.l 0xc(a0),a0                      | +03e
        cmpi.b  #0xff,0x21(a0)                  | +042
        beq.w   .L086cc8                        | +048
        jsr     0x2783a.l                       | +04c
        jsr     0x28d70.l                       | +052
        bcc.w   .L086cce                        | +058
        movea.l 0xc(a6),a0                      | +05c
        ori.b   #0xf,0x21(a0)                   | +060
.L086cc8:
        jmp     0x518.l                         | +066
.L086cce:
        rts                                     | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_086cd0  @ $086CD0  (222 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086cd0, "ax", @progbits
        .global TaskHandler_086cd0
TaskHandler_086cd0:
        move.w  #0xe7,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0x0,0x21(a6)                   | +00a
        bra.w   .L086d34                        | +010
        .global TaskHandler_086cd0__L086ce4
TaskHandler_086cd0__L086ce4:
        move.w  #0xe8,d1                        | +014
        jsr     0x236e.l                        | +018
        move.b  #0x1,0x21(a6)                   | +01e
        bra.w   .L086d34                        | +024
        .global TaskHandler_086cd0__L086cf8
TaskHandler_086cd0__L086cf8:
        move.w  #0xe9,d1                        | +028
        jsr     0x236e.l                        | +02c
        move.b  #0x2,0x21(a6)                   | +032
        bra.w   .L086d34                        | +038
        .global TaskHandler_086cd0__L086d0c
TaskHandler_086cd0__L086d0c:
        move.w  #0xea,d1                        | +03c
        jsr     0x236e.l                        | +040
        move.b  #0x3,0x21(a6)                   | +046
        bra.w   .L086d34                        | +04c
        .global TaskHandler_086cd0__L086d20
TaskHandler_086cd0__L086d20:
        move.w  #0xeb,d1                        | +050
        jsr     0x236e.l                        | +054
        move.b  #0x4,0x21(a6)                   | +05a
        bra.w   .L086d34                        | +060
.L086d34:
        move.b  #0xff,0x32(a6)                  | +064
        move.b  #0xff,0x33(a6)                  | +06a
        move.b  #0x0,0x3a(a6)                   | +070
        move.w  #0x14,0x66(a6)                  | +076
        move.w  #0x0,0x38(a6)                   | +07c
        lea     .L086d58(pc),a1                 | +082
        move.l  a1,(a6)                         | +086
.L086d58:
        movea.l 0xc(a6),a0                      | +088
        cmpi.b  #0xff,0x21(a0)                  | +08c
        beq.w   .L086da8                        | +092
        jsr     0x2783a.l                       | +096
        jsr     0x2870a.l                       | +09c
        bcc.w   .L086d8c                        | +0a2
        lea     0x5e766.l,a0                    | +0a6
        jsr     0x5e770.l                       | +0ac
        bclr    #0x3,0x13(a6)                   | +0b2
        jsr     TaskHandler_08848c(pc)          | +0b8
.L086d8c:
        jsr     0x28758.l                       | +0bc
        bcc.w   JsrPcThunk_086dae               | +0c2
        lea     0xffff.w,a0                     | +0c6
        move.l  a0,0x48(a6)                     | +0ca
        lea     TaskHandler_086db4(pc),a1       | +0ce
        move.l  a1,(a6)                         | +0d2
        bra.w   JsrPcThunk_086dae               | +0d4
.L086da8:
        jmp     0x518.l                         | +0d8

| ----------------------------------------------------------------------------
|  TaskHandler_086db4  @ $086DB4  (150 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086db4, "ax", @progbits
        .global TaskHandler_086db4
TaskHandler_086db4:
        move.l  #0x100,d0                       | +000
        jsr     0x51a28.l                       | +006
        move.w  #0x1039,d0                      | +00c
        jsr     0x2352.l                        | +010
        clr.l   d0                              | +016
        move.b  0x21(a6),d0                     | +018
        movea.l #0x2eb6ba,a0                    | +01c
        lsl.w   #0x2,d0                         | +022
        movea.l (a0,d0.w),a0                    | +024
        cmpa.l  #0xffffffff,a0                  | +028
        beq.w   .L086dec                        | +02e
        jsr     0x28cd4.l                       | +032
.L086dec:
        lea     .L086df2(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L086df2:
        movea.l 0xc(a6),a0                      | +03e
        cmpi.b  #0xff,0x21(a0)                  | +042
        beq.w   .L086e42                        | +048
        jsr     0x2783a.l                       | +04c
        jsr     0x28d70.l                       | +052
        bcc.w   .L086e48                        | +058
        move.w  #0x102f,d0                      | +05c
        jsr     0x2352.l                        | +060
        lea     0x7808a.l,a1                    | +066
        jsr     0x4ae.l                         | +06c
        jsr     0x5dd22.l                       | +072
        movea.l 0x74(a6),a2                     | +078
        jsr     0x5022a.l                       | +07c
        lea     0x2ed05c.l,a1                   | +082
        jsr     0x77c7e.l                       | +088
.L086e42:
        jmp     0x518.l                         | +08e
.L086e48:
        rts                                     | +094

| ----------------------------------------------------------------------------
|  TaskHandler_086e4a  @ $086E4A  (282 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086e4a, "ax", @progbits
        .global TaskHandler_086e4a
TaskHandler_086e4a:
        move.w  #0x30,0x70(a6)                  | +000
        move.b  #0x0,0x20(a6)                   | +006
        move.b  #0x1,0x21(a6)                   | +00c
        lea     0xede9e.l,a1                    | +012
        move.l  a1,0x7c(a6)                     | +018
        lea     0xedcb2.l,a1                    | +01c
        move.l  a1,0x8c(a6)                     | +022
        bra.w   TaskHandler_086f64              | +026
        .global TaskHandler_086e4a__L086e74
TaskHandler_086e4a__L086e74:
        move.w  #0x30,0x70(a6)                  | +02a
        move.b  #0x1,0x20(a6)                   | +030
        move.b  #0x1,0x21(a6)                   | +036
        lea     0xededa.l,a1                    | +03c
        move.l  a1,0x7c(a6)                     | +042
        lea     0xedd0a.l,a1                    | +046
        move.l  a1,0x8c(a6)                     | +04c
        bra.w   TaskHandler_086f64              | +050
        .global TaskHandler_086e4a__L086e9e
TaskHandler_086e4a__L086e9e:
        move.w  #0x30,0x70(a6)                  | +054
        move.b  #0x2,0x20(a6)                   | +05a
        move.b  #0x1,0x21(a6)                   | +060
        lea     0xedefa.l,a1                    | +066
        move.l  a1,0x7c(a6)                     | +06c
        lea     0xedd86.l,a1                    | +070
        move.l  a1,0x8c(a6)                     | +076
        bra.w   TaskHandler_086f64              | +07a
        .global TaskHandler_086e4a__L086ec8
TaskHandler_086e4a__L086ec8:
        move.w  #0x20,0x70(a6)                  | +07e
        move.b  #0x3,0x20(a6)                   | +084
        move.b  #0x1,0x21(a6)                   | +08a
        lea     0xedf1a.l,a1                    | +090
        move.l  a1,0x7c(a6)                     | +096
        lea     0xede02.l,a1                    | +09a
        move.l  a1,0x8c(a6)                     | +0a0
        bra.w   TaskHandler_086f64              | +0a4
        .global TaskHandler_086e4a__L086ef2
TaskHandler_086e4a__L086ef2:
        move.w  #0x20,0x70(a6)                  | +0a8
        move.b  #0x4,0x20(a6)                   | +0ae
        move.b  #0x1,0x21(a6)                   | +0b4
        lea     0xedf52.l,a1                    | +0ba
        move.l  a1,0x7c(a6)                     | +0c0
        lea     0xede7e.l,a1                    | +0c4
        move.l  a1,0x8c(a6)                     | +0ca
        bra.w   TaskHandler_086f64              | +0ce
        movea.l 0x3c(a6),a1                     | +0d2
        jsr     0x2942a.l                       | +0d6
        move.w  #0x30,0x70(a6)                  | +0dc
        move.b  #0x5,0x20(a6)                   | +0e2
        lea     0xedf72.l,a1                    | +0e8
        move.l  a1,0x7c(a6)                     | +0ee
        bra.w   TaskHandler_086f64              | +0f2
        movea.l 0x3c(a6),a1                     | +0f6
        jsr     0x2942a.l                       | +0fa
        move.w  #0x30,0x70(a6)                  | +100
        move.b  #0x6,0x20(a6)                   | +106
        lea     0xedfaa.l,a1                    | +10c
        move.l  a1,0x7c(a6)                     | +112
        bra.w   TaskHandler_086f64              | +116

| ----------------------------------------------------------------------------
|  TaskHandler_086f64  @ $086F64  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086f64, "ax", @progbits
        .global TaskHandler_086f64
TaskHandler_086f64:
        move.w  #0x5a,0x72(a6)                  | +000
        lea     .L086f70(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L086f70:
        jsr     0x2783a.l                       | +00c
        subq.w  #0x1,0x72(a6)                   | +012
        bne.w   .L086f84                        | +016
        lea     TaskHandler_086fb4(pc),a1       | +01a
        move.l  a1,(a6)                         | +01e
.L086f84:
        cmpi.b  #0x1,0x21(a6)                   | +020
        bne.w   .L086fa2                        | +026
        movea.l 0xc(a6),a0                      | +02a
        cmpi.b  #0xff,0x21(a0)                  | +02e
        bne.w   .L086fa2                        | +034
        lea     TaskHandler_087108(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L086fa2:
        jsr     0x4fa70.l                       | +03e
        bcc.w   .L086fb2                        | +044
        jmp     0x518.l                         | +048
.L086fb2:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_086fb4  @ $086FB4  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_086fb4, "ax", @progbits
        .global TaskHandler_086fb4
TaskHandler_086fb4:
        clr.l   d0                              | +000
        move.b  0x20(a6),d0                     | +002
        movea.l #0x2ebfae,a0                    | +006
        lsl.w   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a0                    | +00e
        cmpa.l  #0xffffffff,a0                  | +012
        beq.w   .L086fd6                        | +018
        jsr     0x28cd4.l                       | +01c
.L086fd6:
        lea     .L086fdc(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L086fdc:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L086ff2                        | +034
        lea     TaskHandler_087004(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L086ff2:
        jsr     0x4fa70.l                       | +03e
        bcc.w   .L087002                        | +044
        jmp     0x518.l                         | +048
.L087002:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_087004  @ $087004  (100 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087004, "ax", @progbits
        .global TaskHandler_087004
TaskHandler_087004:
        move.w  #0x28,0x72(a6)                  | +000
        movea.l 0x7c(a6),a1                     | +006
        move.w  #0x82,d0                        | +00a
        move.b  #0x0,0x82(a6)                   | +00e
        jsr     0x4429e.l                       | +014
        lea     .L087024(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L087024:
        jsr     0x2783a.l                       | +020
        jsr     Entity_CmpPrioWithSibling_086552(pc) | +026
        bcs.w   .L087038                        | +02a
        lea     TaskHandler_087068(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L087038:
        cmpi.b  #0x1,0x21(a6)                   | +034
        bne.w   .L087056                        | +03a
        movea.l 0xc(a6),a0                      | +03e
        cmpi.b  #0xff,0x21(a0)                  | +042
        bne.w   .L087056                        | +048
        lea     TaskHandler_087130(pc),a1       | +04c
        move.l  a1,(a6)                         | +050
.L087056:
        jsr     0x4fa70.l                       | +052
        bcc.w   .L087066                        | +058
        jmp     0x518.l                         | +05c
.L087066:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  TaskHandler_087068  @ $087068  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087068, "ax", @progbits
        .global TaskHandler_087068
TaskHandler_087068:
        move.w  #0x50,0x72(a6)                  | +000
        lea     .L087074(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L087074:
        jsr     0x2783a.l                       | +00c
        subq.w  #0x1,0x72(a6)                   | +012
        bne.w   .L087088                        | +016
        lea     TaskHandler_0870b8(pc),a1       | +01a
        move.l  a1,(a6)                         | +01e
.L087088:
        cmpi.b  #0x1,0x21(a6)                   | +020
        bne.w   .L0870a6                        | +026
        movea.l 0xc(a6),a0                      | +02a
        cmpi.b  #0xff,0x21(a0)                  | +02e
        bne.w   .L0870a6                        | +034
        lea     TaskHandler_087130(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L0870a6:
        jsr     0x4fa70.l                       | +03e
        bcc.w   .L0870b6                        | +044
        jmp     0x518.l                         | +048
.L0870b6:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_0870b8  @ $0870B8  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0870b8, "ax", @progbits
        .global TaskHandler_0870b8
TaskHandler_0870b8:
        clr.l   d0                              | +000
        move.b  0x20(a6),d0                     | +002
        movea.l #0x2ebfca,a0                    | +006
        lsl.w   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a0                    | +00e
        cmpa.l  #0xffffffff,a0                  | +012
        beq.w   .L0870da                        | +018
        jsr     0x28cd4.l                       | +01c
.L0870da:
        lea     .L0870e0(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L0870e0:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L0870f6                        | +034
        lea     TaskHandler_086f64(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L0870f6:
        jsr     0x4fa70.l                       | +03e
        bcc.w   .L087106                        | +044
        jmp     0x518.l                         | +048
.L087106:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_087108  @ $087108  (40 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087108, "ax", @progbits
        .global TaskHandler_087108
TaskHandler_087108:
        clr.l   d0                              | +000
        move.b  0x20(a6),d0                     | +002
        movea.l #0x2ebfae,a0                    | +006
        lsl.w   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a0                    | +00e
        cmpa.l  #0xffffffff,a0                  | +012
        beq.w   .L08712a                        | +018
        jsr     0x28cd4.l                       | +01c
.L08712a:
        lea     TaskHandler_087130(pc),a1       | +022
        move.l  a1,(a6)                         | +026

| ----------------------------------------------------------------------------
|  TaskHandler_087130  @ $087130  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087130, "ax", @progbits
        .global TaskHandler_087130
TaskHandler_087130:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L087158                        | +00c
        movea.l 0x8c(a6),a1                     | +010
        move.w  #0x82,d0                        | +014
        move.b  #0x0,0x82(a6)                   | +018
        jsr     0x4429e.l                       | +01e
        bra.w   .L087162                        | +024
.L087158:
        jsr     0x4fa70.l                       | +028
        bcc.w   .L087168                        | +02e
.L087162:
        jmp     0x518.l                         | +032
.L087168:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  TaskHandler_08716a  @ $08716A  (144 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08716a, "ax", @progbits
        .global TaskHandler_08716a
TaskHandler_08716a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        lea     0x2ec726.l,a0                   | +010
        move.l  a0,0x48(a6)                     | +016
        lea     .L08718a(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L08718a:
        jsr     0x2783a.l                       | +020
        jsr     0x2870a.l                       | +026
        bcc.w   .L0871ac                        | +02c
        lea     0x5e766.l,a0                    | +030
        jsr     0x5e770.l                       | +036
        bclr    #0x3,0x13(a6)                   | +03c
.L0871ac:
        jsr     0x28758.l                       | +042
        bcc.w   .L0871e8                        | +048
        move.w  #0x10a6,d0                      | +04c
        jsr     0x2352.l                        | +050
        lea     0x2ed0ae.l,a1                   | +056
        jsr     0x77c7e.l                       | +05c
        lea     0x2ed878.l,a2                   | +062
        jsr     0x5022a.l                       | +068
        lea     0x2ed864.l,a2                   | +06e
        jsr     0x5022a.l                       | +074
        bra.w   .L0871f2                        | +07a
.L0871e8:
        jsr     0x4fa70.l                       | +07e
        bcc.w   .L0871f8                        | +084
.L0871f2:
        jmp     0x518.l                         | +088
.L0871f8:
        rts                                     | +08e

| ----------------------------------------------------------------------------
|  TaskHandler_0871fa  @ $0871FA  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0871fa, "ax", @progbits
        .global TaskHandler_0871fa
TaskHandler_0871fa:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        lea     0x2ec726.l,a0                   | +010
        move.l  a0,0x48(a6)                     | +016
        lea     .L08721a(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L08721a:
        jsr     0x2783a.l                       | +020
        jsr     0x2870a.l                       | +026
        bcc.w   .L08723c                        | +02c
        lea     0x5e766.l,a0                    | +030
        jsr     0x5e770.l                       | +036
        bclr    #0x3,0x13(a6)                   | +03c
.L08723c:
        jsr     0x28758.l                       | +042
        bcc.w   .L08726e                        | +048
        lea     0x2ed0ae.l,a1                   | +04c
        jsr     0x77c7e.l                       | +052
        lea     0x2ed968.l,a2                   | +058
        jsr     0x5022a.l                       | +05e
        lea     0x2ed97c.l,a2                   | +064
        jsr     0x5022a.l                       | +06a
        bra.w   .L087278                        | +070
.L08726e:
        jsr     0x4fa70.l                       | +074
        bcc.w   .L08727e                        | +07a
.L087278:
        jmp     0x518.l                         | +07e
.L08727e:
        rts                                     | +084

| ----------------------------------------------------------------------------
|  TaskHandler_087280  @ $087280  (122 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087280, "ax", @progbits
        .global TaskHandler_087280
TaskHandler_087280:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        lea     0x2ec726.l,a0                   | +010
        move.l  a0,0x48(a6)                     | +016
        lea     .L0872a0(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L0872a0:
        jsr     0x2783a.l                       | +020
        jsr     0x2870a.l                       | +026
        bcc.w   .L0872c2                        | +02c
        lea     0x5e766.l,a0                    | +030
        jsr     0x5e770.l                       | +036
        bclr    #0x3,0x13(a6)                   | +03c
.L0872c2:
        jsr     0x28758.l                       | +042
        bcc.w   .L0872e8                        | +048
        lea     0x2ed0ae.l,a1                   | +04c
        jsr     0x77c7e.l                       | +052
        lea     0x2eda6c.l,a2                   | +058
        jsr     0x5022a.l                       | +05e
        bra.w   .L0872f2                        | +064
.L0872e8:
        jsr     0x4fa70.l                       | +068
        bcc.w   .L0872f8                        | +06e
.L0872f2:
        jmp     0x518.l                         | +072
.L0872f8:
        rts                                     | +078

| ----------------------------------------------------------------------------
|  TaskHandler_0872fa  @ $0872FA  (108 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0872fa, "ax", @progbits
        .global TaskHandler_0872fa
TaskHandler_0872fa:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        lea     0x2ebfe6.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L08731c(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L08731c:
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        jsr     0x2870a.l                       | +02e
        bcc.w   .L08734e                        | +034
        move.w  #0x10a9,d0                      | +038
        jsr     0x2352.l                        | +03c
        lea     0x2ebffc.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        bclr    #0x3,0x13(a6)                   | +04e
.L08734e:
        move.w  #0x7fff,0x66(a6)                | +054
        jsr     0x4fa70.l                       | +05a
        bcc.w   .L087364                        | +060
        jmp     0x518.l                         | +064
.L087364:
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  TaskHandler_087366  @ $087366  (338 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087366, "ax", @progbits
        .global TaskHandler_087366
TaskHandler_087366:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     TaskHandler_0874ba(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd22.l                       | +014
        subi.w  #0x10,0x22(a0)                  | +01a
        addi.w  #0x40,0x24(a0)                  | +020
        lea     TaskHandler_0874ea(pc),a1       | +026
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd22.l                       | +030
        addi.w  #0x10,0x22(a0)                  | +036
        addi.w  #0xa0,0x24(a0)                  | +03c
        lea     TaskHandler_08751c(pc),a1       | +042
        jsr     0x4ae.l                         | +046
        jsr     0x5dd22.l                       | +04c
        addi.w  #0x50,0x22(a0)                  | +052
        addi.w  #0xa0,0x24(a0)                  | +058
        lea     TaskHandler_08754e(pc),a1       | +05e
        jsr     0x4ae.l                         | +062
        jsr     0x5dd22.l                       | +068
        addi.w  #0x80,0x22(a0)                  | +06e
        addi.w  #0xa0,0x24(a0)                  | +074
        lea     TaskHandler_087580(pc),a1       | +07a
        jsr     0x4ae.l                         | +07e
        jsr     0x5dd22.l                       | +084
        addi.w  #0x10,0x22(a0)                  | +08a
        addi.w  #0x80,0x24(a0)                  | +090
        lea     TaskHandler_0875b0(pc),a1       | +096
        jsr     0x4ae.l                         | +09a
        jsr     0x5dd22.l                       | +0a0
        addi.w  #0x60,0x22(a0)                  | +0a6
        addi.w  #0x80,0x24(a0)                  | +0ac
        lea     TaskHandler_0875e0(pc),a1       | +0b2
        jsr     0x4ae.l                         | +0b6
        jsr     0x5dd22.l                       | +0bc
        addi.w  #0x50,0x22(a0)                  | +0c2
        addi.w  #0x50,0x24(a0)                  | +0c8
        lea     TaskHandler_087610(pc),a1       | +0ce
        jsr     0x4ae.l                         | +0d2
        jsr     0x5dd22.l                       | +0d8
        lea     TaskHandler_087640(pc),a1       | +0de
        jsr     0x4ae.l                         | +0e2
        jsr     0x5dd22.l                       | +0e8
        addi.w  #0x20,0x22(a0)                  | +0ee
        lea     TaskHandler_087670(pc),a1       | +0f4
        jsr     0x4ae.l                         | +0f8
        jsr     0x5dd22.l                       | +0fe
        addi.w  #0x40,0x22(a0)                  | +104
        lea     TaskHandler_0876a0(pc),a1       | +10a
        jsr     0x4ae.l                         | +10e
        jsr     0x5dd22.l                       | +114
        addi.w  #0x60,0x22(a0)                  | +11a
        lea     TaskHandler_0876d0(pc),a1       | +120
        jsr     0x4ae.l                         | +124
        jsr     0x5dd22.l                       | +12a
        addi.w  #0x80,0x22(a0)                  | +130
        lea     TaskHandler_087700(pc),a1       | +136
        jsr     0x4ae.l                         | +13a
        jsr     0x5dd22.l                       | +140
        addi.w  #0xa0,0x22(a0)                  | +146
        jmp     0x518.l                         | +14c

| ----------------------------------------------------------------------------
|  TaskHandler_0874b8  @ $0874B8  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0874b8, "ax", @progbits
        .global TaskHandler_0874b8
TaskHandler_0874b8:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0874ba  @ $0874BA  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0874ba, "ax", @progbits
        .global TaskHandler_0874ba
TaskHandler_0874ba:
        lea     0x2ed990.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x0,0x72(a6)                   | +012
        lea     0x2ec822.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ecfde.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_0874ea  @ $0874EA  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0874ea, "ax", @progbits
        .global TaskHandler_0874ea
TaskHandler_0874ea:
        lea     0x2ed9a4.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        lea     0xedfe2.l,a1                    | +00a
        move.l  a1,0x7c(a6)                     | +010
        move.w  #0x1,0x72(a6)                   | +014
        lea     0x2ec876.l,a0                   | +01a
        move.l  a0,0x48(a6)                     | +020
        lea     0x2ecff0.l,a1                   | +024
        move.l  a1,0x88(a6)                     | +02a
        bra.w   TaskHandler_087700__L087730     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_08751c  @ $08751C  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08751c, "ax", @progbits
        .global TaskHandler_08751c
TaskHandler_08751c:
        lea     0x2ed9b8.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        lea     0xee002.l,a1                    | +00a
        move.l  a1,0x7c(a6)                     | +010
        move.w  #0x2,0x72(a6)                   | +014
        lea     0x2ec822.l,a0                   | +01a
        move.l  a0,0x48(a6)                     | +020
        lea     0x2ecfde.l,a1                   | +024
        move.l  a1,0x88(a6)                     | +02a
        bra.w   TaskHandler_087700__L087730     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_08754e  @ $08754E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08754e, "ax", @progbits
        .global TaskHandler_08754e
TaskHandler_08754e:
        lea     0x2ed9cc.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        lea     0xee022.l,a1                    | +00a
        move.l  a1,0x7c(a6)                     | +010
        move.w  #0x3,0x72(a6)                   | +014
        lea     0x2ec822.l,a0                   | +01a
        move.l  a0,0x48(a6)                     | +020
        lea     0x2ecfde.l,a1                   | +024
        move.l  a1,0x88(a6)                     | +02a
        bra.w   TaskHandler_087700__L087730     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_087580  @ $087580  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087580, "ax", @progbits
        .global TaskHandler_087580
TaskHandler_087580:
        lea     0x2eda80.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x4,0x72(a6)                   | +012
        lea     0x2ec8ca.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed002.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_0875b0  @ $0875B0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0875b0, "ax", @progbits
        .global TaskHandler_0875b0
TaskHandler_0875b0:
        lea     0x2eda94.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x5,0x72(a6)                   | +012
        lea     0x2ec8ca.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed002.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_0875e0  @ $0875E0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0875e0, "ax", @progbits
        .global TaskHandler_0875e0
TaskHandler_0875e0:
        lea     0x2ed9e0.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x6,0x72(a6)                   | +012
        lea     0x2ec91e.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed014.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_087610  @ $087610  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087610, "ax", @progbits
        .global TaskHandler_087610
TaskHandler_087610:
        lea     0x2ed9f4.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x7,0x72(a6)                   | +012
        lea     0x2ec972.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed026.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_087640  @ $087640  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087640, "ax", @progbits
        .global TaskHandler_087640
TaskHandler_087640:
        lea     0x2eda08.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x8,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_087670  @ $087670  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087670, "ax", @progbits
        .global TaskHandler_087670
TaskHandler_087670:
        lea     0x2eda1c.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x9,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_0876a0  @ $0876A0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0876a0, "ax", @progbits
        .global TaskHandler_0876a0
TaskHandler_0876a0:
        lea     0x2eda30.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0xa,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_0876d0  @ $0876D0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0876d0, "ax", @progbits
        .global TaskHandler_0876d0
TaskHandler_0876d0:
        lea     0x2eda44.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0xb,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_087700  @ $087700  (206 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087700, "ax", @progbits
        .global TaskHandler_087700
TaskHandler_087700:
        lea     0x2eda58.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0xc,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   TaskHandler_087700__L087730     | +02c
        .global TaskHandler_087700__L087730
TaskHandler_087700__L087730:
        move.w  #0x30,0x70(a6)                  | +030
        move.w  #0x14,0x66(a6)                  | +036
        move.l  0x48(a6),0x84(a6)               | +03c
        lea     .L087748(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L087748:
        jsr     0x2783a.l                       | +048
        jsr     0x2870a.l                       | +04e
        bcc.w   .L08776a                        | +054
        lea     0x5e766.l,a0                    | +058
        jsr     0x5e770.l                       | +05e
        bclr    #0x3,0x13(a6)                   | +064
.L08776a:
        jsr     0x28758.l                       | +06a
        bcc.w   .L0877be                        | +070
        lea     0xffff.w,a0                     | +074
        move.l  a0,0x48(a6)                     | +078
        move.w  #0x1027,d0                      | +07c
        jsr     0x2352.l                        | +080
        movea.l 0x74(a6),a2                     | +086
        jsr     0x5022a.l                       | +08a
        movea.l 0x88(a6),a1                     | +090
        jsr     0x77c7e.l                       | +094
        cmpi.l  #0xffffffff,0x7c(a6)            | +09a
        beq.w   .L0877c8                        | +0a2
        movea.l 0x7c(a6),a1                     | +0a6
        move.w  #0x82,d0                        | +0aa
        move.b  #0x0,0x82(a6)                   | +0ae
        jsr     0x4429e.l                       | +0b4
        bra.w   .L0877c8                        | +0ba
.L0877be:
        jsr     0x4fa70.l                       | +0be
        bcc.w   JsrPcThunk_0877ce               | +0c4
.L0877c8:
        jmp     0x518.l                         | +0c8

| ----------------------------------------------------------------------------
|  TaskHandler_0877d4  @ $0877D4  (106 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0877d4, "ax", @progbits
        .global TaskHandler_0877d4
TaskHandler_0877d4:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xe5,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0xff,0x32(a6)                  | +014
        move.b  #0xff,0x33(a6)                  | +01a
        move.b  #0x0,0x3a(a6)                   | +020
        move.w  #0xf0,0x70(a6)                  | +026
        move.w  #0x0,0x38(a6)                   | +02c
        lea     0x2ec180.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     TaskHandler_087aa0(pc),a1       | +03e
        jsr     0x4ae.l                         | +042
        jsr     0x5dd22.l                       | +048
        lea     .L087828(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L087828:
        cmpi.w  #0x100,0x22(a6)                 | +054
        blt.w   TaskHandler_087846              | +05a
        clr.b   0x10e39a.l                      | +05e
        jsr     0x2783a.l                       | +064

| ----------------------------------------------------------------------------
|  TaskHandler_087846  @ $087846  (146 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087846, "ax", @progbits
        .global TaskHandler_087846
TaskHandler_087846:
        bclr    #0x3,0x13(a6)                   | +000
        bclr    #0x0,0x13(a6)                   | +006
        lea     0x2c0628.l,a0                   | +00c
        jsr     0x799de.l                       | +012
        move.w  d0,0x66(a6)                     | +018
        addi.w  #0x2e4,0x66(a6)                 | +01c
        move.l  #0x2ede46,0x60(a6)              | +022
        lea     .L087876(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L087876:
        clr.b   0x10e39a.l                      | +030
        jsr     0x2783a.l                       | +036
        jsr     0x28998.l                       | +03c
        jsr     0x28d70.l                       | +042
        jsr     0x2870a.l                       | +048
        bcc.w   .L0878aa                        | +04e
        lea     0x5e766.l,a0                    | +052
        jsr     0x5e770.l                       | +058
        bclr    #0x3,0x13(a6)                   | +05e
.L0878aa:
        cmpi.w  #0x29a,0x66(a6)                 | +064
        bgt.w   .L0878c6                        | +06a
        lea     0x2ed0c0.l,a1                   | +06e
        jsr     0x77c7e.l                       | +074
        lea     TaskHandler_0878d8(pc),a1       | +07a
        move.l  a1,(a6)                         | +07e
.L0878c6:
        jsr     0x4fa70.l                       | +080
        bcc.w   .L0878d6                        | +086
        jmp     0x518.l                         | +08a
.L0878d6:
        rts                                     | +090

| ----------------------------------------------------------------------------
|  TaskHandler_0878d8  @ $0878D8  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0878d8, "ax", @progbits
        .global TaskHandler_0878d8
TaskHandler_0878d8:
        move.w  #0x102b,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ec19a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0878f4(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0878f4:
        clr.b   0x10e39a.l                      | +01c
        jsr     0x2783a.l                       | +022
        jsr     0x28998.l                       | +028
        jsr     0x28d70.l                       | +02e
        jsr     0x2870a.l                       | +034
        bcc.w   .L087928                        | +03a
        lea     0x5e766.l,a0                    | +03e
        jsr     0x5e770.l                       | +044
        bclr    #0x3,0x13(a6)                   | +04a
.L087928:
        cmpi.w  #0xf6,0x66(a6)                  | +050
        bgt.w   .L087944                        | +056
        lea     0x2ed0d2.l,a1                   | +05a
        jsr     0x77c7e.l                       | +060
        lea     TaskHandler_087956(pc),a1       | +066
        move.l  a1,(a6)                         | +06a
.L087944:
        jsr     0x4fa70.l                       | +06c
        bcc.w   .L087954                        | +072
        jmp     0x518.l                         | +076
.L087954:
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  TaskHandler_087956  @ $087956  (194 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087956, "ax", @progbits
        .global TaskHandler_087956
TaskHandler_087956:
        move.w  #0x102b,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ec1ae.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L087972(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L087972:
        clr.b   0x10e39a.l                      | +01c
        jsr     0x2783a.l                       | +022
        jsr     0x28998.l                       | +028
        jsr     0x28d70.l                       | +02e
        jsr     0x2870a.l                       | +034
        bcc.w   .L0879a6                        | +03a
        lea     0x5e766.l,a0                    | +03e
        jsr     0x5e770.l                       | +044
        bclr    #0x3,0x13(a6)                   | +04a
.L0879a6:
        jsr     0x28758.l                       | +050
        bcc.w   .L087a06                        | +056
        move.w  #0x1038,d0                      | +05a
        jsr     0x2352.l                        | +05e
        lea     0x2ed0e4.l,a1                   | +064
        jsr     0x77c7e.l                       | +06a
        lea     0x2ecfcc.l,a1                   | +070
        jsr     0x77c7e.l                       | +076
        lea     0x2edd56.l,a1                   | +07c
        jsr     0x43fac.l                       | +082
        lea     0x2edd68.l,a1                   | +088
        jsr     0x43fac.l                       | +08e
        jsr     0x434dc.l                       | +094
        lea     TaskHandler_087a62(pc),a1       | +09a
        jsr     0x4ae.l                         | +09e
        jsr     0x5dd02.l                       | +0a4
        lea     TaskHandler_087a18(pc),a1       | +0aa
        move.l  a1,(a6)                         | +0ae
.L087a06:
        jsr     0x4fa70.l                       | +0b0
        bcc.w   .L087a16                        | +0b6
        jmp     0x518.l                         | +0ba
.L087a16:
        rts                                     | +0c0

| ----------------------------------------------------------------------------
|  TaskHandler_087a18  @ $087A18  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087a18, "ax", @progbits
        .global TaskHandler_087a18
TaskHandler_087a18:
        move.l  #0x5000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        lea     0x2ec1c2.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L087a3e(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L087a3e:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        jsr     0x283d8.l                       | +032
        jsr     0x4fa70.l                       | +038
        bcc.w   .L087a60                        | +03e
        jmp     0x518.l                         | +042
.L087a60:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  TaskHandler_087a62  @ $087A62  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087a62, "ax", @progbits
        .global TaskHandler_087a62
TaskHandler_087a62:
        move.w  #0x10,0x72(a6)                  | +000
        lea     .L087a6e(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L087a6e:
        jsr     0x2783a.l                       | +00c
        lea     0x2ecc34.l,a0                   | +012
        move.l  a0,0x4c(a6)                     | +018
        jsr     0x283ca.l                       | +01c
        jsr     0x283ca.l                       | +022
        jsr     0x283d8.l                       | +028
        subq.w  #0x1,0x72(a6)                   | +02e
        bpl.w   .L087a9e                        | +032
        jmp     0x518.l                         | +036
.L087a9e:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_087aa0  @ $087AA0  (124 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087aa0, "ax", @progbits
        .global TaskHandler_087aa0
TaskHandler_087aa0:
        move.w  #0xe5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x8000,d0                      | +01c
        move.w  #0x80,d1                        | +020
        jsr     0x2813c.l                       | +024
        lea     .L087ad0(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L087ad0:
        movea.l 0xc(a6),a0                      | +030
        move.w  0x22(a0),0x22(a6)               | +034
        move.w  0x24(a0),0x24(a6)               | +03a
        clr.l   d0                              | +040
        move.b  0x20(a0),d0                     | +042
        cmpi.b  #0x3,d0                         | +046
        beq.w   .L087b14                        | +04a
        movea.l #0x2ec21a,a0                    | +04e
        lsl.w   #0x2,d0                         | +054
        movea.l (a0,d0.w),a0                    | +056
        cmpa.l  #0xffffffff,a0                  | +05a
        beq.w   .L087b0a                        | +060
        jsr     0x28cd4.l                       | +064
.L087b0a:
        jsr     0x28d70.l                       | +06a
        bra.w   .L087b1a                        | +070
.L087b14:
        jmp     0x518.l                         | +074
.L087b1a:
        rts                                     | +07a

| ----------------------------------------------------------------------------
|  TaskHandler_087b1c  @ $087B1C  (262 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087b1c, "ax", @progbits
        .global TaskHandler_087b1c
TaskHandler_087b1c:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        jsr     0x2783a.l                       | +00a
        move.w  #0x86,d1                        | +010
        jsr     0x236e.l                        | +014
        move.w  #0x87,d1                        | +01a
        jsr     0x236e.l                        | +01e
        move.b  #0xff,0x32(a6)                  | +024
        move.b  #0xff,0x33(a6)                  | +02a
        move.b  #0x0,0x3a(a6)                   | +030
        move.w  #0x50,0x70(a6)                  | +036
        lea     0x2c05a6.l,a0                   | +03c
        jsr     0x799de.l                       | +042
        move.w  d0,0x66(a6)                     | +048
        addi.w  #0x17c,0x66(a6)                 | +04c
        move.w  #0x0,0x72(a6)                   | +052
        move.b  #0x0,0x20(a6)                   | +058
        move.w  #0x2000,0x38(a6)                | +05e
        lea     0x2ec226.l,a0                   | +064
        jsr     0x28cd4.l                       | +06a
        lea     TaskHandler_087eae(pc),a1       | +070
        jsr     0x4ae.l                         | +074
        jsr     0x5dd22.l                       | +07a
        lea     0x2edd7e.l,a1                   | +080
        jsr     0x43fac.l                       | +086
        move.l  #0x2eddfa,0x60(a6)              | +08c
        lea     .L087bb6(pc),a1                 | +094
        move.l  a1,(a6)                         | +098
.L087bb6:
        jsr     0x28998.l                       | +09a
        jsr     0x2783a.l                       | +0a0
        jsr     Sub_000883EC(pc)                | +0a6
        jsr     0x28d70.l                       | +0aa
        jsr     0x2870a.l                       | +0b0
        bcc.w   .L087bee                        | +0b6
        lea     0x5e766.l,a0                    | +0ba
        jsr     0x5e770.l                       | +0c0
        bclr    #0x3,0x13(a6)                   | +0c6
        move.w  #0xf,0x72(a6)                   | +0cc
.L087bee:
        bclr    #0x0,0x13(a6)                   | +0d2
        move.w  #0x17c,0x66(a6)                 | +0d8
        cmpi.w  #0x100,0x22(a6)                 | +0de
        bgt.w   .L087c0a                        | +0e4
        lea     TaskHandler_087c22(pc),a1       | +0e8
        move.l  a1,(a6)                         | +0ec
.L087c0a:
        jsr     0x4fa70.l                       | +0ee
        bcc.w   .L087c20                        | +0f4
        move.b  #0xff,0x20(a6)                  | +0f8
        jmp     0x518.l                         | +0fe
.L087c20:
        rts                                     | +104

| ----------------------------------------------------------------------------
|  TaskHandler_087c22  @ $087C22  (120 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087c22, "ax", @progbits
        .global TaskHandler_087c22
TaskHandler_087c22:
        lea     0x2ec23c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L087c34(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L087c34:
        jsr     0x28998.l                       | +012
        jsr     0x2783a.l                       | +018
        jsr     Sub_000883EC(pc)                | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L087c54                        | +028
        lea     TaskHandler_087c9a(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L087c54:
        jsr     0x2870a.l                       | +032
        bcc.w   .L087c76                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
        move.w  #0xf,0x72(a6)                   | +04e
.L087c76:
        bclr    #0x0,0x13(a6)                   | +054
        move.w  #0x17c,0x66(a6)                 | +05a
        jsr     0x4fa70.l                       | +060
        bcc.w   .L087c98                        | +066
        move.b  #0xff,0x20(a6)                  | +06a
        jmp     0x518.l                         | +070
.L087c98:
        rts                                     | +076

| ----------------------------------------------------------------------------
|  TaskHandler_087c9a  @ $087C9A  (130 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087c9a, "ax", @progbits
        .global TaskHandler_087c9a
TaskHandler_087c9a:
        lea     0xee042.l,a1                    | +000
        move.w  #0x82,d0                        | +006
        move.b  #0x0,0x82(a6)                   | +00a
        jsr     0x4429e.l                       | +010
        lea     .L087cb6(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L087cb6:
        jsr     0x28998.l                       | +01c
        jsr     0x2783a.l                       | +022
        jsr     Sub_000883EC(pc)                | +028
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L087cee                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
        move.w  #0xf,0x72(a6)                   | +04e
.L087cee:
        cmpi.w  #0xfc,0x66(a6)                  | +054
        bgt.w   .L087cfe                        | +05a
        lea     TaskHandler_087d1c(pc),a1       | +05e
        move.l  a1,(a6)                         | +062
.L087cfe:
        jsr     0x4fa70.l                       | +064
        bcc.w   .L087d1a                        | +06a
        move.b  #0xff,0x82(a6)                  | +06e
        move.b  #0xff,0x20(a6)                  | +074
        jmp     0x518.l                         | +07a
.L087d1a:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  TaskHandler_087d1c  @ $087D1C  (130 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087d1c, "ax", @progbits
        .global TaskHandler_087d1c
TaskHandler_087d1c:
        move.w  #0x1027,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ec2ce.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L087d38(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L087d38:
        jsr     0x28998.l                       | +01c
        jsr     0x2783a.l                       | +022
        jsr     Sub_000883EC(pc)                | +028
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L087d70                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
        move.w  #0xf,0x72(a6)                   | +04e
.L087d70:
        cmpi.w  #0x7e,0x66(a6)                  | +054
        bgt.w   .L087d80                        | +05a
        lea     TaskHandler_087d9e(pc),a1       | +05e
        move.l  a1,(a6)                         | +062
.L087d80:
        jsr     0x4fa70.l                       | +064
        bcc.w   .L087d9c                        | +06a
        move.b  #0xff,0x82(a6)                  | +06e
        move.b  #0xff,0x20(a6)                  | +074
        jmp     0x518.l                         | +07a
.L087d9c:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  TaskHandler_087d9e  @ $087D9E  (130 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087d9e, "ax", @progbits
        .global TaskHandler_087d9e
TaskHandler_087d9e:
        move.w  #0x1027,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ec2de.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L087dba(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L087dba:
        jsr     0x28998.l                       | +01c
        jsr     0x2783a.l                       | +022
        jsr     Sub_000883EC(pc)                | +028
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L087df2                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
        move.w  #0xf,0x72(a6)                   | +04e
.L087df2:
        jsr     0x28758.l                       | +054
        bcc.w   .L087e02                        | +05a
        lea     TaskHandler_087e20(pc),a1       | +05e
        move.l  a1,(a6)                         | +062
.L087e02:
        jsr     0x4fa70.l                       | +064
        bcc.w   .L087e1e                        | +06a
        move.b  #0xff,0x82(a6)                  | +06e
        move.b  #0xff,0x20(a6)                  | +074
        jmp     0x518.l                         | +07a
.L087e1e:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  TaskHandler_087e20  @ $087E20  (142 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087e20, "ax", @progbits
        .global TaskHandler_087e20
TaskHandler_087e20:
        move.l  #0x5000,d0                      | +000
        jsr     0x51a28.l                       | +006
        move.w  #0x1023,d0                      | +00c
        jsr     0x2352.l                        | +010
        jsr     0x2783a.l                       | +016
        move.b  #0xff,0x82(a6)                  | +01c
        move.b  #0xff,0x20(a6)                  | +022
        lea     0x77fd6.l,a1                    | +028
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd22.l                       | +034
        lea     0x2edd92.l,a1                   | +03a
        jsr     0x43fac.l                       | +040
        lea     0x2ed176.l,a1                   | +046
        jsr     0x77c7e.l                       | +04c
        lea     0x2ed188.l,a1                   | +052
        jsr     0x77c7e.l                       | +058
        lea     0x2ec2ee.l,a0                   | +05e
        jsr     0x28cd4.l                       | +064
        lea     .L087e90(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L087e90:
        jsr     0x2783a.l                       | +070
        jsr     0x28d70.l                       | +076
        jsr     0x4fa70.l                       | +07c
        bcc.w   .L087eac                        | +082
        jmp     0x518.l                         | +086
.L087eac:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  TaskHandler_087eae  @ $087EAE  (158 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087eae, "ax", @progbits
        .global TaskHandler_087eae
TaskHandler_087eae:
        move.w  #0x88,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x0,0x38(a6)                   | +01c
        lea     0x2ec2fe.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        move.b  #0x0,0x83(a6)                   | +02e
        lea     0x77228.l,a1                    | +034
        jsr     0x4ae.l                         | +03a
        jsr     0x5dd22.l                       | +040
        addi.w  #0x58,0x22(a0)                  | +046
        move.b  #0x30,0x98(a0)                  | +04c
        move.b  #0x83,0x99(a0)                  | +052
        lea     .L087f0c(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L087f0c:
        jsr     0x2783a.l                       | +05e
        jsr     0x28d70.l                       | +064
        movea.l 0xc(a6),a0                      | +06a
        cmpi.b  #0xff,0x20(a0)                  | +06e
        bne.w   .L087f4a                        | +074
        move.b  #0xff,0x83(a6)                  | +078
        lea     0x77f6a.l,a1                    | +07e
        jsr     0x4ae.l                         | +084
        jsr     0x5dd22.l                       | +08a
        addi.w  #0x58,0x22(a0)                  | +090
        jmp     0x518.l                         | +096
.L087f4a:
        rts                                     | +09c

| ----------------------------------------------------------------------------
|  TaskHandler_087f4c  @ $087F4C  (112 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087f4c, "ax", @progbits
        .global TaskHandler_087f4c
TaskHandler_087f4c:
        move.w  #0xf3,d1                        | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        move.w  #0x7f,d0                        | +010
        jsr     0x5ea1c.l                       | +014
        btst    #0x0,d0                         | +01a
        beq.w   .L087f70                        | +01e
        neg.w   d0                              | +022
.L087f70:
        move.w  d0,0x28(a6)                     | +024
        andi.w  #0x7f,d0                        | +028
        sub.w   d0,0x2a(a6)                     | +02c
        lea     0x2de4b0.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     .L087f8e(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L087f8e:
        jsr     0x27cee.l                       | +042
        jsr     0x28d70.l                       | +048
        bcs.w   .L087fb4                        | +04e
        movea.l #0xffffffff,a0                  | +052
        lea     0x298736.l,a0                   | +058
        jsr     0x5dd56.l                       | +05e
        bcc.w   .L087fba                        | +064
.L087fb4:
        jmp     0x518.l                         | +068
.L087fba:
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  TaskHandler_087fbc  @ $087FBC  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_087fbc, "ax", @progbits
        .global TaskHandler_087fbc
TaskHandler_087fbc:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     TaskHandler_08800e(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd22.l                       | +014
        lea     TaskHandler_088028(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd22.l                       | +024
        lea     TaskHandler_088042(pc),a1       | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd22.l                       | +034
        lea     TaskHandler_08805c(pc),a1       | +03a
        jsr     0x4ae.l                         | +03e
        jsr     0x5dd22.l                       | +044
        jmp     0x518.l                         | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_08800c  @ $08800C  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08800c, "ax", @progbits
        .global TaskHandler_08800c
TaskHandler_08800c:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_08800e  @ $08800E  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08800e, "ax", @progbits
        .global TaskHandler_08800e
TaskHandler_08800e:
        lea     0x2ec30e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ed19a.l,a1                   | +00c
        move.l  a1,0x88(a6)                     | +012
        bra.w   TaskHandler_08805c__L088076     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_088028  @ $088028  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_088028, "ax", @progbits
        .global TaskHandler_088028
TaskHandler_088028:
        lea     0x2ec324.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ed1ac.l,a1                   | +00c
        move.l  a1,0x88(a6)                     | +012
        bra.w   TaskHandler_08805c__L088076     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_088042  @ $088042  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_088042, "ax", @progbits
        .global TaskHandler_088042
TaskHandler_088042:
        lea     0x2ec33a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ed1be.l,a1                   | +00c
        move.l  a1,0x88(a6)                     | +012
        bra.w   TaskHandler_08805c__L088076     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_08805c  @ $08805C  (184 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08805c, "ax", @progbits
        .global TaskHandler_08805c
TaskHandler_08805c:
        lea     0x2ec350.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ed1d0.l,a1                   | +00c
        move.l  a1,0x88(a6)                     | +012
        bra.w   TaskHandler_08805c__L088076     | +016
        .global TaskHandler_08805c__L088076
TaskHandler_08805c__L088076:
        move.w  #0xe4,d1                        | +01a
        jsr     0x236e.l                        | +01e
        move.b  #0xff,0x32(a6)                  | +024
        move.b  #0xff,0x33(a6)                  | +02a
        move.w  #0x40,0x70(a6)                  | +030
        move.w  #0xa,0x66(a6)                   | +036
        move.w  #0x8000,0x38(a6)                | +03c
        jsr     0x267e2.l                       | +042
        jsr     0x27cee.l                       | +048
        lea     .L0880b0(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L0880b0:
        jsr     0x2783a.l                       | +054
        jsr     0x28d70.l                       | +05a
        jsr     0x2870a.l                       | +060
        bcc.w   .L0880d8                        | +066
        lea     0x5e766.l,a0                    | +06a
        jsr     0x5e770.l                       | +070
        bclr    #0x3,0x13(a6)                   | +076
.L0880d8:
        jsr     0x28758.l                       | +07c
        bcc.w   .L088102                        | +082
        move.w  #0x102e,d0                      | +086
        jsr     0x2352.l                        | +08a
        lea     0xffff.w,a0                     | +090
        move.l  a0,0x48(a6)                     | +094
        movea.l 0x88(a6),a1                     | +098
        jsr     0x77c7e.l                       | +09c
        bra.w   .L08810c                        | +0a2
.L088102:
        jsr     0x4fa70.l                       | +0a6
        bcc.w   .L088112                        | +0ac
.L08810c:
        jmp     0x518.l                         | +0b0
.L088112:
        rts                                     | +0b6

| ----------------------------------------------------------------------------
|  TaskHandler_088114  @ $088114  (148 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_088114, "ax", @progbits
        .global TaskHandler_088114
TaskHandler_088114:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xe6,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0xff,0x32(a6)                  | +014
        move.b  #0xff,0x33(a6)                  | +01a
        move.w  #0x40,0x70(a6)                  | +020
        move.w  #0x32,0x66(a6)                  | +026
        move.w  #0x8000,0x38(a6)                | +02c
        jsr     0x267e2.l                       | +032
        jsr     0x27cee.l                       | +038
        lea     .L088158(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L088158:
        jsr     0x2783a.l                       | +044
        jsr     0x2870a.l                       | +04a
        bcc.w   .L08817a                        | +050
        lea     0x5e766.l,a0                    | +054
        jsr     0x5e770.l                       | +05a
        bclr    #0x3,0x13(a6)                   | +060
.L08817a:
        move.w  #0x32,0x66(a6)                  | +066
        bclr    #0x0,0x13(a6)                   | +06c
        cmpi.w  #0x50,0x22(a6)                  | +072
        bgt.w   .L088196                        | +078
        lea     TaskHandler_0881a8(pc),a1       | +07c
        move.l  a1,(a6)                         | +080
.L088196:
        jsr     0x4fa70.l                       | +082
        bcc.w   .L0881a6                        | +088
        jmp     0x518.l                         | +08c
.L0881a6:
        rts                                     | +092

| ----------------------------------------------------------------------------
|  TaskHandler_0881a8  @ $0881A8  (98 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0881a8, "ax", @progbits
        .global TaskHandler_0881a8
TaskHandler_0881a8:
        lea     0x2ec366.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0881ba(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0881ba:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L0881d0                        | +01e
        lea     TaskHandler_08820a(pc),a1       | +022
        move.l  a1,(a6)                         | +026
.L0881d0:
        jsr     0x2870a.l                       | +028
        bcc.w   .L0881ec                        | +02e
        lea     0x5e766.l,a0                    | +032
        jsr     0x5e770.l                       | +038
        bclr    #0x3,0x13(a6)                   | +03e
.L0881ec:
        move.w  #0x32,0x66(a6)                  | +044
        bclr    #0x0,0x13(a6)                   | +04a
        jsr     0x4fa70.l                       | +050
        bcc.w   .L088208                        | +056
        jmp     0x518.l                         | +05a
.L088208:
        rts                                     | +060

| ----------------------------------------------------------------------------
|  TaskHandler_08820a  @ $08820A  (102 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08820a, "ax", @progbits
        .global TaskHandler_08820a
TaskHandler_08820a:
        lea     0xee062.l,a1                    | +000
        move.w  #0x82,d0                        | +006
        move.b  #0x0,0x82(a6)                   | +00a
        jsr     0x4429e.l                       | +010
        lea     .L088226(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L088226:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        jsr     0x2870a.l                       | +028
        bcc.w   .L08824e                        | +02e
        lea     0x5e766.l,a0                    | +032
        jsr     0x5e770.l                       | +038
        bclr    #0x3,0x13(a6)                   | +03e
.L08824e:
        cmpi.w  #0x28,0x66(a6)                  | +044
        bgt.w   .L08825e                        | +04a
        lea     TaskHandler_088270(pc),a1       | +04e
        move.l  a1,(a6)                         | +052
.L08825e:
        jsr     0x4fa70.l                       | +054
        bcc.w   .L08826e                        | +05a
        jmp     0x518.l                         | +05e
.L08826e:
        rts                                     | +064

| ----------------------------------------------------------------------------
|  TaskHandler_088270  @ $088270  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_088270, "ax", @progbits
        .global TaskHandler_088270
TaskHandler_088270:
        lea     0x2ec3de.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L088282(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L088282:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L0882aa                        | +024
        lea     0x5e766.l,a0                    | +028
        jsr     0x5e770.l                       | +02e
        bclr    #0x3,0x13(a6)                   | +034
.L0882aa:
        cmpi.w  #0x1e,0x66(a6)                  | +03a
        bgt.w   .L0882ba                        | +040
        lea     TaskHandler_0882cc(pc),a1       | +044
        move.l  a1,(a6)                         | +048
.L0882ba:
        jsr     0x4fa70.l                       | +04a
        bcc.w   .L0882ca                        | +050
        jmp     0x518.l                         | +054
.L0882ca:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  TaskHandler_0882cc  @ $0882CC  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0882cc, "ax", @progbits
        .global TaskHandler_0882cc
TaskHandler_0882cc:
        lea     0x2ec3ee.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0882de(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0882de:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L088306                        | +024
        lea     0x5e766.l,a0                    | +028
        jsr     0x5e770.l                       | +02e
        bclr    #0x3,0x13(a6)                   | +034
.L088306:
        cmpi.w  #0x14,0x66(a6)                  | +03a
        bgt.w   .L088316                        | +040
        lea     TaskHandler_088328(pc),a1       | +044
        move.l  a1,(a6)                         | +048
.L088316:
        jsr     0x4fa70.l                       | +04a
        bcc.w   .L088326                        | +050
        jmp     0x518.l                         | +054
.L088326:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  TaskHandler_088328  @ $088328  (104 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_088328, "ax", @progbits
        .global TaskHandler_088328
TaskHandler_088328:
        lea     0x2ec3fe.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08833a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08833a:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L088362                        | +024
        lea     0x5e766.l,a0                    | +028
        jsr     0x5e770.l                       | +02e
        bclr    #0x3,0x13(a6)                   | +034
.L088362:
        cmpi.w  #0xa,0x66(a6)                   | +03a
        bgt.w   TaskHandler_088328__L08837e     | +040
        lea     0x2ed1e2.l,a1                   | +044
        jsr     0x77c7e.l                       | +04a
        lea     TaskHandler_088390(pc),a1       | +050
        move.l  a1,(a6)                         | +054
        .global TaskHandler_088328__L08837e
TaskHandler_088328__L08837e:
        jsr     0x4fa70.l                       | +056
        bcc.w   .L08838e                        | +05c
        jmp     0x518.l                         | +060
.L08838e:
        rts                                     | +066

| ----------------------------------------------------------------------------
|  TaskHandler_088390  @ $088390  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_088390, "ax", @progbits
        .global TaskHandler_088390
TaskHandler_088390:
        lea     0x2ec40e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0883a2(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0883a2:
        jsr     Entity_CmpPrioWithSibling_086552(pc) | +012
        bcs.b   TaskHandler_088328__L08837e     | +016
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        jsr     0x2870a.l                       | +024
        bcc.w   .L0883d0                        | +02a
        lea     0x5e766.l,a0                    | +02e
        jsr     0x5e770.l                       | +034
        bclr    #0x3,0x13(a6)                   | +03a
.L0883d0:
        jsr     0x28758.l                       | +040
        bra.w   .L0883e4                        | +046
        jsr     0x4fa70.l                       | +04a
        bcc.w   .L0883ea                        | +050
.L0883e4:
        jmp     0x518.l                         | +054
.L0883ea:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Sub_000883EC  @ $0883EC  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000883EC, "ax", @progbits
        .global Sub_000883EC
Sub_000883EC:
        move.b  0x106f28.l,d0                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +006
        ble.w   .L08841c                        | +00c
        andi.b  #0x3,d0                         | +010
        bne.w   SetTaskWRts_088436              | +014
        move.w  0x18(a6),d0                     | +018
        move.w  0x16(a6),0x18(a6)               | +01c
        move.w  d0,0x16(a6)                     | +022
        move.w  d0,0x14(a6)                     | +026
        subq.w  #0x1,0x72(a6)                   | +02a
        rts                                     | +02e
.L08841c:
        andi.b  #0xf,d0                         | +030
        bne.w   SetTaskWRts_088436              | +034
        move.w  0x18(a6),d0                     | +038
        move.w  0x16(a6),0x18(a6)               | +03c
        move.w  d0,0x16(a6)                     | +042

| ----------------------------------------------------------------------------
|  PcThunkTarget_088438  @ $088438  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_088438, "ax", @progbits
        .global PcThunkTarget_088438
PcThunkTarget_088438:
        addq.w  #0x1,0x72(a6)                   | +000
        move.w  0x72(a6),d0                     | +004
        andi.b  #0x3,d0                         | +008
        bne.w   .L088460                        | +00c
        clr.l   d0                              | +010
        move.b  0x21(a6),d0                     | +012
        asl.l   #0x2,d0                         | +016
        lea     0x2ec712.l,a0                   | +018
        movea.l (a0,d0.w),a0                    | +01e
        move.l  a0,0x48(a6)                     | +022
        rts                                     | +026
.L088460:
        lea     0xffff.w,a0                     | +028
        move.l  a0,0x48(a6)                     | +02c
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PcThunkTarget_08846a  @ $08846A  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_08846a, "ax", @progbits
        .global PcThunkTarget_08846a
PcThunkTarget_08846a:
        addq.w  #0x1,0x72(a6)                   | +000
        move.w  0x72(a6),d0                     | +004
        andi.b  #0x3,d0                         | +008
        bne.w   .L088482                        | +00c
        move.l  0x84(a6),0x48(a6)               | +010
        rts                                     | +016
.L088482:
        lea     0xffff.w,a0                     | +018
        move.l  a0,0x48(a6)                     | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  TaskHandler_08848c  @ $08848C  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08848c, "ax", @progbits
        .global TaskHandler_08848c
TaskHandler_08848c:
        move.w  0x80(a6),d0                     | +000
        move.w  0x66(a6),d1                     | +004
        sub.w   d1,d0                           | +008
        movea.l 0xc(a6),a0                      | +00a
        sub.w   d0,0x66(a0)                     | +00e
        move.w  0x66(a6),0x80(a6)               | +012
        rts                                     | +018

| ----------------------------------------------------------------------------
|  TaskHandler_0884a6  @ $0884A6  (270 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0884a6, "ax", @progbits
        .global TaskHandler_0884a6
TaskHandler_0884a6:
        lea     TaskHandler_08686a__L0868a6(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x28,0x22(a0)                  | +010
        addi.w  #0x50,0x24(a0)                  | +016
        move.w  #0x60,0x38(a0)                  | +01c
        lea     0x2ed738.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee066.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     TaskHandler_08686a(pc),a1       | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x68,0x22(a0)                  | +04c
        addi.w  #0x50,0x24(a0)                  | +052
        move.w  #0x60,0x38(a0)                  | +058
        lea     0x2ed74c.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee07c.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     TaskHandler_08686a__L0868a6(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0xa8,0x22(a0)                  | +088
        addi.w  #0x50,0x24(a0)                  | +08e
        move.w  #0x60,0x38(a0)                  | +094
        lea     0x2ed760.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee092.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     TaskHandler_086cd0(pc),a1       | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0x30,0x22(a0)                  | +0c4
        addi.w  #0x8,0x24(a0)                   | +0ca
        lea     0x2ed774.l,a1                   | +0d0
        move.l  a1,0x74(a0)                     | +0d6
        move.w  #0x3,0x72(a0)                   | +0da
        lea     TaskHandler_086cd0(pc),a1       | +0e0
        jsr     0x4ae.l                         | +0e4
        jsr     0x5dd22.l                       | +0ea
        addi.w  #0xa8,0x22(a0)                  | +0f0
        addi.w  #0x8,0x24(a0)                   | +0f6
        lea     0x2ed788.l,a1                   | +0fc
        move.l  a1,0x74(a0)                     | +102
        move.w  #0x4,0x72(a0)                   | +106
        rts                                     | +10c

| ----------------------------------------------------------------------------
|  TaskHandler_0885b4  @ $0885B4  (270 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0885b4, "ax", @progbits
        .global TaskHandler_0885b4
TaskHandler_0885b4:
        lea     TaskHandler_08686a__L0868a6(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x28,0x22(a0)                  | +010
        addi.w  #0x40,0x24(a0)                  | +016
        move.w  #0x70,0x38(a0)                  | +01c
        lea     0x2ed79c.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee0a8.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     TaskHandler_08686a(pc),a1       | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x68,0x22(a0)                  | +04c
        addi.w  #0x40,0x24(a0)                  | +052
        move.w  #0x70,0x38(a0)                  | +058
        lea     0x2ed7b0.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee0be.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     TaskHandler_08686a__L0868a6(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0xa8,0x22(a0)                  | +088
        addi.w  #0x40,0x24(a0)                  | +08e
        move.w  #0x70,0x38(a0)                  | +094
        lea     0x2ed7c4.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee0d4.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     TaskHandler_086cd0__L086ce4(pc),a1 | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0x28,0x22(a0)                  | +0c4
        addi.w  #0x0,0x24(a0)                   | +0ca
        lea     0x2ed7d8.l,a1                   | +0d0
        move.l  a1,0x74(a0)                     | +0d6
        move.w  #0x3,0x72(a0)                   | +0da
        lea     TaskHandler_086cd0__L086ce4(pc),a1 | +0e0
        jsr     0x4ae.l                         | +0e4
        jsr     0x5dd22.l                       | +0ea
        addi.w  #0xa0,0x22(a0)                  | +0f0
        addi.w  #0x0,0x24(a0)                   | +0f6
        lea     0x2ed7ec.l,a1                   | +0fc
        move.l  a1,0x74(a0)                     | +102
        move.w  #0x4,0x72(a0)                   | +106
        rts                                     | +10c

| ----------------------------------------------------------------------------
|  TaskHandler_0886c2  @ $0886C2  (270 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0886c2, "ax", @progbits
        .global TaskHandler_0886c2
TaskHandler_0886c2:
        lea     TaskHandler_08686a__L0868ba(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x28,0x22(a0)                  | +010
        addi.w  #0x50,0x24(a0)                  | +016
        move.w  #0x60,0x38(a0)                  | +01c
        lea     0x2ed800.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee0ea.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     TaskHandler_08686a__L0868ba(pc),a1 | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x68,0x22(a0)                  | +04c
        addi.w  #0x50,0x24(a0)                  | +052
        move.w  #0x60,0x38(a0)                  | +058
        lea     0x2ed814.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee100.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     TaskHandler_08686a__L0868ba(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0xa8,0x22(a0)                  | +088
        addi.w  #0x50,0x24(a0)                  | +08e
        move.w  #0x60,0x38(a0)                  | +094
        lea     0x2ed828.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee116.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     TaskHandler_086cd0__L086cf8(pc),a1 | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0x28,0x22(a0)                  | +0c4
        addi.w  #0x8,0x24(a0)                   | +0ca
        lea     0x2ed83c.l,a1                   | +0d0
        move.l  a1,0x74(a0)                     | +0d6
        move.w  #0x3,0x72(a0)                   | +0da
        lea     TaskHandler_086cd0__L086cf8(pc),a1 | +0e0
        jsr     0x4ae.l                         | +0e4
        jsr     0x5dd22.l                       | +0ea
        addi.w  #0xa0,0x22(a0)                  | +0f0
        addi.w  #0x8,0x24(a0)                   | +0f6
        lea     0x2ed850.l,a1                   | +0fc
        move.l  a1,0x74(a0)                     | +102
        move.w  #0x4,0x72(a0)                   | +106
        rts                                     | +10c

| ----------------------------------------------------------------------------
|  TaskHandler_0887d0  @ $0887D0  (330 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0887d0, "ax", @progbits
        .global TaskHandler_0887d0
TaskHandler_0887d0:
        lea     TaskHandler_08686a__L086892(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x18,0x22(a0)                  | +010
        addi.w  #0x40,0x24(a0)                  | +016
        move.w  #0x70,0x38(a0)                  | +01c
        lea     0x2ed88c.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee12c.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     TaskHandler_08686a__L0868a6(pc),a1 | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x48,0x22(a0)                  | +04c
        addi.w  #0x40,0x24(a0)                  | +052
        move.w  #0x70,0x38(a0)                  | +058
        lea     0x2ed8a0.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee130.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     TaskHandler_08686a__L0868a6(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0x78,0x22(a0)                  | +088
        addi.w  #0x40,0x24(a0)                  | +08e
        move.w  #0x70,0x38(a0)                  | +094
        lea     0x2ed8b4.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee146.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     TaskHandler_08686a__L0868a6(pc),a1 | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0xa8,0x22(a0)                  | +0c4
        addi.w  #0x40,0x24(a0)                  | +0ca
        move.w  #0x70,0x38(a0)                  | +0d0
        lea     0x2ed8c8.l,a1                   | +0d6
        move.l  a1,0x74(a0)                     | +0dc
        move.w  #0x3,0x72(a0)                   | +0e0
        lea     0xee15c.l,a1                    | +0e6
        move.l  a1,0x7c(a0)                     | +0ec
        lea     TaskHandler_086cd0__L086d0c(pc),a1 | +0f0
        jsr     0x4ae.l                         | +0f4
        jsr     0x5dd22.l                       | +0fa
        addi.w  #0x28,0x22(a0)                  | +100
        addi.w  #0x8,0x24(a0)                   | +106
        lea     0x2ed8dc.l,a1                   | +10c
        move.l  a1,0x74(a0)                     | +112
        move.w  #0x4,0x72(a0)                   | +116
        lea     TaskHandler_086cd0__L086d0c(pc),a1 | +11c
        jsr     0x4ae.l                         | +120
        jsr     0x5dd22.l                       | +126
        addi.w  #0xa0,0x22(a0)                  | +12c
        addi.w  #0x8,0x24(a0)                   | +132
        lea     0x2ed8f0.l,a1                   | +138
        move.l  a1,0x74(a0)                     | +13e
        move.w  #0x5,0x72(a0)                   | +142
        rts                                     | +148

| ----------------------------------------------------------------------------
|  TaskHandler_08891a  @ $08891A  (270 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08891a, "ax", @progbits
        .global TaskHandler_08891a
TaskHandler_08891a:
        lea     TaskHandler_08686a__L0868ba(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x28,0x22(a0)                  | +010
        addi.w  #0x50,0x24(a0)                  | +016
        move.w  #0x60,0x38(a0)                  | +01c
        lea     0x2ed904.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee172.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     TaskHandler_08686a__L0868ba(pc),a1 | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x68,0x22(a0)                  | +04c
        addi.w  #0x50,0x24(a0)                  | +052
        move.w  #0x60,0x38(a0)                  | +058
        lea     0x2ed918.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee188.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     TaskHandler_08686a__L0868ba(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0xa8,0x22(a0)                  | +088
        addi.w  #0x50,0x24(a0)                  | +08e
        move.w  #0x60,0x38(a0)                  | +094
        lea     0x2ed92c.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee19e.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     TaskHandler_086cd0__L086d20(pc),a1 | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0x28,0x22(a0)                  | +0c4
        addi.w  #0x0,0x24(a0)                   | +0ca
        lea     0x2ed940.l,a1                   | +0d0
        move.l  a1,0x74(a0)                     | +0d6
        move.w  #0x3,0x72(a0)                   | +0da
        lea     TaskHandler_086cd0__L086d20(pc),a1 | +0e0
        jsr     0x4ae.l                         | +0e4
        jsr     0x5dd22.l                       | +0ea
        addi.w  #0x98,0x22(a0)                  | +0f0
        addi.w  #0x0,0x24(a0)                   | +0f6
        lea     0x2ed954.l,a1                   | +0fc
        move.l  a1,0x74(a0)                     | +102
        move.w  #0x4,0x72(a0)                   | +106
        rts                                     | +10c

| ----------------------------------------------------------------------------
|  TaskHandler_088a28  @ $088A28  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_088a28, "ax", @progbits
        .global TaskHandler_088a28
TaskHandler_088a28:
        cmpi.b  #0x0,0x20(a6)                   | +000
        bne.w   Sub_00088A64                    | +006  -> $088A64 (hueco futuro, defsym forward)
        lea     0x2edaa8.l,a2                   | +00a
        jsr     0x5022a.l                       | +010
        lea     0x2edabc.l,a2                   | +016
        jsr     0x5022a.l                       | +01c
        lea     0x2edad0.l,a2                   | +022
        jsr     0x5022a.l                       | +028
