| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave NNN — Cápsula de fin de misión, integradores de posición, soldados rasos y plantillas 153..181
|  Región: $08D17A..$08E4E4  (4,442 B, 80 entradas, 67 huecos)
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
|  Screen_InBoundsX_Latched_08d184  @ $08D184  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Screen_InBoundsX_Latched_08d184, "ax", @progbits
        .global Screen_InBoundsX_Latched_08d184
Screen_InBoundsX_Latched_08d184:
        btst    #0x7,0x13(a6)                   | +000
        beq.w   Screen_InBoundsX_Latch_08d1ae   | +006
        cmpi.w  #0x0,0x22(a6)                   | +00a
        blt.w   SetXN_08d1a8                    | +010
        cmpi.w  #0x140,0x22(a6)                 | +014
        bgt.w   SetXN_08d1a8                    | +01a

| ----------------------------------------------------------------------------
|  Screen_InBoundsX_Latch_08d1ae  @ $08D1AE  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Screen_InBoundsX_Latch_08d1ae, "ax", @progbits
        .global Screen_InBoundsX_Latch_08d1ae
Screen_InBoundsX_Latch_08d1ae:
        cmpi.w  #0x140,0x22(a6)                 | +000
        bge.w   ClearC_08d1e2                   | +006
        cmpi.w  #0x0,0x22(a6)                   | +00a
        ble.w   ClearC_08d1e2                   | +010
        cmpi.w  #0x1f0,0x24(a6)                 | +014
        bge.w   ClearC_08d1e2                   | +01a
        cmpi.w  #0x100,0x24(a6)                 | +01e
        ble.w   ClearC_08d1e2                   | +024
        bset    #0x7,0x13(a6)                   | +028

| ----------------------------------------------------------------------------
|  Screen_InBoundsXWide_Latched_08d1e8  @ $08D1E8  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Screen_InBoundsXWide_Latched_08d1e8, "ax", @progbits
        .global Screen_InBoundsXWide_Latched_08d1e8
Screen_InBoundsXWide_Latched_08d1e8:
        btst    #0x7,0x13(a6)                   | +000
        beq.w   Screen_InBoundsXWide_Latch_08d212 | +006
        cmpi.w  #0xffe0,0x22(a6)                | +00a
        blt.w   SetXN_08d20c                    | +010
        cmpi.w  #0x140,0x22(a6)                 | +014
        bgt.w   SetXN_08d20c                    | +01a

| ----------------------------------------------------------------------------
|  Screen_InBoundsXWide_Latch_08d212  @ $08D212  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Screen_InBoundsXWide_Latch_08d212, "ax", @progbits
        .global Screen_InBoundsXWide_Latch_08d212
Screen_InBoundsXWide_Latch_08d212:
        cmpi.w  #0x140,0x22(a6)                 | +000
        bge.w   ClearC_08d246                   | +006
        cmpi.w  #0x0,0x22(a6)                   | +00a
        ble.w   ClearC_08d246                   | +010
        cmpi.w  #0x1f0,0x24(a6)                 | +014
        bge.w   ClearC_08d246                   | +01a
        cmpi.w  #0x100,0x24(a6)                 | +01e
        ble.w   ClearC_08d246                   | +024
        bset    #0x7,0x13(a6)                   | +028

| ----------------------------------------------------------------------------
|  Screen_InBoundsY_Latched_08d24c  @ $08D24C  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Screen_InBoundsY_Latched_08d24c, "ax", @progbits
        .global Screen_InBoundsY_Latched_08d24c
Screen_InBoundsY_Latched_08d24c:
        btst    #0x7,0x13(a6)                   | +000
        beq.w   Screen_InBoundsY_Latch_08d276   | +006
        cmpi.w  #0x1ff,0x24(a6)                 | +00a
        bgt.w   SetXN_08d270                    | +010
        cmpi.w  #0x100,0x24(a6)                 | +014
        blt.w   SetXN_08d270                    | +01a

| ----------------------------------------------------------------------------
|  Screen_InBoundsY_Latch_08d276  @ $08D276  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Screen_InBoundsY_Latch_08d276, "ax", @progbits
        .global Screen_InBoundsY_Latch_08d276
Screen_InBoundsY_Latch_08d276:
        cmpi.w  #0x140,0x22(a6)                 | +000
        bge.w   ClearC_08d2aa                   | +006
        cmpi.w  #0x0,0x22(a6)                   | +00a
        ble.w   ClearC_08d2aa                   | +010
        cmpi.w  #0x1ff,0x24(a6)                 | +014
        bge.w   ClearC_08d2aa                   | +01a
        cmpi.w  #0x100,0x24(a6)                 | +01e
        ble.w   ClearC_08d2aa                   | +024
        bset    #0x7,0x13(a6)                   | +028

| ----------------------------------------------------------------------------
|  Pos_IntegrateX88_08d2b0  @ $08D2B0  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Pos_IntegrateX88_08d2b0, "ax", @progbits
        .global Pos_IntegrateX88_08d2b0
Pos_IntegrateX88_08d2b0:
        move.w  0x22(a6),d0                     | +000
        swap    d0                              | +004
        move.w  0x26(a6),d0                     | +006
        clr.b   d0                              | +00a
        move.w  0x28(a6),d1                     | +00c
        ext.l   d1                              | +010
        asl.l   #0x8,d1                         | +012
        add.l   d1,d0                           | +014
        asr.w   #0x8,d0                         | +016
        move.b  d0,0x26(a6)                     | +018
        swap    d0                              | +01c

| ----------------------------------------------------------------------------
|  Pos_IntegrateY88_08d2d4  @ $08D2D4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Pos_IntegrateY88_08d2d4, "ax", @progbits
        .global Pos_IntegrateY88_08d2d4
Pos_IntegrateY88_08d2d4:
        move.w  0x24(a6),d0                     | +000
        swap    d0                              | +004
        move.b  0x27(a6),d0                     | +006
        asl.w   #0x8,d0                         | +00a
        move.w  0x2a(a6),d1                     | +00c
        ext.l   d1                              | +010
        asl.l   #0x8,d1                         | +012
        add.l   d1,d0                           | +014
        asr.w   #0x8,d0                         | +016
        move.b  d0,0x27(a6)                     | +018
        swap    d0                              | +01c

| ----------------------------------------------------------------------------
|  Pos_IntegrateXY88_08d2f8  @ $08D2F8  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Pos_IntegrateXY88_08d2f8, "ax", @progbits
        .global Pos_IntegrateXY88_08d2f8
Pos_IntegrateXY88_08d2f8:
        tst.w   0x28(a6)                        | +000
        beq.w   .L08d322                        | +004
        move.w  0x22(a6),d0                     | +008
        swap    d0                              | +00c
        move.w  0x26(a6),d0                     | +00e
        clr.b   d0                              | +012
        move.w  0x28(a6),d1                     | +014
        ext.l   d1                              | +018
        asl.l   #0x8,d1                         | +01a
        add.l   d1,d0                           | +01c
        asr.w   #0x8,d0                         | +01e
        move.b  d0,0x26(a6)                     | +020
        swap    d0                              | +024
        move.w  d0,0x22(a6)                     | +026
.L08d322:
        tst.w   0x2a(a6)                        | +02a
        beq.w   SetTaskWRts_08d34c              | +02e
        move.w  0x24(a6),d0                     | +032
        swap    d0                              | +036
        move.b  0x27(a6),d0                     | +038
        asl.w   #0x8,d0                         | +03c
        move.w  0x2a(a6),d1                     | +03e
        ext.l   d1                              | +042
        asl.l   #0x8,d1                         | +044
        add.l   d1,d0                           | +046
        asr.w   #0x8,d0                         | +048
        move.b  d0,0x27(a6)                     | +04a
        swap    d0                              | +04e

| ----------------------------------------------------------------------------
|  Pos_IntegrateXY88_Accel_08d34e  @ $08D34E  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Pos_IntegrateXY88_Accel_08d34e, "ax", @progbits
        .global Pos_IntegrateXY88_Accel_08d34e
