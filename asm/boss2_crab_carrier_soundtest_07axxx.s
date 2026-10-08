| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $07A002..$083000  (18,154 B, 192 entradas, 125 huecos)
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
|  Crew_Hostage_Idle_07a00a  @ $07A00A  (158 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hostage_Idle_07a00a, "ax", @progbits
        .global Crew_Hostage_Idle_07a00a
Crew_Hostage_Idle_07a00a:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x8000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0xc,0x38(a6)                   | +016
        addq.w  #0x1,0x38(a6)                   | +01c
        .global Crew_Hostage_Idle_07a00a__L07a02a
Crew_Hostage_Idle_07a00a__L07a02a:
.L07a02a:
        lea     0x29c36a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L07a03c(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L07a03c:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        movea.l 0xc(a6),a0                      | +03e
        cmpi.w  #0x2,0x72(a0)                   | +042
        bne.w   .L07a05c                        | +048
        lea     Crew_Hostage_ToggleFacing_07a0b0__L07a0b8(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L07a05c:
        movea.l 0xc(a6),a0                      | +052
        cmpi.b  #0xff,0x20(a0)                  | +056
        beq.w   .L07a074                        | +05c
        cmpi.b  #0xff,0x21(a0)                  | +060
        bne.w   .L07a082                        | +066
.L07a074:
        eori.b  #0x1,0x3a(a6)                   | +06a
        lea     0x58fc2.l,a1                    | +070
        move.l  a1,(a6)                         | +076
.L07a082:
        jsr     0x49fd0.l                       | +078
        btst    #0x3,0x13(a6)                   | +07e
        bne.w   Crew_Hostage_ToggleFacing_07a0b0 | +084
        movea.l #0xffffffff,a0                  | +088
        lea     0x2df594.l,a0                   | +08e
        jsr     0x5dd5c.l                       | +094
        bcc.w   SetHandlerRts_07a0ae            | +09a

| ----------------------------------------------------------------------------
|  Crew_Hostage_ToggleFacing_07a0b0  @ $07A0B0  (122 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hostage_ToggleFacing_07a0b0, "ax", @progbits
        .global Crew_Hostage_ToggleFacing_07a0b0
Crew_Hostage_ToggleFacing_07a0b0:
        eori.b  #0x1,0x3a(a6)                   | +000
        rts                                     | +006
        .global Crew_Hostage_ToggleFacing_07a0b0__L07a0b8
Crew_Hostage_ToggleFacing_07a0b0__L07a0b8:
.L07a0b8:
        lea     0x29c39e.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        lea     .L07a0ca(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L07a0ca:
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        bcc.w   .L07a0e0                        | +026
        lea     Crew_Hostage_Idle_07a00a__L07a02a(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L07a0e0:
        movea.l 0xc(a6),a0                      | +030
        cmpi.b  #0xff,0x20(a0)                  | +034
        beq.w   .L07a0f8                        | +03a
        cmpi.b  #0xff,0x21(a0)                  | +03e
        bne.w   .L07a106                        | +044
.L07a0f8:
        eori.b  #0x1,0x3a(a6)                   | +048
        lea     0x58fc2.l,a1                    | +04e
        move.l  a1,(a6)                         | +054
.L07a106:
        jsr     0x49fd0.l                       | +056
        btst    #0x3,0x13(a6)                   | +05c
        bne.b   Crew_Hostage_ToggleFacing_07a0b0 | +062
        movea.l #0xffffffff,a0                  | +064
        lea     0x2df594.l,a0                   | +06a
        jsr     0x5dd5c.l                       | +070
        bcc.w   SetHandlerRts_07a130            | +076

| ----------------------------------------------------------------------------
|  Crew_Hostage_Dying_07a132  @ $07A132  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hostage_Dying_07a132, "ax", @progbits
        .global Crew_Hostage_Dying_07a132
Crew_Hostage_Dying_07a132:
        move.w  #0x8,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x29c4da.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L07a14e(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07a14e:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        move.b  #0x1,0x44(a6)                   | +028
        movea.l 0xc(a6),a0                      | +02e
        tst.w   0x72(a0)                        | +032
        beq.w   .L07a170                        | +036
        clr.b   0x44(a6)                        | +03a
.L07a170:
        cmpi.b  #0xff,0x20(a0)                  | +03e
        bne.w   .L07a180                        | +044
        lea     Crew_Hostage_FadeOut_07a36e(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L07a180:
        movea.l #0xffffffff,a0                  | +04e
        lea     0x2df594.l,a0                   | +054
        jsr     0x5dd5c.l                       | +05a
        bcc.w   SetHandlerRts_07a19c            | +060

| ----------------------------------------------------------------------------
|  Crew_Hostage_Init_07a19e  @ $07A19E  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hostage_Init_07a19e, "ax", @progbits
        .global Crew_Hostage_Init_07a19e
Crew_Hostage_Init_07a19e:
        move.w  #0x2e,d1                        | +000
        jsr     0x236e.l                        | +004
        moveq   #0,d0                           | +00a
        move.b  d0,0x20(a6)                     | +00c
        move.b  d0,0x21(a6)                     | +010
        move.w  #0x1,0x66(a6)                   | +014

| ----------------------------------------------------------------------------
|  Crew_Captor_Idle_07a1c0  @ $07A1C0  (180 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Captor_Idle_07a1c0, "ax", @progbits
        .global Crew_Captor_Idle_07a1c0
Crew_Captor_Idle_07a1c0:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x8000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0xc,0x38(a6)                   | +016
        addq.w  #0x1,0x38(a6)                   | +01c
        lea     Crew_Hostage_Dying_07a132(pc),a1 | +020
        jsr     0x4ae.l                         | +024
        jsr     0x5dd02.l                       | +02a
        bra.w   .L07a20e                        | +030
        .global Crew_Captor_Idle_07a1c0__L07a1f4
Crew_Captor_Idle_07a1c0__L07a1f4:
.L07a1f4:
        movea.l 0xc(a6),a0                      | +034
        tst.w   0x70(a0)                        | +038
        beq.w   .L07a20e                        | +03c
        movea.l 0xc(a0),a0                      | +040
        movea.l 0xc(a0),a0                      | +044
        move.b  #0x2,0x21(a0)                   | +048
.L07a20e:
        lea     0x29c3c4.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
        lea     .L07a220(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L07a220:
        jsr     0x2783a.l                       | +060
        jsr     0x28d70.l                       | +066
        movea.l 0xc(a6),a0                      | +06c
        move.w  0x72(a0),d0                     | +070
        cmpi.w  #0x1,d0                         | +074
        beq.w   .L07a244                        | +078
        cmpi.w  #0x2,d0                         | +07c
        bne.w   .L07a24e                        | +080
.L07a244:
        move.w  d0,0x72(a6)                     | +084
        lea     Crew_Captor_Taunt_07a27c(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
.L07a24e:
        jsr     0x2870a.l                       | +08e
        bcc.w   .L07a25e                        | +094
        lea     Crew_Captor_Death_07a338(pc),a1 | +098
        move.l  a1,(a6)                         | +09c
.L07a25e:
        movea.l #0xffffffff,a0                  | +09e
        lea     0x2df59c.l,a0                   | +0a4
        jsr     0x5dd5c.l                       | +0aa
        bcc.w   SetHandlerRts_07a27a            | +0b0

| ----------------------------------------------------------------------------
|  Crew_Captor_Taunt_07a27c  @ $07A27C  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Captor_Taunt_07a27c, "ax", @progbits
        .global Crew_Captor_Taunt_07a27c
Crew_Captor_Taunt_07a27c:
        movea.l 0xc(a6),a0                      | +000
        tst.w   0x70(a0)                        | +004
        beq.w   .L07a296                        | +008
        movea.l 0xc(a0),a0                      | +00c
        movea.l 0xc(a0),a0                      | +010
        move.b  #0x1,0x21(a0)                   | +014
.L07a296:
        cmpi.w  #0x1,0x72(a6)                   | +01a
        beq.w   .L07a2b0                        | +020
        lea     0x29c3f8.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        bra.w   .L07a2bc                        | +030
.L07a2b0:
        lea     0x29c48c.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
.L07a2bc:
        lea     .L07a2c2(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L07a2c2:
        jsr     0x2783a.l                       | +046
        jsr     0x28d70.l                       | +04c
        bcc.w   .L07a2d8                        | +052
        lea     Crew_Captor_Idle_07a1c0__L07a1f4(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
.L07a2d8:
        jsr     0x2870a.l                       | +05c
        bcc.w   .L07a2e8                        | +062
        lea     Crew_Captor_Death_07a338(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L07a2e8:
        movea.l #0xffffffff,a0                  | +06c
        lea     0x2df59c.l,a0                   | +072
        jsr     0x5dd5c.l                       | +078
        bcc.w   SetHandlerRts_07a304            | +07e

| ----------------------------------------------------------------------------
|  Crew_Captor_ToHut_07a306  @ $07A306  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Captor_ToHut_07a306, "ax", @progbits
        .global Crew_Captor_ToHut_07a306
Crew_Captor_ToHut_07a306:
        eori.b  #0x1,0x3a(a6)                   | +000
        lea     0x29bd40.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L07a31e(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L07a31e:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L07a336                        | +024
        lea     0x5fa00.l,a1                    | +028
        move.l  a1,(a6)                         | +02e
.L07a336:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Crew_Captor_Death_07a338  @ $07A338  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Captor_Death_07a338, "ax", @progbits
        .global Crew_Captor_Death_07a338
Crew_Captor_Death_07a338:
        move.b  #0xff,0x20(a6)                  | +000
        movea.l 0xc(a6),a0                      | +006
        move.b  #0xff,0x21(a0)                  | +00a
        tst.w   0x70(a0)                        | +010
        beq.w   .L07a35e                        | +014
        movea.l 0xc(a0),a0                      | +018
        movea.l 0xc(a0),a0                      | +01c
        move.b  #0xff,0x80(a0)                  | +020
.L07a35e:
        move.w  #0x109f,d0                      | +026
        jsr     0x2352.l                        | +02a
        jmp     0x77f6a.l                       | +030

| ----------------------------------------------------------------------------
|  Crew_Hostage_FadeOut_07a36e  @ $07A36E  (78 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hostage_FadeOut_07a36e, "ax", @progbits
        .global Crew_Hostage_FadeOut_07a36e
Crew_Hostage_FadeOut_07a36e:
        clr.b   0x44(a6)                        | +000
        move.b  #0x14,0x59(a6)                  | +004
        move.w  #0x14,0x74(a6)                  | +00a
        lea     .L07a384(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L07a384:
        jsr     0x2783a.l                       | +016
        jsr     0x28d70.l                       | +01c
        movea.l #0xffffffff,a0                  | +022
        lea     0x2df594.l,a0                   | +028
        jsr     0x5dd5c.l                       | +02e
        bcc.w   .L07a3ac                        | +034
        lea     JmpToScheduler_07a3e6(pc),a1    | +038
        move.l  a1,(a6)                         | +03c
.L07a3ac:
        subq.w  #0x1,0x74(a6)                   | +03e
        bne.w   .L07a3ba                        | +042
        jmp     0x518.l                         | +046
.L07a3ba:
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  Crew_Hostage_WaitOffWorld_07a3bc  @ $07A3BC  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hostage_WaitOffWorld_07a3bc, "ax", @progbits
        .global Crew_Hostage_WaitOffWorld_07a3bc
Crew_Hostage_WaitOffWorld_07a3bc:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        movea.l #0xffffffff,a0                  | +00c
        lea     0x2df594.l,a0                   | +012
        jsr     0x5dd5c.l                       | +018
        bcc.w   SetHandlerRts_07a3e4            | +01e

| ----------------------------------------------------------------------------
|  Crew_Captor_MarkParentDead_07a3ee  @ $07A3EE  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Captor_MarkParentDead_07a3ee, "ax", @progbits
        .global Crew_Captor_MarkParentDead_07a3ee
Crew_Captor_MarkParentDead_07a3ee:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0xff,0x21(a0)                  | +004
        tst.w   0x70(a0)                        | +00a
        beq.w   Jsr5B6ThenJmpScheduler_07a40e   | +00e
        movea.l 0xc(a0),a0                      | +012
        movea.l 0xc(a0),a0                      | +016
        move.b  #0xff,0x80(a0)                  | +01a

| ----------------------------------------------------------------------------
|  Crew_Captor_PickRandCmd_07a41c  @ $07A41C  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Captor_PickRandCmd_07a41c, "ax", @progbits
        .global Crew_Captor_PickRandCmd_07a41c
Crew_Captor_PickRandCmd_07a41c:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x3,d0                         | +006
        add.w   d0,d0                           | +00a
        lea     0x2df4ea.l,a0                   | +00c
        movea.l 0xc(a6),a1                      | +012
        move.w  (a0,d0.w),0x72(a1)              | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Crew_RetGateD_07a43a  @ $07A43A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_RetGateD_07a43a, "ax", @progbits
        .global Crew_RetGateD_07a43a
Crew_RetGateD_07a43a:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_07a450                    | +00c

| ----------------------------------------------------------------------------
|  SoundTest_Init_07a456  @ $07A456  (160 B)
| ----------------------------------------------------------------------------
        .section .text.SoundTest_Init_07a456, "ax", @progbits
        .global SoundTest_Init_07a456
SoundTest_Init_07a456:
        movea.l #0x7072,a1                      | +000
        lea     SoundTest_Strings_07a7e8__L07a7fa(pc),a2 | +006
        move.w  #0x2300,d0                      | +00a
        jsr     0x5dad8.l                       | +00e
        movea.l #0x72d2,a1                      | +014
        lea     SoundTest_Strings_07a7e8__L07a88e(pc),a2 | +01a
        move.w  #0x2300,d0                      | +01e
        jsr     0x5dad8.l                       | +022
        move.w  #0x20,0x70(a6)                  | +028
        move.w  #0x20,0x72(a6)                  | +02e
        move.w  #0x20,0x74(a6)                  | +034
        move.w  #0x20,0x76(a6)                  | +03a
        move.w  #0x0,0x78(a6)                   | +040
        move.w  #0x0,0x7a(a6)                   | +046
        move.w  #0x0,0x7c(a6)                   | +04c
        move.w  #0x0,0x7e(a6)                   | +052
        move.w  #0x0,0x80(a6)                   | +058
        lea     0x433be.l,a1                    | +05e
        jsr     0x4ae.l                         | +064
        move.w  #0xf0,0x22(a0)                  | +06a
        move.w  #0x180,0x24(a0)                 | +070
        move.b  #0xff,0x98(a0)                  | +076
        lea     0x433be.l,a1                    | +07c
        jsr     0x4ae.l                         | +082
        move.w  #0x50,0x22(a0)                  | +088
        move.w  #0x180,0x24(a0)                 | +08e
        move.b  #0x0,0x98(a0)                   | +094
        move.l  #0x7a4f6,(a6)                   | +09a

| ----------------------------------------------------------------------------
|  SoundTest_Run_07a4f6  @ $07A4F6  (746 B)
| ----------------------------------------------------------------------------
        .section .text.SoundTest_Run_07a4f6, "ax", @progbits
        .global SoundTest_Run_07a4f6
SoundTest_Run_07a4f6:
        movea.l #0x71e8,a1                      | +000
        lea     SoundTest_Strings_07a7e8(pc),a2 | +006
        move.w  #0x2300,d0                      | +00a
        jsr     0x5dad8.l                       | +00e
        move.b  0x10e203.l,d0                   | +014
        btst    #0x6,d0                         | +01a
        beq.w   .L07a526                        | +01e
        move.b  #0x5,d0                         | +022
        jsr     0x2152.l                        | +026
        bra.w   .L07a614                        | +02c
.L07a526:
        btst    #0x7,d0                         | +030
        beq.w   .L07a53c                        | +034
        move.b  #0x6,d0                         | +038
        jsr     0x2152.l                        | +03c
        bra.w   .L07a614                        | +042
.L07a53c:
        btst    #0x0,d0                         | +046
        beq.w   .L07a562                        | +04a
        move.w  0x80(a6),d1                     | +04e
        add.w   d1,d1                           | +052
        addq.w  #0x1,0x70(a6,d1.w)              | +054
        cmpi.w  #0xff,0x70(a6,d1.w)             | +058
        ble.w   .L07a5e0                        | +05e
        move.w  #0x20,0x70(a6,d1.w)             | +062
        bra.w   .L07a5e0                        | +068
.L07a562:
        btst    #0x1,d0                         | +06c
        beq.w   .L07a588                        | +070
        move.w  0x80(a6),d1                     | +074
        add.w   d1,d1                           | +078
        subq.w  #0x1,0x70(a6,d1.w)              | +07a
        cmpi.w  #0x20,0x70(a6,d1.w)             | +07e
        bge.w   .L07a5e0                        | +084
        move.w  #0xff,0x70(a6,d1.w)             | +088
        bra.w   .L07a5e0                        | +08e
.L07a588:
        btst    #0x2,d0                         | +092
        beq.w   .L07a5b6                        | +096
        move.w  0x80(a6),d1                     | +09a
        add.w   d1,d1                           | +09e
        subi.w  #0x10,0x70(a6,d1.w)             | +0a0
        cmpi.w  #0x20,0x70(a6,d1.w)             | +0a6
        bge.w   .L07a5e0                        | +0ac
        andi.w  #0xf,0x70(a6,d1.w)              | +0b0
        addi.w  #0xf0,0x70(a6,d1.w)             | +0b6
        bra.w   .L07a5e0                        | +0bc
.L07a5b6:
        btst    #0x3,d0                         | +0c0
        beq.w   .L07a5e0                        | +0c4
        move.w  0x80(a6),d1                     | +0c8
        add.w   d1,d1                           | +0cc
        addi.w  #0x10,0x70(a6,d1.w)             | +0ce
        cmpi.w  #0xff,0x70(a6,d1.w)             | +0d4
        ble.w   .L07a5e0                        | +0da
        andi.w  #0xf,0x70(a6,d1.w)              | +0de
        addi.w  #0x20,0x70(a6,d1.w)             | +0e4
.L07a5e0:
        btst    #0x5,d0                         | +0ea
        beq.w   .L07a602                        | +0ee
        move.w  0x80(a6),d1                     | +0f2
        add.w   d1,d1                           | +0f6
        addq.w  #0x1,0x78(a6,d1.w)              | +0f8
        cmpi.w  #0x3,0x78(a6,d1.w)              | +0fc
        blt.w   .L07a602                        | +102
        move.w  #0x0,0x78(a6,d1.w)              | +106
.L07a602:
        btst    #0x4,d0                         | +10c
        beq.w   .L07a614                        | +110
        addq.w  #0x1,0x80(a6)                   | +114
        andi.w  #0x3,0x80(a6)                   | +118
.L07a614:
        move.b  0x10e209.l,d0                   | +11e
        btst    #0x0,d0                         | +124
        beq.w   .L07a630                        | +128
        subq.w  #0x1,0x80(a6)                   | +12c
        andi.w  #0x3,0x80(a6)                   | +130
        bra.w   .L07a6c4                        | +136
.L07a630:
        btst    #0x1,d0                         | +13a
        beq.w   .L07a646                        | +13e
        addq.w  #0x1,0x80(a6)                   | +142
        andi.w  #0x3,0x80(a6)                   | +146
        bra.w   .L07a6c4                        | +14c
.L07a646:
        move.b  0x10e20a.l,d0                   | +150
        btst    #0x3,d0                         | +156
        beq.w   .L07a672                        | +15a
        move.w  0x80(a6),d1                     | +15e
        add.w   d1,d1                           | +162
        addq.w  #0x1,0x70(a6,d1.w)              | +164
        cmpi.w  #0xff,0x70(a6,d1.w)             | +168
        ble.w   .L07a6c4                        | +16e
        move.w  #0x20,0x70(a6,d1.w)             | +172
        bra.w   .L07a6c4                        | +178
.L07a672:
        btst    #0x2,d0                         | +17c
        beq.w   .L07a698                        | +180
        move.w  0x80(a6),d1                     | +184
        add.w   d1,d1                           | +188
        subq.w  #0x1,0x70(a6,d1.w)              | +18a
        cmpi.w  #0x20,0x70(a6,d1.w)             | +18e
        bge.w   .L07a6c4                        | +194
        move.w  #0xff,0x70(a6,d1.w)             | +198
        bra.w   .L07a6c4                        | +19e
.L07a698:
        move.b  0x10e209.l,d0                   | +1a2
        btst    #0x4,d0                         | +1a8
        beq.w   .L07a6c4                        | +1ac
        move.l  d0,-(a7)                        | +1b0
        move.w  0x70(a6),d0                     | +1b2
        move.w  0x78(a6),d1                     | +1b6
        lsl.w   #0x1,d1                         | +1ba
        lea     SoundTest_Strings_07a7e8__L07a7f4(pc),a1 | +1bc
        move.w  (a1,d1.w),d1                    | +1c0
        add.w   d1,d0                           | +1c4
        jsr     0x219c.l                        | +1c6
        move.l  (a7)+,d0                        | +1cc
.L07a6c4:
        btst    #0x5,d0                         | +1ce
        beq.w   .L07a6ea                        | +1d2
        move.l  d0,-(a7)                        | +1d6
        move.w  0x72(a6),d0                     | +1d8
        move.w  0x7a(a6),d1                     | +1dc
        lsl.w   #0x1,d1                         | +1e0
        lea     SoundTest_Strings_07a7e8__L07a7f4(pc),a1 | +1e2
        move.w  (a1,d1.w),d1                    | +1e6
        add.w   d1,d0                           | +1ea
        jsr     0x219c.l                        | +1ec
        move.l  (a7)+,d0                        | +1f2
.L07a6ea:
        btst    #0x6,d0                         | +1f4
        beq.w   .L07a710                        | +1f8
        move.l  d0,-(a7)                        | +1fc
        move.w  0x74(a6),d0                     | +1fe
        move.w  0x7c(a6),d1                     | +202
        lsl.w   #0x1,d1                         | +206
        lea     SoundTest_Strings_07a7e8__L07a7f4(pc),a1 | +208
        move.w  (a1,d1.w),d1                    | +20c
        add.w   d1,d0                           | +210
        jsr     0x219c.l                        | +212
        move.l  (a7)+,d0                        | +218
.L07a710:
        btst    #0x7,d0                         | +21a
        beq.w   .L07a736                        | +21e
        move.l  d0,-(a7)                        | +222
        move.w  0x76(a6),d0                     | +224
        move.w  0x7e(a6),d1                     | +228
        lsl.w   #0x1,d1                         | +22c
        lea     SoundTest_Strings_07a7e8__L07a7f4(pc),a1 | +22e
        move.w  (a1,d1.w),d1                    | +232
        add.w   d1,d0                           | +236
        jsr     0x219c.l                        | +238
        move.l  (a7)+,d0                        | +23e
.L07a736:
        move.w  0x70(a6),d0                     | +240
        move.w  0x78(a6),d1                     | +244
        lsl.w   #0x1,d1                         | +248
        lea     SoundTest_Strings_07a7e8__L07a7f4(pc),a1 | +24a
        move.w  (a1,d1.w),d1                    | +24e
        add.w   d1,d0                           | +252
        movea.l #0x724a,a1                      | +254
        move.w  #0x2300,d1                      | +25a
        jsr     0x5d6c2.l                       | +25e
        move.w  0x72(a6),d0                     | +264
        move.w  0x7a(a6),d1                     | +268
        lsl.w   #0x1,d1                         | +26c
        lea     SoundTest_Strings_07a7e8__L07a7f4(pc),a1 | +26e
        move.w  (a1,d1.w),d1                    | +272
        add.w   d1,d0                           | +276
        movea.l #0x724c,a1                      | +278
        move.w  #0x2300,d1                      | +27e
        jsr     0x5d6c2.l                       | +282
        move.w  0x74(a6),d0                     | +288
        move.w  0x7c(a6),d1                     | +28c
        lsl.w   #0x1,d1                         | +290
        lea     SoundTest_Strings_07a7e8__L07a7f4(pc),a1 | +292
        move.w  (a1,d1.w),d1                    | +296
        add.w   d1,d0                           | +29a
        movea.l #0x724e,a1                      | +29c
        move.w  #0x2300,d1                      | +2a2
        jsr     0x5d6c2.l                       | +2a6
        move.w  0x76(a6),d0                     | +2ac
        move.w  0x7e(a6),d1                     | +2b0
        lsl.w   #0x1,d1                         | +2b4
        lea     SoundTest_Strings_07a7e8__L07a7f4(pc),a1 | +2b6
        move.w  (a1,d1.w),d1                    | +2ba
        add.w   d1,d0                           | +2be
        movea.l #0x7250,a1                      | +2c0
        move.w  #0x2300,d1                      | +2c6
        jsr     0x5d6c2.l                       | +2ca
        lea     SoundTest_Strings_07a7e8__L07a914(pc),a0 | +2d0
        move.w  0x80(a6),d0                     | +2d4
        add.w   d0,d0                           | +2d8
        add.w   d0,d0                           | +2da
        movea.l (a0,d0.w),a2                    | +2dc
        movea.l #0x720a,a1                      | +2e0
        move.w  #0x2300,d0                      | +2e6

| ----------------------------------------------------------------------------
|  SoundTest_Strings_07a7e8  @ $07A7E8  (364 B)
| ----------------------------------------------------------------------------
        .section .text.SoundTest_Strings_07a7e8, "ax", @progbits
        .global SoundTest_Strings_07a7e8
SoundTest_Strings_07a7e8:
        .dc.w   0x534f                        | +000  (dato / opcode no decodificado)
        .dc.w   0x554e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x4420                        | +004  (dato / opcode no decodificado)
        .dc.w   0x5445                        | +006  (dato / opcode no decodificado)
        .dc.w   0x5354                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +00a  (dato / opcode no decodificado)
        .global SoundTest_Strings_07a7e8__L07a7f4
SoundTest_Strings_07a7e8__L07a7f4:
.L07a7f4:
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x1000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x1100                        | +010  (dato / opcode no decodificado)
        .global SoundTest_Strings_07a7e8__L07a7fa
SoundTest_Strings_07a7e8__L07a7fa:
.L07a7fa:
        .dc.w   0x3150                        | +012  (dato / opcode no decodificado)
        .dc.w   0x204a                        | +014  (dato / opcode no decodificado)
        .dc.w   0x4f59                        | +016  (dato / opcode no decodificado)
        .dc.w   0x5354                        | +018  (dato / opcode no decodificado)
        .dc.w   0x4943                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x4bfd                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xfd55                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x502d                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2d2d                        | +022  (dato / opcode no decodificado)
        .dc.w   0x2d2d                        | +024  (dato / opcode no decodificado)
        .dc.w   0x434f                        | +026  (dato / opcode no decodificado)
        .dc.w   0x4445                        | +028  (dato / opcode no decodificado)
        .dc.w   0x202b                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x3031                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x68fd                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x444f                        | +030  (dato / opcode no decodificado)
        .dc.w   0x574e                        | +032  (dato / opcode no decodificado)
        .dc.w   0x2d2d                        | +034  (dato / opcode no decodificado)
        .dc.w   0x2d43                        | +036  (dato / opcode no decodificado)
        .dc.w   0x4f44                        | +038  (dato / opcode no decodificado)
        .dc.w   0x4520                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x2d30                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x3168                        | +03e  (dato / opcode no decodificado)
        .dc.w   0xfd4c                        | +040  (dato / opcode no decodificado)
        .dc.w   0x4546                        | +042  (dato / opcode no decodificado)
        .dc.w   0x542d                        | +044  (dato / opcode no decodificado)
        .dc.w   0x2d2d                        | +046  (dato / opcode no decodificado)
        .dc.w   0x434f                        | +048  (dato / opcode no decodificado)
        .dc.w   0x4445                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x202d                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x3130                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x68fd                        | +050  (dato / opcode no decodificado)
        .dc.w   0x5249                        | +052  (dato / opcode no decodificado)
        .dc.w   0x4748                        | +054  (dato / opcode no decodificado)
        .dc.w   0x542d                        | +056  (dato / opcode no decodificado)
        .dc.w   0x2d43                        | +058  (dato / opcode no decodificado)
        .dc.w   0x4f44                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x4520                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x2b31                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x3068                        | +060  (dato / opcode no decodificado)
        .dc.w   0xfd50                        | +062  (dato / opcode no decodificado)
        .dc.w   0x5553                        | +064  (dato / opcode no decodificado)
        .dc.w   0x4820                        | +066  (dato / opcode no decodificado)
        .dc.w   0x412d                        | +068  (dato / opcode no decodificado)
        .dc.w   0x4353                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x4c20                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x4348                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x414e                        | +070  (dato / opcode no decodificado)
        .dc.w   0x4745                        | +072  (dato / opcode no decodificado)
        .dc.w   0xfd50                        | +074  (dato / opcode no decodificado)
        .dc.w   0x5553                        | +076  (dato / opcode no decodificado)
        .dc.w   0x4820                        | +078  (dato / opcode no decodificado)
        .dc.w   0x422d                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x4241                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x4e4b                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x2043                        | +080  (dato / opcode no decodificado)
        .dc.w   0x4841                        | +082  (dato / opcode no decodificado)
        .dc.w   0x4e47                        | +084  (dato / opcode no decodificado)
        .dc.w   0x45fd                        | +086  (dato / opcode no decodificado)
        .dc.w   0x5055                        | +088  (dato / opcode no decodificado)
        .dc.w   0x5348                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x2043                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x2d42                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x474d                        | +090  (dato / opcode no decodificado)
        .dc.w   0x204f                        | +092  (dato / opcode no decodificado)
        .dc.w   0x4646                        | +094  (dato / opcode no decodificado)
        .dc.w   0xfd50                        | +096  (dato / opcode no decodificado)
        .dc.w   0x5553                        | +098  (dato / opcode no decodificado)
        .dc.w   0x4820                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x442d                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x5345                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x204f                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x4646                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +0a4  (dato / opcode no decodificado)
        .global SoundTest_Strings_07a7e8__L07a88e
SoundTest_Strings_07a7e8__L07a88e:
.L07a88e:
        .dc.w   0x3250                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x204a                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x4f59                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x5354                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x4943                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x4bfd                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0xfd55                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x502d                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x2d2d                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x2d2d                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x4353                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x4c20                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x5550                        | +0be  (dato / opcode no decodificado)
        .dc.w   0xfd44                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x4f57                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x4e2d                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x2d2d                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x4353                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x4c20                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x444f                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x574e                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0xfd4c                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x4546                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x542d                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x2d2d                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x434f                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x4445                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x202d                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x3031                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x68fd                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x5249                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x4748                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x542d                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x2d43                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x4f44                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x4520                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x2b30                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x3168                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0xfd50                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x5553                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x4820                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x412d                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x4348                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x3020                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x4f4e                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0xfd50                        | +100  (dato / opcode no decodificado)
        .dc.w   0x5553                        | +102  (dato / opcode no decodificado)
        .dc.w   0x4820                        | +104  (dato / opcode no decodificado)
        .dc.w   0x422d                        | +106  (dato / opcode no decodificado)
        .dc.w   0x4348                        | +108  (dato / opcode no decodificado)
        .dc.w   0x3120                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x4f4e                        | +10c  (dato / opcode no decodificado)
        .dc.w   0xfd50                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x5553                        | +110  (dato / opcode no decodificado)
        .dc.w   0x4820                        | +112  (dato / opcode no decodificado)
        .dc.w   0x432d                        | +114  (dato / opcode no decodificado)
        .dc.w   0x4348                        | +116  (dato / opcode no decodificado)
        .dc.w   0x3220                        | +118  (dato / opcode no decodificado)
        .dc.w   0x4f4e                        | +11a  (dato / opcode no decodificado)
        .dc.w   0xfd50                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x5553                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x4820                        | +120  (dato / opcode no decodificado)
        .dc.w   0x442d                        | +122  (dato / opcode no decodificado)
        .dc.w   0x4348                        | +124  (dato / opcode no decodificado)
        .dc.w   0x3320                        | +126  (dato / opcode no decodificado)
        .dc.w   0x4f4e                        | +128  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +12a  (dato / opcode no decodificado)
        .global SoundTest_Strings_07a7e8__L07a914
SoundTest_Strings_07a7e8__L07a914:
.L07a914:
        .dc.w   0x0007                        | +12c  (dato / opcode no decodificado)
        .dc.w   0xa924                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +130  (dato / opcode no decodificado)
        .dc.w   0xa930                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +134  (dato / opcode no decodificado)
        .dc.w   0xa93c                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +138  (dato / opcode no decodificado)
        .dc.w   0xa948                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x3efd                        | +13c  (dato / opcode no decodificado)
        .dc.w   0xfd20                        | +13e  (dato / opcode no decodificado)
        .dc.w   0xfdfd                        | +140  (dato / opcode no decodificado)
        .dc.w   0x20fd                        | +142  (dato / opcode no decodificado)
        .dc.w   0xfd20                        | +144  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +146  (dato / opcode no decodificado)
        .dc.w   0x20fd                        | +148  (dato / opcode no decodificado)
        .dc.w   0xfd3e                        | +14a  (dato / opcode no decodificado)
        .dc.w   0xfdfd                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x20fd                        | +14e  (dato / opcode no decodificado)
        .dc.w   0xfd20                        | +150  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +152  (dato / opcode no decodificado)
        .dc.w   0x20fd                        | +154  (dato / opcode no decodificado)
        .dc.w   0xfd20                        | +156  (dato / opcode no decodificado)
        .dc.w   0xfdfd                        | +158  (dato / opcode no decodificado)
        .dc.w   0x3efd                        | +15a  (dato / opcode no decodificado)
        .dc.w   0xfd20                        | +15c  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x20fd                        | +160  (dato / opcode no decodificado)
        .dc.w   0xfd20                        | +162  (dato / opcode no decodificado)
        .dc.w   0xfdfd                        | +164  (dato / opcode no decodificado)
        .dc.w   0x20fd                        | +166  (dato / opcode no decodificado)
        .dc.w   0xfd3e                        | +168  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +16a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Banner_RetGate_07a954  @ $07A954  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Banner_RetGate_07a954, "ax", @progbits
        .global Banner_RetGate_07a954
Banner_RetGate_07a954:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_07a96a                    | +00c

| ----------------------------------------------------------------------------
|  Banner_LetterTimingFromDist_07ba2e  @ $07BA2E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Banner_LetterTimingFromDist_07ba2e, "ax", @progbits
        .global Banner_LetterTimingFromDist_07ba2e
Banner_LetterTimingFromDist_07ba2e:
        move.w  0x70(a6),d2                     | +000
        move.w  0x72(a6),d3                     | +004
        move.w  0x22(a6),d0                     | +008
        move.w  0x24(a6),d1                     | +00c
        jsr     0x5e23a.l                       | +010
        asr.w   #0x4,d0                         | +016
        clr.w   d1                              | +018
        move.b  0x32(a6),d1                     | +01a
        sub.w   d0,d1                           | +01e
        move.b  d1,0x32(a6)                     | +020
        clr.w   d1                              | +024
        move.b  0x33(a6),d1                     | +026
        sub.w   d0,d1                           | +02a
        move.b  d1,0x33(a6)                     | +02c
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Boss2Intro_RetGate_07ba60  @ $07BA60  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Boss2Intro_RetGate_07ba60, "ax", @progbits
        .global Boss2Intro_RetGate_07ba60
Boss2Intro_RetGate_07ba60:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_07ba76                    | +00c

| ----------------------------------------------------------------------------
|  M2Boss_Tmpl132_07ba7c  @ $07BA7C  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Tmpl132_07ba7c, "ax", @progbits
        .global M2Boss_Tmpl132_07ba7c
M2Boss_Tmpl132_07ba7c:
        move.w  #0x34,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xa90,d0                       | +00a
        move.l  #0x100,d1                       | +00e
        jsr     0x96a5a.l                       | +014
        subq.w  #0x1,d0                         | +01a
        move.w  d0,0x22(a6)                     | +01c
        move.w  d1,0x24(a6)                     | +020
        move.w  #0x0,d0                         | +024
        jsr     0x28134.l                       | +028
        andi.w  #0xffe3,0x38(a6)                | +02e
        ori.w   #0x1c,0x38(a6)                  | +034
        move.b  #0x1,0x20(a6)                   | +03a
        clr.b   0x21(a6)                        | +040
        jsr     0x267e2.l                       | +044

| ----------------------------------------------------------------------------
|  M2Boss_WaitScroll_07bace  @ $07BACE  (68 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_WaitScroll_07bace, "ax", @progbits
        .global M2Boss_WaitScroll_07bace
M2Boss_WaitScroll_07bace:
        lea     M2Boss_Body_SpawnTurretAndArm_07bc8e(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        lea     0x2e0148.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L07baf0(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L07baf0:
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        move.w  0x2bcf52.l,d0                   | +02e
        cmp.w   0x106f50.l,d0                   | +034
        bgt.w   SetHandlerRts_07bb18            | +03a
        clr.b   0x10e39a.l                      | +03e

| ----------------------------------------------------------------------------
|  M2Boss_IntroAnim_07bb1a  @ $07BB1A  (50 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_IntroAnim_07bb1a, "ax", @progbits
        .global M2Boss_IntroAnim_07bb1a
M2Boss_IntroAnim_07bb1a:
        move.w  #0x28,d0                        | +000
        jsr     0x2352.l                        | +004
        lea     0x2e015a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L07bb36(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07bb36:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   ClrRamWord_07bb4c               | +028
        lea     M2Boss_WaitDeath_07bb54(pc),a1  | +02c
        move.l  a1,(a6)                         | +030

| ----------------------------------------------------------------------------
|  M2Boss_WaitDeath_07bb54  @ $07BB54  (26 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_WaitDeath_07bb54, "ax", @progbits
        .global M2Boss_WaitDeath_07bb54
M2Boss_WaitDeath_07bb54:
        jsr     0x2783a.l                       | +000
        tst.b   0x21(a6)                        | +006
        beq.w   ClrRamWord_07bb70               | +00a
        bclr    #0x1,0x12(a6)                   | +00e
        jmp     0x518.l                         | +014

| ----------------------------------------------------------------------------
|  M2Boss_Rts_07bb6e  @ $07BB6E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Rts_07bb6e, "ax", @progbits
        .global M2Boss_Rts_07bb6e
M2Boss_Rts_07bb6e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  M2Boss_Body_Init_07bb78  @ $07BB78  (270 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Body_Init_07bb78, "ax", @progbits
        .global M2Boss_Body_Init_07bb78
M2Boss_Body_Init_07bb78:
        move.w  #0x2a,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        move.w  #0x0,d0                         | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x8,0x38(a6)                   | +026
        move.w  #0x1,0x66(a6)                   | +02c
        jsr     0x267e2.l                       | +032
        clr.b   0x21(a6)                        | +038
        move.w  #0xffff,0x7c(a6)                | +03c
        lea     .L07bbc0(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L07bbc0:
        movea.l 0xc(a6),a0                      | +048
        move.w  0x22(a0),0x22(a6)               | +04c
        move.w  0x24(a0),0x24(a6)               | +052
        movea.l 0xc(a6),a0                      | +058
        move.w  0x7c(a0),d0                     | +05c
        cmp.w   0x7c(a6),d0                     | +060
        beq.w   .L07bc00                        | +064
        move.l  d0,-(a7)                        | +068
        movea.l #0x2df7d6,a0                    | +06a
        lsl.w   #0x2,d0                         | +070
        movea.l (a0,d0.w),a0                    | +072
        cmpa.l  #0xffffffff,a0                  | +076
        beq.w   .L07bbfe                        | +07c
        jsr     0x28cd4.l                       | +080
.L07bbfe:
        move.l  (a7)+,d0                        | +086
.L07bc00:
        move.w  d0,0x7c(a6)                     | +088
        movea.l 0xc(a6),a0                      | +08c
        btst    #0x7,0x5a(a0)                   | +090
        beq.w   .L07bc18                        | +096
        bset    #0x0,0x5a(a6)                   | +09a
.L07bc18:
        jsr     0x28d70.l                       | +0a0
        movea.l 0xc(a6),a0                      | +0a6
        cmpi.b  #0xff,0x20(a0)                  | +0aa
        bne.w   .L07bc78                        | +0b0
        cmpi.w  #0x140,0x22(a6)                 | +0b4
        ble.w   .L07bc78                        | +0ba
        tst.b   0x21(a6)                        | +0be
        bne.w   .L07bc78                        | +0c2
        move.b  #0x1,0x10a2d1.l                 | +0c6
        jsr     0x434ce.l                       | +0ce
        move.b  #0xff,0x21(a6)                  | +0d4
        lea     0x2df7bc.l,a1                   | +0da
        jsr     0x77c7e.l                       | +0e0
        move.w  #0xd000,0x38(a0)                | +0e6
        move.w  0x22(a6),-(a7)                  | +0ec
        subi.w  #0x20,0x22(a6)                  | +0f0
        jsr     0x628ea.l                       | +0f6
        move.w  (a7)+,0x22(a6)                  | +0fc
.L07bc78:
        movea.l 0xc(a6),a0                      | +100
        cmpi.b  #0xff,0x21(a0)                  | +104
        bne.w   SetHandlerRts_07bc8c            | +10a

| ----------------------------------------------------------------------------
|  M2Boss_Body_SpawnTurretAndArm_07bc8e  @ $07BC8E  (140 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Body_SpawnTurretAndArm_07bc8e, "ax", @progbits
        .global M2Boss_Body_SpawnTurretAndArm_07bc8e
M2Boss_Body_SpawnTurretAndArm_07bc8e:
        moveq   #0,d0                           | +000
        move.b  d0,0x20(a6)                     | +002
        move.b  d0,0x21(a6)                     | +006
        move.w  d0,0x70(a6)                     | +00a
        move.w  d0,0x72(a6)                     | +00e
        move.w  d0,0x74(a6)                     | +012
        move.w  d0,0x76(a6)                     | +016
        move.w  d0,0x78(a6)                     | +01a
        move.w  d0,0x7a(a6)                     | +01e
        lea     M2Boss_Body_Init_07bb78(pc),a1  | +022
        jsr     0x4ae.l                         | +026
        jsr     0x5dd02.l                       | +02c
        lea     M2Boss_Turret_Init_07bf36(pc),a1 | +032
        jsr     0x4ae.l                         | +036
        jsr     0x5dd02.l                       | +03c
        .global M2Boss_Body_SpawnTurretAndArm_07bc8e__L07bcd0
M2Boss_Body_SpawnTurretAndArm_07bc8e__L07bcd0:
.L07bcd0:
        move.w  #0x0,0x7c(a6)                   | +042
        lea     .L07bcdc(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L07bcdc:
        jsr     M2Boss_Arm_PosFromAngle_07bef4(pc) | +04e
        cmpi.w  #0xffff,0x70(a6)                | +052
        bne.w   .L07bcf0                        | +058
        lea     M2Boss_Arm_SwingLeft_07bd22(pc),a1 | +05c
        move.l  a1,(a6)                         | +060
.L07bcf0:
        cmpi.w  #0x1,0x70(a6)                   | +062
        bne.w   .L07bd00                        | +068
        lea     M2Boss_Arm_SwingRight_07bdcc(pc),a1 | +06c
        move.l  a1,(a6)                         | +070
.L07bd00:
        cmpi.w  #0x2,0x70(a6)                   | +072
        bne.w   .L07bd10                        | +078
        lea     M2Boss_Arm_SwingFar_07be7c(pc),a1 | +07c
        move.l  a1,(a6)                         | +080
.L07bd10:
        cmpi.b  #0xff,0x20(a6)                  | +082
        bne.w   SetHandlerRts_07bd20            | +088

| ----------------------------------------------------------------------------
|  M2Boss_Arm_SwingLeft_07bd22  @ $07BD22  (162 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Arm_SwingLeft_07bd22, "ax", @progbits
        .global M2Boss_Arm_SwingLeft_07bd22
M2Boss_Arm_SwingLeft_07bd22:
        move.w  #0x1,0x7c(a6)                   | +000
        lea     .L07bd2e(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L07bd2e:
        cmpi.w  #0xffff,0x70(a6)                | +00c
        beq.w   .L07bd58                        | +012
        tst.w   0x78(a6)                        | +016
        bne.w   .L07bd5c                        | +01a
        subi.w  #0xa0,0x74(a6)                  | +01e
        bpl.w   .L07bd72                        | +024
        clr.w   0x74(a6)                        | +028
        lea     M2Boss_Body_SpawnTurretAndArm_07bc8e__L07bcd0(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
        bra.w   .L07bd72                        | +032
.L07bd58:
        clr.w   0x78(a6)                        | +036
.L07bd5c:
        addi.w  #0x40,0x74(a6)                  | +03a
        cmpi.w  #0x180,0x74(a6)                 | +040
        blt.w   .L07bd72                        | +046
        move.w  #0x180,0x74(a6)                 | +04a
.L07bd72:
        move.w  0x74(a6),d0                     | +050
        sub.w   d0,0x72(a6)                     | +054
        bpl.w   .L07bdac                        | +058
        move.w  #0x0,0x72(a6)                   | +05c
        addq.w  #0x1,0x76(a6)                   | +062
        cmpi.w  #0x14,0x76(a6)                  | +066
        blt.w   .L07bdb0                        | +06c
        moveq   #0,d0                           | +070
        move.w  d0,0x74(a6)                     | +072
        move.w  d0,0x76(a6)                     | +076
        move.w  #0xffff,0x78(a6)                | +07a
        lea     M2Boss_Arm_SwingRight_07bdcc(pc),a1 | +080
        move.l  a1,(a6)                         | +084
        bra.w   .L07bdb0                        | +086
.L07bdac:
        clr.w   0x76(a6)                        | +08a
.L07bdb0:
        jsr     M2Boss_Arm_PosFromAngle_07bef4(pc) | +08e
        jsr     0x28d70.l                       | +092
        cmpi.b  #0xff,0x20(a6)                  | +098
        bne.w   SetHandlerRts_07bdca            | +09e

| ----------------------------------------------------------------------------
|  M2Boss_Arm_SwingRight_07bdcc  @ $07BDCC  (168 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Arm_SwingRight_07bdcc, "ax", @progbits
        .global M2Boss_Arm_SwingRight_07bdcc
M2Boss_Arm_SwingRight_07bdcc:
        move.w  #0x2,0x7c(a6)                   | +000
        lea     .L07bdd8(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L07bdd8:
        cmpi.w  #0x1,0x70(a6)                   | +00c
        beq.w   .L07be02                        | +012
        tst.w   0x78(a6)                        | +016
        bne.w   .L07be06                        | +01a
        subi.w  #0xa0,0x74(a6)                  | +01e
        bpl.w   .L07be1c                        | +024
        clr.w   0x74(a6)                        | +028
        lea     M2Boss_Body_SpawnTurretAndArm_07bc8e__L07bcd0(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
        bra.w   .L07be1c                        | +032
.L07be02:
        clr.w   0x78(a6)                        | +036
.L07be06:
        addi.w  #0x40,0x74(a6)                  | +03a
        cmpi.w  #0x180,0x74(a6)                 | +040
        blt.w   .L07be1c                        | +046
        move.w  #0x180,0x74(a6)                 | +04a
.L07be1c:
        move.w  0x74(a6),d0                     | +050
        add.w   d0,0x72(a6)                     | +054
        cmpi.w  #0x7e00,0x72(a6)                | +058
        ble.w   .L07be5c                        | +05e
        move.w  #0x7e00,0x72(a6)                | +062
        addq.w  #0x1,0x76(a6)                   | +068
        cmpi.w  #0x14,0x76(a6)                  | +06c
        blt.w   .L07be60                        | +072
        moveq   #0,d0                           | +076
        move.w  d0,0x74(a6)                     | +078
        move.w  d0,0x76(a6)                     | +07c
        move.w  #0xffff,0x78(a6)                | +080
        lea     M2Boss_Arm_SwingLeft_07bd22(pc),a1 | +086
        move.l  a1,(a6)                         | +08a
        bra.w   .L07be60                        | +08c
.L07be5c:
        clr.w   0x76(a6)                        | +090
.L07be60:
        jsr     M2Boss_Arm_PosFromAngle_07bef4(pc) | +094
        jsr     0x28d70.l                       | +098
        cmpi.b  #0xff,0x20(a6)                  | +09e
        bne.w   SetHandlerRts_07be7a            | +0a4

| ----------------------------------------------------------------------------
|  M2Boss_Arm_SwingFar_07be7c  @ $07BE7C  (112 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Arm_SwingFar_07be7c, "ax", @progbits
        .global M2Boss_Arm_SwingFar_07be7c
M2Boss_Arm_SwingFar_07be7c:
        move.w  #0x2,0x7c(a6)                   | +000
        lea     .L07be88(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L07be88:
        cmpi.w  #0x2,0x70(a6)                   | +00c
        beq.w   .L07beaa                        | +012
        subi.w  #0xa0,0x74(a6)                  | +016
        bpl.w   .L07bec0                        | +01c
        clr.w   0x74(a6)                        | +020
        lea     M2Boss_Body_SpawnTurretAndArm_07bc8e__L07bcd0(pc),a1 | +024
        move.l  a1,(a6)                         | +028
        bra.w   .L07bec0                        | +02a
.L07beaa:
        addi.w  #0x40,0x74(a6)                  | +02e
        cmpi.w  #0x140,0x74(a6)                 | +034
        blt.w   .L07bec0                        | +03a
        move.w  #0x140,0x74(a6)                 | +03e
.L07bec0:
        move.w  0x74(a6),d0                     | +044
        add.w   d0,0x72(a6)                     | +048
        cmpi.w  #0x7e00,0x72(a6)                | +04c
        ble.w   .L07bed8                        | +052
        move.w  #0x7e00,0x72(a6)                | +056
.L07bed8:
        jsr     M2Boss_Arm_PosFromAngle_07bef4(pc) | +05c
        jsr     0x28d70.l                       | +060
        cmpi.b  #0xff,0x20(a6)                  | +066
        bne.w   SetHandlerRts_07bef2            | +06c

| ----------------------------------------------------------------------------
|  M2Boss_Arm_PosFromAngle_07bef4  @ $07BEF4  (66 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Arm_PosFromAngle_07bef4, "ax", @progbits
        .global M2Boss_Arm_PosFromAngle_07bef4
M2Boss_Arm_PosFromAngle_07bef4:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        subi.w  #0x60,0x22(a6)                  | +010
        lea     0x2df8a4.l,a0                   | +016
        move.w  0x72(a6),d0                     | +01c
        lsr.w   #0x8,d0                         | +020
        andi.w  #0xff,d0                        | +022
        add.w   d0,d0                           | +026
        move.b  (a0,d0.w),d1                    | +028
        andi.w  #0xff,d1                        | +02c
        add.w   d1,0x22(a6)                     | +030
        move.b  0x1(a0,d0.w),d1                 | +034
        andi.w  #0xff,d1                        | +038
        sub.w   d1,0x24(a6)                     | +03c
        rts                                     | +040

| ----------------------------------------------------------------------------
|  M2Boss_Turret_Init_07bf36  @ $07BF36  (306 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_Init_07bf36, "ax", @progbits
        .global M2Boss_Turret_Init_07bf36
M2Boss_Turret_Init_07bf36:
        move.w  #0x2a,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        bset    #0x1,0x12(a6)                   | +016
        move.w  #0x0,d0                         | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x14,0x38(a6)                  | +02c
        lea     0x2bcf54.l,a0                   | +032
        jsr     0x799de.l                       | +038
        move.w  d0,0x66(a6)                     | +03e
        move.w  d0,0x7c(a6)                     | +042
        jsr     0x267e2.l                       | +046
        moveq   #0,d0                           | +04c
        move.b  d0,0x20(a6)                     | +04e
        move.b  d0,0x21(a6)                     | +052
        move.w  #0x3,0x70(a6)                   | +056
        move.w  d0,0x72(a6)                     | +05c
        move.w  d0,0x74(a6)                     | +060
        move.w  d0,0x76(a6)                     | +064
        move.w  d0,0x78(a6)                     | +068
        move.w  d0,0x7a(a6)                     | +06c
        move.w  d0,0x7e(a6)                     | +070
        move.w  d0,0x80(a6)                     | +074
        move.w  d0,0x82(a6)                     | +078
        lea     M2Boss_Smoke_Init_07c1b6(pc),a1 | +07c
        jsr     0x4ae.l                         | +080
        jsr     0x5dd02.l                       | +086
        bset    #0x3,0x13(a6)                   | +08c
        lea     .L07bfce(pc),a1                 | +092
        move.l  a1,(a6)                         | +096
        .global M2Boss_Turret_Init_07bf36__L07bfce
M2Boss_Turret_Init_07bf36__L07bfce:
.L07bfce:
        movea.l 0xc(a6),a0                      | +098
        move.w  0x22(a0),0x22(a6)               | +09c
        move.w  0x24(a0),0x24(a6)               | +0a2
        lea     0x2df850.l,a0                   | +0a8
        move.w  0x70(a6),d0                     | +0ae
        add.w   d0,d0                           | +0b2
        add.w   d0,d0                           | +0b4
        movea.l (a0,d0.w),a0                    | +0b6
        jsr     0x28cd4.l                       | +0ba
        jsr     0x28d70.l                       | +0c0
        btst    #0x7,0x5a(a6)                   | +0c6
        beq.w   .L07c014                        | +0cc
        move.l  a6,-(a7)                        | +0d0
        movea.l 0xc(a6),a6                      | +0d2
        bset    #0x7,0x5a(a6)                   | +0d6
        movea.l (a7)+,a6                        | +0dc
.L07c014:
        movea.l 0xc(a6),a0                      | +0de
        movea.l 0xc(a0),a0                      | +0e2
        tst.b   0x20(a0)                        | +0e6
        bne.w   M2Boss_Turret_ParentPhase_07c070 | +0ea
        bclr    #0x3,0x13(a6)                   | +0ee
        jsr     M2Boss_Turret_AimAtPlayer_07c512(pc) | +0f4
        bcc.w   .L07c038                        | +0f8
        lea     M2Boss_Turret_Recoil_07c106(pc),a1 | +0fc
        move.l  a1,(a6)                         | +100
.L07c038:
        jsr     0x2870a.l                       | +102
        lea     0x5e766.l,a0                    | +108
        jsr     0x5e770.l                       | +10e
        jsr     0x28758.l                       | +114
        bcc.w   .L07c05a                        | +11a
        lea     M2Boss_Turret_Destroyed_07c47a(pc),a1 | +11e
        move.l  a1,(a6)                         | +122
.L07c05a:
        movea.l 0xc(a6),a0                      | +124
        cmpi.b  #0xff,0x21(a0)                  | +128
        bne.w   SetHandlerRts_07c06e            | +12e

| ----------------------------------------------------------------------------
|  M2Boss_Turret_ParentPhase_07c070  @ $07C070  (142 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_ParentPhase_07c070, "ax", @progbits
        .global M2Boss_Turret_ParentPhase_07c070
M2Boss_Turret_ParentPhase_07c070:
        cmpi.b  #0x1,0x20(a0)                   | +000
        beq.b   SetHandlerRts_07c06e            | +006
        bclr    #0x3,0x13(a6)                   | +008
        move.w  0x7c(a6),0x66(a6)               | +00e
        move.l  a0,-(a7)                        | +014
        jsr     0x2870a.l                       | +016
        lea     0x5e766.l,a0                    | +01c
        jsr     0x5e770.l                       | +022
        movea.l (a7)+,a0                        | +028
        cmpi.b  #0x2,0x20(a0)                   | +02a
        bne.w   .L07c0b0                        | +030
        movea.l 0xc(a6),a0                      | +034
        move.w  #0x2,0x70(a0)                   | +038
        rts                                     | +03e
.L07c0b0:
        cmpi.b  #0x3,0x20(a0)                   | +040
        bne.b   SetHandlerRts_07c06e            | +046
        movea.l 0xc(a6),a0                      | +048
        clr.w   0x70(a0)                        | +04c
        tst.w   0x80(a6)                        | +050
        bne.w   .L07c0d8                        | +054
        move.w  #0xffff,0x80(a6)                | +058
        move.w  #0x1074,d0                      | +05e
        jsr     0x2352.l                        | +062
.L07c0d8:
        addi.w  #0x1,0x7a(a6)                   | +068
        cmpi.w  #0x5,0x7a(a6)                   | +06e
        blt.b   SetHandlerRts_07c06e            | +074
        clr.w   0x7a(a6)                        | +076
        addq.w  #0x1,0x70(a6)                   | +07a
        cmpi.w  #0x7,0x70(a6)                   | +07e
        blt.w   SetHandlerRts_07c06e            | +084
        move.w  #0x6,0x70(a6)                   | +088

| ----------------------------------------------------------------------------
|  M2Boss_Turret_Recoil_07c106  @ $07C106  (168 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_Recoil_07c106, "ax", @progbits
        .global M2Boss_Turret_Recoil_07c106
M2Boss_Turret_Recoil_07c106:
        lea     0x2df86c.l,a0                   | +000
        move.w  0x70(a6),d0                     | +006
        add.w   d0,d0                           | +00a
        add.w   d0,d0                           | +00c
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        lea     0x2bcfd6.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x82(a6)                     | +024
        bclr    #0x3,0x13(a6)                   | +028
        lea     .L07c13a(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L07c13a:
        movea.l 0xc(a6),a0                      | +034
        move.w  0x22(a0),0x22(a6)               | +038
        move.w  0x24(a0),0x24(a6)               | +03e
        jsr     0x28d70.l                       | +044
        bcc.w   .L07c15a                        | +04a
        lea     M2Boss_Turret_Init_07bf36__L07bfce(pc),a1 | +04e
        move.l  a1,(a6)                         | +052
.L07c15a:
        btst    #0x7,0x5a(a6)                   | +054
        beq.w   .L07c172                        | +05a
        move.l  a6,-(a7)                        | +05e
        movea.l 0xc(a6),a6                      | +060
        bset    #0x7,0x5a(a6)                   | +064
        movea.l (a7)+,a6                        | +06a
.L07c172:
        bclr    #0x3,0x13(a6)                   | +06c
        movea.l 0xc(a6),a0                      | +072
        movea.l 0xc(a0),a0                      | +076
        tst.b   0x20(a0)                        | +07a
        beq.w   .L07c18e                        | +07e
        move.w  0x7c(a6),0x66(a6)               | +082
.L07c18e:
        jsr     0x2870a.l                       | +088
        lea     0x5e766.l,a0                    | +08e
        jsr     0x5e770.l                       | +094
        movea.l 0xc(a6),a0                      | +09a
        cmpi.b  #0xff,0x21(a0)                  | +09e
        bne.w   SetHandlerRts_07c1b4            | +0a4

| ----------------------------------------------------------------------------
|  M2Boss_Smoke_Init_07c1b6  @ $07C1B6  (206 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Smoke_Init_07c1b6, "ax", @progbits
        .global M2Boss_Smoke_Init_07c1b6
M2Boss_Smoke_Init_07c1b6:
        moveq   #0,d0                           | +000
        move.w  d0,0x70(a6)                     | +002
        move.w  #0x5,0x72(a6)                   | +006
        move.w  d0,0x74(a6)                     | +00c
        lea     .L07c1cc(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L07c1cc:
        movea.l 0xc(a6),a0                      | +016
        move.w  0x22(a0),0x22(a6)               | +01a
        move.w  0x24(a0),0x24(a6)               | +020
        move.w  #0x0,d0                         | +026
        jsr     0x28134.l                       | +02a
        andi.w  #0xffe3,0x38(a6)                | +030
        ori.w   #0xc,0x38(a6)                   | +036
        movea.l 0xc(a0),a0                      | +03c
        cmpi.b  #0xff,0x20(a0)                  | +040
        bne.w   .L07c206                        | +046
        lea     JmpToScheduler_07c65c(pc),a1    | +04a
        move.l  a1,(a6)                         | +04e
.L07c206:
        movea.l 0xc(a6),a0                      | +050
        move.w  0x66(a0),d0                     | +054
        mulu.w  #0xa,d0                         | +058
        move.w  0x7c(a0),d1                     | +05c
        divu.w  d1,d0                           | +060
        add.w   d0,d0                           | +062
        lea     0x2df802.l,a0                   | +064
        tst.w   (a0,d0.w)                       | +06a
        beq.w   .L07c282                        | +06e
        move.w  (a0,d0.w),0x74(a6)              | +072
        addq.w  #0x1,0x70(a6)                   | +078
        move.w  0x70(a6),d0                     | +07c
        cmp.w   0x74(a6),d0                     | +080
        blt.w   .L07c258                        | +084
        clr.w   0x70(a6)                        | +088
        lea     M2Boss_SmokePuff_Left_07c284(pc),a1 | +08c
        jsr     0x4ae.l                         | +090
        jsr     0x5dd02.l                       | +096
        move.w  0x38(a6),0x38(a0)               | +09c
.L07c258:
        addq.w  #0x1,0x72(a6)                   | +0a2
        move.w  0x72(a6),d0                     | +0a6
        cmp.w   0x74(a6),d0                     | +0aa
        blt.w   .L07c282                        | +0ae
        clr.w   0x72(a6)                        | +0b2
        lea     M2Boss_SmokePuff_Right_07c2ea(pc),a1 | +0b6
        jsr     0x4ae.l                         | +0ba
        jsr     0x5dd02.l                       | +0c0
        move.w  0x38(a6),0x38(a0)               | +0c6
.L07c282:
        rts                                     | +0cc

| ----------------------------------------------------------------------------
|  M2Boss_SmokePuff_Left_07c284  @ $07C284  (102 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_SmokePuff_Left_07c284, "ax", @progbits
        .global M2Boss_SmokePuff_Left_07c284
M2Boss_SmokePuff_Left_07c284:
        move.w  #0xd,d1                         | +000
        jsr     0x236e.l                        | +004
        subi.w  #0x20,0x22(a6)                  | +00a
        addi.w  #0x10,0x24(a6)                  | +010
        move.w  #0xfd9a,d0                      | +016
        jsr     0x5dca4.l                       | +01a
        move.w  d0,0x28(a6)                     | +020
        move.w  #0x3fc,0x2a(a6)                 | +024
        move.w  #0xff34,0x2e(a6)                | +02a
        move.w  #0x0,0x2c(a6)                   | +030
        neg.w   0x2a(a6)                        | +036
        neg.w   0x2e(a6)                        | +03a
        jsr     0x5e9b6.l                       | +03e
        andi.w  #0x3f,d0                        | +044
        subi.w  #0x1f,d0                        | +048
        add.w   d0,0x2e(a6)                     | +04c
        lea     0x2dfe06.l,a0                   | +050
        jsr     0x28cd4.l                       | +056
        lea     M2Boss_SmokePuff_Run_07c346(pc),a1 | +05c
        move.l  a1,(a6)                         | +060
        bra.w   M2Boss_SmokePuff_Run_07c346     | +062

| ----------------------------------------------------------------------------
|  M2Boss_SmokePuff_Right_07c2ea  @ $07C2EA  (92 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_SmokePuff_Right_07c2ea, "ax", @progbits
        .global M2Boss_SmokePuff_Right_07c2ea
M2Boss_SmokePuff_Right_07c2ea:
        move.w  #0xd,d1                         | +000
        jsr     0x236e.l                        | +004
        addi.w  #0x20,0x22(a6)                  | +00a
        move.w  #0x333,d0                       | +010
        jsr     0x5dca4.l                       | +014
        move.w  d0,0x28(a6)                     | +01a
        move.w  #0x3fc,0x2a(a6)                 | +01e
        move.w  #0xff34,0x2e(a6)                | +024
        move.w  #0x0,0x2c(a6)                   | +02a
        neg.w   0x2a(a6)                        | +030
        neg.w   0x2e(a6)                        | +034
        jsr     0x5e9b6.l                       | +038
        andi.w  #0x3f,d0                        | +03e
        subi.w  #0x1f,d0                        | +042
        add.w   d0,0x2e(a6)                     | +046
        lea     0x2dfe06.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     M2Boss_SmokePuff_Run_07c346(pc),a1 | +056
        move.l  a1,(a6)                         | +05a

| ----------------------------------------------------------------------------
|  M2Boss_SmokePuff_Run_07c346  @ $07C346  (38 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_SmokePuff_Run_07c346, "ax", @progbits
        .global M2Boss_SmokePuff_Run_07c346
M2Boss_SmokePuff_Run_07c346:
        jsr     0x27bc8.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L07c35c                        | +00c
        lea     JmpToScheduler_07c65c(pc),a1    | +010
        move.l  a1,(a6)                         | +014
.L07c35c:
        movea.l #0xffffffff,a0                  | +016
        jsr     0x5dd56.l                       | +01c
        bcc.w   SetHandlerRts_07c372            | +022

| ----------------------------------------------------------------------------
|  M2Boss_Turret_DeathSwing_07c374  @ $07C374  (168 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_DeathSwing_07c374, "ax", @progbits
        .global M2Boss_Turret_DeathSwing_07c374
M2Boss_Turret_DeathSwing_07c374:
        jsr     0x4a16e.l                       | +000
        subi.w  #0x10,0x22(a0)                  | +006
        addi.w  #0x10,0x24(a0)                  | +00c
        jsr     0x4a16e.l                       | +012
        addi.w  #0x10,0x22(a0)                  | +018
        addi.w  #0x10,0x24(a0)                  | +01e
        move.w  #0x0,0x80(a6)                   | +024
        move.w  #0x102d,d0                      | +02a
        jsr     0x2352.l                        | +02e
        move.w  #0x2,0x7c(a6)                   | +034
        lea     .L07c3b4(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L07c3b4:
        addi.w  #0x40,0x74(a6)                  | +040
        cmpi.w  #0x180,0x74(a6)                 | +046
        blt.w   .L07c3ca                        | +04c
        move.w  #0x180,0x74(a6)                 | +050
.L07c3ca:
        move.w  0x74(a6),d0                     | +056
        add.w   d0,0x72(a6)                     | +05a
        cmpi.w  #0xaf00,0x72(a6)                | +05e
        bls.w   .L07c3e2                        | +064
        move.w  #0xaf00,0x72(a6)                | +068
.L07c3e2:
        jsr     M2Boss_Arm_PosFromAngle_07bef4(pc) | +06e
        addq.w  #0x1,0x80(a6)                   | +072
        cmpi.w  #0x19,0x80(a6)                  | +076
        bne.w   .L07c3fe                        | +07c
        move.w  #0x1032,d0                      | +080
        jsr     0x2352.l                        | +084
.L07c3fe:
        cmpi.w  #0x5a,0x80(a6)                  | +08a
        bne.w   .L07c412                        | +090
        move.w  #0x1032,d0                      | +094
        jsr     0x2352.l                        | +098
.L07c412:
        cmpi.w  #0xaf00,0x72(a6)                | +09e
        bcs.w   SetHandlerRts_07c422            | +0a4

| ----------------------------------------------------------------------------
|  M2Boss_Turret_DeathFall_07c424  @ $07C424  (78 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_DeathFall_07c424, "ax", @progbits
        .global M2Boss_Turret_DeathFall_07c424
M2Boss_Turret_DeathFall_07c424:
        move.w  #0x266,d0                       | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0xfa,0x2a(a6)                  | +00e
        move.w  #0xffe7,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        move.w  #0x23,d0                        | +020
        jsr     0x2352.l                        | +024
        move.w  #0x1033,d0                      | +02a
        jsr     0x2352.l                        | +02e
        move.w  #0x2,0x7c(a6)                   | +034
        lea     .L07c464(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L07c464:
        jsr     0x27cee.l                       | +040
        jsr     M2Boss_Turret_OffWorldBox_07c628(pc) | +046
        bcc.w   SetHandlerRts_07c478            | +04a

| ----------------------------------------------------------------------------
|  M2Boss_Turret_Destroyed_07c47a  @ $07C47A  (144 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_Destroyed_07c47a, "ax", @progbits
        .global M2Boss_Turret_Destroyed_07c47a
M2Boss_Turret_Destroyed_07c47a:
        lea     0x2e0124.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        lea     0x2e0136.l,a1                   | +00c
        jsr     0x77c7e.l                       | +012
        move.w  #0xd000,0x38(a0)                | +018
        movea.l 0xc(a6),a0                      | +01e
        move.b  #0xff,0x20(a0)                  | +022
        bclr    #0x1,0x12(a6)                   | +028
        move.b  #0x1,0x10a2d1.l                 | +02e
        lea     .L07c4b6(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L07c4b6:
        movea.l 0xc(a6),a0                      | +03c
        move.w  0x22(a0),0x22(a6)               | +040
        move.w  0x24(a0),0x24(a6)               | +046
        lea     0x2df850.l,a0                   | +04c
        move.w  0x70(a6),d0                     | +052
        add.w   d0,d0                           | +056
        add.w   d0,d0                           | +058
        movea.l (a0,d0.w),a0                    | +05a
        jsr     0x28cd4.l                       | +05e
        jsr     0x28d70.l                       | +064
        btst    #0x7,0x5a(a6)                   | +06a
        beq.w   .L07c4fc                        | +070
        move.l  a6,-(a7)                        | +074
        movea.l 0xc(a6),a6                      | +076
        bset    #0x7,0x5a(a6)                   | +07a
        movea.l (a7)+,a6                        | +080
.L07c4fc:
        movea.l 0xc(a6),a0                      | +082
        cmpi.b  #0xff,0x21(a0)                  | +086
        bne.w   SetHandlerRts_07c510            | +08c

| ----------------------------------------------------------------------------
|  M2Boss_Turret_AimAtPlayer_07c512  @ $07C512  (256 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_AimAtPlayer_07c512, "ax", @progbits
        .global M2Boss_Turret_AimAtPlayer_07c512
M2Boss_Turret_AimAtPlayer_07c512:
        jsr     0x5e1aa.l                       | +000
        lea     0x100440.l,a0                   | +006
        cmpi.w  #0x0,d0                         | +00c
        beq.w   .L07c56a                        | +010
        cmpi.w  #0x1,d0                         | +014
        beq.w   .L07c56a                        | +018
        lea     0x1004e0.l,a0                   | +01c
        cmpi.w  #0x2,d0                         | +022
        beq.w   .L07c56a                        | +026
        addq.w  #0x1,0x76(a6)                   | +02a
        cmpi.w  #0xc8,0x76(a6)                  | +02e
        blt.w   .L07c558                        | +034
        clr.w   0x76(a6)                        | +038
        addq.w  #0x1,0x78(a6)                   | +03c
        andi.w  #0x1,0x78(a6)                   | +040
.L07c558:
        lea     0x2df7ce.l,a0                   | +046
        move.w  0x78(a6),d0                     | +04c
        add.w   d0,d0                           | +050
        add.w   d0,d0                           | +052
        movea.l (a0,d0.w),a0                    | +054
.L07c56a:
        tst.w   0x82(a6)                        | +058
        beq.w   .L07c576                        | +05c
        subq.w  #0x1,0x82(a6)                   | +060
.L07c576:
        jsr     0x5e070.l                       | +064
        lea     0x2df818.l,a0                   | +06a
        moveq   #0,d1                           | +070
        moveq   #6,d2                           | +072
.L07c586:
        cmp.w   (a0,d1.w),d0                    | +074
        blt.w   .L07c596                        | +078
        cmp.w   0x2(a0,d1.w),d0                 | +07c
        blt.w   .L07c59e                        | +080
.L07c596:
        addq.w  #0x8,d1                         | +084
        dbra    d2,.L07c586                     | +086
        rts                                     | +08a
.L07c59e:
        move.w  0x6(a0,d1.w),d3                 | +08c
        cmp.w   0x70(a6),d3                     | +090
        beq.w   .L07c5de                        | +094
        blt.w   .L07c5c8                        | +098
        addq.w  #0x1,0x7a(a6)                   | +09c
        cmpi.w  #0x5,0x7a(a6)                   | +0a0
        blt.w   .L07c5de                        | +0a6
        clr.w   0x7a(a6)                        | +0aa
        addq.w  #0x1,0x70(a6)                   | +0ae
        bra.w   .L07c5de                        | +0b2
.L07c5c8:
        addq.w  #0x1,0x7a(a6)                   | +0b6
        cmpi.w  #0x5,0x7a(a6)                   | +0ba
        blt.w   .L07c5de                        | +0c0
        clr.w   0x7a(a6)                        | +0c4
        subq.w  #0x1,0x70(a6)                   | +0c8
.L07c5de:
        movea.l 0xc(a6),a1                      | +0cc
        clr.w   0x70(a1)                        | +0d0
        move.w  0x4(a0,d1.w),d1                 | +0d4
        sub.w   d0,d1                           | +0d8
        bpl.w   .L07c604                        | +0da
        neg.w   d1                              | +0de
        cmpi.w  #0x3,d1                         | +0e0
        ble.w   M2Boss_Turret_LatchAngle_07c618 | +0e4
        move.w  #0x1,0x70(a1)                   | +0e8
        bra.w   ClearC_07c612                   | +0ee
.L07c604:
        cmpi.w  #0x3,d1                         | +0f2
        ble.w   M2Boss_Turret_LatchAngle_07c618 | +0f6
        move.w  #0xffff,0x70(a1)                | +0fa

| ----------------------------------------------------------------------------
|  M2Boss_Turret_LatchAngle_07c618  @ $07C618  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_LatchAngle_07c618, "ax", @progbits
        .global M2Boss_Turret_LatchAngle_07c618
M2Boss_Turret_LatchAngle_07c618:
        tst.w   0x82(a6)                        | +000
        bne.b   ClearC_07c612                   | +004
        move.w  d0,0x72(a6)                     | +006

| ----------------------------------------------------------------------------
|  M2Boss_Turret_OffWorldBox_07c628  @ $07C628  (16 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_OffWorldBox_07c628, "ax", @progbits
        .global M2Boss_Turret_OffWorldBox_07c628
M2Boss_Turret_OffWorldBox_07c628:
        lea     0x2e011a.l,a0                   | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   ClearC_07c63e                   | +00c

| ----------------------------------------------------------------------------
|  M2Boss_Turret_FreeWithParent_07c644  @ $07C644  (22 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Turret_FreeWithParent_07c644, "ax", @progbits
        .global M2Boss_Turret_FreeWithParent_07c644
M2Boss_Turret_FreeWithParent_07c644:
        move.b  #0xff,0x21(a6)                  | +000
        movea.l 0xc(a6),a0                      | +006
        move.b  #0xff,0x21(a0)                  | +00a
        jmp     0x518.l                         | +010

| ----------------------------------------------------------------------------
|  M2Boss_Rts_07c65a  @ $07C65A  (2 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Rts_07c65a, "ax", @progbits
        .global M2Boss_Rts_07c65a
M2Boss_Rts_07c65a:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  M2Boss_Shell_Init_07c664  @ $07C664  (232 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Shell_Init_07c664, "ax", @progbits
        .global M2Boss_Shell_Init_07c664
M2Boss_Shell_Init_07c664:
        move.w  #0x122,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2bd058.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.w  d0,0x28(a6)                     | +016
        move.w  0x28(a6),d1                     | +01a
        move.w  0x72(a6),d0                     | +01e
        jsr     0x13c0e.l                       | +022
        move.w  d1,0x28(a6)                     | +028
        move.w  d2,0x2a(a6)                     | +02c
        move.w  #0x1,0x66(a6)                   | +030
        move.w  #0xd000,d0                      | +036
        jsr     0x28134.l                       | +03a
        andi.w  #0xffe3,0x38(a6)                | +040
        ori.w   #0x10,0x38(a6)                  | +046
        bset    #0x4,0x6b(a6)                   | +04c
        clr.w   0x70(a6)                        | +052
        lea     0x2dfcf0.l,a0                   | +056
        jsr     0x28cd4.l                       | +05c
        lea     .L07c6cc(pc),a1                 | +062
        move.l  a1,(a6)                         | +066
.L07c6cc:
        jsr     0x27d50.l                       | +068
        bcc.w   .L07c6dc                        | +06e
        lea     M2Boss_Shell_Impact_07c754(pc),a1 | +072
        move.l  a1,(a6)                         | +076
.L07c6dc:
        jsr     0x28d70.l                       | +078
        jsr     0x283d8.l                       | +07e
        btst    #0x1,0x13(a6)                   | +084
        beq.w   .L07c6f8                        | +08a
        lea     M2Boss_Shell_Impact_07c754__L07c808(pc),a1 | +08e
        move.l  a1,(a6)                         | +092
.L07c6f8:
        addq.w  #0x1,0x70(a6)                   | +094
        cmpi.w  #0xf,0x70(a6)                   | +098
        blt.w   .L07c71a                        | +09e
        clr.w   0x70(a6)                        | +0a2
        lea     M2Boss_Shell_Flash_07c868(pc),a1 | +0a6
        jsr     0x4ae.l                         | +0aa
        jsr     0x5dd02.l                       | +0b0
.L07c71a:
        movea.l 0xc(a6),a0                      | +0b6
        movea.l 0xc(a0),a0                      | +0ba
        cmpi.b  #0xff,0x20(a0)                  | +0be
        bne.w   .L07c73c                        | +0c4
        lea     0xffff.w,a0                     | +0c8
        move.l  a0,0x48(a6)                     | +0cc
        lea     0x77f6a.l,a1                    | +0d0
        move.l  a1,(a6)                         | +0d6
.L07c73c:
        movea.l #0xffffffff,a0                  | +0d8
        jsr     0x5dd56.l                       | +0de
        bcc.w   SetHandlerRts_07c752            | +0e4

| ----------------------------------------------------------------------------
|  M2Boss_Shell_Impact_07c754  @ $07C754  (192 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Shell_Impact_07c754, "ax", @progbits
        .global M2Boss_Shell_Impact_07c754
M2Boss_Shell_Impact_07c754:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     0x434dc.l                       | +00a
        lea     0x2df7aa.l,a1                   | +010
        jsr     0x77c7e.l                       | +016
        lea     M2Boss_Shell_Trail_07c838(pc),a1 | +01c
        jsr     0x4ae.l                         | +020
        jsr     0x5dd02.l                       | +026
        lea     M2Boss_Shell_FlashDelay_07c814(pc),a1 | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        addi.w  #0x20,0x24(a0)                  | +03c
        move.w  #0x4,0x88(a0)                   | +042
        move.w  #0x2000,0x38(a0)                | +048
        lea     M2Boss_Shell_FlashDelay_07c814(pc),a1 | +04e
        jsr     0x4ae.l                         | +052
        jsr     0x5dd02.l                       | +058
        addi.w  #0x30,0x24(a0)                  | +05e
        move.w  #0x8,0x88(a0)                   | +064
        move.w  #0x2000,0x38(a0)                | +06a
        lea     M2Boss_Shell_FlashDelay_07c814(pc),a1 | +070
        jsr     0x4ae.l                         | +074
        jsr     0x5dd02.l                       | +07a
        addi.w  #0x40,0x24(a0)                  | +080
        move.w  #0xc,0x88(a0)                   | +086
        move.w  #0x2000,0x38(a0)                | +08c
        lea     M2Boss_Shell_FlashDelay_07c814(pc),a1 | +092
        jsr     0x4ae.l                         | +096
        jsr     0x5dd02.l                       | +09c
        addi.w  #0x50,0x24(a0)                  | +0a2
        move.w  #0x10,0x88(a0)                  | +0a8
        move.w  #0x2000,0x38(a0)                | +0ae
        .global M2Boss_Shell_Impact_07c754__L07c808
M2Boss_Shell_Impact_07c754__L07c808:
.L07c808:
        move.w  #0x2000,0x38(a6)                | +0b4
        jmp     0x77fd6.l                       | +0ba

| ----------------------------------------------------------------------------
|  M2Boss_Shell_FlashDelay_07c814  @ $07C814  (36 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Shell_FlashDelay_07c814, "ax", @progbits
        .global M2Boss_Shell_FlashDelay_07c814
M2Boss_Shell_FlashDelay_07c814:
        subq.w  #0x1,0x88(a6)                   | +000
        bne.w   .L07c836                        | +004
        jsr     0x5e9b6.l                       | +008
        andi.w  #0x2f,d0                        | +00e
        subi.w  #0x1f,d0                        | +012
        add.w   d0,0x22(a6)                     | +016
        lea     0x77f6a.l,a1                    | +01a
        move.l  a1,(a6)                         | +020
.L07c836:
        rts                                     | +022

| ----------------------------------------------------------------------------
|  M2Boss_Shell_Trail_07c838  @ $07C838  (40 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Shell_Trail_07c838, "ax", @progbits
        .global M2Boss_Shell_Trail_07c838
M2Boss_Shell_Trail_07c838:
        lea     0x2dfed6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07c84a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07c84a:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   JsrAbsThunk_07c860              | +01e
        lea     JmpToScheduler_07c65c(pc),a1    | +022
        move.l  a1,(a6)                         | +026

| ----------------------------------------------------------------------------
|  M2Boss_Shell_Flash_07c868  @ $07C868  (66 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Shell_Flash_07c868, "ax", @progbits
        .global M2Boss_Shell_Flash_07c868
M2Boss_Shell_Flash_07c868:
        move.w  #0x122,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        lea     0x2dfd44.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L07c89a(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L07c89a:
        jsr     0x27cee.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   SetHandlerRts_07c8b0            | +03e

| ----------------------------------------------------------------------------
|  M2Boss_Shell_Spark_07c8b2  @ $07C8B2  (94 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Shell_Spark_07c8b2, "ax", @progbits
        .global M2Boss_Shell_Spark_07c8b2
M2Boss_Shell_Spark_07c8b2:
        move.w  #0x8,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x14,0x38(a6)                  | +01a
        jsr     0x267e2.l                       | +020
        lea     0x29e440.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L07c8ea(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L07c8ea:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bcc.w   .L07c900                        | +044
        lea     JmpToScheduler_07c65c(pc),a1    | +048
        move.l  a1,(a6)                         | +04c
.L07c900:
        movea.l #0xffffffff,a0                  | +04e
        jsr     0x5dd56.l                       | +054
        bcc.w   SetHandlerRts_07c916            | +05a

| ----------------------------------------------------------------------------
|  M2Boss_Shell_Spawn_07c918  @ $07C918  (126 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Shell_Spawn_07c918, "ax", @progbits
        .global M2Boss_Shell_Spawn_07c918
M2Boss_Shell_Spawn_07c918:
        movea.l 0xc(a6),a0                      | +000
        movea.l 0xc(a0),a0                      | +004
        tst.b   0x20(a0)                        | +008
        bne.w   .L07c966                        | +00c
        move.w  #0x1065,d0                      | +010
        jsr     0x2352.l                        | +014
        lea     M2Boss_Shell_Init_07c664(pc),a1 | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        move.w  0x72(a6),0x72(a0)               | +02a
        move.w  0x70(a6),d1                     | +030
        add.w   d1,d1                           | +034
        add.w   d1,d1                           | +036
        lea     0x2df888.l,a1                   | +038
        move.w  (a1,d1.w),d0                    | +03e
        add.w   d0,0x22(a0)                     | +042
        move.w  0x2(a1,d1.w),d0                 | +046
        add.w   d0,0x24(a0)                     | +04a
.L07c966:
        lea     M2Boss_Shell_Spark_07c8b2(pc),a1 | +04e
        jsr     0x4ae.l                         | +052
        jsr     0x5dd02.l                       | +058
        move.w  0x70(a6),d1                     | +05e
        add.w   d1,d1                           | +062
        add.w   d1,d1                           | +064
        lea     0x2df888.l,a1                   | +066
        move.w  (a1,d1.w),d0                    | +06c
        add.w   d0,0x22(a0)                     | +070
        move.w  0x2(a1,d1.w),d0                 | +074
        add.w   d0,0x24(a0)                     | +078
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  M2Boss_Debris_Init_07c996  @ $07C996  (194 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Debris_Init_07c996, "ax", @progbits
        .global M2Boss_Debris_Init_07c996
M2Boss_Debris_Init_07c996:
        jsr     0x5e9b6.l                       | +000
        andi.l  #0x7,d0                         | +006
        movea.l #0x2df7e2,a0                    | +00c
        lsl.w   #0x2,d0                         | +012
        movea.l (a0,d0.w),a0                    | +014
        cmpa.l  #0xffffffff,a0                  | +018
        beq.w   .L07c9be                        | +01e
        jsr     0x28cd4.l                       | +022
.L07c9be:
        move.w  #0xa,d1                         | +028
        jsr     0x236e.l                        | +02c
        jsr     0x5e9b6.l                       | +032
        move.w  d0,d3                           | +038
        andi.l  #0x3e,d0                        | +03a
        subi.w  #0x20,d0                        | +040
        andi.w  #0xff,d0                        | +044
        add.w   d0,d0                           | +048
        lea     0x2c072c.l,a1                   | +04a
        move.w  (a1,d0.w),d1                    | +050
        lea     0x2c07ac.l,a1                   | +054
        move.w  (a1,d0.w),d2                    | +05a
        andi.w  #0x7,d3                         | +05e
        addq.w  #0x1,d3                         | +062
        muls.w  d3,d1                           | +064
        asl.w   #0x2,d2                         | +066
        move.w  d1,0x2a(a6)                     | +068
        move.w  d2,0x28(a6)                     | +06c
        jsr     0x5e9b6.l                       | +070
        andi.w  #0x3,d0                         | +076
        addq.w  #0x2,d0                         | +07a
        move.w  0x28(a6),d1                     | +07c
        muls.w  d1,d0                           | +080
        asr.w   #0x1,d0                         | +082
        move.w  d0,0x28(a6)                     | +084
        jsr     0x5e9b6.l                       | +088
        andi.w  #0x3,d0                         | +08e
        addq.w  #0x2,d0                         | +092
        move.w  0x2a(a6),d1                     | +094
        muls.w  d1,d0                           | +098
        asr.w   #0x1,d0                         | +09a
        move.w  d0,0x2a(a6)                     | +09c
        lea     .L07ca3c(pc),a1                 | +0a0
        move.l  a1,(a6)                         | +0a4
.L07ca3c:
        jsr     0x27d50.l                       | +0a6
        jsr     0x28d70.l                       | +0ac
        movea.l #0xffffffff,a0                  | +0b2
        jsr     0x5dd56.l                       | +0b8
        bcc.w   SetHandlerRts_07ca5e            | +0be

| ----------------------------------------------------------------------------
|  M2Boss_Wreck_Spawn0_07ca60  @ $07CA60  (80 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Wreck_Spawn0_07ca60, "ax", @progbits
        .global M2Boss_Wreck_Spawn0_07ca60
M2Boss_Wreck_Spawn0_07ca60:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        subi.w  #0x60,0x22(a0)                  | +012
        subi.w  #0x8,0x24(a0)                   | +018
        lea     0x2df73e.l,a1                   | +01e
        jsr     0x77c7e.l                       | +024
        subi.w  #0x60,0x22(a0)                  | +02a
        subi.w  #0x8,0x24(a0)                   | +030
        lea     0x2df762.l,a1                   | +036
        jsr     0x77c7e.l                       | +03c
        subi.w  #0x60,0x22(a0)                  | +042
        subi.w  #0x8,0x24(a0)                   | +048
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  M2Boss_Wreck_Spawn1_07cab0  @ $07CAB0  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Wreck_Spawn1_07cab0, "ax", @progbits
        .global M2Boss_Wreck_Spawn1_07cab0
M2Boss_Wreck_Spawn1_07cab0:
        lea     0x2df786.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        subi.w  #0x40,0x22(a0)                  | +00c
        subi.w  #0xc,0x24(a0)                   | +012
        lea     0x2df73e.l,a1                   | +018
        jsr     0x77c7e.l                       | +01e
        subi.w  #0x40,0x22(a0)                  | +024
        subi.w  #0xc,0x24(a0)                   | +02a
        lea     0x2df762.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        subi.w  #0x40,0x22(a0)                  | +03c
        subi.w  #0xc,0x24(a0)                   | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  M2Boss_Wreck_Spawn2_07cafa  @ $07CAFA  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Wreck_Spawn2_07cafa, "ax", @progbits
        .global M2Boss_Wreck_Spawn2_07cafa
M2Boss_Wreck_Spawn2_07cafa:
        lea     0x2df786.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        subi.w  #0x28,0x22(a0)                  | +00c
        subi.w  #0x10,0x24(a0)                  | +012
        lea     0x2df750.l,a1                   | +018
        jsr     0x77c7e.l                       | +01e
        subi.w  #0x28,0x22(a0)                  | +024
        subi.w  #0x10,0x24(a0)                  | +02a
        lea     0x2df774.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        subi.w  #0x28,0x22(a0)                  | +03c
        subi.w  #0x10,0x24(a0)                  | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  M2Boss_Wreck_Spawn3_07cb44  @ $07CB44  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Wreck_Spawn3_07cb44, "ax", @progbits
        .global M2Boss_Wreck_Spawn3_07cb44
M2Boss_Wreck_Spawn3_07cb44:
        lea     0x2df786.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        subi.w  #0x18,0x22(a0)                  | +00c
        subi.w  #0x10,0x24(a0)                  | +012
        lea     0x2df750.l,a1                   | +018
        jsr     0x77c7e.l                       | +01e
        subi.w  #0x18,0x22(a0)                  | +024
        subi.w  #0x10,0x24(a0)                  | +02a
        lea     0x2df774.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        subi.w  #0x18,0x22(a0)                  | +03c
        subi.w  #0x10,0x24(a0)                  | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  M2Boss_Wreck_Spawn4_07cb8e  @ $07CB8E  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Wreck_Spawn4_07cb8e, "ax", @progbits
        .global M2Boss_Wreck_Spawn4_07cb8e
M2Boss_Wreck_Spawn4_07cb8e:
        lea     0x2df786.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        subi.w  #0x8,0x22(a0)                   | +00c
        subi.w  #0x18,0x24(a0)                  | +012
        lea     0x2df750.l,a1                   | +018
        jsr     0x77c7e.l                       | +01e
        subi.w  #0x8,0x22(a0)                   | +024
        subi.w  #0x18,0x24(a0)                  | +02a
        lea     0x2df774.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        subi.w  #0x8,0x22(a0)                   | +03c
        subi.w  #0x18,0x24(a0)                  | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  M2Boss_Wreck_Spawn5_07cbd8  @ $07CBD8  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Wreck_Spawn5_07cbd8, "ax", @progbits
        .global M2Boss_Wreck_Spawn5_07cbd8
M2Boss_Wreck_Spawn5_07cbd8:
        lea     0x2df786.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        addi.w  #0x10,0x22(a0)                  | +00c
        subi.w  #0x1c,0x24(a0)                  | +012
        lea     0x2df750.l,a1                   | +018
        jsr     0x77c7e.l                       | +01e
        addi.w  #0x10,0x22(a0)                  | +024
        subi.w  #0x1c,0x24(a0)                  | +02a
        lea     0x2df774.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        addi.w  #0x10,0x22(a0)                  | +03c
        subi.w  #0x1c,0x24(a0)                  | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  M2Boss_Wreck_Spawn6_07cc22  @ $07CC22  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Wreck_Spawn6_07cc22, "ax", @progbits
        .global M2Boss_Wreck_Spawn6_07cc22
M2Boss_Wreck_Spawn6_07cc22:
        lea     0x2df786.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        addi.w  #0x20,0x22(a0)                  | +00c
        subi.w  #0x20,0x24(a0)                  | +012
        lea     0x2df750.l,a1                   | +018
        jsr     0x77c7e.l                       | +01e
        addi.w  #0x20,0x22(a0)                  | +024
        subi.w  #0x20,0x24(a0)                  | +02a
        lea     0x2df774.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        addi.w  #0x20,0x22(a0)                  | +03c
        subi.w  #0x20,0x24(a0)                  | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  M2Boss_Wreck_Spawn7_07cc6c  @ $07CC6C  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Wreck_Spawn7_07cc6c, "ax", @progbits
        .global M2Boss_Wreck_Spawn7_07cc6c
M2Boss_Wreck_Spawn7_07cc6c:
        lea     0x2df786.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        addi.w  #0x30,0x22(a0)                  | +00c
        subi.w  #0x20,0x24(a0)                  | +012
        lea     0x2df750.l,a1                   | +018
        jsr     0x77c7e.l                       | +01e
        addi.w  #0x30,0x22(a0)                  | +024
        subi.w  #0x20,0x24(a0)                  | +02a
        lea     0x2df774.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        addi.w  #0x30,0x22(a0)                  | +03c
        subi.w  #0x20,0x24(a0)                  | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  M2Boss_Wreck_Spawn8_07ccb6  @ $07CCB6  (80 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Wreck_Spawn8_07ccb6, "ax", @progbits
        .global M2Boss_Wreck_Spawn8_07ccb6
M2Boss_Wreck_Spawn8_07ccb6:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        addi.w  #0x48,0x22(a0)                  | +012
        subi.w  #0x20,0x24(a0)                  | +018
        lea     0x2df762.l,a1                   | +01e
        jsr     0x77c7e.l                       | +024
        addi.w  #0x48,0x22(a0)                  | +02a
        subi.w  #0x20,0x24(a0)                  | +030
        lea     0x2df798.l,a1                   | +036
        jsr     0x77c7e.l                       | +03c
        addi.w  #0x48,0x22(a0)                  | +042
        subi.w  #0x20,0x24(a0)                  | +048
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  M2Boss_Burst_Spawn0_07cd06  @ $07CD06  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_Spawn0_07cd06, "ax", @progbits
        .global M2Boss_Burst_Spawn0_07cd06
M2Boss_Burst_Spawn0_07cd06:
        lea     M2Boss_Burst_HitboxA_07ce14(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subi.w  #0x80,0x22(a0)                  | +010
        subi.w  #0x20,0x24(a0)                  | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M2Boss_Burst_Spawn1_07cd24  @ $07CD24  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_Spawn1_07cd24, "ax", @progbits
        .global M2Boss_Burst_Spawn1_07cd24
M2Boss_Burst_Spawn1_07cd24:
        lea     M2Boss_Burst_HitboxB_07ce28(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subi.w  #0x58,0x22(a0)                  | +010
        subi.w  #0x28,0x24(a0)                  | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M2Boss_Burst_Spawn2_07cd42  @ $07CD42  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_Spawn2_07cd42, "ax", @progbits
        .global M2Boss_Burst_Spawn2_07cd42
M2Boss_Burst_Spawn2_07cd42:
        lea     M2Boss_Burst_HitboxB_07ce28(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subi.w  #0x48,0x22(a0)                  | +010
        subi.w  #0x38,0x24(a0)                  | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M2Boss_Burst_Spawn3_07cd60  @ $07CD60  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_Spawn3_07cd60, "ax", @progbits
        .global M2Boss_Burst_Spawn3_07cd60
M2Boss_Burst_Spawn3_07cd60:
        lea     M2Boss_Burst_HitboxB_07ce28(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subi.w  #0x38,0x22(a0)                  | +010
        subi.w  #0x38,0x24(a0)                  | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M2Boss_Burst_Spawn4_07cd7e  @ $07CD7E  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_Spawn4_07cd7e, "ax", @progbits
        .global M2Boss_Burst_Spawn4_07cd7e
M2Boss_Burst_Spawn4_07cd7e:
        lea     M2Boss_Burst_HitboxB_07ce28(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subi.w  #0x28,0x22(a0)                  | +010
        subi.w  #0x38,0x24(a0)                  | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M2Boss_Burst_Spawn5_07cd9c  @ $07CD9C  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_Spawn5_07cd9c, "ax", @progbits
        .global M2Boss_Burst_Spawn5_07cd9c
M2Boss_Burst_Spawn5_07cd9c:
        lea     M2Boss_Burst_HitboxB_07ce28(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subi.w  #0x8,0x22(a0)                   | +010
        subi.w  #0x38,0x24(a0)                  | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M2Boss_Burst_Spawn6_07cdba  @ $07CDBA  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_Spawn6_07cdba, "ax", @progbits
        .global M2Boss_Burst_Spawn6_07cdba
M2Boss_Burst_Spawn6_07cdba:
        lea     M2Boss_Burst_HitboxB_07ce28(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0x8,0x22(a0)                   | +010
        subi.w  #0x38,0x24(a0)                  | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M2Boss_Burst_Spawn7_07cdd8  @ $07CDD8  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_Spawn7_07cdd8, "ax", @progbits
        .global M2Boss_Burst_Spawn7_07cdd8
M2Boss_Burst_Spawn7_07cdd8:
        lea     M2Boss_Burst_HitboxB_07ce28(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0x8,0x22(a0)                   | +010
        subi.w  #0x38,0x24(a0)                  | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M2Boss_Burst_Spawn8_07cdf6  @ $07CDF6  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_Spawn8_07cdf6, "ax", @progbits
        .global M2Boss_Burst_Spawn8_07cdf6
M2Boss_Burst_Spawn8_07cdf6:
        lea     M2Boss_Burst_HitboxC_07ce3c(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0x20,0x22(a0)                  | +010
        subi.w  #0x38,0x24(a0)                  | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M2Boss_Burst_HitboxA_07ce14  @ $07CE14  (20 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_HitboxA_07ce14, "ax", @progbits
        .global M2Boss_Burst_HitboxA_07ce14
M2Boss_Burst_HitboxA_07ce14:
        lea     0x2e001e.l,a0                   | +000
        move.l  a0,0x4c(a6)                     | +006
        jsr     0x283ca.l                       | +00a
        bra.w   M2Boss_Burst_HitboxC_07ce3c__L07ce4c | +010

| ----------------------------------------------------------------------------
|  M2Boss_Burst_HitboxB_07ce28  @ $07CE28  (20 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_HitboxB_07ce28, "ax", @progbits
        .global M2Boss_Burst_HitboxB_07ce28
M2Boss_Burst_HitboxB_07ce28:
        lea     0x2e0072.l,a0                   | +000
        move.l  a0,0x4c(a6)                     | +006
        jsr     0x283ca.l                       | +00a
        bra.w   M2Boss_Burst_HitboxC_07ce3c__L07ce4c | +010

| ----------------------------------------------------------------------------
|  M2Boss_Burst_HitboxC_07ce3c  @ $07CE3C  (84 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Burst_HitboxC_07ce3c, "ax", @progbits
        .global M2Boss_Burst_HitboxC_07ce3c
M2Boss_Burst_HitboxC_07ce3c:
        lea     0x2e00c6.l,a0                   | +000
        move.l  a0,0x4c(a6)                     | +006
        jsr     0x283ca.l                       | +00a
        .global M2Boss_Burst_HitboxC_07ce3c__L07ce4c
M2Boss_Burst_HitboxC_07ce3c__L07ce4c:
.L07ce4c:
        move.w  #0x122,d1                       | +010
        jsr     0x236e.l                        | +014
        jsr     0x267e2.l                       | +01a
        move.w  #0xf000,d0                      | +020
        jsr     0x28134.l                       | +024
        andi.w  #0xffe3,0x38(a6)                | +02a
        ori.w   #0x10,0x38(a6)                  | +030
        lea     0x2dfef4.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        jsr     0x28d70.l                       | +042
        jsr     0x283d8.l                       | +048
        jmp     0x518.l                         | +04e

| ----------------------------------------------------------------------------
|  M2Boss_Rts_07ce90  @ $07CE90  (2 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_Rts_07ce90, "ax", @progbits
        .global M2Boss_Rts_07ce90
M2Boss_Rts_07ce90:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  M2Boss_LeaSprite0_07ce92  @ $07CE92  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_LeaSprite0_07ce92, "ax", @progbits
        .global M2Boss_LeaSprite0_07ce92
M2Boss_LeaSprite0_07ce92:
        lea     0x29856c.l,a2                   | +000
        bra.w   SdsRts_07cede                   | +006

| ----------------------------------------------------------------------------
|  M2Boss_LeaSprite1_07ce9c  @ $07CE9C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_LeaSprite1_07ce9c, "ax", @progbits
        .global M2Boss_LeaSprite1_07ce9c
M2Boss_LeaSprite1_07ce9c:
        lea     0x298580.l,a2                   | +000
        bra.w   SdsRts_07cede                   | +006

| ----------------------------------------------------------------------------
|  M2Boss_LeaSprite2_07cea6  @ $07CEA6  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_LeaSprite2_07cea6, "ax", @progbits
        .global M2Boss_LeaSprite2_07cea6
M2Boss_LeaSprite2_07cea6:
        lea     0x298594.l,a2                   | +000
        bra.w   SdsRts_07cede                   | +006

| ----------------------------------------------------------------------------
|  M2Boss_LeaSprite3_07ceb0  @ $07CEB0  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_LeaSprite3_07ceb0, "ax", @progbits
        .global M2Boss_LeaSprite3_07ceb0
M2Boss_LeaSprite3_07ceb0:
        lea     0x2985a8.l,a2                   | +000
        bra.w   SdsRts_07cede                   | +006

| ----------------------------------------------------------------------------
|  M2Boss_LeaSprite4_07ceba  @ $07CEBA  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_LeaSprite4_07ceba, "ax", @progbits
        .global M2Boss_LeaSprite4_07ceba
M2Boss_LeaSprite4_07ceba:
        lea     0x2985bc.l,a2                   | +000
        bra.w   SdsRts_07cede                   | +006

| ----------------------------------------------------------------------------
|  M2Boss_LeaSprite5_07cec4  @ $07CEC4  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_LeaSprite5_07cec4, "ax", @progbits
        .global M2Boss_LeaSprite5_07cec4
M2Boss_LeaSprite5_07cec4:
        lea     0x2985d0.l,a2                   | +000
        bra.w   SdsRts_07cede                   | +006

| ----------------------------------------------------------------------------
|  M2Boss_LeaSprite6_07cece  @ $07CECE  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M2Boss_LeaSprite6_07cece, "ax", @progbits
        .global M2Boss_LeaSprite6_07cece
M2Boss_LeaSprite6_07cece:
        lea     0x2985e4.l,a2                   | +000
        bra.w   SdsRts_07cede                   | +006

| ----------------------------------------------------------------------------
|  Crab_RetGate_07cee6  @ $07CEE6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_RetGate_07cee6, "ax", @progbits
        .global Crab_RetGate_07cee6
Crab_RetGate_07cee6:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_07cefc                    | +00c

| ----------------------------------------------------------------------------
|  Crab_Tmpl138_07cf02  @ $07CF02  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Tmpl138_07cf02, "ax", @progbits
        .global Crab_Tmpl138_07cf02
Crab_Tmpl138_07cf02:
        move.w  #0x0,0x70(a6)                   | +000
        bra.w   Crab_Tmpl139_07cf0c__L07cf12    | +006

| ----------------------------------------------------------------------------
|  Crab_Tmpl139_07cf0c  @ $07CF0C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Tmpl139_07cf0c, "ax", @progbits
        .global Crab_Tmpl139_07cf0c
Crab_Tmpl139_07cf0c:
        move.w  #0x1,0x70(a6)                   | +000
        .global Crab_Tmpl139_07cf0c__L07cf12
Crab_Tmpl139_07cf0c__L07cf12:
.L07cf12:
        move.w  #0x4b,d1                        | +006
        jsr     0x236e.l                        | +00a
        move.w  #0xa,0x1c(a6)                   | +010
        jsr     0x138fe.l                       | +016
        moveq   #0,d0                           | +01c
        move.b  d0,0x20(a6)                     | +01e
        move.b  d0,0x21(a6)                     | +022
        move.b  0x9c(a6),0x84(a6)               | +026
        lea     0x2bda80.l,a0                   | +02c
        jsr     0x799de.l                       | +032
        move.w  d0,0x66(a6)                     | +038
        jsr     0x267e2.l                       | +03c
        move.b  0x9a(a6),0x3a(a6)               | +042

| ----------------------------------------------------------------------------
|  Crab_SpawnClaws_07cf5c  @ $07CF5C  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_SpawnClaws_07cf5c, "ax", @progbits
        .global Crab_SpawnClaws_07cf5c
Crab_SpawnClaws_07cf5c:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x4001,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0xc,0x38(a6)                   | +016
        lea     Crab_Claw_Init_07d4aa(pc),a1    | +01c
        jsr     0x4ae.l                         | +020
        jsr     0x5dd02.l                       | +026
        lea     Crab_Claw_Init_07d4aa__L07d520(pc),a1 | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036

| ----------------------------------------------------------------------------
|  Crab_Idle_07cf98  @ $07CF98  (182 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Idle_07cf98, "ax", @progbits
        .global Crab_Idle_07cf98
Crab_Idle_07cf98:
        moveq   #0,d0                           | +000
        move.b  d0,0x72(a6)                     | +002
        move.w  d0,0x74(a6)                     | +006
        lea     0x2e04d6.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L07cfb4(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07cfb4:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        tst.w   0x70(a6)                        | +028
        bne.w   .L07cfd6                        | +02c
        move.w  0x106f50.l,d0                   | +030
        cmp.w   0x98(a6),d0                     | +036
        blt.w   .L07cffc                        | +03a
.L07cfd6:
        jsr     0x5e0d4.l                       | +03e
        move.w  0x22(a6),d0                     | +044
        sub.w   0x22(a0),d0                     | +048
        cmp.w   0x98(a6),d0                     | +04c
        bgt.w   .L07cffc                        | +050
        move.b  #0x1,0x20(a6)                   | +054
        clr.w   0x76(a6)                        | +05a
        lea     Crab_Charge_07d056(pc),a1       | +05e
        move.l  a1,(a6)                         | +062
.L07cffc:
        jsr     0x2870a.l                       | +064
        bcc.w   .L07d018                        | +06a
        lea     0x5e766.l,a0                    | +06e
        jsr     0x5e770.l                       | +074
        bclr    #0x3,0x13(a6)                   | +07a
.L07d018:
        jsr     0x27eba.l                       | +080
        bcc.w   .L07d028                        | +086
        lea     Crab_Knockback_07d3e2(pc),a1    | +08a
        move.l  a1,(a6)                         | +08e
.L07d028:
        jsr     0x28758.l                       | +090
        bcc.w   .L07d038                        | +096
        lea     Crab_Death_07d332(pc),a1        | +09a
        move.l  a1,(a6)                         | +09e
.L07d038:
        movea.l #0xffffffff,a0                  | +0a0
        lea     0x2e0b5c.l,a0                   | +0a6
        jsr     0x5dd5c.l                       | +0ac
        bcc.w   SetHandlerRts_07d054            | +0b2

| ----------------------------------------------------------------------------
|  Crab_Charge_07d056  @ $07D056  (282 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Charge_07d056, "ax", @progbits
        .global Crab_Charge_07d056
Crab_Charge_07d056:
        jsr     0x267e2.l                       | +000
        move.w  #0xffe0,0x28(a6)                | +006
        btst    #0x0,0x3a(a6)                   | +00c
        beq.w   .L07d070                        | +012
        neg.w   0x28(a6)                        | +016
.L07d070:
        move.w  0x28(a6),0x2c(a6)               | +01a
        moveq   #0,d0                           | +020
        move.w  d0,0x28(a6)                     | +022
        move.b  d0,0x72(a6)                     | +026
        move.w  d0,0x74(a6)                     | +02a
        lea     0x2e050a.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     .L07d096(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L07d096:
        jsr     0x27afc.l                       | +040
        bcc.w   .L07d0a6                        | +046
        move.b  #0xff,0x72(a6)                  | +04a
.L07d0a6:
        cmpi.w  #0xfc00,0x28(a6)                | +050
        blt.w   .L07d0ba                        | +056
        cmpi.w  #0x400,0x28(a6)                 | +05a
        blt.w   .L07d0d2                        | +060
.L07d0ba:
        clr.w   0x2c(a6)                        | +064
        move.w  #0xfc00,0x28(a6)                | +068
        btst    #0x0,0x3a(a6)                   | +06e
        beq.w   .L07d0d2                        | +074
        neg.w   0x28(a6)                        | +078
.L07d0d2:
        jsr     0x28d70.l                       | +07c
        tst.b   0x72(a6)                        | +082
        bne.w   .L07d0fa                        | +086
        move.w  0x76(a6),d0                     | +08a
        add.w   0x28(a6),d0                     | +08e
        move.w  d0,0x76(a6)                     | +092
        bpl.w   .L07d0f2                        | +096
        neg.w   d0                              | +09a
.L07d0f2:
        cmpi.w  #0x6000,d0                      | +09c
        blt.w   .L07d11e                        | +0a0
.L07d0fa:
        move.b  #0x2,0x20(a6)                   | +0a4
        clr.b   0x72(a6)                        | +0aa
        move.w  0x76(a6),d0                     | +0ae
        bpl.w   .L07d10e                        | +0b2
        neg.w   d0                              | +0b6
.L07d10e:
        move.w  d0,0x78(a6)                     | +0b8
        move.w  #0x28,0x80(a6)                  | +0bc
        lea     Crab_Pause_07d178(pc),a1        | +0c2
        move.l  a1,(a6)                         | +0c6
.L07d11e:
        jsr     0x2870a.l                       | +0c8
        bcc.w   .L07d13a                        | +0ce
        lea     0x5e766.l,a0                    | +0d2
        jsr     0x5e770.l                       | +0d8
        bclr    #0x3,0x13(a6)                   | +0de
.L07d13a:
        jsr     0x27eba.l                       | +0e4
        bcc.w   .L07d14a                        | +0ea
        lea     Crab_Knockback_07d3e2(pc),a1    | +0ee
        move.l  a1,(a6)                         | +0f2
.L07d14a:
        jsr     0x28758.l                       | +0f4
        bcc.w   .L07d15a                        | +0fa
        lea     Crab_Death_07d332(pc),a1        | +0fe
        move.l  a1,(a6)                         | +102
.L07d15a:
        movea.l #0xffffffff,a0                  | +104
        lea     0x2e0b5c.l,a0                   | +10a
        jsr     0x5dd5c.l                       | +110
        bcc.w   SetHandlerRts_07d176            | +116

| ----------------------------------------------------------------------------
|  Crab_Pause_07d178  @ $07D178  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Pause_07d178, "ax", @progbits
        .global Crab_Pause_07d178
Crab_Pause_07d178:
        jsr     0x267e2.l                       | +000
        clr.w   0x74(a6)                        | +006
        move.w  #0x109e,d0                      | +00a
        jsr     0x2222.l                        | +00e
        lea     0x2e04d6.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L07d19e(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L07d19e:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        tst.b   0x9b(a6)                        | +032
        beq.w   .L07d1d4                        | +036
        cmpi.b  #0x4,0x20(a6)                   | +03a
        beq.w   .L07d1d4                        | +040
        subq.w  #0x1,0x80(a6)                   | +044
        bne.w   .L07d1d4                        | +048
        move.b  #0x3,0x20(a6)                   | +04c
        clr.w   0x76(a6)                        | +052
        lea     Crab_Retreat_07d22e(pc),a1      | +056
        move.l  a1,(a6)                         | +05a
.L07d1d4:
        jsr     0x2870a.l                       | +05c
        bcc.w   .L07d1f0                        | +062
        lea     0x5e766.l,a0                    | +066
        jsr     0x5e770.l                       | +06c
        bclr    #0x3,0x13(a6)                   | +072
.L07d1f0:
        jsr     0x27eba.l                       | +078
        bcc.w   .L07d200                        | +07e
        lea     Crab_Knockback_07d3e2(pc),a1    | +082
        move.l  a1,(a6)                         | +086
.L07d200:
        jsr     0x28758.l                       | +088
        bcc.w   .L07d210                        | +08e
        lea     Crab_Death_07d332(pc),a1        | +092
        move.l  a1,(a6)                         | +096
.L07d210:
        movea.l #0xffffffff,a0                  | +098
        lea     0x2e0b5c.l,a0                   | +09e
        jsr     0x5dd5c.l                       | +0a4
        bcc.w   SetHandlerRts_07d22c            | +0aa

| ----------------------------------------------------------------------------
|  Crab_Retreat_07d22e  @ $07D22E  (252 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Retreat_07d22e, "ax", @progbits
        .global Crab_Retreat_07d22e
Crab_Retreat_07d22e:
        jsr     0x267e2.l                       | +000
        move.w  #0x20,0x28(a6)                  | +006
        btst    #0x0,0x3a(a6)                   | +00c
        beq.w   .L07d248                        | +012
        neg.w   0x28(a6)                        | +016
.L07d248:
        move.w  0x28(a6),0x2c(a6)               | +01a
        moveq   #0,d0                           | +020
        move.w  d0,0x28(a6)                     | +022
        move.w  d0,0x74(a6)                     | +026
        lea     0x2e057e.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L07d26a(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L07d26a:
        jsr     0x27afc.l                       | +03c
        bcc.w   .L07d280                        | +042
        move.b  #0x4,0x20(a6)                   | +046
        lea     Crab_Pause_07d178(pc),a1        | +04c
        move.l  a1,(a6)                         | +050
.L07d280:
        cmpi.w  #0xfc00,0x28(a6)                | +052
        blt.w   .L07d294                        | +058
        cmpi.w  #0x400,0x28(a6)                 | +05c
        blt.w   .L07d2ac                        | +062
.L07d294:
        clr.w   0x2c(a6)                        | +066
        move.w  #0x400,0x28(a6)                 | +06a
        btst    #0x0,0x3a(a6)                   | +070
        beq.w   .L07d2ac                        | +076
        neg.w   0x28(a6)                        | +07a
.L07d2ac:
        jsr     0x28d70.l                       | +07e
        move.w  0x76(a6),d0                     | +084
        add.w   0x28(a6),d0                     | +088
        move.w  d0,0x76(a6)                     | +08c
        bpl.w   .L07d2c4                        | +090
        neg.w   d0                              | +094
.L07d2c4:
        cmp.w   0x78(a6),d0                     | +096
        blt.w   .L07d2d8                        | +09a
        move.b  #0x4,0x20(a6)                   | +09e
        lea     Crab_Pause_07d178(pc),a1        | +0a4
        move.l  a1,(a6)                         | +0a8
.L07d2d8:
        jsr     0x2870a.l                       | +0aa
        bcc.w   .L07d2f4                        | +0b0
        lea     0x5e766.l,a0                    | +0b4
        jsr     0x5e770.l                       | +0ba
        bclr    #0x3,0x13(a6)                   | +0c0
.L07d2f4:
        jsr     0x27eba.l                       | +0c6
        bcc.w   .L07d304                        | +0cc
        lea     Crab_Knockback_07d3e2(pc),a1    | +0d0
        move.l  a1,(a6)                         | +0d4
.L07d304:
        jsr     0x28758.l                       | +0d6
        bcc.w   .L07d314                        | +0dc
        lea     Crab_Death_07d332(pc),a1        | +0e0
        move.l  a1,(a6)                         | +0e4
.L07d314:
        movea.l #0xffffffff,a0                  | +0e6
        lea     0x2e0b5c.l,a0                   | +0ec
        jsr     0x5dd5c.l                       | +0f2
        bcc.w   SetHandlerRts_07d330            | +0f8

| ----------------------------------------------------------------------------
|  Crab_Death_07d332  @ $07D332  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Death_07d332, "ax", @progbits
        .global Crab_Death_07d332
Crab_Death_07d332:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.b  #0xff,0x20(a6)                  | +012
        bclr    #0x1,0x12(a6)                   | +018
        lea     0x2e064a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        move.w  #0x109e,d0                      | +02a
        jsr     0x2222.l                        | +02e
        lea     .L07d36c(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L07d36c:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        bcc.w   .L07d382                        | +046
        lea     Crab_Explode_07d3b0(pc),a1      | +04a
        move.l  a1,(a6)                         | +04e
.L07d382:
        jsr     0x27eba.l                       | +050
        bcc.w   .L07d392                        | +056
        lea     Crab_Explode_07d3b0(pc),a1      | +05a
        move.l  a1,(a6)                         | +05e
.L07d392:
        movea.l #0xffffffff,a0                  | +060
        lea     0x2e0b5c.l,a0                   | +066
        jsr     0x5dd5c.l                       | +06c
        bcc.w   SetHandlerRts_07d3ae            | +072

| ----------------------------------------------------------------------------
|  Crab_Explode_07d3b0  @ $07D3B0  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Explode_07d3b0, "ax", @progbits
        .global Crab_Explode_07d3b0
Crab_Explode_07d3b0:
        move.w  #0xc000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x14,0x38(a6)                  | +010
        lea     0x2e0348.l,a1                   | +016
        jsr     0x77c7e.l                       | +01c
        move.w  #0x1033,d0                      | +022
        jsr     0x2352.l                        | +026
        jmp     0x77f6a.l                       | +02c

| ----------------------------------------------------------------------------
|  Crab_Knockback_07d3e2  @ $07D3E2  (192 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Knockback_07d3e2, "ax", @progbits
        .global Crab_Knockback_07d3e2
Crab_Knockback_07d3e2:
        move.b  #0x0,0x21(a6)                   | +000
        move.b  #0x80,0x20(a6)                  | +006
        bclr    #0x1,0x12(a6)                   | +00c
        asr.w   0x28(a6)                        | +012
        move.w  #0x0,0x2c(a6)                   | +016
        move.w  #0xfff0,0x2e(a6)                | +01c
        move.w  #0xc000,d0                      | +022
        jsr     0x28134.l                       | +026
        andi.w  #0xffe3,0x38(a6)                | +02c
        ori.w   #0x1c,0x38(a6)                  | +032
        lea     0x2e075a.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        move.w  #0x109e,d0                      | +044
        jsr     0x2222.l                        | +048
        lea     .L07d436(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L07d436:
        bset    #0x6,0x13(a6)                   | +054
        jsr     0x27cee.l                       | +05a
        bclr    #0x6,0x13(a6)                   | +060
        jsr     0x44048.l                       | +066
        jsr     0x28d70.l                       | +06c
        cmpi.w  #0x120,0x24(a6)                 | +072
        bgt.w   .L07d48c                        | +078
        tst.b   0x21(a6)                        | +07c
        bne.w   .L07d48c                        | +080
        move.b  #0xff,0x21(a6)                  | +084
        tst.b   0x84(a6)                        | +08a
        bne.w   .L07d48c                        | +08e
        lea     0x78864.l,a1                    | +092
        jsr     0x4ae.l                         | +098
        jsr     0x5dd02.l                       | +09e
        move.w  #0xffff,0x38(a0)                | +0a4
        .global Crab_Knockback_07d3e2__L07d48c
Crab_Knockback_07d3e2__L07d48c:
.L07d48c:
        movea.l #0xffffffff,a0                  | +0aa
        lea     0x2e0340.l,a0                   | +0b0
        jsr     0x5dd5c.l                       | +0b6
        bcc.w   SetHandlerRts_07d4a8            | +0bc

| ----------------------------------------------------------------------------
|  Crab_Claw_Init_07d4aa  @ $07D4AA  (186 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Claw_Init_07d4aa, "ax", @progbits
        .global Crab_Claw_Init_07d4aa
Crab_Claw_Init_07d4aa:
        move.w  #0x4b,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xa,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        jsr     0x267e2.l                       | +016
        moveq   #0,d0                           | +01c
        move.b  d0,0x20(a6)                     | +01e
        move.b  d0,0x21(a6)                     | +022
        move.w  d0,0x72(a6)                     | +026
        move.w  #0x1,0x82(a6)                   | +02a
        lea     Crab_Leg_Init_07d6a6(pc),a1     | +030
        jsr     0x4ae.l                         | +034
        jsr     0x5dd02.l                       | +03a
        lea     Crab_Leg_Init_07d6a6__L07d6b6(pc),a1 | +040
        jsr     0x4ae.l                         | +044
        jsr     0x5dd02.l                       | +04a
        lea     Crab_Leg_Init_07d6a6__L07d6c6(pc),a1 | +050
        jsr     0x4ae.l                         | +054
        jsr     0x5dd02.l                       | +05a
        lea     0x2e0770.l,a0                   | +060
        jsr     0x28cd4.l                       | +066
        lea     Crab_Claw_Follow_07d564(pc),a1  | +06c
        move.l  a1,(a6)                         | +070
        bra.w   Crab_Claw_Follow_07d564         | +072
        .global Crab_Claw_Init_07d4aa__L07d520
Crab_Claw_Init_07d4aa__L07d520:
.L07d520:
        move.w  #0x4b,d1                        | +076
        jsr     0x236e.l                        | +07a
        move.w  #0xa,0x1c(a6)                   | +080
        jsr     0x138fe.l                       | +086
        jsr     0x267e2.l                       | +08c
        moveq   #0,d0                           | +092
        move.b  d0,0x20(a6)                     | +094
        move.b  d0,0x21(a6)                     | +098
        move.w  #0x1,0x72(a6)                   | +09c
        move.w  #0xffff,0x82(a6)                | +0a2
        lea     0x2e081e.l,a0                   | +0a8
        jsr     0x28cd4.l                       | +0ae
        lea     Crab_Claw_Follow_07d564(pc),a1  | +0b4
        move.l  a1,(a6)                         | +0b8

| ----------------------------------------------------------------------------
|  Crab_Claw_Follow_07d564  @ $07D564  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Claw_Follow_07d564, "ax", @progbits
        .global Crab_Claw_Follow_07d564
Crab_Claw_Follow_07d564:
        jsr     Crab_Claw_PosFromParent_07dc1a(pc) | +000
        movea.l 0xc(a6),a0                      | +004
        btst    #0x7,0x5a(a0)                   | +008
        beq.w   .L07d57c                        | +00e
        bset    #0x0,0x5a(a6)                   | +012
.L07d57c:
        jsr     0x28d70.l                       | +018
        movea.l 0xc(a6),a0                      | +01e
        cmpi.b  #0x1,0x20(a0)                   | +022
        bne.w   .L07d596                        | +028
        lea     Crab_Claw_Open_07d5c0(pc),a1    | +02c
        move.l  a1,(a6)                         | +030
.L07d596:
        movea.l 0xc(a6),a0                      | +032
        cmpi.b  #0xff,0x20(a0)                  | +036
        beq.w   SetTaskHandler_07d5b8           | +03c
        cmpi.b  #0x80,0x20(a0)                  | +040
        beq.w   SetTaskHandler_07d5b8           | +046
        cmpi.b  #0x40,0x20(a0)                  | +04a
        bne.w   SetHandlerRts_07d5be            | +050

| ----------------------------------------------------------------------------
|  Crab_Claw_Open_07d5c0  @ $07D5C0  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Claw_Open_07d5c0, "ax", @progbits
        .global Crab_Claw_Open_07d5c0
Crab_Claw_Open_07d5c0:
        move.w  0x72(a6),d0                     | +000
        movea.l #0x2e0456,a0                    | +004
        lsl.w   #0x2,d0                         | +00a
        movea.l (a0,d0.w),a0                    | +00c
        cmpa.l  #0xffffffff,a0                  | +010
        beq.w   .L07d5e0                        | +016
        jsr     0x28cd4.l                       | +01a
.L07d5e0:
        lea     .L07d5e6(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L07d5e6:
        jsr     Crab_Claw_PosFromParent_07dc1a(pc) | +026
        movea.l 0xc(a6),a0                      | +02a
        btst    #0x7,0x5a(a0)                   | +02e
        beq.w   .L07d5fe                        | +034
        bset    #0x0,0x5a(a6)                   | +038
.L07d5fe:
        jsr     0x28d70.l                       | +03e
        bcc.w   .L07d60e                        | +044
        lea     Crab_Claw_Close_07d638(pc),a1   | +048
        move.l  a1,(a6)                         | +04c
.L07d60e:
        movea.l 0xc(a6),a0                      | +04e
        cmpi.b  #0xff,0x20(a0)                  | +052
        beq.w   SetTaskHandler_07d630           | +058
        cmpi.b  #0x80,0x20(a0)                  | +05c
        beq.w   SetTaskHandler_07d630           | +062
        cmpi.b  #0x40,0x20(a0)                  | +066
        bne.w   SetHandlerRts_07d636            | +06c

| ----------------------------------------------------------------------------
|  Crab_Claw_Close_07d638  @ $07D638  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Claw_Close_07d638, "ax", @progbits
        .global Crab_Claw_Close_07d638
Crab_Claw_Close_07d638:
        move.w  0x72(a6),d0                     | +000
        movea.l #0x2e044e,a0                    | +004
        lsl.w   #0x2,d0                         | +00a
        movea.l (a0,d0.w),a0                    | +00c
        cmpa.l  #0xffffffff,a0                  | +010
        beq.w   .L07d658                        | +016
        jsr     0x28cd4.l                       | +01a
.L07d658:
        lea     .L07d65e(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L07d65e:
        jsr     Crab_Claw_PosFromParent_07dc1a(pc) | +026
        movea.l 0xc(a6),a0                      | +02a
        btst    #0x7,0x5a(a0)                   | +02e
        beq.w   .L07d676                        | +034
        bset    #0x0,0x5a(a6)                   | +038
.L07d676:
        jsr     0x28d70.l                       | +03e
        movea.l 0xc(a6),a0                      | +044
        cmpi.b  #0xff,0x20(a0)                  | +048
        beq.w   SetTaskHandler_07d69e           | +04e
        cmpi.b  #0x80,0x20(a0)                  | +052
        beq.w   SetTaskHandler_07d69e           | +058
        cmpi.b  #0x40,0x20(a0)                  | +05c
        bne.w   SetHandlerRts_07d6a4            | +062

| ----------------------------------------------------------------------------
|  Crab_Leg_Init_07d6a6  @ $07D6A6  (192 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Leg_Init_07d6a6, "ax", @progbits
        .global Crab_Leg_Init_07d6a6
Crab_Leg_Init_07d6a6:
        move.w  #0x0,0x70(a6)                   | +000
        move.w  #0x26,0x74(a6)                  | +006
        bra.w   .L07d6d2                        | +00c
        .global Crab_Leg_Init_07d6a6__L07d6b6
Crab_Leg_Init_07d6a6__L07d6b6:
.L07d6b6:
        move.w  #0x1,0x70(a6)                   | +010
        move.w  #0x2f,0x74(a6)                  | +016
        bra.w   .L07d6d2                        | +01c
        .global Crab_Leg_Init_07d6a6__L07d6c6
Crab_Leg_Init_07d6a6__L07d6c6:
.L07d6c6:
        move.w  #0x2,0x70(a6)                   | +020
        move.w  #0x38,0x74(a6)                  | +026
.L07d6d2:
        move.w  #0x4c,d1                        | +02c
        jsr     0x236e.l                        | +030
        moveq   #0,d0                           | +036
        move.b  d0,0x20(a6)                     | +038
        move.b  d0,0x21(a6)                     | +03c
        bset    #0x4,0x6b(a6)                   | +040
        move.w  #0x1,0x66(a6)                   | +046
        jsr     0x267e2.l                       | +04c
        jsr     0x5e9b6.l                       | +052
        andi.w  #0x7,d0                         | +058
        subq.w  #0x4,d0                         | +05c
        move.w  d0,0x72(a6)                     | +05e
        lea     0x2e08ac.l,a0                   | +062
        jsr     0x28cd4.l                       | +068
        lea     .L07d71a(pc),a1                 | +06e
        move.l  a1,(a6)                         | +072
.L07d71a:
        jsr     Crab_Leg_PosFromParent_07dbda(pc) | +074
        jsr     0x28d70.l                       | +078
        movea.l 0xc(a6),a0                      | +07e
        movea.l 0xc(a0),a0                      | +082
        cmpi.b  #0x1,0x20(a0)                   | +086
        bne.w   .L07d73c                        | +08c
        lea     Crab_Leg_Step_07d76e(pc),a1     | +090
        move.l  a1,(a6)                         | +094
.L07d73c:
        cmpi.b  #0xff,0x20(a0)                  | +096
        bne.w   .L07d74c                        | +09c
        lea     Crab_Free_07dba8(pc),a1         | +0a0
        move.l  a1,(a6)                         | +0a4
.L07d74c:
        cmpi.b  #0x80,0x20(a0)                  | +0a6
        bne.w   .L07d75c                        | +0ac
        lea     Crab_Free_07dba8(pc),a1         | +0b0
        move.l  a1,(a6)                         | +0b4
.L07d75c:
        cmpi.b  #0x40,0x20(a0)                  | +0b6
        bne.w   SetHandlerRts_07d76c            | +0bc

| ----------------------------------------------------------------------------
|  Crab_Leg_Step_07d76e  @ $07D76E  (290 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Leg_Step_07d76e, "ax", @progbits
        .global Crab_Leg_Step_07d76e
Crab_Leg_Step_07d76e:
        jsr     0x267e2.l                       | +000
        clr.w   0x36(a6)                        | +006
        lea     0x2e08cc.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L07d78a(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07d78a:
        jsr     Crab_Leg_PosFromParent_07dbda(pc) | +01c
        lea     0x2e045e.l,a0                   | +020
        move.w  0x70(a6),d0                     | +026
        add.w   d0,d0                           | +02a
        add.w   d0,d0                           | +02c
        movea.l (a0,d0.w),a0                    | +02e
        movea.l 0xc(a6),a1                      | +032
        move.b  0x20(a1),d0                     | +036
        andi.w  #0xff,d0                        | +03a
        add.w   d0,d0                           | +03e
        add.w   d0,d0                           | +040
        move.w  (a0,d0.w),d1                    | +042
        btst    #0x0,0x3a(a6)                   | +046
        beq.w   .L07d7c0                        | +04c
        neg.w   d1                              | +050
.L07d7c0:
        add.w   0x22(a6),d1                     | +052
        move.w  d1,0x22(a6)                     | +056
        move.w  0x22(a6),0x76(a6)               | +05a
        move.w  0x2(a0,d0.w),d1                 | +060
        add.w   0x24(a6),d1                     | +064
        move.w  d1,0x24(a6)                     | +068
        lea     0x2e03f8.l,a0                   | +06c
        move.w  0x70(a6),d0                     | +072
        add.w   d0,d0                           | +076
        add.w   d0,d0                           | +078
        movea.l (a0,d0.w),a0                    | +07a
        move.b  0x20(a1),d0                     | +07e
        andi.w  #0xff,d0                        | +082
        add.w   d0,d0                           | +086
        move.w  (a0,d0.w),d1                    | +088
        add.w   0x36(a6),d1                     | +08c
        move.w  d1,0x36(a6)                     | +090
        lea     0x2e03e6.l,a0                   | +094
        move.w  (a0,d0.w),d0                    | +09a
        btst    #0x0,0x3a(a6)                   | +09e
        beq.w   .L07d820                        | +0a4
        neg.w   d0                              | +0a8
        subi.w  #0x80,d0                        | +0aa
        andi.w  #0xff,d0                        | +0ae
.L07d820:
        jsr     0x13c0e.l                       | +0b2
        move.w  d1,0x28(a6)                     | +0b8
        move.w  d2,0x2a(a6)                     | +0bc
        jsr     Crab_IntegrateVelFrac_07dbb0(pc) | +0c0
        jsr     0x28d70.l                       | +0c4
        bcc.w   .L07d842                        | +0ca
        lea     Crab_Leg_Detach_07d898(pc),a1   | +0ce
        move.l  a1,(a6)                         | +0d2
.L07d842:
        movea.l 0xc(a6),a0                      | +0d4
        movea.l 0xc(a0),a0                      | +0d8
        cmpi.b  #0xff,0x20(a0)                  | +0dc
        bne.w   .L07d85a                        | +0e2
        lea     Crab_Free_07dba8(pc),a1         | +0e6
        move.l  a1,(a6)                         | +0ea
.L07d85a:
        cmpi.b  #0x80,0x20(a0)                  | +0ec
        bne.w   .L07d86a                        | +0f2
        lea     Crab_Free_07dba8(pc),a1         | +0f6
        move.l  a1,(a6)                         | +0fa
.L07d86a:
        cmpi.b  #0x40,0x20(a0)                  | +0fc
        bne.w   .L07d87a                        | +102
        lea     Crab_Free_07dba8(pc),a1         | +106
        move.l  a1,(a6)                         | +10a
.L07d87a:
        move.w  0x22(a6),d0                     | +10c
        sub.w   0x76(a6),d0                     | +110
        bpl.w   .L07d888                        | +114
        neg.w   d0                              | +118
.L07d888:
        cmpi.w  #0x1c,d0                        | +11a
        blt.w   SetHandlerRts_07d896            | +11e

| ----------------------------------------------------------------------------
|  Crab_Leg_Detach_07d898  @ $07D898  (422 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Leg_Detach_07d898, "ax", @progbits
        .global Crab_Leg_Detach_07d898
Crab_Leg_Detach_07d898:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x7f,d0                        | +006
        move.w  0x28(a6),d1                     | +00a
        asr.w   #0x5,d1                         | +00e
        add.w   d1,d0                           | +010
        move.w  d0,0x28(a6)                     | +012
        move.w  0x2a(a6),d0                     | +016
        asr.w   #0x4,d0                         | +01a
        move.w  d0,0x2a(a6)                     | +01c
        move.w  #0x0,0x2c(a6)                   | +020
        move.w  #0xffe0,0x2e(a6)                | +026
        jsr     0x5e9b6.l                       | +02c
        andi.w  #0x3,d0                         | +032
        addq.w  #0x3,d0                         | +036
        add.w   d0,d0                           | +038
        add.w   d0,d0                           | +03a
        lea     0x2e035a.l,a0                   | +03c
        move.l  (a0,d0.w),0x5c(a6)              | +042
        move.w  #0xd000,d0                      | +048
        jsr     0x28134.l                       | +04c
        andi.w  #0xffe3,0x38(a6)                | +052
        ori.w   #0x14,0x38(a6)                  | +058
        cmpi.w  #0x2,0x70(a6)                   | +05e
        bne.w   .L07d90a                        | +064
        move.w  #0x1061,d0                      | +068
        jsr     0x2352.l                        | +06c
.L07d90a:
        move.w  #0x0,0x70(a6)                   | +072
        move.w  #0xffff,0x78(a6)                | +078
        lea     0x2e094a.l,a0                   | +07e
        jsr     0x28cd4.l                       | +084
        lea     .L07d928(pc),a1                 | +08a
        move.l  a1,(a6)                         | +08e
        .global Crab_Leg_Detach_07d898__L07d928
Crab_Leg_Detach_07d898__L07d928:
.L07d928:
        jsr     0x27cee.l                       | +090
        jsr     0x28d70.l                       | +096
        tst.w   0x78(a6)                        | +09c
        beq.w   .L07d9e8                        | +0a0
        jsr     0x2870a.l                       | +0a4
        bcc.w   .L07d99c                        | +0aa
        lea     0x5e766.l,a0                    | +0ae
        jsr     0x5e770.l                       | +0b4
        bclr    #0x3,0x13(a6)                   | +0ba
        cmpi.b  #0x2,0x58(a6)                   | +0c0
        beq.w   .L07d96c                        | +0c6
        cmpi.b  #0x3,0x58(a6)                   | +0ca
        bne.w   .L07d99c                        | +0d0
.L07d96c:
        lea     0x2e0ab0.l,a0                   | +0d4
        jsr     0x28cd4.l                       | +0da
        move.l  #0x2e0376,0x5c(a6)              | +0e0
        bset    #0x3,0x13(a6)                   | +0e8
        movea.l 0x50(a6),a0                     | +0ee
        move.w  0x28(a0),d0                     | +0f2
        add.w   0x28(a6),d0                     | +0f6
        move.w  d0,0x28(a6)                     | +0fa
        move.w  #0x0,0x78(a6)                   | +0fe
.L07d99c:
        tst.w   0x78(a6)                        | +104
        beq.w   .L07d9e8                        | +108
        lea     0x2e141a.l,a1                   | +10c
        move.w  0x70(a6),d1                     | +112
        add.w   d1,d1                           | +116
        add.w   d1,d1                           | +118
        movea.l (a1,d1.w),a1                    | +11a
        moveq   #0,d1                           | +11e
.L07d9b8:
        movea.l (a1,d1.w),a0                    | +120
        cmpa.l  #0xffffffff,a0                  | +124
        beq.w   .L07d9e8                        | +12a
        movem.l d1/a1,-(a7)                     | +12e
        move.l  a0,0x4c(a6)                     | +132
        jsr     0x283ca.l                       | +136
        jsr     0x283d8.l                       | +13c
        movem.l (a7)+,d1/a1                     | +142
        addq.w  #0x4,d1                         | +146
        btst    #0x1,0x13(a6)                   | +148
        beq.b   .L07d9b8                        | +14e
.L07d9e8:
        cmpi.w  #0x120,0x24(a6)                 | +150
        bgt.w   .L07da28                        | +156
        tst.b   0x21(a6)                        | +15a
        bne.w   .L07da28                        | +15e
        move.b  #0xff,0x21(a6)                  | +162
        movea.l 0xc(a6),a0                      | +168
        movea.l 0xc(a0),a0                      | +16c
        tst.b   0x84(a0)                        | +170
        bne.w   Crab_Knockback_07d3e2__L07d48c  | +174
        lea     0x78890.l,a1                    | +178
        jsr     0x4ae.l                         | +17e
        jsr     0x5dd02.l                       | +184
        move.w  #0xffff,0x38(a0)                | +18a
.L07da28:
        movea.l #0xffffffff,a0                  | +190
        lea     0x2e0340.l,a0                   | +196
        jsr     0x5dd56.l                       | +19c
        bcc.w   SetHandlerRts_07da44            | +1a2

| ----------------------------------------------------------------------------
|  Crab_Shard_Big_07da46  @ $07DA46  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Shard_Big_07da46, "ax", @progbits
        .global Crab_Shard_Big_07da46
Crab_Shard_Big_07da46:
        move.w  #0x0,d0                         | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x190,0x2a(a6)                 | +00e
        move.w  #0xfff0,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        jsr     0x5e9b6.l                       | +020
        andi.w  #0x3ff,d0                       | +026
        move.w  d0,0x28(a6)                     | +02a
        jsr     0x5e9b6.l                       | +02e
        andi.w  #0x3ff,d0                       | +034
        add.w   d0,0x2a(a6)                     | +038
        jsr     0x5e9b6.l                       | +03c
        andi.w  #0x1,d0                         | +042
        eor.b   d0,0x3a(a6)                     | +046
        btst    #0x0,0x3a(a6)                   | +04a
        beq.w   .L07da9e                        | +050
        neg.w   0x28(a6)                        | +054
.L07da9e:
        jsr     0x5e9b6.l                       | +058
        andi.w  #0x3,d0                         | +05e
        add.w   d0,d0                           | +062
        add.w   d0,d0                           | +064
        lea     0x2e035a.l,a0                   | +066
        move.l  (a0,d0.w),0x5c(a6)              | +06c
        move.w  #0xd000,d0                      | +072
        jsr     0x28134.l                       | +076
        andi.w  #0xffe3,0x38(a6)                | +07c
        ori.w   #0x14,0x38(a6)                  | +082
        moveq   #0,d0                           | +088
        move.w  d0,0x70(a6)                     | +08a
        move.w  d0,0x78(a6)                     | +08e
        bset    #0x3,0x13(a6)                   | +092
        lea     0x2e0ab0.l,a0                   | +098
        jsr     0x28cd4.l                       | +09e
        lea     Crab_Leg_Detach_07d898__L07d928(pc),a1 | +0a4
        move.l  a1,(a6)                         | +0a8
        bra.w   Crab_Leg_Detach_07d898__L07d928 | +0aa

| ----------------------------------------------------------------------------
|  Crab_Shard_Small_07daf4  @ $07DAF4  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Shard_Small_07daf4, "ax", @progbits
        .global Crab_Shard_Small_07daf4
Crab_Shard_Small_07daf4:
        move.w  #0x0,d0                         | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x118,0x2a(a6)                 | +00e
        move.w  #0xfff8,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        jsr     0x5e9b6.l                       | +020
        andi.w  #0x3ff,d0                       | +026
        move.w  d0,0x28(a6)                     | +02a
        jsr     0x5e9b6.l                       | +02e
        andi.w  #0x1ff,d0                       | +034
        add.w   d0,0x2a(a6)                     | +038
        jsr     0x5e9b6.l                       | +03c
        andi.w  #0x1,d0                         | +042
        eor.b   d0,0x3a(a6)                     | +046
        btst    #0x0,0x3a(a6)                   | +04a
        beq.w   .L07db4c                        | +050
        neg.w   0x28(a6)                        | +054
.L07db4c:
        jsr     0x5e9b6.l                       | +058
        andi.w  #0x3,d0                         | +05e
        add.w   d0,d0                           | +062
        add.w   d0,d0                           | +064
        lea     0x2e035a.l,a0                   | +066
        move.l  (a0,d0.w),0x5c(a6)              | +06c
        move.w  #0xd000,d0                      | +072
        jsr     0x28134.l                       | +076
        andi.w  #0xffe3,0x38(a6)                | +07c
        ori.w   #0x14,0x38(a6)                  | +082
        moveq   #0,d0                           | +088
        move.w  d0,0x70(a6)                     | +08a
        move.w  d0,0x78(a6)                     | +08e
        bset    #0x3,0x13(a6)                   | +092
        lea     0x2e0ab0.l,a0                   | +098
        jsr     0x28cd4.l                       | +09e
        lea     Crab_Leg_Detach_07d898__L07d928(pc),a1 | +0a4
        move.l  a1,(a6)                         | +0a8
        bra.w   Crab_Leg_Detach_07d898__L07d928 | +0aa

| ----------------------------------------------------------------------------
|  Crab_MarkDetached_07dba2  @ $07DBA2  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_MarkDetached_07dba2, "ax", @progbits
        .global Crab_MarkDetached_07dba2
Crab_MarkDetached_07dba2:
        move.b  #0x40,0x20(a6)                  | +000

| ----------------------------------------------------------------------------
|  Crab_Free_07dba8  @ $07DBA8  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Free_07dba8, "ax", @progbits
        .global Crab_Free_07dba8
Crab_Free_07dba8:
        jmp     0x518.l                         | +000

| ----------------------------------------------------------------------------
|  Crab_Rts_07dbae  @ $07DBAE  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Rts_07dbae, "ax", @progbits
        .global Crab_Rts_07dbae
Crab_Rts_07dbae:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Crab_IntegrateVelFrac_07dbb0  @ $07DBB0  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_IntegrateVelFrac_07dbb0, "ax", @progbits
        .global Crab_IntegrateVelFrac_07dbb0
Crab_IntegrateVelFrac_07dbb0:
        move.w  0x28(a6),d0                     | +000
        move.w  d0,d1                           | +004
        asr.w   #0x8,d0                         | +006
        add.b   d1,0x26(a6)                     | +008
        moveq   #0,d1                           | +00c
        addx.w  d1,d0                           | +00e
        add.w   d0,0x22(a6)                     | +010
        move.w  0x2a(a6),d0                     | +014
        move.w  d0,d1                           | +018
        asr.w   #0x8,d0                         | +01a
        add.b   d1,0x27(a6)                     | +01c
        moveq   #0,d1                           | +020
        addx.w  d1,d0                           | +022
        add.w   d0,0x24(a6)                     | +024
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Crab_Leg_PosFromParent_07dbda  @ $07DBDA  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Leg_PosFromParent_07dbda, "ax", @progbits
        .global Crab_Leg_PosFromParent_07dbda
Crab_Leg_PosFromParent_07dbda:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),d0                     | +004
        add.w   0x72(a6),d0                     | +008
        move.w  d0,0x22(a6)                     | +00c
        move.w  0x24(a0),d0                     | +010
        cmpi.w  #0x2,0x70(a6)                   | +014
        bne.w   .L07dc00                        | +01a
        movea.l 0xc(a0),a1                      | +01e
        add.w   0x74(a1),d0                     | +022
.L07dc00:
        add.w   0x74(a6),d0                     | +026
        move.w  d0,0x24(a6)                     | +02a
        move.w  0x38(a0),d0                     | +02e
        andi.w  #0xffe3,d0                      | +032
        ori.w   #0x8,d0                         | +036

| ----------------------------------------------------------------------------
|  Crab_Claw_PosFromParent_07dc1a  @ $07DC1A  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Claw_PosFromParent_07dc1a, "ax", @progbits
        .global Crab_Claw_PosFromParent_07dc1a
Crab_Claw_PosFromParent_07dc1a:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),d0                     | +00a
        add.w   0x74(a0),d0                     | +00e
        move.w  d0,0x24(a6)                     | +012
        move.w  0x38(a0),d0                     | +016
        add.w   0x82(a6),d0                     | +01a

| ----------------------------------------------------------------------------
|  Crab_Tmpl140_07dc3e  @ $07DC3E  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Tmpl140_07dc3e, "ax", @progbits
        .global Crab_Tmpl140_07dc3e
Crab_Tmpl140_07dc3e:
        move.w  #0x4b,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xa,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        moveq   #0,d0                           | +016
        move.b  d0,0x20(a6)                     | +018
        move.b  d0,0x21(a6)                     | +01c
        lea     0x2bda80.l,a0                   | +020
        jsr     0x799de.l                       | +026
        move.w  d0,0x66(a6)                     | +02c
        lea     0x2bdb02.l,a0                   | +030
        jsr     0x799de.l                       | +036
        move.w  d0,0x7e(a6)                     | +03c
        jsr     0x267e2.l                       | +040
        move.b  0x98(a6),d0                     | +046
        andi.w  #0xff,d0                        | +04a
        lsl.w   #0x4,d0                         | +04e
        move.w  d0,0x7c(a6)                     | +050
        neg.w   d0                              | +054
        move.w  d0,0x7a(a6)                     | +056
        move.b  #0x0,0x3a(a6)                   | +05a
        cmpi.w  #0xa0,0x22(a6)                  | +060
        blt.w   SetTaskHandler_07dcae           | +066
        move.b  #0x1,0x3a(a6)                   | +06a

| ----------------------------------------------------------------------------
|  Crab_Patrol_Walk_07dcb6  @ $07DCB6  (290 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Patrol_Walk_07dcb6, "ax", @progbits
        .global Crab_Patrol_Walk_07dcb6
Crab_Patrol_Walk_07dcb6:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x4001,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0xc,0x38(a6)                   | +016
        lea     Crab_Claw_Init_07d4aa(pc),a1    | +01c
        jsr     0x4ae.l                         | +020
        jsr     0x5dd02.l                       | +026
        lea     Crab_Claw_Init_07d4aa__L07d520(pc),a1 | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        move.w  #0x20,0x28(a6)                  | +03c
        btst    #0x0,0x3a(a6)                   | +042
        beq.w   .L07dd06                        | +048
        neg.w   0x28(a6)                        | +04c
.L07dd06:
        move.w  0x28(a6),0x2c(a6)               | +050
        moveq   #0,d0                           | +056
        move.w  d0,0x28(a6)                     | +058
        move.b  d0,0x72(a6)                     | +05c
        move.w  d0,0x74(a6)                     | +060
        move.w  d0,0x76(a6)                     | +064
        move.b  0x99(a6),0x84(a6)               | +068
        lea     0x2e057e.l,a0                   | +06e
        jsr     0x28cd4.l                       | +074
        lea     .L07dd36(pc),a1                 | +07a
        move.l  a1,(a6)                         | +07e
.L07dd36:
        jsr     0x27afc.l                       | +080
        bcc.w   .L07dd46                        | +086
        lea     Crab_Patrol_Pause_07dde0(pc),a1 | +08a
        move.l  a1,(a6)                         | +08e
.L07dd46:
        move.w  0x28(a6),d0                     | +090
        cmp.w   0x7c(a6),d0                     | +094
        bgt.w   .L07dd5a                        | +098
        cmp.w   0x7a(a6),d0                     | +09c
        bge.w   .L07dd72                        | +0a0
.L07dd5a:
        clr.w   0x2c(a6)                        | +0a4
        move.w  0x7c(a6),0x28(a6)               | +0a8
        btst    #0x0,0x3a(a6)                   | +0ae
        beq.w   .L07dd72                        | +0b4
        neg.w   0x28(a6)                        | +0b8
.L07dd72:
        jsr     0x28d70.l                       | +0bc
        jsr     0x2870a.l                       | +0c2
        bcc.w   .L07dd94                        | +0c8
        lea     0x5e766.l,a0                    | +0cc
        jsr     0x5e770.l                       | +0d2
        bclr    #0x3,0x13(a6)                   | +0d8
.L07dd94:
        jsr     Crab_Patrol_PlayerNear_07df6c(pc) | +0de
        bcc.w   .L07dda2                        | +0e2
        lea     Crab_Patrol_Pause_07dde0(pc),a1 | +0e6
        move.l  a1,(a6)                         | +0ea
.L07dda2:
        jsr     0x27eba.l                       | +0ec
        bcc.w   .L07ddb2                        | +0f2
        lea     Crab_Knockback_07d3e2(pc),a1    | +0f6
        move.l  a1,(a6)                         | +0fa
.L07ddb2:
        jsr     0x28758.l                       | +0fc
        bcc.w   .L07ddc2                        | +102
        lea     Crab_Death_07d332(pc),a1        | +106
        move.l  a1,(a6)                         | +10a
.L07ddc2:
        movea.l #0xffffffff,a0                  | +10c
        lea     0x2e0b5c.l,a0                   | +112
        jsr     0x5dd5c.l                       | +118
        bcc.w   SetHandlerRts_07ddde            | +11e

| ----------------------------------------------------------------------------
|  Crab_Patrol_Pause_07dde0  @ $07DDE0  (166 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Patrol_Pause_07dde0, "ax", @progbits
        .global Crab_Patrol_Pause_07dde0
Crab_Patrol_Pause_07dde0:
        jsr     0x267e2.l                       | +000
        clr.w   0x74(a6)                        | +006
        move.w  #0x109e,d0                      | +00a
        jsr     0x2222.l                        | +00e
        lea     0x2e04d6.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L07de06(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L07de06:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L07de2e                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
.L07de2e:
        cmpi.b  #0xff,0x21(a6)                  | +04e
        beq.w   .L07de50                        | +054
        addq.w  #0x1,0x76(a6)                   | +058
        cmpi.w  #0x3c,0x76(a6)                  | +05c
        blt.w   .L07de4c                        | +062
        lea     Crab_Patrol_WalkBack_07de8e(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L07de4c:
        jsr     Crab_Patrol_PlayerNear_07df6c(pc) | +06c
.L07de50:
        jsr     0x27eba.l                       | +070
        bcc.w   .L07de60                        | +076
        lea     Crab_Knockback_07d3e2(pc),a1    | +07a
        move.l  a1,(a6)                         | +07e
.L07de60:
        jsr     0x28758.l                       | +080
        bcc.w   .L07de70                        | +086
        lea     Crab_Death_07d332(pc),a1        | +08a
        move.l  a1,(a6)                         | +08e
.L07de70:
        movea.l #0xffffffff,a0                  | +090
        lea     0x2e0b5c.l,a0                   | +096
        jsr     0x5dd5c.l                       | +09c
        bcc.w   SetHandlerRts_07de8c            | +0a2

| ----------------------------------------------------------------------------
|  Crab_Patrol_WalkBack_07de8e  @ $07DE8E  (214 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Patrol_WalkBack_07de8e, "ax", @progbits
        .global Crab_Patrol_WalkBack_07de8e
Crab_Patrol_WalkBack_07de8e:
        jsr     0x267e2.l                       | +000
        move.w  #0xffe0,0x28(a6)                | +006
        btst    #0x0,0x3a(a6)                   | +00c
        beq.w   .L07dea8                        | +012
        neg.w   0x28(a6)                        | +016
.L07dea8:
        move.w  0x28(a6),0x2c(a6)               | +01a
        moveq   #0,d0                           | +020
        move.w  d0,0x28(a6)                     | +022
        move.w  d0,0x74(a6)                     | +026
        lea     0x2e050a.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L07deca(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L07deca:
        jsr     0x27afc.l                       | +03c
        bcc.w   .L07dee0                        | +042
        move.b  #0xff,0x21(a6)                  | +046
        lea     Crab_Patrol_Pause_07dde0(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L07dee0:
        move.w  0x28(a6),d0                     | +052
        cmp.w   0x7c(a6),d0                     | +056
        bgt.w   .L07def4                        | +05a
        cmp.w   0x7a(a6),d0                     | +05e
        bge.w   .L07df0c                        | +062
.L07def4:
        clr.w   0x2c(a6)                        | +066
        move.w  0x7a(a6),0x28(a6)               | +06a
        btst    #0x0,0x3a(a6)                   | +070
        beq.w   .L07df0c                        | +076
        neg.w   0x28(a6)                        | +07a
.L07df0c:
        jsr     0x28d70.l                       | +07e
        jsr     0x2870a.l                       | +084
        bcc.w   .L07df2e                        | +08a
        lea     0x5e766.l,a0                    | +08e
        jsr     0x5e770.l                       | +094
        bclr    #0x3,0x13(a6)                   | +09a
.L07df2e:
        jsr     0x27eba.l                       | +0a0
        bcc.w   .L07df3e                        | +0a6
        lea     Crab_Knockback_07d3e2(pc),a1    | +0aa
        move.l  a1,(a6)                         | +0ae
.L07df3e:
        jsr     0x28758.l                       | +0b0
        bcc.w   .L07df4e                        | +0b6
        lea     Crab_Death_07d332(pc),a1        | +0ba
        move.l  a1,(a6)                         | +0be
.L07df4e:
        movea.l #0xffffffff,a0                  | +0c0
        lea     0x2e0b5c.l,a0                   | +0c6
        jsr     0x5dd5c.l                       | +0cc
        bcc.w   SetHandlerRts_07df6a            | +0d2

| ----------------------------------------------------------------------------
|  Crab_Patrol_PlayerNear_07df6c  @ $07DF6C  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Crab_Patrol_PlayerNear_07df6c, "ax", @progbits
        .global Crab_Patrol_PlayerNear_07df6c
Crab_Patrol_PlayerNear_07df6c:
        cmpi.b  #0x1,0x20(a6)                   | +000
        beq.w   ClearC_07dfa2                   | +006
        clr.w   0x76(a6)                        | +00a
        jsr     0x5e0d4.l                       | +00e
        move.w  0x22(a6),d0                     | +014
        sub.w   0x22(a0),d0                     | +018
        bpl.w   .L07df8e                        | +01c
        neg.w   d0                              | +020
.L07df8e:
        cmp.w   0x7e(a6),d0                     | +022
        bhi.w   ClearC_07dfa2                   | +026
        move.b  #0x1,0x20(a6)                   | +02a

| ----------------------------------------------------------------------------
|  Carrier_RetGate_07dfa8  @ $07DFA8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_RetGate_07dfa8, "ax", @progbits
        .global Carrier_RetGate_07dfa8
Carrier_RetGate_07dfa8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_07dfbe                    | +00c

| ----------------------------------------------------------------------------
|  Carrier_Tmpl129_07dfc4  @ $07DFC4  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Tmpl129_07dfc4, "ax", @progbits
        .global Carrier_Tmpl129_07dfc4
Carrier_Tmpl129_07dfc4:
        moveq   #0,d0                           | +000
        move.b  d0,0x20(a6)                     | +002
        move.b  d0,0x21(a6)                     | +006
        move.b  d0,0x82(a6)                     | +00a
        jsr     0x267e2.l                       | +00e
        move.w  #0x5000,0x38(a6)                | +014
        lea     0x2bcb42.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        move.w  d0,0x66(a6)                     | +026
        lea     Carrier_Child_FreeWhenDone_07e2f2(pc),a1 | +02a
        jsr     0x4ae.l                         | +02e

| ----------------------------------------------------------------------------
|  Carrier_Tmpl130_07e000  @ $07E000  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Tmpl130_07e000, "ax", @progbits
        .global Carrier_Tmpl130_07e000
Carrier_Tmpl130_07e000:
        moveq   #0,d0                           | +000
        move.b  d0,0x20(a6)                     | +002
        move.b  d0,0x21(a6)                     | +006
        move.b  d0,0x82(a6)                     | +00a
        jsr     0x267e2.l                       | +00e
        move.w  #0x5000,0x38(a6)                | +014
        lea     0x2bcb42.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        move.w  d0,0x66(a6)                     | +026
        lea     Carrier_Child_FreeWhenDone_07e2f2(pc),a1 | +02a
        jsr     0x4ae.l                         | +02e

| ----------------------------------------------------------------------------
|  Carrier_Tmpl131_07e03c  @ $07E03C  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Tmpl131_07e03c, "ax", @progbits
        .global Carrier_Tmpl131_07e03c
Carrier_Tmpl131_07e03c:
        clr.b   0x20(a6)                        | +000
        clr.b   0x21(a6)                        | +004
        move.b  #0xff,0x82(a6)                  | +008
        jsr     0x267e2.l                       | +00e
        move.w  #0x5000,0x38(a6)                | +014
        lea     0x2bcc46.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        move.w  d0,0x66(a6)                     | +026
        lea     Carrier_Child_FreeWhenDone_07e2f2(pc),a1 | +02a
        jsr     0x4ae.l                         | +02e

| ----------------------------------------------------------------------------
|  Carrier_InitWithHatchB_07e078  @ $07E078  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_InitWithHatchB_07e078, "ax", @progbits
        .global Carrier_InitWithHatchB_07e078
Carrier_InitWithHatchB_07e078:
        lea     Carrier_Spawner_Init_07eb76(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        bra.w   Carrier_Init_07e08c__L07e09c    | +010

| ----------------------------------------------------------------------------
|  Carrier_Init_07e08c  @ $07E08C  (464 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Init_07e08c, "ax", @progbits
        .global Carrier_Init_07e08c
Carrier_Init_07e08c:
        lea     Carrier_Spawner_Init_07eb76__L07eb7e(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        .global Carrier_Init_07e08c__L07e09c
Carrier_Init_07e08c__L07e09c:
.L07e09c:
        moveq   #0,d0                           | +010
        move.w  #0x24,0x70(a6)                  | +012
        move.w  d0,0x72(a6)                     | +018
        move.w  d0,0x74(a6)                     | +01c
        move.w  d0,0x76(a6)                     | +020
        move.w  d0,0x78(a6)                     | +024
        move.w  #0x3c,0x7a(a6)                  | +028
        move.w  d0,0x7c(a6)                     | +02e
        move.l  d0,0x7e(a6)                     | +032
        move.w  0x66(a6),0x80(a6)               | +036
        lea     0x2e26b0.l,a0                   | +03c
        move.l  a0,0x48(a6)                     | +042
        lea     Carrier_Hull_Init_07e30e(pc),a1 | +046
        jsr     0x4ae.l                         | +04a
        jsr     0x5dd02.l                       | +050
        lea     Carrier_HullB_Init_07e40c(pc),a1 | +056
        jsr     0x4ae.l                         | +05a
        jsr     0x5dd02.l                       | +060
        lea     Carrier_HullC_Init_07e504(pc),a1 | +066
        jsr     0x4ae.l                         | +06a
        jsr     0x5dd02.l                       | +070
        lea     Carrier_Hatch_Init_07e8f2(pc),a1 | +076
        jsr     0x4ae.l                         | +07a
        jsr     0x5dd02.l                       | +080
        lea     Carrier_Mark_Left_07e854(pc),a1 | +086
        jsr     0x4ae.l                         | +08a
        jsr     0x5dd02.l                       | +090
        lea     Carrier_Mark_Right_07e86a(pc),a1 | +096
        jsr     0x4ae.l                         | +09a
        jsr     0x5dd02.l                       | +0a0
        lea     .L07e138(pc),a1                 | +0a6
        move.l  a1,(a6)                         | +0aa
.L07e138:
        jsr     Squad_AIDecide_07fee8(pc)       | +0ac
        jsr     0x27cee.l                       | +0b0
        move.w  0x22(a6),d0                     | +0b6
        move.w  0x24(a6),d1                     | +0ba
        addi.w  #0xffa8,d0                      | +0be
        addi.w  #0x20,d1                        | +0c2
        move.w  #0x30,d2                        | +0c6
        jsr     0x99812.l                       | +0ca
        move.w  0x22(a6),d0                     | +0d0
        move.w  0x24(a6),d1                     | +0d4
        addi.w  #0x38,d0                        | +0d8
        addi.w  #0x20,d1                        | +0dc
        move.w  #0x30,d2                        | +0e0
        jsr     0x99812.l                       | +0e4
        move.w  0x22(a6),d0                     | +0ea
        move.w  0x24(a6),d1                     | +0ee
        addi.w  #0xffd0,d0                      | +0f2
        addq.w  #0x8,d1                         | +0f6
        move.w  #0x70,d2                        | +0f8
        jsr     0x99812.l                       | +0fc
        addq.w  #0x1,0x72(a6)                   | +102
        move.w  0x72(a6),d0                     | +106
        cmp.w   0x70(a6),d0                     | +10a
        blt.w   .L07e1d4                        | +10e
        jsr     0x5e9b6.l                       | +112
        andi.w  #0x3,d0                         | +118
        add.w   d0,d0                           | +11c
        lea     0x2e1d4a.l,a0                   | +11e
        move.w  (a0,d0.w),d0                    | +124
        move.w  d0,0x72(a6)                     | +128
        lea     Carrier_Cannon_Init_07eac2(pc),a1 | +12c
        jsr     0x4ae.l                         | +130
        jsr     0x5dd02.l                       | +136
        addi.w  #0x38,0x22(a0)                  | +13c
        addi.w  #0x48,0x24(a0)                  | +142
.L07e1d4:
        jsr     0x2870a.l                       | +148
        bcc.w   .L07e1f0                        | +14e
        lea     0x5e766.l,a0                    | +152
        jsr     0x5e770.l                       | +158
        bclr    #0x3,0x13(a6)                   | +15e
.L07e1f0:
        jsr     Squad_TrackArc_07fdfa__L07feb4(pc) | +164
        jsr     0x28758.l                       | +168
        bcc.w   .L07e254                        | +16e
        move.b  #0x3,0x20(a6)                   | +172
        subq.w  #0x1,0x7a(a6)                   | +178
        bne.w   .L07e254                        | +17c
        move.b  #0x1,0x21(a6)                   | +180
        move.w  #0x0,0x76(a6)                   | +186
        lea     0xffff.w,a0                     | +18c
        move.l  a0,0x48(a6)                     | +190
        lea     Carrier_Gunner_Init_07f674(pc),a1 | +194
        jsr     0x4ae.l                         | +198
        jsr     0x5dd02.l                       | +19e
        lea     Carrier_Cockpit_Init_07e602(pc),a1 | +1a4
        jsr     0x4ae.l                         | +1a8
        jsr     0x5dd02.l                       | +1ae
        move.b  0x9a(a6),d0                     | +1b4
        move.w  #0x9b,d1                        | +1b8
        jsr     0x9a7aa.l                       | +1bc
        lea     Carrier_Phase4_07e264(pc),a1    | +1c2
        move.l  a1,(a6)                         | +1c6
.L07e254:
        jsr     Squad_LeaderNotify_07fda8__L07fdc4(pc) | +1c8
        bcc.w   SetHandlerRts_07e262            | +1cc

| ----------------------------------------------------------------------------
|  Carrier_Phase4_07e264  @ $07E264  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Phase4_07e264, "ax", @progbits
        .global Carrier_Phase4_07e264
Carrier_Phase4_07e264:
        jsr     0x2783a.l                       | +000
        cmpi.b  #0x4,0x20(a6)                   | +006
        bne.w   .L07e286                        | +00c
        bclr    #0x1,0x12(a6)                   | +010
        move.b  #0x2,0x21(a6)                   | +016
        lea     Carrier_Phase5_07e29a(pc),a1    | +01c
        move.l  a1,(a6)                         | +020
.L07e286:
        jsr     Squad_TrackArc_07fdfa__L07feb4(pc) | +022
        jsr     Squad_LeaderNotify_07fda8__L07fdc4(pc) | +026
        bcc.w   SetHandlerRts_07e298            | +02a

| ----------------------------------------------------------------------------
|  Carrier_Phase5_07e29a  @ $07E29A  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Phase5_07e29a, "ax", @progbits
        .global Carrier_Phase5_07e29a
Carrier_Phase5_07e29a:
        cmpi.b  #0x5,0x20(a6)                   | +000
        bne.w   .L07e2b8                        | +006
        move.b  #0x3,0x21(a6)                   | +00a
        lea     0xffff.w,a0                     | +010
        move.l  a0,0x48(a6)                     | +014
        lea     Carrier_WaitChildrenGone_07e2d2(pc),a1 | +018
        move.l  a1,(a6)                         | +01c
.L07e2b8:
        jsr     0x2783a.l                       | +01e
        jsr     Squad_TrackArc_07fdfa__L07feb4(pc) | +024
        jsr     Squad_LeaderNotify_07fda8__L07fdc4(pc) | +028
        bcc.w   SetHandlerRts_07e2d0            | +02c

| ----------------------------------------------------------------------------
|  Carrier_WaitChildrenGone_07e2d2  @ $07E2D2  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_WaitChildrenGone_07e2d2, "ax", @progbits
        .global Carrier_WaitChildrenGone_07e2d2
Carrier_WaitChildrenGone_07e2d2:
        cmpi.b  #0xff,0x21(a6)                  | +000
        bne.w   .L07e2e2                        | +006
        lea     TaskHandler_07fdbc(pc),a1       | +00a
        move.l  a1,(a6)                         | +00e
.L07e2e2:
        jsr     Squad_LeaderNotify_07fda8__L07fdc4(pc) | +010
        bcc.w   SetHandlerRts_07e2f0            | +014

| ----------------------------------------------------------------------------
|  Carrier_Child_FreeWhenDone_07e2f2  @ $07E2F2  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Child_FreeWhenDone_07e2f2, "ax", @progbits
        .global Carrier_Child_FreeWhenDone_07e2f2
Carrier_Child_FreeWhenDone_07e2f2:
        clr.b   0x10e39a.l                      | +000
        movea.l 0xc(a6),a0                      | +006
        cmpi.b  #0x2,0x21(a0)                   | +00a
        bne.w   .L07e30c                        | +010
        jmp     0x518.l                         | +014
.L07e30c:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Carrier_Hull_Init_07e30e  @ $07E30E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Hull_Init_07e30e, "ax", @progbits
        .global Carrier_Hull_Init_07e30e
Carrier_Hull_Init_07e30e:
        move.w  #0x24,d1                        | +000
        jsr     0x236e.l                        | +004
        clr.b   0x20(a6)                        | +00a
        clr.b   0x21(a6)                        | +00e
        movea.l 0xc(a6),a0                      | +012
        move.w  0x66(a0),d0                     | +016
        divu.w  #0x3,d0                         | +01a
        move.w  d0,0x66(a6)                     | +01e
        move.w  d0,0x72(a6)                     | +022
        move.w  #0xffff,0x70(a6)                | +026

| ----------------------------------------------------------------------------
|  Carrier_Hull_Rank_07e33a  @ $07E33A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Hull_Rank_07e33a, "ax", @progbits
        .global Carrier_Hull_Rank_07e33a
Carrier_Hull_Rank_07e33a:
        bclr    #0x3,0x13(a6)                   | +000
        jsr     Squad_AIDecide_07fee8__L07fffc(pc) | +006
        cmp.w   0x70(a6),d0                     | +00a

| ----------------------------------------------------------------------------
|  Carrier_Hull_RankChanged_07e348  @ $07E348  (188 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Hull_RankChanged_07e348, "ax", @progbits
        .global Carrier_Hull_RankChanged_07e348
Carrier_Hull_RankChanged_07e348:
        beq.w   .L07e3ac                        | +000
        move.w  d0,0x70(a6)                     | +004
        lea     0x2e1d52.l,a0                   | +008
        add.w   d0,d0                           | +00e
        add.w   d0,d0                           | +010
        movea.l (a0,d0.w),a0                    | +012
        jsr     0x28cd4.l                       | +016
        lea     0x77f6a.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        addi.w  #0x28,0x38(a0)                  | +02e
        subi.w  #0x48,0x22(a0)                  | +034
        move.w  0x54(a6),0x22(a0)               | +03a
        move.w  0x56(a6),0x24(a0)               | +040
        lea     0x2e2b0c.l,a1                   | +046
        jsr     0x77c7e.l                       | +04c
        move.w  0x54(a6),0x22(a0)               | +052
        move.w  0x56(a6),0x24(a0)               | +058
        addi.w  #0x28,0x38(a0)                  | +05e
.L07e3ac:
        lea     .L07e3b2(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L07e3b2:
        movea.l 0xc(a6),a0                      | +06a
        move.w  0x22(a0),0x22(a6)               | +06e
        move.w  0x24(a0),0x24(a6)               | +074
        jsr     0x28d70.l                       | +07a
        jsr     0x2870a.l                       | +080
        bcc.w   .L07e3d8                        | +086
        lea     Carrier_Hull_Rank_07e33a(pc),a1 | +08a
        move.l  a1,(a6)                         | +08e
.L07e3d8:
        movea.l 0xc(a6),a0                      | +090
        cmpi.b  #0x3,0x20(a0)                   | +094
        bne.w   .L07e3ec                        | +09a
        lea     Squad_HatchRow3Spawn_07fc1a__L07fc38(pc),a1 | +09e
        move.l  a1,(a6)                         | +0a2
.L07e3ec:
        jsr     0x28758.l                       | +0a4
        bcc.w   .L07e3fc                        | +0aa
        lea     Squad_HatchRow3Spawn_07fc1a(pc),a1 | +0ae
        move.l  a1,(a6)                         | +0b2
.L07e3fc:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +0b4
        bcc.w   SetHandlerRts_07e40a            | +0b8

| ----------------------------------------------------------------------------
|  Carrier_HullB_Init_07e40c  @ $07E40C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_HullB_Init_07e40c, "ax", @progbits
        .global Carrier_HullB_Init_07e40c
Carrier_HullB_Init_07e40c:
        move.w  #0x24,d1                        | +000
        jsr     0x236e.l                        | +004
        clr.b   0x20(a6)                        | +00a
        clr.b   0x21(a6)                        | +00e
        movea.l 0xc(a6),a0                      | +012
        move.w  0x66(a0),d0                     | +016
        divu.w  #0x3,d0                         | +01a
        move.w  d0,0x66(a6)                     | +01e
        move.w  d0,0x72(a6)                     | +022
        move.w  #0xffff,0x70(a6)                | +026

| ----------------------------------------------------------------------------
|  Carrier_HullB_Run_07e438  @ $07E438  (196 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_HullB_Run_07e438, "ax", @progbits
        .global Carrier_HullB_Run_07e438
Carrier_HullB_Run_07e438:
        bclr    #0x3,0x13(a6)                   | +000
        jsr     Squad_AIDecide_07fee8__L07fffc(pc) | +006
        cmp.w   0x70(a6),d0                     | +00a
        beq.w   .L07e4a4                        | +00e
        move.w  d0,0x70(a6)                     | +012
        lea     0x2e1de4.l,a0                   | +016
        add.w   d0,d0                           | +01c
        add.w   d0,d0                           | +01e
        movea.l (a0,d0.w),a0                    | +020
        jsr     0x28cd4.l                       | +024
        lea     0x77f6a.l,a1                    | +02a
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        move.w  0x54(a6),0x22(a0)               | +03c
        move.w  0x56(a6),0x24(a0)               | +042
        addi.w  #0x28,0x38(a0)                  | +048
        lea     0x2e2b0c.l,a1                   | +04e
        jsr     0x77c7e.l                       | +054
        move.w  0x54(a6),0x22(a0)               | +05a
        move.w  0x56(a6),0x24(a0)               | +060
        addi.w  #0x28,0x38(a0)                  | +066
.L07e4a4:
        lea     .L07e4aa(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L07e4aa:
        movea.l 0xc(a6),a0                      | +072
        move.w  0x22(a0),0x22(a6)               | +076
        move.w  0x24(a0),0x24(a6)               | +07c
        jsr     0x28d70.l                       | +082
        jsr     0x2870a.l                       | +088
        bcc.w   .L07e4d0                        | +08e
        lea     Carrier_HullB_Run_07e438(pc),a1 | +092
        move.l  a1,(a6)                         | +096
.L07e4d0:
        movea.l 0xc(a6),a0                      | +098
        cmpi.b  #0x3,0x20(a0)                   | +09c
        bne.w   .L07e4e4                        | +0a2
        lea     Squad_HatchRow3Spawn_07fc1a__L07fc6c(pc),a1 | +0a6
        move.l  a1,(a6)                         | +0aa
.L07e4e4:
        jsr     0x28758.l                       | +0ac
        bcc.w   .L07e4f4                        | +0b2
        lea     Squad_HatchRow3Spawn_07fc1a__L07fc54(pc),a1 | +0b6
        move.l  a1,(a6)                         | +0ba
.L07e4f4:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +0bc
        bcc.w   SetHandlerRts_07e502            | +0c0

| ----------------------------------------------------------------------------
|  Carrier_HullC_Init_07e504  @ $07E504  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_HullC_Init_07e504, "ax", @progbits
        .global Carrier_HullC_Init_07e504
Carrier_HullC_Init_07e504:
        move.w  #0x24,d1                        | +000
        jsr     0x236e.l                        | +004
        clr.b   0x20(a6)                        | +00a
        clr.b   0x21(a6)                        | +00e
        movea.l 0xc(a6),a0                      | +012
        move.w  0x66(a0),d0                     | +016
        divu.w  #0x3,d0                         | +01a
        move.w  d0,0x66(a6)                     | +01e
        move.w  d0,0x72(a6)                     | +022
        move.w  #0xffff,0x70(a6)                | +026

| ----------------------------------------------------------------------------
|  Carrier_HullC_Run_07e530  @ $07E530  (202 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_HullC_Run_07e530, "ax", @progbits
        .global Carrier_HullC_Run_07e530
Carrier_HullC_Run_07e530:
        bclr    #0x3,0x13(a6)                   | +000
        jsr     Squad_AIDecide_07fee8__L07fffc(pc) | +006
        cmp.w   0x70(a6),d0                     | +00a
        beq.w   .L07e5a2                        | +00e
        move.w  d0,0x70(a6)                     | +012
        lea     0x2e1e76.l,a0                   | +016
        add.w   d0,d0                           | +01c
        add.w   d0,d0                           | +01e
        movea.l (a0,d0.w),a0                    | +020
        jsr     0x28cd4.l                       | +024
        lea     0x77f6a.l,a1                    | +02a
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        addi.w  #0x40,0x22(a0)                  | +03c
        move.w  0x54(a6),0x22(a0)               | +042
        move.w  0x56(a6),0x24(a0)               | +048
        addi.w  #0x28,0x38(a0)                  | +04e
        lea     0x2e2b0c.l,a1                   | +054
        jsr     0x77c7e.l                       | +05a
        move.w  0x54(a6),0x22(a0)               | +060
        move.w  0x56(a6),0x24(a0)               | +066
        addi.w  #0x28,0x38(a0)                  | +06c
.L07e5a2:
        lea     .L07e5a8(pc),a1                 | +072
        move.l  a1,(a6)                         | +076
.L07e5a8:
        movea.l 0xc(a6),a0                      | +078
        move.w  0x22(a0),0x22(a6)               | +07c
        move.w  0x24(a0),0x24(a6)               | +082
        jsr     0x28d70.l                       | +088
        jsr     0x2870a.l                       | +08e
        bcc.w   .L07e5ce                        | +094
        lea     Carrier_HullC_Run_07e530(pc),a1 | +098
        move.l  a1,(a6)                         | +09c
.L07e5ce:
        movea.l 0xc(a6),a0                      | +09e
        cmpi.b  #0x3,0x20(a0)                   | +0a2
        bne.w   .L07e5e2                        | +0a8
        lea     Squad_HatchRow3Spawn_07fc1a__L07fca6(pc),a1 | +0ac
        move.l  a1,(a6)                         | +0b0
.L07e5e2:
        jsr     0x28758.l                       | +0b2
        bcc.w   .L07e5f2                        | +0b8
        lea     Squad_HatchRow3Spawn_07fc1a__L07fc88(pc),a1 | +0bc
        move.l  a1,(a6)                         | +0c0
.L07e5f2:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +0c2
        bcc.w   SetHandlerRts_07e600            | +0c6

| ----------------------------------------------------------------------------
|  Carrier_Cockpit_Init_07e602  @ $07E602  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Cockpit_Init_07e602, "ax", @progbits
        .global Carrier_Cockpit_Init_07e602
Carrier_Cockpit_Init_07e602:
        move.w  #0x24,d1                        | +000
        jsr     0x236e.l                        | +004
        bset    #0x1,0x12(a6)                   | +00a
        clr.b   0x20(a6)                        | +010
        clr.b   0x21(a6)                        | +014
        move.w  #0x5000,0x38(a6)                | +018
        movea.l 0xc(a6),a0                      | +01e
        tst.b   0x82(a0)                        | +022
        bne.w   .L07e640                        | +026
        lea     0x2bcbc4.l,a0                   | +02a
        jsr     0x799de.l                       | +030
        move.w  d0,0x66(a6)                     | +036
        bra.w   .L07e650                        | +03a
.L07e640:
        lea     0x2bccc8.l,a0                   | +03e
        jsr     0x799de.l                       | +044
        move.w  d0,0x66(a6)                     | +04a
.L07e650:
        lea     0x2e1f08.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
        lea     Carrier_Mark_Cockpit_07e8b4(pc),a1 | +05a
        jsr     0x4ae.l                         | +05e
        jsr     0x5dd02.l                       | +064

| ----------------------------------------------------------------------------
|  Carrier_Cockpit_Run_07e674  @ $07E674  (148 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Cockpit_Run_07e674, "ax", @progbits
        .global Carrier_Cockpit_Run_07e674
Carrier_Cockpit_Run_07e674:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        subq.w  #0x4,0x24(a6)                   | +010
        move.w  0x22(a6),0x78(a0)               | +014
        move.w  0x24(a6),0x7a(a0)               | +01a
        jsr     0x28d70.l                       | +020
        move.w  0x22(a6),d0                     | +026
        move.w  0x24(a6),d1                     | +02a
        addi.w  #0xffd8,d0                      | +02e
        addi.w  #0x30,d1                        | +032
        move.w  #0x28,d2                        | +036
        jsr     0x99812.l                       | +03a
        move.w  0x22(a6),d0                     | +040
        move.w  0x24(a6),d1                     | +044
        addi.w  #0xffc8,d0                      | +048
        move.w  #0x58,d2                        | +04c
        jsr     0x99812.l                       | +050
        move.w  0x22(a6),d0                     | +056
        move.w  0x24(a6),d1                     | +05a
        addi.w  #0x10,d0                        | +05e
        addi.w  #0x10,d1                        | +062
        move.w  #0x20,d2                        | +066
        jsr     0x99812.l                       | +06a
        jsr     0x2870a.l                       | +070
        bcc.w   .L07e700                        | +076
        bclr    #0x3,0x13(a6)                   | +07a
        lea     0x5e766.l,a0                    | +080
        jsr     0x5e770.l                       | +086
.L07e700:
        tst.w   0x66(a6)                        | +08c
        bne.w   Carrier_Cockpit_MarkHit_07e708__L07e70e | +090

| ----------------------------------------------------------------------------
|  Carrier_Cockpit_MarkHit_07e708  @ $07E708  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Cockpit_MarkHit_07e708, "ax", @progbits
        .global Carrier_Cockpit_MarkHit_07e708
Carrier_Cockpit_MarkHit_07e708:
        bset    #0x0,0x13(a6)                   | +000
        .global Carrier_Cockpit_MarkHit_07e708__L07e70e
Carrier_Cockpit_MarkHit_07e708__L07e70e:
.L07e70e:
        jsr     0x28758.l                       | +006
        bcc.w   .L07e71e                        | +00c
        lea     Carrier_Cockpit_Eject_07e72e(pc),a1 | +010
        move.l  a1,(a6)                         | +014
.L07e71e:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +016
        bcc.w   SetHandlerRts_07e72c            | +01a

| ----------------------------------------------------------------------------
|  Carrier_Cockpit_Eject_07e72e  @ $07E72E  (286 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Cockpit_Eject_07e72e, "ax", @progbits
        .global Carrier_Cockpit_Eject_07e72e
Carrier_Cockpit_Eject_07e72e:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        subq.w  #0x4,0x24(a6)                   | +010
        move.b  #0x4,0x20(a0)                   | +014
        jsr     0x267e2.l                       | +01a
        move.w  #0xff40,0x2a(a6)                | +020
        moveq   #0,d0                           | +026
        move.b  d0,0x21(a6)                     | +028
        move.w  d0,0x66(a6)                     | +02c
        move.w  d0,0x74(a6)                     | +030
        move.w  d0,0x76(a6)                     | +034
        jsr     0x434dc.l                       | +038
        bclr    #0x1,0x12(a6)                   | +03e
        lea     0x2e1f1e.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        lea     0x77fd6.l,a1                    | +050
        jsr     0x4ae.l                         | +056
        jsr     0x5dd02.l                       | +05c
        lea     .L07e796(pc),a1                 | +062
        move.l  a1,(a6)                         | +066
.L07e796:
        jsr     0x27cee.l                       | +068
        movea.l 0xc(a6),a0                      | +06e
        move.w  0x22(a6),0x78(a0)               | +072
        move.w  0x24(a6),0x7a(a0)               | +078
        move.l  #0x2e23ca,0x60(a6)              | +07e
        jsr     0x28998.l                       | +086
        move.l  #0x2e2416,0x60(a6)              | +08c
        jsr     0x28998.l                       | +094
        move.l  #0x2e2462,0x60(a6)              | +09a
        jsr     0x28998.l                       | +0a2
        jsr     0x28d70.l                       | +0a8
        move.w  0x22(a6),d0                     | +0ae
        move.w  0x24(a6),d1                     | +0b2
        addi.w  #0xffd8,d0                      | +0b6
        addi.w  #0x30,d1                        | +0ba
        move.w  #0x28,d2                        | +0be
        jsr     0x99812.l                       | +0c2
        move.w  0x22(a6),d0                     | +0c8
        move.w  0x24(a6),d1                     | +0cc
        addi.w  #0xffc8,d0                      | +0d0
        move.w  #0x58,d2                        | +0d4
        jsr     0x99812.l                       | +0d8
        move.w  0x22(a6),d0                     | +0de
        move.w  0x24(a6),d1                     | +0e2
        addi.w  #0x10,d0                        | +0e6
        addi.w  #0x10,d1                        | +0ea
        move.w  #0x20,d2                        | +0ee
        jsr     0x99812.l                       | +0f2
        jsr     Squad_TrackArc_07fdfa__L07feb4(pc) | +0f8
        cmpi.w  #0xd0,0x24(a6)                  | +0fc
        bgt.w   .L07e844                        | +102
        movea.l 0xc(a6),a0                      | +106
        move.b  #0x5,0x20(a0)                   | +10a
        move.w  #0xfbc0,0x2a(a6)                | +110
.L07e844:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +116
        bcc.w   SetHandlerRts_07e852            | +11a

| ----------------------------------------------------------------------------
|  Carrier_Mark_Left_07e854  @ $07E854  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Mark_Left_07e854, "ax", @progbits
        .global Carrier_Mark_Left_07e854
Carrier_Mark_Left_07e854:
        move.l  #0x2e2332,0x60(a6)              | +000
        move.w  #0xffc8,0x70(a6)                | +008

| ----------------------------------------------------------------------------
|  Carrier_Mark_Right_07e86a  @ $07E86A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Mark_Right_07e86a, "ax", @progbits
        .global Carrier_Mark_Right_07e86a
Carrier_Mark_Right_07e86a:
        move.l  #0x2e237e,0x60(a6)              | +000
        move.w  #0x48,0x70(a6)                  | +008

| ----------------------------------------------------------------------------
|  Carrier_Mark_Run_07e880  @ $07E880  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Mark_Run_07e880, "ax", @progbits
        .global Carrier_Mark_Run_07e880
Carrier_Mark_Run_07e880:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        move.w  0x70(a6),d0                     | +010
        add.w   d0,0x22(a6)                     | +014
        jsr     0x28998.l                       | +018
        movea.l 0xc(a6),a0                      | +01e
        cmpi.b  #0x1,0x21(a0)                   | +022
        bne.w   .L07e8b2                        | +028
        jmp     0x518.l                         | +02c
.L07e8b2:
        rts                                     | +032

| ----------------------------------------------------------------------------
|  Carrier_Mark_Cockpit_07e8b4  @ $07E8B4  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Mark_Cockpit_07e8b4, "ax", @progbits
        .global Carrier_Mark_Cockpit_07e8b4
Carrier_Mark_Cockpit_07e8b4:
        move.l  #0x2e24ae,0x60(a6)              | +000
        lea     .L07e8c2(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L07e8c2:
        movea.l 0xc(a6),a0                      | +00e
        move.w  0x22(a0),0x22(a6)               | +012
        move.w  0x24(a0),0x24(a6)               | +018
        addi.w  #0x20,0x22(a6)                  | +01e
        jsr     0x28998.l                       | +024
        movea.l 0xc(a6),a0                      | +02a
        tst.b   0x21(a0)                        | +02e
        beq.w   .L07e8f0                        | +032
        jmp     0x518.l                         | +036
.L07e8f0:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  Carrier_Hatch_Init_07e8f2  @ $07E8F2  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Hatch_Init_07e8f2, "ax", @progbits
        .global Carrier_Hatch_Init_07e8f2
Carrier_Hatch_Init_07e8f2:
        move.w  #0x25,d1                        | +000
        jsr     0x236e.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.w  0x38(a0),0x38(a6)               | +00e
        addi.w  #0x18,0x38(a6)                  | +014
        clr.b   0x20(a6)                        | +01a
        clr.b   0x21(a6)                        | +01e
        clr.w   0x7c(a6)                        | +022
        lea     0x2e1f42.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L07e92a(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L07e92a:
        movea.l 0xc(a6),a0                      | +038
        move.w  0x22(a0),0x22(a6)               | +03c
        move.w  0x24(a0),0x24(a6)               | +042
        move.w  0x76(a0),d0                     | +048
        sub.w   d0,0x24(a6)                     | +04c
        jsr     Squad_AIDecide_07fee8__L080026(pc) | +050
        move.w  0x7c(a6),d0                     | +054
        andi.w  #0x3,d0                         | +058
        lea     0x2e1f7a.l,a0                   | +05c
        asl.w   #0x2,d0                         | +062
        move.l  (a0,d0.w),0x5c(a6)              | +064
        jsr     0x28d70.l                       | +06a
        movea.l 0xc(a6),a0                      | +070
        cmpi.b  #0x1,0x21(a0)                   | +074
        bne.w   .L07e976                        | +07a
        lea     Carrier_Hatch_Open_07e986(pc),a1 | +07e
        move.l  a1,(a6)                         | +082
.L07e976:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +084
        bcc.w   SetHandlerRts_07e984            | +088

| ----------------------------------------------------------------------------
|  Carrier_Hatch_Open_07e986  @ $07E986  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Hatch_Open_07e986, "ax", @progbits
        .global Carrier_Hatch_Open_07e986
Carrier_Hatch_Open_07e986:
        lea     0x2e1f9c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07e998(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07e998:
        movea.l 0xc(a6),a0                      | +012
        move.w  0x22(a0),0x22(a6)               | +016
        move.w  0x24(a0),0x24(a6)               | +01c
        move.w  0x76(a0),d0                     | +022
        sub.w   d0,0x24(a6)                     | +026
        jsr     0x28d70.l                       | +02a
        movea.l 0xc(a6),a0                      | +030
        cmpi.b  #0x2,0x21(a0)                   | +034
        bne.w   .L07e9ca                        | +03a
        lea     Carrier_Hatch_Open2_07e9da(pc),a1 | +03e
        move.l  a1,(a6)                         | +042
.L07e9ca:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +044
        bcc.w   SetHandlerRts_07e9d8            | +048

| ----------------------------------------------------------------------------
|  Carrier_Hatch_Open2_07e9da  @ $07E9DA  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Hatch_Open2_07e9da, "ax", @progbits
        .global Carrier_Hatch_Open2_07e9da
Carrier_Hatch_Open2_07e9da:
        lea     0x2e1ff8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07e9ec(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07e9ec:
        movea.l 0xc(a6),a0                      | +012
        move.w  0x22(a0),0x22(a6)               | +016
        move.w  0x24(a0),0x24(a6)               | +01c
        move.w  0x76(a0),d0                     | +022
        sub.w   d0,0x24(a6)                     | +026
        jsr     0x28d70.l                       | +02a
        movea.l 0xc(a6),a0                      | +030
        cmpi.b  #0x3,0x21(a0)                   | +034
        bne.w   .L07ea1e                        | +03a
        lea     Carrier_Hatch_Drop_07ea2e(pc),a1 | +03e
        move.l  a1,(a6)                         | +042
.L07ea1e:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +044
        bcc.w   SetHandlerRts_07ea2c            | +048

| ----------------------------------------------------------------------------
|  Carrier_Hatch_Drop_07ea2e  @ $07EA2E  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Hatch_Drop_07ea2e, "ax", @progbits
        .global Carrier_Hatch_Drop_07ea2e
Carrier_Hatch_Drop_07ea2e:
        lea     0x2e202c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07ea40(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07ea40:
        movea.l 0xc(a6),a0                      | +012
        move.w  0x22(a0),0x22(a6)               | +016
        move.w  0x24(a0),0x24(a6)               | +01c
        move.w  0x76(a0),d0                     | +022
        sub.w   d0,0x24(a6)                     | +026
        move.w  0x78(a6),d0                     | +02a
        sub.w   d0,0x22(a6)                     | +02e
        jsr     0x28d70.l                       | +032
        bcc.w   .L07ea70                        | +038
        lea     Carrier_Hatch_Fall_07ea80(pc),a1 | +03c
        move.l  a1,(a6)                         | +040
.L07ea70:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +042
        bcc.w   SetHandlerRts_07ea7e            | +046

| ----------------------------------------------------------------------------
|  Carrier_Hatch_Fall_07ea80  @ $07EA80  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Hatch_Fall_07ea80, "ax", @progbits
        .global Carrier_Hatch_Fall_07ea80
Carrier_Hatch_Fall_07ea80:
        move.w  #0xc,d1                         | +000
        jsr     0x236e.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.b  #0xff,0x21(a0)                  | +00e
        subq.w  #0x4,0x24(a6)                   | +014
        lea     0x2de6e0.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     .L07eaaa(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L07eaaa:
        jsr     0x2783a.l                       | +02a
        jsr     0x28d70.l                       | +030
        bcc.w   SetHandlerRts_07eac0            | +036

| ----------------------------------------------------------------------------
|  Carrier_Cannon_Init_07eac2  @ $07EAC2  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Cannon_Init_07eac2, "ax", @progbits
        .global Carrier_Cannon_Init_07eac2
Carrier_Cannon_Init_07eac2:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.w  0x38(a0),0x38(a6)               | +00e
        addi.w  #0x30,0x38(a6)                  | +014
        clr.b   0x20(a6)                        | +01a
        clr.b   0x21(a6)                        | +01e
        jsr     0x267e2.l                       | +022
        lea     0x2e20aa.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L07eafc(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L07eafc:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        bcc.w   .L07eb12                        | +046
        lea     Carrier_Cannon_Fall_07eb2a(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L07eb12:
        movea.l #0xffffffff,a0                  | +050
        jsr     0x5dd56.l                       | +056
        bcc.w   SetHandlerRts_07eb28            | +05c

| ----------------------------------------------------------------------------
|  Carrier_Cannon_Fall_07eb2a  @ $07EB2A  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Cannon_Fall_07eb2a, "ax", @progbits
        .global Carrier_Cannon_Fall_07eb2a
Carrier_Cannon_Fall_07eb2a:
        jsr     0x267e2.l                       | +000
        move.w  #0x100,0x2a(a6)                 | +006
        lea     0x2e20e4.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L07eb48(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L07eb48:
        jsr     0x27cee.l                       | +01e
        jsr     0x28d70.l                       | +024
        bcc.w   .L07eb5e                        | +02a
        lea     TaskHandler_07fdbc(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L07eb5e:
        movea.l #0xffffffff,a0                  | +034
        jsr     0x5dd56.l                       | +03a
        bcc.w   SetHandlerRts_07eb74            | +040

| ----------------------------------------------------------------------------
|  Carrier_Spawner_Init_07eb76  @ $07EB76  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Spawner_Init_07eb76, "ax", @progbits
        .global Carrier_Spawner_Init_07eb76
Carrier_Spawner_Init_07eb76:
        clr.b   0x20(a6)                        | +000
        bra.w   .L07ebc4                        | +004
        .global Carrier_Spawner_Init_07eb76__L07eb7e
Carrier_Spawner_Init_07eb76__L07eb7e:
.L07eb7e:
        clr.b   0x20(a6)                        | +008
        lea     0x79b6e.l,a1                    | +00c
        jsr     0x4ae.l                         | +012
        jsr     0x5dd02.l                       | +018
        move.w  #0x0,0x72(a0)                   | +01e
        move.w  #0xb800,0x74(a0)                | +024
        lea     0x79b6e.l,a1                    | +02a
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        move.b  #0x1,0x3a(a0)                   | +03c
        move.w  #0x2,0x72(a0)                   | +042
        move.w  #0x4800,0x74(a0)                | +048
.L07ebc4:
        moveq   #0,d0                           | +04e
        move.b  d0,0x21(a6)                     | +050
        move.b  d0,0x70(a6)                     | +054
        move.b  d0,0x72(a6)                     | +058
        move.b  d0,0x73(a6)                     | +05c
        move.b  d0,0x74(a6)                     | +060
        move.w  d0,0x76(a6)                     | +064

| ----------------------------------------------------------------------------
|  Carrier_Spawner_Run_07ebe6  @ $07EBE6  (226 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Spawner_Run_07ebe6, "ax", @progbits
        .global Carrier_Spawner_Run_07ebe6
Carrier_Spawner_Run_07ebe6:
        jsr     Carrier_Spawner_RateFromTbl_07ed00(pc) | +000
        btst    #0x0,0x70(a6)                   | +004
        bne.w   .L07ec26                        | +00a
        addq.b  #0x1,0x72(a6)                   | +00e
        move.b  0x72(a6),d0                     | +012
        cmp.b   0x92(a6),d0                     | +016
        bcs.w   .L07ec26                        | +01a
        move.b  #0x0,0x72(a6)                   | +01e
        lea     Carrier_Trooper_Init_07ed7c(pc),a1 | +024
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        move.w  #0xffb8,0x70(a0)                | +034
        move.w  #0x0,0x72(a0)                   | +03a
.L07ec26:
        btst    #0x1,0x70(a6)                   | +040
        bne.w   .L07ec60                        | +046
        addq.b  #0x1,0x73(a6)                   | +04a
        move.b  0x73(a6),d0                     | +04e
        cmp.b   0x92(a6),d0                     | +052
        bcs.w   .L07ec60                        | +056
        clr.b   0x73(a6)                        | +05a
        lea     Carrier_Trooper_Init_07ed7c(pc),a1 | +05e
        jsr     0x4ae.l                         | +062
        jsr     0x5dd02.l                       | +068
        move.w  #0x28,0x70(a0)                  | +06e
        move.w  #0x1,0x72(a0)                   | +074
.L07ec60:
        btst    #0x2,0x70(a6)                   | +07a
        bne.w   .L07ec9a                        | +080
        addq.b  #0x1,0x74(a6)                   | +084
        move.b  0x74(a6),d0                     | +088
        cmp.b   0x92(a6),d0                     | +08c
        bcs.w   .L07ec9a                        | +090
        clr.b   0x74(a6)                        | +094
        lea     Carrier_Trooper_Init_07ed7c(pc),a1 | +098
        jsr     0x4ae.l                         | +09c
        jsr     0x5dd02.l                       | +0a2
        move.w  #0x48,0x70(a0)                  | +0a8
        move.w  #0x2,0x72(a0)                   | +0ae
.L07ec9a:
        movea.l 0xc(a6),a0                      | +0b4
        move.w  0x22(a0),0x22(a6)               | +0b8
        move.w  0x24(a0),0x24(a6)               | +0be
        cmpi.b  #0x3,0x20(a0)                   | +0c4
        bne.w   .L07ecc0                        | +0ca
        move.b  #0xff,0x20(a6)                  | +0ce
        lea     Carrier_Spawner_WaitParent_07ecd0(pc),a1 | +0d4
        move.l  a1,(a6)                         | +0d8
.L07ecc0:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +0da
        bcc.w   SetHandlerRts_07ecce            | +0de

| ----------------------------------------------------------------------------
|  Carrier_Spawner_WaitParent_07ecd0  @ $07ECD0  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Spawner_WaitParent_07ecd0, "ax", @progbits
        .global Carrier_Spawner_WaitParent_07ecd0
Carrier_Spawner_WaitParent_07ecd0:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        cmpi.b  #0x1,0x21(a0)                   | +010
        bne.w   .L07ecf0                        | +016
        lea     TaskHandler_07fdbc(pc),a1       | +01a
        move.l  a1,(a6)                         | +01e
.L07ecf0:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +020
        bcc.w   SetHandlerRts_07ecfe            | +024

| ----------------------------------------------------------------------------
|  Carrier_Spawner_RateFromTbl_07ed00  @ $07ED00  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Spawner_RateFromTbl_07ed00, "ax", @progbits
        .global Carrier_Spawner_RateFromTbl_07ed00
Carrier_Spawner_RateFromTbl_07ed00:
        cmpi.w  #0xf,0x76(a6)                   | +000
        bgt.w   Carrier_Spawner_RateFromTblB_07ed2a | +006
        lea     0x2bcd4a.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.b  d0,0x92(a6)                     | +016
        moveq   #0,d0                           | +01a
        move.b  0x92(a6),d0                     | +01c
        mulu.w  #0x1e,d0                        | +020

| ----------------------------------------------------------------------------
|  Carrier_Spawner_RateFromTblB_07ed2a  @ $07ED2A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Spawner_RateFromTblB_07ed2a, "ax", @progbits
        .global Carrier_Spawner_RateFromTblB_07ed2a
Carrier_Spawner_RateFromTblB_07ed2a:
        lea     0x2bcdcc.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x92(a6)                     | +00c
        moveq   #0,d0                           | +010
        move.b  0x92(a6),d0                     | +012
        mulu.w  #0x1e,d0                        | +016

| ----------------------------------------------------------------------------
|  Carrier_Trooper_HPFromTbl_07ed4a  @ $07ED4A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_HPFromTbl_07ed4a, "ax", @progbits
        .global Carrier_Trooper_HPFromTbl_07ed4a
Carrier_Trooper_HPFromTbl_07ed4a:
        movea.l 0xc(a6),a0                      | +000
        cmpi.w  #0xf,0x76(a0)                   | +004
        bgt.w   Carrier_Trooper_HPFromTblB_07ed6a | +00a
        lea     0x2bce4e.l,a0                   | +00e
        jsr     0x799de.l                       | +014

| ----------------------------------------------------------------------------
|  Carrier_Trooper_HPFromTblB_07ed6a  @ $07ED6A  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_HPFromTblB_07ed6a, "ax", @progbits
        .global Carrier_Trooper_HPFromTblB_07ed6a
Carrier_Trooper_HPFromTblB_07ed6a:
        lea     0x2bced0.l,a0                   | +000
        jsr     0x799de.l                       | +006

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Init_07ed7c  @ $07ED7C  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Init_07ed7c, "ax", @progbits
        .global Carrier_Trooper_Init_07ed7c
Carrier_Trooper_Init_07ed7c:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x72(a6),d0                     | +004
        bset    d0,0x70(a0)                     | +008
        move.w  #0x0,0x76(a6)                   | +00c
        move.w  #0x1d,d1                        | +012
        jsr     0x236e.l                        | +016
        move.b  #0x1,0x5c(a6)                   | +01c
        jsr     Carrier_Trooper_HPFromTbl_07ed4a(pc) | +022
        clr.b   0x20(a6)                        | +026
        clr.b   0x21(a6)                        | +02a
        move.w  #0x1,0x66(a6)                   | +02e
        jsr     0x267e2.l                       | +034
        lea     0x2e2286.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        lea     .L07edc8(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L07edc8:
        jsr     Squad_TrackArc_07fdfa(pc)       | +04c
        jsr     0x28d70.l                       | +050
        bcc.w   .L07eddc                        | +056
        lea     Carrier_Trooper_Jump_07edf4(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
.L07eddc:
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +060
        jsr     Squad_LeaderGoneCheck_080034(pc) | +064
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +068
        bcc.w   SetHandlerRts_07edf2            | +06c

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Jump_07edf4  @ $07EDF4  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Jump_07edf4, "ax", @progbits
        .global Carrier_Trooper_Jump_07edf4
Carrier_Trooper_Jump_07edf4:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2e22e8.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L07ee10(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07ee10:
        jsr     Squad_TrackArc_07fdfa(pc)       | +01c
        jsr     0x28d70.l                       | +020
        bcc.w   .L07ee24                        | +026
        lea     Carrier_Trooper_Run_07ee7a__L07ee84(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L07ee24:
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +030
        jsr     Squad_LeaderGoneCheck_080034(pc) | +034
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +038
        bcc.w   SetHandlerRts_07ee3a            | +03c

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Land_07ee3c  @ $07EE3C  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Land_07ee3c, "ax", @progbits
        .global Carrier_Trooper_Land_07ee3c
Carrier_Trooper_Land_07ee3c:
        lea     0x29bfb8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07ee4e(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07ee4e:
        jsr     Squad_TrackArc_07fdfa(pc)       | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L07ee62                        | +01c
        lea     Carrier_Trooper_Run_07ee7a(pc),a1 | +020
        move.l  a1,(a6)                         | +024
.L07ee62:
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +026
        jsr     Squad_LeaderGoneCheck_080034(pc) | +02a
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +02e
        bcc.w   SetHandlerRts_07ee78            | +032

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Run_07ee7a  @ $07EE7A  (144 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Run_07ee7a, "ax", @progbits
        .global Carrier_Trooper_Run_07ee7a
Carrier_Trooper_Run_07ee7a:
        move.b  #0x1,0x3a(a6)                   | +000
        bra.w   .L07ee8c                        | +006
        .global Carrier_Trooper_Run_07ee7a__L07ee84
Carrier_Trooper_Run_07ee7a__L07ee84:
.L07ee84:
        cmpi.w  #0x0,0x72(a6)                   | +00a
        bne.b   Carrier_Trooper_Land_07ee3c     | +010
.L07ee8c:
        move.w  #0xe,d1                         | +012
        jsr     0x236e.l                        | +016
        jsr     0x267e2.l                       | +01c
        move.w  #0xfe00,0x28(a6)                | +022
        btst    #0x0,0x3a(a6)                   | +028
        beq.w   .L07eeb0                        | +02e
        neg.w   0x28(a6)                        | +032
.L07eeb0:
        clr.w   0x74(a6)                        | +036
        lea     0x29b744.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        lea     .L07eec6(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L07eec6:
        jsr     Squad_TrackArc_07fdfa(pc)       | +04c
        jsr     0x28d70.l                       | +050
        move.w  0x74(a6),d0                     | +056
        asr.w   #0x8,d0                         | +05a
        cmp.w   0x70(a6),d0                     | +05c
        bne.w   .L07eeee                        | +060
        jsr     0x267e2.l                       | +064
        lea     Carrier_Trooper_Stand_07ef4a(pc),a1 | +06a
        move.l  a1,(a6)                         | +06e
        bra.w   .L07eefa                        | +070
.L07eeee:
        move.w  0x74(a6),d0                     | +074
        add.w   0x28(a6),d0                     | +078
        move.w  d0,0x74(a6)                     | +07c
.L07eefa:
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +080
        jsr     Squad_LeaderGoneCheck_080034(pc) | +084
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +088
        bcc.w   SetHandlerRts_07ef10            | +08c

| ----------------------------------------------------------------------------
|  Carrier_Trooper_OnHit_07ef12  @ $07EF12  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_OnHit_07ef12, "ax", @progbits
        .global Carrier_Trooper_OnHit_07ef12
Carrier_Trooper_OnHit_07ef12:
        jsr     0x2870a.l                       | +000
        bcc.w   JsrAbsRts_07ef48                | +006
        move.w  0x72(a6),d0                     | +00a
        movea.l 0xc(a6),a0                      | +00e
        bclr    d0,0x70(a0)                     | +012
        addq.w  #0x1,0x76(a0)                   | +016
        move.w  #0xc000,d0                      | +01a
        jsr     0x28134.l                       | +01e
        andi.w  #0xffe3,0x38(a6)                | +024
        ori.w   #0x10,0x38(a6)                  | +02a

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Stand_07ef4a  @ $07EF4A  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Stand_07ef4a, "ax", @progbits
        .global Carrier_Trooper_Stand_07ef4a
Carrier_Trooper_Stand_07ef4a:
        lea     0x29b7c8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07ef5c(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07ef5c:
        jsr     Squad_TrackArc_07fdfa(pc)       | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L07ef70                        | +01c
        lea     Carrier_Trooper_Idle_07ef88(pc),a1 | +020
        move.l  a1,(a6)                         | +024
.L07ef70:
        jsr     Carrier_Rider_ParentDeadCheck_07f63c(pc) | +026
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +02a
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +02e
        bcc.w   SetHandlerRts_07ef86            | +032

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Idle_07ef88  @ $07EF88  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Idle_07ef88, "ax", @progbits
        .global Carrier_Trooper_Idle_07ef88
Carrier_Trooper_Idle_07ef88:
        lea     0x29b816.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07ef9a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07ef9a:
        jsr     Squad_TrackArc_07fdfa(pc)       | +012
        jsr     0x28d70.l                       | +016
        tst.w   0x90(a6)                        | +01c
        bne.w   .L07efb0                        | +020
        jsr     Carrier_Trooper_DecideByPlayer_07efcc(pc) | +024
.L07efb0:
        subq.w  #0x1,0x90(a6)                   | +028
        jsr     Carrier_Rider_ParentDeadCheck_07f63c(pc) | +02c
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +030
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +034
        bcc.w   SetHandlerRts_07efca            | +038

| ----------------------------------------------------------------------------
|  Carrier_Trooper_DecideByPlayer_07efcc  @ $07EFCC  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_DecideByPlayer_07efcc, "ax", @progbits
        .global Carrier_Trooper_DecideByPlayer_07efcc
Carrier_Trooper_DecideByPlayer_07efcc:
        jsr     0x5e0d4.l                       | +000
        move.w  0x22(a0),d0                     | +006
        cmp.w   0x22(a6),d0                     | +00a
        ble.w   .L07eff8                        | +00e
        sub.w   0x22(a6),d0                     | +012
        cmpi.w  #0x30,d0                        | +016
        blt.w   Carrier_Trooper_Fire_07f186__L07f1b4 | +01a
        btst    #0x0,0x3a(a6)                   | +01e
        bne.w   SetTaskHandler_07f012           | +024
        bra.w   SetTaskHandler_07f01a           | +028
.L07eff8:
        move.w  0x22(a6),d0                     | +02c
        sub.w   0x22(a0),d0                     | +030
        cmpi.w  #0x30,d0                        | +034
        blt.w   Carrier_Trooper_Fire_07f186__L07f1b4 | +038
        btst    #0x0,0x3a(a6)                   | +03c
        bne.w   SetTaskHandler_07f01a           | +042

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Aim_07f022  @ $07F022  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Aim_07f022, "ax", @progbits
        .global Carrier_Trooper_Aim_07f022
Carrier_Trooper_Aim_07f022:
        lea     0x29bf34.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07f034(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07f034:
        jsr     Squad_TrackArc_07fdfa(pc)       | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L07f048                        | +01c
        lea     Carrier_Trooper_Fire_07f186(pc),a1 | +020
        move.l  a1,(a6)                         | +024
.L07f048:
        jsr     Carrier_Rider_ParentDeadCheck_07f63c(pc) | +026
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +02a
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +02e
        bcc.w   SetHandlerRts_07f05e            | +032

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Walk_07f060  @ $07F060  (148 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Walk_07f060, "ax", @progbits
        .global Carrier_Trooper_Walk_07f060
Carrier_Trooper_Walk_07f060:
        eori.b  #0x1,0x3a(a6)                   | +000
        .global Carrier_Trooper_Walk_07f060__L07f066
Carrier_Trooper_Walk_07f060__L07f066:
.L07f066:
        jsr     0x267e2.l                       | +006
        move.w  #0xfe00,0x28(a6)                | +00c
        btst    #0x0,0x3a(a6)                   | +012
        beq.w   .L07f080                        | +018
        neg.w   0x28(a6)                        | +01c
.L07f080:
        move.w  #0x0,0x76(a6)                   | +020
        lea     0x29b744.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L07f098(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L07f098:
        jsr     Squad_TrackArc_07fdfa(pc)       | +038
        jsr     0x28d70.l                       | +03c
        subq.w  #0x1,0x90(a6)                   | +042
        beq.w   .L07f0d8                        | +046
        move.w  0x74(a6),d0                     | +04a
        add.w   0x28(a6),d0                     | +04e
        move.w  d0,0x74(a6)                     | +052
        move.w  0x74(a6),d0                     | +056
        asr.w   #0x8,d0                         | +05a
        tst.w   0x28(a6)                        | +05c
        bmi.w   .L07f0d0                        | +060
        cmp.w   0x70(a6),d0                     | +064
        bge.w   .L07f0d8                        | +068
        bra.w   .L07f0e4                        | +06c
.L07f0d0:
        cmp.w   0x76(a6),d0                     | +070
        bgt.w   .L07f0e4                        | +074
.L07f0d8:
        jsr     0x267e2.l                       | +078
        lea     Carrier_Trooper_Stop_07f0fc(pc),a1 | +07e
        move.l  a1,(a6)                         | +082
.L07f0e4:
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +084
        jsr     Squad_LeaderGoneCheck_080034(pc) | +088
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +08c
        bcc.w   SetHandlerRts_07f0fa            | +090

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Stop_07f0fc  @ $07F0FC  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Stop_07f0fc, "ax", @progbits
        .global Carrier_Trooper_Stop_07f0fc
Carrier_Trooper_Stop_07f0fc:
        lea     0x29b7c8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07f10e(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07f10e:
        jsr     Squad_TrackArc_07fdfa(pc)       | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L07f130                        | +01c
        lea     Carrier_Trooper_Turn_07f148(pc),a1 | +020
        move.l  a1,(a6)                         | +024
        tst.w   0x90(a6)                        | +026
        bne.w   .L07f130                        | +02a
        lea     Carrier_Trooper_Idle_07ef88(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L07f130:
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +034
        jsr     Squad_LeaderGoneCheck_080034(pc) | +038
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +03c
        bcc.w   SetHandlerRts_07f146            | +040

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Turn_07f148  @ $07F148  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Turn_07f148, "ax", @progbits
        .global Carrier_Trooper_Turn_07f148
Carrier_Trooper_Turn_07f148:
        lea     0x29bfb8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07f15a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07f15a:
        jsr     Squad_TrackArc_07fdfa(pc)       | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L07f16e                        | +01c
        lea     Carrier_Trooper_Walk_07f060(pc),a1 | +020
        move.l  a1,(a6)                         | +024
.L07f16e:
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +026
        jsr     Squad_LeaderGoneCheck_080034(pc) | +02a
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +02e
        bcc.w   SetHandlerRts_07f184            | +032

| ----------------------------------------------------------------------------
|  Carrier_Trooper_Fire_07f186  @ $07F186  (156 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Trooper_Fire_07f186, "ax", @progbits
        .global Carrier_Trooper_Fire_07f186
Carrier_Trooper_Fire_07f186:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x3,d0                         | +006
        move.b  d0,0x5c(a6)                     | +00a
        lea     0x29ba70.l,a0                   | +00e
        jsr     0x28cd4.l                       | +014
        lea     .L07f1a6(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L07f1a6:
        jsr     Squad_TrackArc_07fdfa(pc)       | +020
        jsr     0x28d70.l                       | +024
        bcc.w   .L07f212                        | +02a
        .global Carrier_Trooper_Fire_07f186__L07f1b4
Carrier_Trooper_Fire_07f186__L07f1b4:
.L07f1b4:
        jsr     Carrier_Trooper_HPFromTbl_07ed4a(pc) | +02e
        lea     Carrier_Trooper_Idle_07ef88(pc),a1 | +032
        move.l  a1,(a6)                         | +036
        movea.l 0xc(a6),a0                      | +038
        movea.l 0xc(a0),a0                      | +03c
        cmpi.w  #0x1f4,0x66(a0)                 | +040
        bgt.w   .L07f202                        | +046
        jsr     0x5e9b6.l                       | +04a
        andi.w  #0xf,d0                         | +050
        cmpi.w  #0x9,d0                         | +054
        blt.w   .L07f202                        | +058
        move.w  0x72(a6),d0                     | +05c
        movea.l 0xc(a6),a0                      | +060
        bclr    d0,0x70(a0)                     | +064
        jsr     0x5e9b6.l                       | +068
        andi.w  #0x1,d0                         | +06e
        add.w   d0,d0                           | +072
        move.w  d0,0x72(a6)                     | +074
        jmp     Carrier_Rider_ParentDeadCheck_07f63c__L07f64a(pc) | +078
.L07f202:
        cmpi.w  #0x1,0x72(a6)                   | +07c
        bne.w   .L07f212                        | +082
        lea     Carrier_Trooper_Walk_07f060__L07f066(pc),a1 | +086
        move.l  a1,(a6)                         | +08a
.L07f212:
        jsr     Carrier_Rider_ParentDeadCheck_07f63c(pc) | +08c
        jsr     Carrier_Trooper_OnHit_07ef12(pc) | +090
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +094
        bcc.w   SetHandlerRts_07f228            | +098

| ----------------------------------------------------------------------------
|  Carrier_Rider_Init_07f22a  @ $07F22A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_Init_07f22a, "ax", @progbits
        .global Carrier_Rider_Init_07f22a
Carrier_Rider_Init_07f22a:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x1,d0                         | +006
        move.b  d0,0x3a(a6)                     | +00a
        jsr     0x5e9b6.l                       | +00e
        andi.w  #0x3,d0                         | +014
        cmpi.w  #0x3,d0                         | +018
        bne.w   .L07f24e                        | +01c
        move.w  #0x2,d0                         | +020
.L07f24e:
        move.w  d0,0x72(a6)                     | +024
        move.w  #0x0,0x76(a6)                   | +028
        movea.l 0xc(a6),a0                      | +02e
        move.w  0x24(a0),d0                     | +032
        subi.w  #0xc,d0                         | +036
        move.w  d0,0x70(a6)                     | +03a
        cmpi.w  #0x1,0x72(a6)                   | +03e
        bne.w   SetTaskHandler_07f27a           | +044

| ----------------------------------------------------------------------------
|  Carrier_Rider_Ride_07f282  @ $07F282  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_Ride_07f282, "ax", @progbits
        .global Carrier_Rider_Ride_07f282
Carrier_Rider_Ride_07f282:
        move.w  #0x28,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x29c77c.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L07f29e(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07f29e:
        jsr     Squad_TrackArc_07fdfa(pc)       | +01c
        jsr     0x28d70.l                       | +020
        bcc.w   .L07f2b2                        | +026
        lea     Carrier_Rider_Jump_07f2ca(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L07f2b2:
        movea.l #0xffffffff,a0                  | +030
        jsr     0x5dd56.l                       | +036
        bcc.w   SetHandlerRts_07f2c8            | +03c

| ----------------------------------------------------------------------------
|  Carrier_Rider_Jump_07f2ca  @ $07F2CA  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_Jump_07f2ca, "ax", @progbits
        .global Carrier_Rider_Jump_07f2ca
Carrier_Rider_Jump_07f2ca:
        jsr     0x13600.l                       | +000
        move.w  #0x28,d1                        | +006
        jsr     0x236e.l                        | +00a
        move.w  #0xc000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x10,0x38(a6)                  | +020
        clr.b   0x20(a6)                        | +026
        jsr     0x267e2.l                       | +02a
        lea     Carrier_Rider_JumpRun_07f324(pc),a1 | +030
        move.l  a1,(a6)                         | +034
        tst.w   0x72(a6)                        | +036
        bne.w   .L07f318                        | +03a
        lea     0x29c7c0.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        bra.w   Carrier_Rider_JumpRun_07f324    | +04a
.L07f318:
        lea     0x29c858.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054

| ----------------------------------------------------------------------------
|  Carrier_Rider_JumpRun_07f324  @ $07F324  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_JumpRun_07f324, "ax", @progbits
        .global Carrier_Rider_JumpRun_07f324
Carrier_Rider_JumpRun_07f324:
        tst.b   0x20(a6)                        | +000
        bne.w   .L07f334                        | +004
        jsr     Squad_TrackArc_07fdfa(pc)       | +008
        jmp     Carrier_Rider_Fall_07f3a8__L07f3ae(pc) | +00c
.L07f334:
        lea     Carrier_Rider_Fall_07f3a8(pc),a1 | +010
        move.l  a1,(a6)                         | +014
        jmp     Carrier_Rider_Fall_07f3a8(pc)   | +016

| ----------------------------------------------------------------------------
|  Carrier_Rider_JumpB_07f33e  @ $07F33E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_JumpB_07f33e, "ax", @progbits
        .global Carrier_Rider_JumpB_07f33e
Carrier_Rider_JumpB_07f33e:
        move.w  #0x28,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xc000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        clr.b   0x20(a6)                        | +020
        jsr     0x267e2.l                       | +024
        lea     Carrier_Rider_JumpBRun_07f392(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
        tst.w   0x72(a6)                        | +030
        bne.w   .L07f386                        | +034
        lea     0x29c7c0.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        bra.w   Carrier_Rider_JumpBRun_07f392   | +044
.L07f386:
        lea     0x29c858.l,a0                   | +048
        jsr     0x28cd4.l                       | +04e

| ----------------------------------------------------------------------------
|  Carrier_Rider_JumpBRun_07f392  @ $07F392  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_JumpBRun_07f392, "ax", @progbits
        .global Carrier_Rider_JumpBRun_07f392
Carrier_Rider_JumpBRun_07f392:
        tst.b   0x20(a6)                        | +000
        bne.w   .L07f3a2                        | +004
        jsr     Squad_TrackArc_07fdfa__L07fe66(pc) | +008
        jmp     Carrier_Rider_Fall_07f3a8__L07f3ae(pc) | +00c
.L07f3a2:
        lea     Carrier_Rider_Fall_07f3a8(pc),a1 | +010
        move.l  a1,(a6)                         | +014

| ----------------------------------------------------------------------------
|  Carrier_Rider_Fall_07f3a8  @ $07F3A8  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_Fall_07f3a8, "ax", @progbits
        .global Carrier_Rider_Fall_07f3a8
Carrier_Rider_Fall_07f3a8:
        jsr     0x27cee.l                       | +000
        .global Carrier_Rider_Fall_07f3a8__L07f3ae
Carrier_Rider_Fall_07f3a8__L07f3ae:
.L07f3ae:
        jsr     0x28d70.l                       | +006
        move.w  0x70(a6),d0                     | +00c
        subi.w  #0x10,d0                        | +010
        cmp.w   0x24(a6),d0                     | +014
        blt.w   .L07f3d0                        | +018
        move.w  0x70(a6),0x24(a6)               | +01c
        lea     Carrier_Rider_Land_07f3e8(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L07f3d0:
        movea.l #0xffffffff,a0                  | +028
        jsr     0x5dd56.l                       | +02e
        bcc.w   SetHandlerRts_07f3e6            | +034

| ----------------------------------------------------------------------------
|  Carrier_Rider_Land_07f3e8  @ $07F3E8  (116 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_Land_07f3e8, "ax", @progbits
        .global Carrier_Rider_Land_07f3e8
Carrier_Rider_Land_07f3e8:
        move.w  #0xc,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0xc000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        jsr     0x267e2.l                       | +020
        btst    #0x0,0x3a(a6)                   | +026
        beq.w   .L07f41c                        | +02c
        neg.w   0x28(a6)                        | +030
.L07f41c:
        lea     0x2de7c2.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        lea     0xffff.w,a0                     | +040
        move.l  a0,0x48(a6)                     | +044
        lea     .L07f436(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L07f436:
        jsr     0x27cee.l                       | +04e
        jsr     0x28d70.l                       | +054
        bcc.w   .L07f44c                        | +05a
        lea     Carrier_Rider_Flee_07f464(pc),a1 | +05e
        move.l  a1,(a6)                         | +062
.L07f44c:
        movea.l #0xffffffff,a0                  | +064
        jsr     0x5dd56.l                       | +06a
        bcc.w   SetHandlerRts_07f462            | +070

| ----------------------------------------------------------------------------
|  Carrier_Rider_Flee_07f464  @ $07F464  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_Flee_07f464, "ax", @progbits
        .global Carrier_Rider_Flee_07f464
Carrier_Rider_Flee_07f464:
        jsr     0x267e2.l                       | +000
        move.w  #0xffa0,0x28(a6)                | +006
        btst    #0x0,0x3a(a6)                   | +00c
        beq.w   .L07f47e                        | +012
        neg.w   0x28(a6)                        | +016
.L07f47e:
        lea     0x2de6e0.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     0xffff.w,a0                     | +026
        move.l  a0,0x48(a6)                     | +02a
        lea     .L07f498(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L07f498:
        jsr     0x27cee.l                       | +034
        jsr     0x28d70.l                       | +03a
        bcc.w   .L07f4ae                        | +040
        lea     Carrier_Rider_Dive_07f4c6(pc),a1 | +044
        move.l  a1,(a6)                         | +048
.L07f4ae:
        movea.l #0xffffffff,a0                  | +04a
        jsr     0x5dd56.l                       | +050
        bcc.w   SetHandlerRts_07f4c4            | +056

| ----------------------------------------------------------------------------
|  Carrier_Rider_Dive_07f4c6  @ $07F4C6  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_Dive_07f4c6, "ax", @progbits
        .global Carrier_Rider_Dive_07f4c6
Carrier_Rider_Dive_07f4c6:
        move.w  #0x28,d1                        | +000
        jsr     0x236e.l                        | +004
        subi.w  #0x18,0x24(a6)                  | +00a
        move.w  #0x8000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0xc,0x38(a6)                   | +020
        lea     0x29c962.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L07f4fe(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L07f4fe:
        jsr     0x27cee.l                       | +038
        jsr     0x28d70.l                       | +03e
        bcc.w   .L07f514                        | +044
        lea     Carrier_Rider_DiveRun_07f532(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L07f514:
        movea.l #0xffffffff,a0                  | +04e
        lea     0x2e2ae0.l,a0                   | +054
        jsr     0x5dd56.l                       | +05a
        bcc.w   SetHandlerRts_07f530            | +060

| ----------------------------------------------------------------------------
|  Carrier_Rider_DiveRun_07f532  @ $07F532  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_DiveRun_07f532, "ax", @progbits
        .global Carrier_Rider_DiveRun_07f532
Carrier_Rider_DiveRun_07f532:
        lea     0x29c9c2.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.w  #0x0,0x70(a6)                   | +00c
        lea     .L07f54a(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L07f54a:
        jsr     0x27cee.l                       | +018
        move.w  0x28(a6),d0                     | +01e
        add.w   0x70(a6),d0                     | +022
        move.w  d0,0x70(a6)                     | +026
        bpl.w   .L07f562                        | +02a
        neg.w   d0                              | +02e
.L07f562:
        cmpi.w  #0x4000,d0                      | +030
        blt.w   .L07f570                        | +034
        move.w  #0xffa0,0x2a(a6)                | +038
.L07f570:
        jsr     0x28d70.l                       | +03e
        jsr     0x2870a.l                       | +044
        bcc.w   .L07f586                        | +04a
        lea     Carrier_Rider_DiveLand_07f5a4(pc),a1 | +04e
        move.l  a1,(a6)                         | +052
.L07f586:
        movea.l #0xffffffff,a0                  | +054
        lea     0x2e2ae0.l,a0                   | +05a
        jsr     0x5dd56.l                       | +060
        bcc.w   SetHandlerRts_07f5a2            | +066

| ----------------------------------------------------------------------------
|  Carrier_Rider_DiveLand_07f5a4  @ $07F5A4  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_DiveLand_07f5a4, "ax", @progbits
        .global Carrier_Rider_DiveLand_07f5a4
Carrier_Rider_DiveLand_07f5a4:
        lea     0x29c992.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07f5b6(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07f5b6:
        jsr     0x27cee.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L07f5cc                        | +01e
        lea     Carrier_Rider_FleeB_07f5ea(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L07f5cc:
        movea.l #0xffffffff,a0                  | +028
        lea     0x2e2ae0.l,a0                   | +02e
        jsr     0x5dd56.l                       | +034
        bcc.w   SetHandlerRts_07f5e8            | +03a

| ----------------------------------------------------------------------------
|  Carrier_Rider_FleeB_07f5ea  @ $07F5EA  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_FleeB_07f5ea, "ax", @progbits
        .global Carrier_Rider_FleeB_07f5ea
Carrier_Rider_FleeB_07f5ea:
        addi.w  #0x18,0x24(a6)                  | +000
        jsr     0x267e2.l                       | +006
        lea     0x2de6e0.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L07f608(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L07f608:
        jsr     0x2783a.l                       | +01e
        jsr     0x28d70.l                       | +024
        bcc.w   .L07f61e                        | +02a
        lea     TaskHandler_07fdbc(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L07f61e:
        movea.l #0xffffffff,a0                  | +034
        lea     0x2e2ae0.l,a0                   | +03a
        jsr     0x5dd56.l                       | +040
        bcc.w   SetHandlerRts_07f63a            | +046

| ----------------------------------------------------------------------------
|  Carrier_Rider_ParentDeadCheck_07f63c  @ $07F63C  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Rider_ParentDeadCheck_07f63c, "ax", @progbits
        .global Carrier_Rider_ParentDeadCheck_07f63c
Carrier_Rider_ParentDeadCheck_07f63c:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0xff,0x20(a0)                  | +004
        bne.w   JsrAbsRts_07f66a                | +00a
        .global Carrier_Rider_ParentDeadCheck_07f63c__L07f64a
Carrier_Rider_ParentDeadCheck_07f63c__L07f64a:
.L07f64a:
        movea.l 0xc(a6),a0                      | +00e
        move.w  0x24(a0),d0                     | +012
        subi.w  #0xc,d0                         | +016
        move.w  d0,0x70(a6)                     | +01a
        cmpi.w  #0x1,0x72(a6)                   | +01e
        bne.w   SetTaskHandler_07f66c           | +024

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Init_07f674  @ $07F674  (236 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Init_07f674, "ax", @progbits
        .global Carrier_Gunner_Init_07f674
Carrier_Gunner_Init_07f674:
        lea     Carrier_Gunner_Pose_07f84a(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        lea     Carrier_Gunner_Reload_07f768(pc),a1 | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        moveq   #0,d0                           | +020
        move.b  d0,0x20(a6)                     | +022
        move.b  #0x1,0x21(a6)                   | +026
        move.w  d0,0x70(a6)                     | +02c
        move.w  #0x28,0x72(a6)                  | +030
        move.w  #0xf0,0x74(a6)                  | +036
        move.w  #0x3c,0x76(a6)                  | +03c
        move.b  d0,0x78(a6)                     | +042
        lea     .L07f6c0(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L07f6c0:
        btst    #0x0,0x21(a6)                   | +04c
        bne.w   .L07f6e8                        | +052
        subq.w  #0x1,0x72(a6)                   | +056
        bne.w   .L07f6e8                        | +05a
        move.w  #0x28,0x72(a6)                  | +05e
        lea     Carrier_Gunner_Reload_07f768(pc),a1 | +064
        jsr     0x4ae.l                         | +068
        jsr     0x5dd02.l                       | +06e
.L07f6e8:
        btst    #0x1,0x21(a6)                   | +074
        bne.w   .L07f710                        | +07a
        subq.w  #0x1,0x76(a6)                   | +07e
        bne.w   .L07f710                        | +082
        move.w  #0x3c,0x76(a6)                  | +086
        lea     Carrier_Gunner_Fire_07f8dc(pc),a1 | +08c
        jsr     0x4ae.l                         | +090
        jsr     0x5dd02.l                       | +096
.L07f710:
        subq.w  #0x1,0x74(a6)                   | +09c
        bne.w   .L07f73e                        | +0a0
        jsr     0x5e9b6.l                       | +0a4
        andi.w  #0x7f,d0                        | +0aa
        subi.w  #0x3f,d0                        | +0ae
        addi.w  #0x258,d0                       | +0b2
        move.w  d0,0x74(a6)                     | +0b6
        lea     Carrier_Gunner_Fire_07f8dc__L07f902(pc),a1 | +0ba
        jsr     0x4ae.l                         | +0be
        jsr     0x5dd02.l                       | +0c4
.L07f73e:
        movea.l 0xc(a6),a0                      | +0ca
        cmpi.b  #0x3,0x20(a0)                   | +0ce
        beq.w   .L07f758                        | +0d4
        move.b  #0xff,0x20(a6)                  | +0d8
        lea     TaskHandler_07fdbc(pc),a1       | +0de
        move.l  a1,(a6)                         | +0e2
.L07f758:
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +0e4
        bcc.w   SetHandlerRts_07f766            | +0e8

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Reload_07f768  @ $07F768  (128 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Reload_07f768, "ax", @progbits
        .global Carrier_Gunner_Reload_07f768
Carrier_Gunner_Reload_07f768:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        addi.w  #0x18,0x38(a6)                  | +00a
        movea.l 0xc(a6),a0                      | +010
        move.b  #0x1,0x78(a0)                   | +014
        bset    #0x0,0x21(a0)                   | +01a
        move.b  #0xff,0x21(a6)                  | +020
        jsr     0x267e2.l                       | +026
        move.w  #0x0,0x76(a6)                   | +02c
        move.w  #0x0,0x78(a6)                   | +032
        lea     0x2e2286.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        lea     .L07f7b2(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L07f7b2:
        movea.l 0xc(a6),a0                      | +04a
        movea.l 0xc(a0),a0                      | +04e
        move.w  0x78(a0),0x22(a6)               | +052
        move.w  0x7a(a0),0x24(a6)               | +058
        subi.w  #0x9,0x22(a6)                   | +05e
        jsr     0x28d70.l                       | +064
        bcc.w   .L07f7dc                        | +06a
        lea     Carrier_Gunner_Hold_07f7f0(pc),a1 | +06e
        move.l  a1,(a6)                         | +072
.L07f7dc:
        jsr     Squad_DeferredRelease_080054(pc) | +074
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +078
        bcc.w   SetHandlerRts_07f7ee            | +07c

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Hold_07f7f0  @ $07F7F0  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Hold_07f7f0, "ax", @progbits
        .global Carrier_Gunner_Hold_07f7f0
Carrier_Gunner_Hold_07f7f0:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0x0,0x78(a0)                   | +004
        lea     0x2e2128.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L07f80c(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07f80c:
        movea.l 0xc(a6),a0                      | +01c
        movea.l 0xc(a0),a0                      | +020
        move.w  0x78(a0),0x22(a6)               | +024
        move.w  0x7a(a0),0x24(a6)               | +02a
        subi.w  #0x9,0x22(a6)                   | +030
        jsr     0x28d70.l                       | +036
        movea.l 0xc(a6),a0                      | +03c
        move.w  0x70(a6),0x70(a0)               | +040
        jsr     Squad_DeferredRelease_080054(pc) | +046
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +04a
        bcc.w   SetHandlerRts_07f848            | +04e

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Pose_07f84a  @ $07F84A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Pose_07f84a, "ax", @progbits
        .global Carrier_Gunner_Pose_07f84a
Carrier_Gunner_Pose_07f84a:
        move.w  #0x26,d1                        | +000
        jsr     0x236e.l                        | +004
        addi.w  #0x10,0x38(a6)                  | +00a
        clr.b   0x20(a6)                        | +010
        clr.b   0x21(a6)                        | +014
        jsr     0x267e2.l                       | +018
        lea     0x2e21ac.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024

| ----------------------------------------------------------------------------
|  Carrier_Gunner_PoseRun_07f87c  @ $07F87C  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_PoseRun_07f87c, "ax", @progbits
        .global Carrier_Gunner_PoseRun_07f87c
Carrier_Gunner_PoseRun_07f87c:
        movea.l 0xc(a6),a0                      | +000
        movea.l 0xc(a0),a0                      | +004
        move.w  0x78(a0),0x22(a6)               | +008
        move.w  0x7a(a0),0x24(a6)               | +00e
        subi.w  #0x9,0x22(a6)                   | +014
        movea.l 0xc(a6),a0                      | +01a
        cmpi.b  #0x1,0x78(a0)                   | +01e
        beq.w   .L07f8c6                        | +024
        cmpi.b  #0xff,0x20(a0)                  | +028
        beq.w   TaskHandler_07fdbc              | +02e
        move.w  0x70(a0),d0                     | +032
        add.w   d0,d0                           | +036
        add.w   d0,d0                           | +038
        lea     0x2e2194.l,a0                   | +03a
        movea.l (a0,d0.w),a0                    | +040
        jsr     0x28cd4.l                       | +044
.L07f8c6:
        jsr     0x28d70.l                       | +04a
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +050
        bcc.w   SetHandlerRts_07f8da            | +054

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Fire_07f8dc  @ $07F8DC  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Fire_07f8dc, "ax", @progbits
        .global Carrier_Gunner_Fire_07f8dc
Carrier_Gunner_Fire_07f8dc:
        movea.l 0xc(a6),a0                      | +000
        bset    #0x1,0x21(a0)                   | +004
        move.b  #0xff,0x21(a6)                  | +00a
        move.w  #0xffff,0x76(a6)                | +010
        move.w  #0x1,0x78(a6)                   | +016
        addi.w  #0x1c,0x38(a6)                  | +01c
        bra.w   .L07f918                        | +022
        .global Carrier_Gunner_Fire_07f8dc__L07f902
Carrier_Gunner_Fire_07f8dc__L07f902:
.L07f902:
        clr.b   0x21(a6)                        | +026
        move.w  #0x0,0x76(a6)                   | +02a
        move.w  #0x2,0x78(a6)                   | +030
        addi.w  #0x20,0x38(a6)                  | +036
.L07f918:
        move.w  #0xe,d1                         | +03c
        jsr     0x236e.l                        | +040
        jsr     0x267e2.l                       | +046
        lea     0x2e2286.l,a0                   | +04c
        jsr     0x28cd4.l                       | +052
        lea     .L07f93a(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L07f93a:
        movea.l 0xc(a6),a0                      | +05e
        movea.l 0xc(a0),a0                      | +062
        move.w  0x78(a0),0x22(a6)               | +066
        move.w  0x7a(a0),0x24(a6)               | +06c
        subi.w  #0x9,0x22(a6)                   | +072
        jsr     0x28d70.l                       | +078
        bcc.w   .L07f974                        | +07e
        jsr     0x5e9b6.l                       | +082
        andi.w  #0x1,d0                         | +088
        add.w   d0,d0                           | +08c
        move.w  d0,0x72(a6)                     | +08e
        lea     Carrier_Gunner_Land_07f988(pc),a1 | +092
        move.l  a1,(a6)                         | +096
.L07f974:
        jsr     Squad_DeferredRelease_080054(pc) | +098
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +09c
        bcc.w   SetHandlerRts_07f986            | +0a0

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Land_07f988  @ $07F988  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Land_07f988, "ax", @progbits
        .global Carrier_Gunner_Land_07f988
Carrier_Gunner_Land_07f988:
        tst.w   0x76(a6)                        | +000
        bne.w   .L07f990                        | +004
.L07f990:
        move.w  #0x0,0x74(a6)                   | +008
        lea     0x2e22e8.l,a0                   | +00e
        jsr     0x28cd4.l                       | +014
        lea     .L07f9a8(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L07f9a8:
        jsr     Squad_TrackArc_07fdfa__L07fe66(pc) | +020
        jsr     0x28d70.l                       | +024
        bcc.w   .L07f9bc                        | +02a
        lea     Carrier_Gunner_Stand_07f9d0(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L07f9bc:
        jsr     Squad_DeferredRelease_080054(pc) | +034
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +038
        bcc.w   SetHandlerRts_07f9ce            | +03c

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Stand_07f9d0  @ $07F9D0  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Stand_07f9d0, "ax", @progbits
        .global Carrier_Gunner_Stand_07f9d0
Carrier_Gunner_Stand_07f9d0:
        tst.w   0x76(a6)                        | +000
        bne.w   .L07f9e6                        | +004
        jsr     0x5e9b6.l                       | +008
        btst    #0x0,d0                         | +00e
        bne.w   Carrier_Gunner_Walk_07fa20__L07fa26 | +012
.L07f9e6:
        lea     0x29bfb8.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L07f9f8(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L07f9f8:
        jsr     Squad_TrackArc_07fdfa__L07fe66(pc) | +028
        jsr     0x28d70.l                       | +02c
        bcc.w   .L07fa0c                        | +032
        lea     Carrier_Gunner_Walk_07fa20(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
.L07fa0c:
        jsr     Squad_DeferredRelease_080054(pc) | +03c
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +040
        bcc.w   SetHandlerRts_07fa1e            | +044

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Walk_07fa20  @ $07FA20  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Walk_07fa20, "ax", @progbits
        .global Carrier_Gunner_Walk_07fa20
Carrier_Gunner_Walk_07fa20:
        move.b  #0x1,0x3a(a6)                   | +000
        .global Carrier_Gunner_Walk_07fa20__L07fa26
Carrier_Gunner_Walk_07fa20__L07fa26:
.L07fa26:
        jsr     0x267e2.l                       | +006
        move.w  #0xfe00,0x28(a6)                | +00c
        btst    #0x0,0x3a(a6)                   | +012
        beq.w   .L07fa40                        | +018
        neg.w   0x28(a6)                        | +01c
.L07fa40:
        clr.w   0x74(a6)                        | +020
        lea     0x29b744.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        lea     .L07fa56(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L07fa56:
        jsr     Squad_TrackArc_07fdfa__L07fe66(pc) | +036
        jsr     0x28d70.l                       | +03a
        move.w  0x74(a6),d0                     | +040
        bpl.w   .L07fa6a                        | +044
        neg.w   d0                              | +048
.L07fa6a:
        cmpi.w  #0x2000,d0                      | +04a
        blt.w   .L07fa7e                        | +04e
        jsr     0x267e2.l                       | +052
        lea     Carrier_Gunner_Stop_07fa92(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L07fa7e:
        jsr     Squad_DeferredRelease_080054(pc) | +05e
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +062
        bcc.w   SetHandlerRts_07fa90            | +066

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Stop_07fa92  @ $07FA92  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Stop_07fa92, "ax", @progbits
        .global Carrier_Gunner_Stop_07fa92
Carrier_Gunner_Stop_07fa92:
        lea     0x29b7c8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07faa4(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07faa4:
        jsr     Squad_TrackArc_07fdfa__L07fe66(pc) | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L07fab8                        | +01c
        lea     Carrier_Gunner_Ride_07facc(pc),a1 | +020
        move.l  a1,(a6)                         | +024
.L07fab8:
        jsr     Squad_DeferredRelease_080054(pc) | +026
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +02a
        bcc.w   SetHandlerRts_07faca            | +02e

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Ride_07facc  @ $07FACC  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Ride_07facc, "ax", @progbits
        .global Carrier_Gunner_Ride_07facc
Carrier_Gunner_Ride_07facc:
        tst.w   0x76(a6)                        | +000
        bne.w   Carrier_Gunner_Idle_07fb28      | +004
        move.w  #0x28,d1                        | +008
        jsr     0x236e.l                        | +00c
        lea     0x29c77c.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L07faf0(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L07faf0:
        jsr     Squad_TrackArc_07fdfa__L07fe66(pc) | +024
        jsr     0x28d70.l                       | +028
        bcc.w   .L07fb14                        | +02e
        movea.l 0xc(a6),a0                      | +032
        move.w  0x24(a0),d0                     | +036
        subi.w  #0xc,d0                         | +03a
        move.w  d0,0x70(a6)                     | +03e
        lea     Carrier_Rider_JumpB_07f33e(pc),a1 | +042
        move.l  a1,(a6)                         | +046
.L07fb14:
        jsr     Squad_DeferredRelease_080054(pc) | +048
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +04c
        bcc.w   SetHandlerRts_07fb26            | +050

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Idle_07fb28  @ $07FB28  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Idle_07fb28, "ax", @progbits
        .global Carrier_Gunner_Idle_07fb28
Carrier_Gunner_Idle_07fb28:
        jsr     Carrier_Trooper_HPFromTblB_07ed6a(pc) | +000
        lea     0x29b816.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L07fb3e(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L07fb3e:
        jsr     Squad_TrackArc_07fdfa__L07fe66(pc) | +016
        jsr     0x28d70.l                       | +01a
        tst.w   0x90(a6)                        | +020
        bne.w   .L07fb56                        | +024
        lea     Carrier_Gunner_Decide_07fb6e(pc),a1 | +028
        move.l  a1,(a6)                         | +02c
.L07fb56:
        subq.w  #0x1,0x90(a6)                   | +02e
        jsr     Squad_DeferredRelease_080054(pc) | +032
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +036
        bcc.w   SetHandlerRts_07fb6c            | +03a

| ----------------------------------------------------------------------------
|  Carrier_Gunner_Decide_07fb6e  @ $07FB6E  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Carrier_Gunner_Decide_07fb6e, "ax", @progbits
        .global Carrier_Gunner_Decide_07fb6e
Carrier_Gunner_Decide_07fb6e:
        jsr     0x5e0d4.l                       | +000
        move.w  0x22(a0),d0                     | +006
        cmp.w   0x22(a6),d0                     | +00a
        ble.w   .L07fb8e                        | +00e
        btst    #0x0,0x3a(a6)                   | +012
        bne.w   Squad_EscortInit_07fbd2         | +018
        bra.w   .L07fb98                        | +01c
.L07fb8e:
        btst    #0x0,0x3a(a6)                   | +020
        beq.w   Squad_EscortInit_07fbd2         | +026
.L07fb98:
        lea     0x29bf34.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L07fbaa(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L07fbaa:
        jsr     Squad_TrackArc_07fdfa__L07fe66(pc) | +03c
        jsr     0x28d70.l                       | +040
        bcc.w   .L07fbbe                        | +046
        lea     Squad_EscortInit_07fbd2(pc),a1  | +04a
        move.l  a1,(a6)                         | +04e
.L07fbbe:
        jsr     Squad_DeferredRelease_080054(pc) | +050
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +054
        bcc.w   SetHandlerRts_07fbd0            | +058