Pos_IntegrateXY88_Accel_08d34e:
        tst.w   0x28(a6)                        | +000
        beq.w   .L08d378                        | +004
        move.w  0x22(a6),d0                     | +008
        swap    d0                              | +00c
        move.w  0x26(a6),d0                     | +00e
        clr.b   d0                              | +012
        move.w  0x28(a6),d1                     | +014
        ext.l   d1                              | +018
        asl.l   #0x8,d1                         | +01a
        add.l   d1,d0                           | +01c
        asr.w   #0x8,d0                         | +01e
        move.b  d0,0x26(a6)                     | +020
        swap    d0                              | +024
        move.w  d0,0x22(a6)                     | +026
.L08d378:
        tst.w   0x2a(a6)                        | +02a
        beq.w   .L08d3a2                        | +02e
        move.w  0x24(a6),d0                     | +032
        swap    d0                              | +036
        move.b  0x27(a6),d0                     | +038
        asl.w   #0x8,d0                         | +03c
        move.w  0x2a(a6),d1                     | +03e
        ext.l   d1                              | +042
        asl.l   #0x8,d1                         | +044
        add.l   d1,d0                           | +046
        asr.w   #0x8,d0                         | +048
        move.b  d0,0x27(a6)                     | +04a
        swap    d0                              | +04e
        move.w  d0,0x24(a6)                     | +050
.L08d3a2:
        move.w  0x2c(a6),d0                     | +054
        add.w   d0,0x28(a6)                     | +058
        move.w  0x2e(a6),d0                     | +05c
        add.w   d0,0x2a(a6)                     | +060
        rts                                     | +064

| ----------------------------------------------------------------------------
|  Capsule_Fly_08d3b4  @ $08D3B4  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_Fly_08d3b4, "ax", @progbits
        .global Capsule_Fly_08d3b4
Capsule_Fly_08d3b4:
        move.w  #0x8000,0x38(a6)                | +000
        move.w  #0xe,d1                         | +006
        jsr     0x236e.l                        | +00a
        lea     0x2f460c.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        move.b  #0x1,0x3a(a6)                   | +01c
        move.w  #0x120,0x22(a6)                 | +022
        move.w  #0x190,0x24(a6)                 | +028
        clr.w   0x26(a6)                        | +02e
        jsr     0x5e7c0.l                       | +032
        lea     .L08d3f2(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L08d3f2:
        jsr     0x2783a.l                       | +03e
        jsr     0x28d70.l                       | +044
        cmpi.w  #0x100,0x22(a6)                 | +04a
        bgt.w   SetHandlerRts_08d41a            | +050
        lea     0x2f4628.l,a0                   | +054
        jsr     0x28cd4.l                       | +05a

| ----------------------------------------------------------------------------
|  Capsule_Descend_08d41c  @ $08D41C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_Descend_08d41c, "ax", @progbits
        .global Capsule_Descend_08d41c
Capsule_Descend_08d41c:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   SetHandlerRts_08d44e            | +00c
        lea     Capsule_Open_08d474(pc),a1      | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        lea     0x2f46ca.l,a0                   | +020
        jsr     0x28cd4.l                       | +026

| ----------------------------------------------------------------------------
|  Capsule_WaitGround_08d450  @ $08D450  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_WaitGround_08d450, "ax", @progbits
        .global Capsule_WaitGround_08d450
Capsule_WaitGround_08d450:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        movea.l #0xffffffff,a0                  | +00c
        jsr     0x5dd5c.l                       | +012
        bcc.w   TaskHandler_08d472              | +018

| ----------------------------------------------------------------------------
|  Capsule_Open_08d474  @ $08D474  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_Open_08d474, "ax", @progbits
        .global Capsule_Open_08d474
Capsule_Open_08d474:
        clr.b   0x3a(a6)                        | +000
        move.w  #0xf000,0x38(a6)                | +004
        move.w  #0xf5,d1                        | +00a
        jsr     0x236e.l                        | +00e
        lea     0x2f3904.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        addi.w  #0x4,0x22(a6)                   | +020
        addi.w  #0x20,0x24(a6)                  | +026
        move.b  #0xff,0x32(a6)                  | +02c
        move.b  #0xff,0x33(a6)                  | +032
        move.b  #0x0,0x5c(a6)                   | +038
        move.w  #0x200,0x28(a6)                 | +03e
        move.w  #0x4,0x70(a6)                   | +044

| ----------------------------------------------------------------------------
|  Capsule_Flash_08d4c6  @ $08D4C6  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_Flash_08d4c6, "ax", @progbits
        .global Capsule_Flash_08d4c6
Capsule_Flash_08d4c6:
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +000
        jsr     0x28d70.l                       | +004
        subq.w  #0x1,0x70(a6)                   | +00a
        cmpi.w  #0x0,0x70(a6)                   | +00e
        bgt.w   SetHandlerRts_08d4fc            | +014
        move.b  #0x38,0x32(a6)                  | +018
        move.b  #0x38,0x33(a6)                  | +01e
        lea     0x2f38f8.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a

| ----------------------------------------------------------------------------
|  Capsule_FadeIn_08d4fe  @ $08D4FE  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_FadeIn_08d4fe, "ax", @progbits
        .global Capsule_FadeIn_08d4fe
Capsule_FadeIn_08d4fe:
        addq.b  #0x2,0x32(a6)                   | +000
        addq.b  #0x2,0x33(a6)                   | +004
        cmpi.b  #0x7f,0x32(a6)                  | +008
        bcs.w   .L08d51c                        | +00e
        move.b  #0x7f,0x32(a6)                  | +012
        move.b  #0x7f,0x33(a6)                  | +018
.L08d51c:
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +01e
        jsr     0x28d70.l                       | +022
        cmpi.w  #0xa0,0x22(a6)                  | +028
        blt.w   SetHandlerRts_08d536            | +02e

| ----------------------------------------------------------------------------
|  Capsule_Glow_08d538  @ $08D538  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_Glow_08d538, "ax", @progbits
        .global Capsule_Glow_08d538
Capsule_Glow_08d538:
        lea     0x2f377e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08d54a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08d54a:
        jsr     0x28d70.l                       | +012
        bcc.w   SetHandlerRts_08d55a            | +018

| ----------------------------------------------------------------------------
|  Capsule_GlowDown_08d55c  @ $08D55C  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_GlowDown_08d55c, "ax", @progbits
        .global Capsule_GlowDown_08d55c
Capsule_GlowDown_08d55c:
        subq.b  #0x2,0x32(a6)                   | +000
        subq.b  #0x2,0x33(a6)                   | +004
        cmpi.b  #0x7f,0x32(a6)                  | +008
        bhi.w   .L08d574                        | +00e
        lea     Capsule_Glow2_08d580(pc),a1     | +012
        move.l  a1,(a6)                         | +016
.L08d574:
        jsr     Capsule_CheckMissionEnd_08d804(pc) | +018

| ----------------------------------------------------------------------------
|  Capsule_Glow2_08d580  @ $08D580  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_Glow2_08d580, "ax", @progbits
        .global Capsule_Glow2_08d580
Capsule_Glow2_08d580:
        lea     0x2f37da.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08d592(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08d592:
        jsr     0x28d70.l                       | +012
        bcc.w   JsrPcThunk_08d5a2               | +018
        lea     Capsule_GlowUp_08d5a8(pc),a1    | +01c
        move.l  a1,(a6)                         | +020

| ----------------------------------------------------------------------------
|  Capsule_GlowUp_08d5a8  @ $08D5A8  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_GlowUp_08d5a8, "ax", @progbits
        .global Capsule_GlowUp_08d5a8
Capsule_GlowUp_08d5a8:
        addq.b  #0x2,0x32(a6)                   | +000
        addq.b  #0x2,0x33(a6)                   | +004
        cmpi.b  #0xd0,0x32(a6)                  | +008
        bcs.w   .L08d5c0                        | +00e
        lea     Capsule_Glow_08d538(pc),a1      | +012
        move.l  a1,(a6)                         | +016
.L08d5c0:
        jsr     0x28d70.l                       | +018

| ----------------------------------------------------------------------------
|  Capsule_Rise_08d5cc  @ $08D5CC  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_Rise_08d5cc, "ax", @progbits
        .global Capsule_Rise_08d5cc
Capsule_Rise_08d5cc:
        move.w  0x24(a6),0x5c(a6)               | +000
        move.w  #0xffc0,0x2a(a6)                | +006
        move.w  #0x2000,0x38(a6)                | +00c
        move.w  #0xc8,0x70(a6)                  | +012
        lea     .L08d5ea(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L08d5ea:
        subq.b  #0x2,0x32(a6)                   | +01e
        subq.b  #0x2,0x33(a6)                   | +022
        cmpi.b  #0x34,0x32(a6)                  | +026
        bhi.w   .L08d608                        | +02c
        move.b  #0x34,0x32(a6)                  | +030
        move.b  #0x34,0x33(a6)                  | +036
.L08d608:
        jsr     Pos_IntegrateY88_08d2d4(pc)     | +03c
        jsr     0x28d70.l                       | +040
        subq.w  #0x1,0x70(a6)                   | +046
        cmpi.w  #0x0,0x70(a6)                   | +04a
        bgt.w   SetHandlerRts_08d62c            | +050
        move.w  #0x80,0x2a(a6)                  | +054

| ----------------------------------------------------------------------------
|  Capsule_Fall_08d62e  @ $08D62E  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_Fall_08d62e, "ax", @progbits
        .global Capsule_Fall_08d62e
Capsule_Fall_08d62e:
        addq.b  #0x2,0x32(a6)                   | +000
        addq.b  #0x2,0x33(a6)                   | +004
        cmpi.b  #0x7f,0x32(a6)                  | +008
        bcs.w   .L08d64c                        | +00e
        move.b  #0x7f,0x32(a6)                  | +012
        move.b  #0x7f,0x33(a6)                  | +018
.L08d64c:
        jsr     Pos_IntegrateY88_08d2d4(pc)     | +01e
        jsr     0x28d70.l                       | +022
        move.w  0x5c(a6),d0                     | +028
        cmp.w   0x24(a6),d0                     | +02c
        bge.w   SetHandlerRts_08d66e            | +030
        move.w  #0xf000,0x38(a6)                | +034

| ----------------------------------------------------------------------------
|  MissionEnd_ScrollOut_08d670  @ $08D670  (116 B)
| ----------------------------------------------------------------------------
        .section .text.MissionEnd_ScrollOut_08d670, "ax", @progbits
        .global MissionEnd_ScrollOut_08d670
MissionEnd_ScrollOut_08d670:
        move.b  #0x0,0x5c(a6)                   | +000
        move.l  #0x20000,0x80(a6)               | +006
        move.l  #0x0,0x84(a6)                   | +00e
        move.l  #0xffff0000,0x88(a6)            | +016
        move.l  #0x0,0x8c(a6)                   | +01e
        lea     0x2f3910.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L08d6a8(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L08d6a8:
        jsr     Scroll_StepVelX_08ccc6(pc)      | +038
        jsr     Scroll_StepVelY_08ccd8(pc)      | +03c
        jsr     PcThunkTarget_08efb0(pc)        | +040
        cmpi.b  #0x2,0x32(a6)                   | +044
        bcs.w   .L08d6ce                        | +04a
        move.b  0x5c(a6),d0                     | +04e
        add.b   d0,0x32(a6)                     | +052
        add.b   d0,0x33(a6)                     | +056
        bra.w   JsrAbsThunk_08d6e4              | +05a
.L08d6ce:
        lea     MissionEnd_Flash_08d6ec(pc),a1  | +05e
        jsr     0x4ae.l                         | +062
        jsr     0x5dd02.l                       | +068
        jmp     0x518.l                         | +06e

| ----------------------------------------------------------------------------
|  MissionEnd_Flash_08d6ec  @ $08D6EC  (54 B)
| ----------------------------------------------------------------------------
        .section .text.MissionEnd_Flash_08d6ec, "ax", @progbits
        .global MissionEnd_Flash_08d6ec
MissionEnd_Flash_08d6ec:
        move.w  #0xf4,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x2f44f6.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x14,0x70(a6)                  | +016
        lea     .L08d70e(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L08d70e:
        jsr     0x4407a.l                       | +022
        subq.w  #0x1,0x70(a6)                   | +028
        cmpi.w  #0x0,0x70(a6)                   | +02c
        bgt.w   SetHandlerRts_08d728            | +032

| ----------------------------------------------------------------------------
|  MissionEnd_SpawnDropper_08d72a  @ $08D72A  (74 B)
| ----------------------------------------------------------------------------
        .section .text.MissionEnd_SpawnDropper_08d72a, "ax", @progbits
        .global MissionEnd_SpawnDropper_08d72a
MissionEnd_SpawnDropper_08d72a:
        jsr     0x4407a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L08d772                        | +00c
        lea     Sub_0008EDC6(pc),a1             | +010  -> $08EDC6 (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        lea     Cut_Dropper_08ccea(pc),a1       | +020
        jsr     0x4ae.l                         | +024
        move.b  #0xa,0x98(a0)                   | +02a
        move.b  #0x8,0x99(a0)                   | +030
        move.w  #0xa0,0x22(a0)                  | +036
        move.w  #0x100,0x24(a0)                 | +03c
        jmp     0x518.l                         | +042
.L08d772:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  Capsule_DimWaitScroll_08d774  @ $08D774  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_DimWaitScroll_08d774, "ax", @progbits
        .global Capsule_DimWaitScroll_08d774
Capsule_DimWaitScroll_08d774:
        subq.b  #0x2,0x32(a6)                   | +000
        subq.b  #0x2,0x33(a6)                   | +004
        cmpi.b  #0x34,0x32(a6)                  | +008
        bcc.w   .L08d792                        | +00e
        move.b  #0x34,0x32(a6)                  | +012
        move.b  #0x34,0x33(a6)                  | +018
.L08d792:
        jsr     0x28d70.l                       | +01e
        cmpi.w  #0x433f,0x106f5c.l              | +024
        bcs.w   SetHandlerRts_08d7bc            | +02c
        move.w  #0x0,0x28(a6)                   | +030
        move.w  #0xff80,0x2a(a6)                | +036
        move.w  #0xffc0,0x2e(a6)                | +03c

| ----------------------------------------------------------------------------
|  Capsule_FallToGround_08d7be  @ $08D7BE  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_FallToGround_08d7be, "ax", @progbits
        .global Capsule_FallToGround_08d7be
Capsule_FallToGround_08d7be:
        jsr     0x2783a.l                       | +000
        jsr     0x27bc8.l                       | +006
        bcc.w   JsrAbsThunk_08d7da              | +00c
        move.w  #0x75,0x70(a6)                  | +010
        lea     Capsule_Vanish_08d7e2(pc),a1    | +016
        move.l  a1,(a6)                         | +01a

| ----------------------------------------------------------------------------
|  Capsule_Vanish_08d7e2  @ $08D7E2  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_Vanish_08d7e2, "ax", @progbits
        .global Capsule_Vanish_08d7e2
Capsule_Vanish_08d7e2:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L08d7f6                        | +00a
        jmp     0x518.l                         | +00e
.L08d7f6:
        jsr     0x2783a.l                       | +014

| ----------------------------------------------------------------------------
|  Capsule_CheckMissionEnd_08d804  @ $08D804  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Capsule_CheckMissionEnd_08d804, "ax", @progbits
        .global Capsule_CheckMissionEnd_08d804
Capsule_CheckMissionEnd_08d804:
        cmpi.b  #0xb,0x106ece.l                 | +000
        bne.w   .L08d822                        | +008
        cmpi.w  #0x42c0,0x106f5c.l              | +00c
        bcs.w   .L08d822                        | +014
        lea     MissionEnd_ScrollOut_08d670(pc),a1 | +018
        move.l  a1,(a6)                         | +01c
.L08d822:
        cmpi.b  #0xc,0x106ece.l                 | +01e
        bne.w   SetHandlerRts_08d840            | +026
        cmpi.w  #0x4280,0x106f5c.l              | +02a
        bcs.w   SetHandlerRts_08d840            | +032

| ----------------------------------------------------------------------------
|  MissionEnd_Marker_08d842  @ $08D842  (80 B)
| ----------------------------------------------------------------------------
        .section .text.MissionEnd_Marker_08d842, "ax", @progbits
        .global MissionEnd_Marker_08d842
MissionEnd_Marker_08d842:
        move.w  #0xf0,0x22(a6)                  | +000
        move.w  #0x1a0,0x24(a6)                 | +006
        clr.w   0x26(a6)                        | +00c
        move.w  #0x98,d1                        | +010
        jsr     0x236e.l                        | +014
        lea     0x2f3960.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L08d86e(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L08d86e:
        cmpi.w  #0x4300,0x106f5c.l              | +02c
        bcs.w   .L08d88c                        | +034
        lea     0x2f39dc.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        lea     MissionEnd_Marker2_08d89a(pc),a1 | +044
        move.l  a1,(a6)                         | +048
.L08d88c:
        jsr     0x2783a.l                       | +04a

| ----------------------------------------------------------------------------
|  MissionEnd_Marker2_08d89a  @ $08D89A  (36 B)
| ----------------------------------------------------------------------------
        .section .text.MissionEnd_Marker2_08d89a, "ax", @progbits
        .global MissionEnd_Marker2_08d89a
MissionEnd_Marker2_08d89a:
        cmpi.w  #0x433f,0x106f5c.l              | +000
        bcs.w   .L08d8b8                        | +008
        lea     0x2f39e8.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     MissionEnd_Marker3_08d8c6(pc),a1 | +018
        move.l  a1,(a6)                         | +01c
.L08d8b8:
        jsr     0x2783a.l                       | +01e

| ----------------------------------------------------------------------------
|  MissionEnd_Marker3_08d8c6  @ $08D8C6  (38 B)
| ----------------------------------------------------------------------------
        .section .text.MissionEnd_Marker3_08d8c6, "ax", @progbits
        .global MissionEnd_Marker3_08d8c6
MissionEnd_Marker3_08d8c6:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   SetHandlerRts_08d8f2            | +00c
        move.w  #0x3c,0x70(a6)                  | +010
        move.l  #0xffff8000,0x88(a6)            | +016
        move.l  #0x0,0x8c(a6)                   | +01e

| ----------------------------------------------------------------------------
|  MissionEnd_ScrollUp_08d8f4  @ $08D8F4  (60 B)
| ----------------------------------------------------------------------------
        .section .text.MissionEnd_ScrollUp_08d8f4, "ax", @progbits
        .global MissionEnd_ScrollUp_08d8f4
MissionEnd_ScrollUp_08d8f4:
        cmpi.w  #0xd8,0x106f54.l                | +000
        bhi.w   .L08d926                        | +008
        move.w  #0xb4,0x70(a6)                  | +00c
        move.l  #0x0,0x106f64.l                 | +012
        move.l  #0x0,0x88(a6)                   | +01c
        move.l  #0x0,0x8c(a6)                   | +024
        lea     MissionEnd_Wait_08d938(pc),a1   | +02c
        move.l  a1,(a6)                         | +030
.L08d926:
        jsr     Scroll_StepVelY_08ccd8(pc)      | +032
        jsr     0x2783a.l                       | +036

| ----------------------------------------------------------------------------
|  MissionEnd_Wait_08d938  @ $08D938  (70 B)
| ----------------------------------------------------------------------------
        .section .text.MissionEnd_Wait_08d938, "ax", @progbits
        .global MissionEnd_Wait_08d938
MissionEnd_Wait_08d938:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L08d978                        | +00a
        lea     Cut_Dropper_08ccea(pc),a1       | +00e
        jsr     0x4ae.l                         | +012
        move.b  #0xa,0x98(a0)                   | +018
        move.b  #0x8,0x99(a0)                   | +01e
        move.w  #0xa0,0x22(a0)                  | +024
        move.w  #0x100,0x24(a0)                 | +02a
        move.l  #0x0,0x106f64.l                 | +030
        lea     MissionEnd_Idle_08d986(pc),a1   | +03a
        move.l  a1,(a6)                         | +03e
.L08d978:
        jsr     0x2783a.l                       | +040

| ----------------------------------------------------------------------------
|  MissionEnd_Idle_08d986  @ $08D986  (6 B)
| ----------------------------------------------------------------------------
        .section .text.MissionEnd_Idle_08d986, "ax", @progbits
        .global MissionEnd_Idle_08d986
MissionEnd_Idle_08d986:
        jsr     0x2783a.l                       | +000

| ----------------------------------------------------------------------------
|  Grunt_Tmpl157_08d994  @ $08D994  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Tmpl157_08d994, "ax", @progbits
        .global Grunt_Tmpl157_08d994
Grunt_Tmpl157_08d994:
        jsr     Sub_0008F108(pc)                | +000  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +004  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F02C(pc)                | +008  -> $08F02C (hueco futuro, defsym forward)
        jsr     Sub_0008F084(pc)                | +00c  -> $08F084 (hueco futuro, defsym forward)
        lea     0x2f3ab4.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L08d9b6(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L08d9b6:
        jsr     Sub_0008F040(pc)                | +022  -> $08F040 (hueco futuro, defsym forward)
        bcc.w   .L08d9d0                        | +026
        lea     0x2f3ac0.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L08d9d0(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L08d9d0:
        jsr     0x2783a.l                       | +03c
        jsr     PcThunkTarget_08efb0(pc)        | +042

| ----------------------------------------------------------------------------
|  Grunt_Tmpl158_08d9e2  @ $08D9E2  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Tmpl158_08d9e2, "ax", @progbits
        .global Grunt_Tmpl158_08d9e2
Grunt_Tmpl158_08d9e2:
        jsr     Sub_0008F108(pc)                | +000  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +004  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F02C(pc)                | +008  -> $08F02C (hueco futuro, defsym forward)
        jsr     Sub_0008F084(pc)                | +00c  -> $08F084 (hueco futuro, defsym forward)
        lea     0x2f3af0.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L08da04(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L08da04:
        jsr     Sub_0008F040(pc)                | +022  -> $08F040 (hueco futuro, defsym forward)
        bcc.w   .L08da1e                        | +026
        lea     0x2f3afc.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L08da1e(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L08da1e:
        jsr     0x2783a.l                       | +03c
        jsr     PcThunkTarget_08efb0(pc)        | +042

| ----------------------------------------------------------------------------
|  Grunt_Carrier_08da30  @ $08DA30  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Carrier_08da30, "ax", @progbits
        .global Grunt_Carrier_08da30
Grunt_Carrier_08da30:
        move.b  #0x0,0x98(a6)                   | +000
        move.b  #0x0,0x99(a6)                   | +006
        move.w  #0x40,0x22(a6)                  | +00c
        move.w  #0x150,0x24(a6)                 | +012
        clr.w   0x26(a6)                        | +018
        jsr     Sub_0008F108(pc)                | +01c  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +020  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F02C(pc)                | +024  -> $08F02C (hueco futuro, defsym forward)
        jsr     Sub_0008F0D0(pc)                | +028  -> $08F0D0 (hueco futuro, defsym forward)
        clr.w   0x5c(a6)                        | +02c
        lea     0x2f3b2c.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     .L08da72(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L08da72:
        jsr     Sub_0008F040(pc)                | +042  -> $08F040 (hueco futuro, defsym forward)
        bcc.w   .L08da8c                        | +046
        lea     0x2f3b70.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     Grunt_Carrier_Drop_08da9e(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
.L08da8c:
        jsr     0x2783a.l                       | +05c
        jsr     PcThunkTarget_08efb0(pc)        | +062

| ----------------------------------------------------------------------------
|  Grunt_Carrier_Drop_08da9e  @ $08DA9E  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Carrier_Drop_08da9e, "ax", @progbits
        .global Grunt_Carrier_Drop_08da9e
Grunt_Carrier_Drop_08da9e:
        cmpi.w  #0x0,0x5c(a6)                   | +000
        beq.w   .L08dad0                        | +006
        lea     Grunt_Dropped_08dae2(pc),a1     | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        move.b  0x3a(a6),0x98(a0)               | +01a
        move.b  #0x30,0x99(a0)                  | +020
        move.b  0x9a(a6),0x9a(a0)               | +026
        lea     .L08dad0(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L08dad0:
        jsr     0x2783a.l                       | +032
        jsr     PcThunkTarget_08efb0(pc)        | +038

| ----------------------------------------------------------------------------
|  Grunt_Dropped_08dae2  @ $08DAE2  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Dropped_08dae2, "ax", @progbits
        .global Grunt_Dropped_08dae2
Grunt_Dropped_08dae2:
        move.w  #0xc000,0x38(a6)                | +000
        andi.w  #0xffe3,0x38(a6)                | +006
        ori.w   #0x1c,0x38(a6)                  | +00c
        jsr     Sub_0008F002(pc)                | +012  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +016  -> $08F010 (hueco futuro, defsym forward)
        neg.w   0x28(a6)                        | +01a
        move.w  #0x10,0x2e(a6)                  | +01e
        addi.w  #0x20,0x24(a6)                  | +024
        jsr     Sub_0008F0D0(pc)                | +02a  -> $08F0D0 (hueco futuro, defsym forward)
        lea     0x2f3c04.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     .L08db22(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L08db22:
        jsr     0x2783a.l                       | +040
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +046
        movea.l #0xffffffff,a0                  | +04a
        jsr     0x5dd56.l                       | +050
        bcc.w   JsrAbsThunk_08db48              | +056
        jsr     0x5b6.l                         | +05a
        jmp     0x518.l                         | +060

| ----------------------------------------------------------------------------
|  Grunt_Tmpl162_08db50  @ $08DB50  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Tmpl162_08db50, "ax", @progbits
        .global Grunt_Tmpl162_08db50
Grunt_Tmpl162_08db50:
        move.w  #0xd000,0x38(a6)                | +000
        jsr     Sub_0008F002(pc)                | +006  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +00a  -> $08F010 (hueco futuro, defsym forward)
        move.w  #0xf8,d1                        | +00e
        jsr     0x236e.l                        | +012
        lea     0x2f3c84.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     Grunt_HopDown_08db7a(pc),a1     | +024
        move.l  a1,(a6)                         | +028

| ----------------------------------------------------------------------------
|  Grunt_HopDown_08db7a  @ $08DB7A  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_HopDown_08db7a, "ax", @progbits
        .global Grunt_HopDown_08db7a
Grunt_HopDown_08db7a:
        move.w  #0xe,0x2e(a6)                   | +000
        move.w  #0x8,0x70(a6)                   | +006
        move.w  #0x7,d0                         | +00c
        jsr     0x5ea1c.l                       | +010
        add.w   d0,0x70(a6)                     | +016
        lea     0x2f3c84.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L08dba6(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L08dba6:
        jsr     0x2783a.l                       | +02c
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +032
        jsr     0x28d70.l                       | +036
        cmpi.w  #0x1d0,0x24(a6)                 | +03c
        blt.w   Grunt_HopDown_Wait_08dbc8       | +042

| ----------------------------------------------------------------------------
|  Grunt_HopDown_Wait_08dbc8  @ $08DBC8  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_HopDown_Wait_08dbc8, "ax", @progbits
        .global Grunt_HopDown_Wait_08dbc8
Grunt_HopDown_Wait_08dbc8:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bhi.w   JsrPcThunk_08dbdc               | +00a
        lea     Grunt_HopUp_08dbe2(pc),a1       | +00e
        move.l  a1,(a6)                         | +012

| ----------------------------------------------------------------------------
|  Grunt_HopUp_08dbe2  @ $08DBE2  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_HopUp_08dbe2, "ax", @progbits
        .global Grunt_HopUp_08dbe2
Grunt_HopUp_08dbe2:
        move.w  #0xfff2,0x2e(a6)                | +000
        move.w  #0x8,0x70(a6)                   | +006
        move.w  #0x7,d0                         | +00c
        jsr     0x5ea1c.l                       | +010
        add.w   d0,0x70(a6)                     | +016
        lea     0x2f3cb4.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L08dc0e(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L08dc0e:
        jsr     0x2783a.l                       | +02c
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +032
        jsr     0x28d70.l                       | +036
        cmpi.w  #0x160,0x24(a6)                 | +03c
        bge.w   Grunt_HopUp_Wait_08dc30         | +042

| ----------------------------------------------------------------------------
|  Grunt_HopUp_Wait_08dc30  @ $08DC30  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_HopUp_Wait_08dc30, "ax", @progbits
        .global Grunt_HopUp_Wait_08dc30
Grunt_HopUp_Wait_08dc30:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bhi.w   JsrPcThunk_08dc44               | +00a
        lea     Grunt_HopDown_08db7a(pc),a1     | +00e
        move.l  a1,(a6)                         | +012

| ----------------------------------------------------------------------------
|  Grunt_Rand_Idle_08dc4a  @ $08DC4A  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Rand_Idle_08dc4a, "ax", @progbits
        .global Grunt_Rand_Idle_08dc4a
Grunt_Rand_Idle_08dc4a:
        move.w  #0x40,0x22(a6)                  | +000
        move.w  #0x150,0x24(a6)                 | +006
        clr.w   0x26(a6)                        | +00c
        jsr     Sub_0008F108(pc)                | +010  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +014  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F02C(pc)                | +018  -> $08F02C (hueco futuro, defsym forward)
        move.w  #0xf8,d1                        | +01c
        jsr     0x236e.l                        | +020
        lea     0x2f3c58.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        jsr     0x27cee.l                       | +032
        jsr     0x28d70.l                       | +038
        lea     Grunt_Rand_Stand_08dc94(pc),a1  | +03e
        move.l  a1,(a6)                         | +042

| ----------------------------------------------------------------------------
|  Grunt_Rand_Stand_08dc94  @ $08DC94  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Rand_Stand_08dc94, "ax", @progbits
        .global Grunt_Rand_Stand_08dc94
Grunt_Rand_Stand_08dc94:
        lea     0x2f3c58.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08dca6(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08dca6:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   JsrPcThunk_08dcf4               | +01e
        cmpi.b  #0x0,0x99(a6)                   | +022
        bne.w   .L08dcde                        | +028
        move.w  #0x60,d0                        | +02c
        jsr     Sub_0008EFCE(pc)                | +030  -> $08EFCE (hueco futuro, defsym forward)
        bcc.w   .L08dcd6                        | +034
        lea     Grunt_Hit_08dd7e(pc),a1         | +038
        move.l  a1,(a6)                         | +03c
        bra.w   .L08dcda                        | +03e
.L08dcd6:
        jsr     Grunt_PickRand_2f3712_08dec2(pc) | +042
.L08dcda:
        bra.w   JsrPcThunk_08dcf4               | +046
.L08dcde:
        jsr     Sub_0008F040(pc)                | +04a  -> $08F040 (hueco futuro, defsym forward)
        bcc.w   .L08dcf0                        | +04e
        lea     Grunt_Hit_08dd7e(pc),a1         | +052
        move.l  a1,(a6)                         | +056
        bra.w   JsrPcThunk_08dcf4               | +058
.L08dcf0:
        jsr     Grunt_PickRand_2f3712_08dec2(pc) | +05c

| ----------------------------------------------------------------------------
|  Grunt_Rand_Stand2_08dcfa  @ $08DCFA  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Rand_Stand2_08dcfa, "ax", @progbits
        .global Grunt_Rand_Stand2_08dcfa
Grunt_Rand_Stand2_08dcfa:
        lea     0x2f3c64.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08dd0c(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08dd0c:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   JsrPcThunk_08dd20               | +01e
        jsr     Grunt_PickRand_2f3712_08dec2(pc) | +022

| ----------------------------------------------------------------------------
|  Grunt_Rand_Walk_08dd26  @ $08DD26  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Rand_Walk_08dd26, "ax", @progbits
        .global Grunt_Rand_Walk_08dd26
Grunt_Rand_Walk_08dd26:
        move.w  #0x1,d0                         | +000
        jsr     0x5ea1c.l                       | +004
        move.b  d0,0x98(a6)                     | +00a
        bne.w   .L08dd42                        | +00e
        move.w  #0xff80,0x28(a6)                | +012
        bra.w   .L08dd48                        | +018
.L08dd42:
        move.w  #0x80,0x28(a6)                  | +01c
.L08dd48:
        jsr     Sub_0008F002(pc)                | +022  -> $08F002 (hueco futuro, defsym forward)
        lea     0x2f3c10.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L08dd5e(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L08dd5e:
        jsr     0x2783a.l                       | +038
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +03e
        jsr     0x28d70.l                       | +042
        bcc.w   JsrPcThunk_08dd78               | +048
        lea     Grunt_Rand_Stand_08dc94(pc),a1  | +04c
        move.l  a1,(a6)                         | +050

| ----------------------------------------------------------------------------
|  Grunt_Hit_08dd7e  @ $08DD7E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Hit_08dd7e, "ax", @progbits
        .global Grunt_Hit_08dd7e
Grunt_Hit_08dd7e:
        addi.w  #0x4,0x24(a6)                   | +000
        move.w  #0x1e,0x70(a6)                  | +006
        move.w  #0xf,d0                         | +00c
        jsr     0x5ea1c.l                       | +010
        add.w   d0,0x70(a6)                     | +016
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        lea     Grunt_Hit_Angle_08ddb0(pc),a1   | +026
        move.l  a1,(a6)                         | +02a

| ----------------------------------------------------------------------------
|  Grunt_Hit_Angle_08ddb0  @ $08DDB0  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Hit_Angle_08ddb0, "ax", @progbits
        .global Grunt_Hit_Angle_08ddb0
Grunt_Hit_Angle_08ddb0:
        move.w  #0x1f,d0                        | +000
        jsr     0x5ea1c.l                       | +004
        addi.w  #0x10,d0                        | +00a
        move.w  d0,0x34(a6)                     | +00e
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        lea     Grunt_Hit_Launch_08ddda(pc),a1  | +01e
        move.l  a1,(a6)                         | +022

| ----------------------------------------------------------------------------
|  Grunt_Hit_Launch_08ddda  @ $08DDDA  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Hit_Launch_08ddda, "ax", @progbits
        .global Grunt_Hit_Launch_08ddda
Grunt_Hit_Launch_08ddda:
        move.w  #0xff,d0                        | +000
        jsr     0x5ea1c.l                       | +004
        move.w  #0x200,d1                       | +00a
        add.w   d0,d1                           | +00e
        move.w  d1,0x36(a6)                     | +010
        move.w  0x34(a6),d0                     | +014
        jsr     0x13c0e.l                       | +018
        move.w  d1,0x28(a6)                     | +01e
        move.w  d2,0x2a(a6)                     | +022
        move.b  #0x1,0x3a(a6)                   | +026
        lea     0x2f3c84.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        lea     Grunt_Hit_Fly_08de2a(pc),a1     | +044
        move.l  a1,(a6)                         | +048

| ----------------------------------------------------------------------------
|  Grunt_Hit_Fly_08de2a  @ $08DE2A  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Hit_Fly_08de2a, "ax", @progbits
        .global Grunt_Hit_Fly_08de2a
Grunt_Hit_Fly_08de2a:
        jsr     0x2783a.l                       | +000
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +006
        jsr     0x28d70.l                       | +00a
        subq.w  #0x1,0x70(a6)                   | +010
        cmpi.w  #0x0,0x70(a6)                   | +014
        bhi.w   JsrPcThunk_08de5e               | +01a
        move.w  #0xd000,0x38(a6)                | +01e
        move.w  0x36(a6),0x28(a6)               | +024
        clr.w   0x2a(a6)                        | +02a
        lea     Grunt_HopDown_08db7a(pc),a1     | +02e
        move.l  a1,(a6)                         | +032

| ----------------------------------------------------------------------------
|  Grunt_Tmpl_Standing_08de64  @ $08DE64  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Tmpl_Standing_08de64, "ax", @progbits
        .global Grunt_Tmpl_Standing_08de64
Grunt_Tmpl_Standing_08de64:
        move.b  #0x8,0x99(a6)                   | +000
        move.b  #0x1,0x98(a6)                   | +006
        jsr     Sub_0008F108(pc)                | +00c  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +010  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +014  -> $08F010 (hueco futuro, defsym forward)
        move.w  #0xf8,d1                        | +018
        jsr     0x236e.l                        | +01c
        lea     .L08de8c(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L08de8c:
        lea     0x2f3c58.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L08de9e(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L08de9e:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        move.w  #0x60,d0                        | +046
        jsr     Sub_0008EFCE(pc)                | +04a  -> $08EFCE (hueco futuro, defsym forward)
        bcc.w   JsrPcThunk_08debc               | +04e
        lea     Grunt_Hit_08dd7e(pc),a1         | +052
        move.l  a1,(a6)                         | +056

| ----------------------------------------------------------------------------
|  Grunt_PickRand_2f3712_08dec2  @ $08DEC2  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_PickRand_2f3712_08dec2, "ax", @progbits
        .global Grunt_PickRand_2f3712_08dec2
Grunt_PickRand_2f3712_08dec2:
        move.w  #0x3,d0                         | +000
        jsr     0x5ea1c.l                       | +004
        lea     0x2f3712.l,a0                   | +00a
        movea.l #0xffffffff,a1                  | +010

| ----------------------------------------------------------------------------
|  Grunt_Runner_08dee0  @ $08DEE0  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Runner_08dee0, "ax", @progbits
        .global Grunt_Runner_08dee0
Grunt_Runner_08dee0:
        move.w  #0x60,0x22(a6)                  | +000
        move.w  #0x150,0x24(a6)                 | +006
        clr.w   0x26(a6)                        | +00c
        move.b  #0x10,0x99(a6)                  | +010
        move.b  #0x1,0x98(a6)                   | +016
        cmpi.b  #0x0,0x99(a6)                   | +01c
        bne.w   .L08df0c                        | +022
        move.b  #0x10,0x99(a6)                  | +026
.L08df0c:
        jsr     Sub_0008F108(pc)                | +02c  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +030  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +034  -> $08F010 (hueco futuro, defsym forward)
        move.w  #0xf9,d1                        | +038
        jsr     0x236e.l                        | +03c
        lea     0x2f3cc0.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        lea     .L08df34(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L08df34:
        jsr     0x2783a.l                       | +054
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +05a
        jsr     PcThunkTarget_08efb0(pc)        | +05e

| ----------------------------------------------------------------------------
|  Grunt_Tmpl153_08df4a  @ $08DF4A  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Tmpl153_08df4a, "ax", @progbits
        .global Grunt_Tmpl153_08df4a
Grunt_Tmpl153_08df4a:
        jsr     Sub_0008F108(pc)                | +000  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +004  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +008  -> $08F010 (hueco futuro, defsym forward)
        jsr     Sub_0008F070(pc)                | +00c  -> $08F070 (hueco futuro, defsym forward)
        jsr     Grunt_LoadMapByDir_08e172(pc)   | +010
        jsr     0x27cee.l                       | +014
        jsr     0x28d70.l                       | +01a

| ----------------------------------------------------------------------------
|  Grunt_Run_08df72  @ $08DF72  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Run_08df72, "ax", @progbits
        .global Grunt_Run_08df72
Grunt_Run_08df72:
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +000
        jsr     0x2783a.l                       | +004
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +00a
        bcc.w   JsrAbsThunk_08df8c              | +00e
        jmp     0x518.l                         | +012

| ----------------------------------------------------------------------------
|  Rts_08df8a  @ $08DF8A  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08df8a, "ax", @progbits
        .global Rts_08df8a
Rts_08df8a:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Grunt_Tmpl154_08df94  @ $08DF94  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Tmpl154_08df94, "ax", @progbits
        .global Grunt_Tmpl154_08df94
Grunt_Tmpl154_08df94:
        jsr     Sub_0008F108(pc)                | +000  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +004  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +008  -> $08F010 (hueco futuro, defsym forward)
        jsr     Sub_0008F070(pc)                | +00c  -> $08F070 (hueco futuro, defsym forward)
        jsr     0x27cee.l                       | +010
        jsr     0x28d70.l                       | +016

| ----------------------------------------------------------------------------
|  Grunt2_Rand_Stand_08dfb8  @ $08DFB8  (114 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt2_Rand_Stand_08dfb8, "ax", @progbits
        .global Grunt2_Rand_Stand_08dfb8
Grunt2_Rand_Stand_08dfb8:
        lea     0x2f3e4a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08dfca(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08dfca:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L08e01a                        | +01e
        move.w  #0x60,d0                        | +022
        jsr     Sub_0008EFCE(pc)                | +026  -> $08EFCE (hueco futuro, defsym forward)
        bcc.w   .L08e016                        | +02a
        move.w  #0xf,d0                         | +02e
        jsr     0x5ea1c.l                       | +032
        move.b  #0x20,0x99(a6)                  | +038
        add.b   d0,0x99(a6)                     | +03e
        move.b  #0x1,0x98(a6)                   | +042
        jsr     Sub_0008F002(pc)                | +048  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +04c  -> $08F010 (hueco futuro, defsym forward)
        jsr     Grunt_LoadMapByDir_08e172(pc)   | +050
        lea     Grunt_Run_08df72(pc),a1         | +054
        move.l  a1,(a6)                         | +058
        bra.w   .L08e01a                        | +05a
.L08e016:
        jsr     Grunt_PickRand_2f3722_08e09c(pc) | +05e
.L08e01a:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +062
        bcc.w   .L08e028                        | +066
        jmp     0x518.l                         | +06a
.L08e028:
        rts                                     | +070

| ----------------------------------------------------------------------------
|  Grunt2_Rand_Stand2_08e02a  @ $08E02A  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt2_Rand_Stand2_08e02a, "ax", @progbits
        .global Grunt2_Rand_Stand2_08e02a
Grunt2_Rand_Stand2_08e02a:
        lea     0x2f3e56.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e03c(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e03c:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L08e050                        | +01e
        jsr     Grunt_PickRand_2f3722_08e09c(pc) | +022
.L08e050:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +026
        bcc.w   .L08e05e                        | +02a
        jmp     0x518.l                         | +02e
.L08e05e:
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Grunt2_Rand_Walk_08e060  @ $08E060  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt2_Rand_Walk_08e060, "ax", @progbits
        .global Grunt2_Rand_Walk_08e060
Grunt2_Rand_Walk_08e060:
        lea     0x2f3e0c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e072(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e072:
        jsr     0x2783a.l                       | +012
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .L08e08c                        | +022
        lea     Grunt2_Rand_Stand_08dfb8(pc),a1 | +026
        move.l  a1,(a6)                         | +02a
.L08e08c:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +02c
        bcc.w   .L08e09a                        | +030
        jmp     0x518.l                         | +034
.L08e09a:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Grunt_PickRand_2f3722_08e09c  @ $08E09C  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_PickRand_2f3722_08e09c, "ax", @progbits
        .global Grunt_PickRand_2f3722_08e09c
Grunt_PickRand_2f3722_08e09c:
        move.w  #0x3,d0                         | +000
        jsr     0x5ea1c.l                       | +004
        lea     0x2f3722.l,a0                   | +00a
        movea.l #0xffffffff,a1                  | +010

| ----------------------------------------------------------------------------
|  Grunt2_Spawn_08e0ba  @ $08E0BA  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt2_Spawn_08e0ba, "ax", @progbits
        .global Grunt2_Spawn_08e0ba
Grunt2_Spawn_08e0ba:
        move.w  #0x80,0x22(a6)                  | +000
        move.w  #0x150,0x24(a6)                 | +006
        clr.w   0x26(a6)                        | +00c
        move.b  #0x1,0x98(a6)                   | +010
        move.b  #0x2,0x9a(a6)                   | +016
        move.b  #0x8,0x99(a6)                   | +01c
        jsr     Sub_0008F108(pc)                | +022  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +026  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +02a  -> $08F010 (hueco futuro, defsym forward)
        jsr     Sub_0008F070(pc)                | +02e  -> $08F070 (hueco futuro, defsym forward)
        jsr     0x27cee.l                       | +032
        jsr     0x28d70.l                       | +038
        lea     Grunt2_Run_08e10c(pc),a1        | +03e
        move.l  a1,(a6)                         | +042
        lea     0x2f3e0c.l,a0                   | +044

| ----------------------------------------------------------------------------
|  Grunt2_Run_08e10c  @ $08E10C  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt2_Run_08e10c, "ax", @progbits
        .global Grunt2_Run_08e10c
Grunt2_Run_08e10c:
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +000
        jsr     0x28d70.l                       | +004
        bcc.w   .L08e162                        | +00a
        move.w  #0x60,d0                        | +00e
        jsr     Sub_0008EFCE(pc)                | +012  -> $08EFCE (hueco futuro, defsym forward)
        bcc.w   .L08e156                        | +016
        move.w  #0xf,d0                         | +01a
        jsr     0x5ea1c.l                       | +01e
        move.b  #0x20,0x99(a6)                  | +024
        add.b   d0,0x99(a6)                     | +02a
        move.b  #0x1,0x98(a6)                   | +02e
        jsr     Sub_0008F002(pc)                | +034  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +038  -> $08F010 (hueco futuro, defsym forward)
        jsr     Grunt_LoadMapByDir_08e172(pc)   | +03c
        lea     Grunt_Run_08df72(pc),a1         | +040
        move.l  a1,(a6)                         | +044
        bra.w   .L08e162                        | +046
.L08e156:
        lea     0x2f3e0c.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
.L08e162:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +056
        bcc.w   .L08e170                        | +05a
        jmp     0x518.l                         | +05e
.L08e170:
        rts                                     | +064

| ----------------------------------------------------------------------------
|  Grunt_LoadMapByDir_08e172  @ $08E172  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_LoadMapByDir_08e172, "ax", @progbits
        .global Grunt_LoadMapByDir_08e172
Grunt_LoadMapByDir_08e172:
        clr.w   d0                              | +000
        move.b  0x99(a6),d0                     | +002
        lsr.b   #0x4,d0                         | +006
        andi.b  #0x7,d0                         | +008
        movea.l #0x2f3cdc,a0                    | +00c
        lsl.w   #0x2,d0                         | +012
        movea.l (a0,d0.w),a0                    | +014
        cmpa.l  #0xffffffff,a0                  | +018
        beq.w   JsrAbsRts_08e19a                | +01e

| ----------------------------------------------------------------------------
|  Grunt_Tmpl160_08e19c  @ $08E19C  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Tmpl160_08e19c, "ax", @progbits
        .global Grunt_Tmpl160_08e19c
Grunt_Tmpl160_08e19c:
        jsr     Sub_0008F108(pc)                | +000  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +004  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F02C(pc)                | +008  -> $08F02C (hueco futuro, defsym forward)
        move.w  #0xfd,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f3e86.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L08e1c4(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L08e1c4:
        jsr     Sub_0008F040(pc)                | +028  -> $08F040 (hueco futuro, defsym forward)
        bcc.w   .L08e1de                        | +02c
        lea     0x2f3e92.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     .L08e1de(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L08e1de:
        jsr     0x2783a.l                       | +042
        jsr     Screen_InBoundsXWide_Latched_08d1e8(pc) | +048
        bcc.w   JsrAbsThunk_08e1f2              | +04c
        jmp     0x518.l                         | +050

| ----------------------------------------------------------------------------
|  Sentry_Init_08e1fa  @ $08E1FA  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Sentry_Init_08e1fa, "ax", @progbits
        .global Sentry_Init_08e1fa
Sentry_Init_08e1fa:
        move.b  #0x1,0x98(a6)                   | +000
        move.b  #0x20,0x99(a6)                  | +006
        move.b  #0x14,0x9a(a6)                  | +00c
        move.w  #0x40,0x22(a6)                  | +012
        move.w  #0x180,0x24(a6)                 | +018
        clr.w   0x26(a6)                        | +01e
        jsr     Sub_0008F002(pc)                | +022  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +026  -> $08F010 (hueco futuro, defsym forward)
        eori.b  #0x1,0x3a(a6)                   | +02a
        move.w  #0xfe,d1                        | +030
        jsr     0x236e.l                        | +034
        lea     0x2f3e9e.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        move.w  #0xffff,0x38(a6)                | +046
        bset    #0x6,0x12(a6)                   | +04c
        lea     Sentry_Run_08e252(pc),a1 | +052
        move.l  a1,(a6)                         | +056
        .global Sentry_Run_08e252
Sentry_Run_08e252:
        jsr     PcThunkTarget_08efb0(pc)        | +058
        jsr     0x28d70.l                       | +05c
        bcc.w   SetHandlerRts_08e286            | +062
        cmpi.b  #0x0,0x98(a6)                   | +066
        bne.w   .L08e274                        | +06c
        subi.w  #0x48,0x22(a6)                  | +070
        bra.w   .L08e27a                        | +076
.L08e274:
        addi.w  #0x48,0x22(a6)                  | +07a
.L08e27a:
        move.b  0x9a(a6),0x70(a6)               | +080

| ----------------------------------------------------------------------------
|  Sentry_Wait_08e288  @ $08E288  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Sentry_Wait_08e288, "ax", @progbits
        .global Sentry_Wait_08e288
Sentry_Wait_08e288:
        subq.b  #0x1,0x70(a6)                   | +000
        bne.w   .L08e2a2                        | +004
        lea     0x2f3e9e.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        lea     Sentry_Run_08e252(pc),a1 | +014
        move.l  a1,(a6)                         | +018
.L08e2a2:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +01a

| ----------------------------------------------------------------------------
|  Grunt_Tmpl180_08e2ac  @ $08E2AC  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Tmpl180_08e2ac, "ax", @progbits
        .global Grunt_Tmpl180_08e2ac
Grunt_Tmpl180_08e2ac:
        jsr     Sub_0008F108(pc)                | +000  -> $08F108 (hueco futuro, defsym forward)
        jsr     Sub_0008F002(pc)                | +004  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F02C(pc)                | +008  -> $08F02C (hueco futuro, defsym forward)
        move.w  #0x124,d1                       | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f3f4c.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        move.w  #0xffff,0x38(a6)                | +022
        bset    #0x6,0x12(a6)                   | +028
        lea     Grunt_Tmpl180_Run_08e2f0(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
        jsr     Sub_0008F040(pc)                | +034  -> $08F040 (hueco futuro, defsym forward)
        bcc.w   SetHandlerRts_08e2ee            | +038

| ----------------------------------------------------------------------------
|  Grunt_Tmpl180_Run_08e2f0  @ $08E2F0  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Grunt_Tmpl180_Run_08e2f0, "ax", @progbits
        .global Grunt_Tmpl180_Run_08e2f0
Grunt_Tmpl180_Run_08e2f0:
        jsr     0x2783a.l                       | +000
        jsr     PcThunkTarget_08efb0(pc)        | +006
        jsr     0x28d70.l                       | +00a
        bcc.w   .L08e30a                        | +010
        jmp     0x518.l                         | +014
.L08e30a:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Swinger_Tmpl181_08e30c  @ $08E30C  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Swinger_Tmpl181_08e30c, "ax", @progbits
        .global Swinger_Tmpl181_08e30c
Swinger_Tmpl181_08e30c:
        jsr     Sub_0008F002(pc)                | +000  -> $08F002 (hueco futuro, defsym forward)
        jsr     Sub_0008F010(pc)                | +004  -> $08F010 (hueco futuro, defsym forward)
        asr.w   0x28(a6)                        | +008
        move.w  #0x125,d1                       | +00c
        jsr     0x236e.l                        | +010
        move.w  #0xffff,0x38(a6)                | +016
        bset    #0x6,0x12(a6)                   | +01c
        move.w  0x28(a6),0x74(a6)               | +022
        .global Swinger_Swing_08e334
Swinger_Swing_08e334:
        lea     0x2f3fc6.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        clr.w   0x28(a6)                        | +034
        cmpi.b  #0x0,0x98(a6)                   | +038
        bne.w   .L08e358                        | +03e
        move.w  #0xfff8,0x2c(a6)                | +042
        bra.w   .L08e35e                        | +048
.L08e358:
        move.w  #0x8,0x2c(a6)                   | +04c
.L08e35e:
        jsr     Sub_0008F002(pc)                | +052  -> $08F002 (hueco futuro, defsym forward)
        lea     .L08e368(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L08e368:
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +05c
        jsr     0x2783a.l                       | +060
        move.w  0x2c(a6),d0                     | +066
        add.w   d0,0x28(a6)                     | +06a
        cmpi.b  #0x0,0x98(a6)                   | +06e
        bne.w   .L08e39a                        | +074
        move.w  0x28(a6),d0                     | +078
        cmp.w   0x74(a6),d0                     | +07c
        bgt.w   .L08e396                        | +080
        lea     Swinger_Pause_08e3c4(pc),a1     | +084
        move.l  a1,(a6)                         | +088
.L08e396:
        bra.w   .L08e3ac                        | +08a
.L08e39a:
        move.w  0x28(a6),d0                     | +08e
        cmp.w   0x74(a6),d0                     | +092
        blt.w   .L08e3ac                        | +096
        lea     Swinger_Pause_08e3c4(pc),a1     | +09a
        move.l  a1,(a6)                         | +09e
.L08e3ac:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +0a0
        bcc.w   JsrAbsThunk_08e3bc              | +0a4
        jmp     0x518.l                         | +0a8

| ----------------------------------------------------------------------------
|  Rts_08e3ba  @ $08E3BA  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08e3ba, "ax", @progbits
        .global Rts_08e3ba
Rts_08e3ba:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Swinger_Pause_08e3c4  @ $08E3C4  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Swinger_Pause_08e3c4, "ax", @progbits
        .global Swinger_Pause_08e3c4
Swinger_Pause_08e3c4:
        move.w  #0x7,d0                         | +000
        jsr     0x5ea1c.l                       | +004
        add.w   d0,0x70(a6)                     | +00a
        lea     .L08e3d8(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L08e3d8:
        subq.w  #0x1,0x70(a6)                   | +014
        cmpi.w  #0x0,0x70(a6)                   | +018
        bgt.w   .L08e3ec                        | +01e
        lea     Swinger_Back_08e40e(pc),a1      | +022
        move.l  a1,(a6)                         | +026
.L08e3ec:
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +028
        jsr     0x2783a.l                       | +02c
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +032
        bcc.w   JsrAbsThunk_08e406              | +036
        jmp     0x518.l                         | +03a

| ----------------------------------------------------------------------------
|  Rts_08e404  @ $08E404  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08e404, "ax", @progbits
        .global Rts_08e404
Rts_08e404:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Swinger_Back_08e40e  @ $08E40E  (138 B)
| ----------------------------------------------------------------------------
        .section .text.Swinger_Back_08e40e, "ax", @progbits
        .global Swinger_Back_08e40e
Swinger_Back_08e40e:
        move.w  #0x7,d0                         | +000
        jsr     0x5ea1c.l                       | +004
        addi.w  #0xc,d0                         | +00a
        move.w  d0,0x70(a6)                     | +00e
        neg.w   0x2c(a6)                        | +012
        lea     0x2f3fe2.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L08e436(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L08e436:
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +028
        jsr     0x2783a.l                       | +02c
        move.w  0x2c(a6),d0                     | +032
        add.w   d0,0x28(a6)                     | +036
        subq.w  #0x1,0x70(a6)                   | +03a
        cmpi.w  #0x0,0x70(a6)                   | +03e
        bgt.w   .L08e45c                        | +044
        lea     Swinger_Turn_08e4a2(pc),a1      | +048
        move.l  a1,(a6)                         | +04c
.L08e45c:
        cmpi.b  #0x0,0x98(a6)                   | +04e
        bne.w   .L08e47a                        | +054
        cmpi.w  #0x0,0x28(a6)                   | +058
        blt.w   .L08e476                        | +05e
        lea     Swinger_Turn_08e4a2(pc),a1      | +062
        move.l  a1,(a6)                         | +066
.L08e476:
        bra.w   .L08e48a                        | +068
.L08e47a:
        cmpi.w  #0x0,0x28(a6)                   | +06c
        bgt.w   .L08e48a                        | +072
        lea     Swinger_Turn_08e4a2(pc),a1      | +076
        move.l  a1,(a6)                         | +07a
.L08e48a:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +07c
        bcc.w   JsrAbsThunk_08e49a              | +080
        jmp     0x518.l                         | +084

| ----------------------------------------------------------------------------
|  Rts_08e498  @ $08E498  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08e498, "ax", @progbits
        .global Rts_08e498
Rts_08e498:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Swinger_Turn_08e4a2  @ $08E4A2  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Swinger_Turn_08e4a2, "ax", @progbits
        .global Swinger_Turn_08e4a2
Swinger_Turn_08e4a2:
        lea     0x2f3fee.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e4b4(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e4b4:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L08e4d4                        | +01e
        neg.w   0x74(a6)                        | +022
        eori.b  #0x1,0x98(a6)                   | +026
        lea     Swinger_Swing_08e334(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L08e4d4:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +032
        bcc.w   Stub_0008E4E4                   | +036
        jmp     0x518.l                         | +03a

| ----------------------------------------------------------------------------
|  Rts_08e4e2  @ $08E4E2  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08e4e2, "ax", @progbits
        .global Rts_08e4e2
Rts_08e4e2:
        rts                                     | +000
