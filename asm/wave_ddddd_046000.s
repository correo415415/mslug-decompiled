| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $046000..$048000  (4,386 B, 79 entradas, 45 huecos)
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
|  Enemy46_PhaseC_046260  @ $046260  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Enemy46_PhaseC_046260, "ax", @progbits
        .global Enemy46_PhaseC_046260
Enemy46_PhaseC_046260:
        lea     0x29b7c8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L046272(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L046272:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L046288                        | +01e
        lea     Enemy46_PhaseD_Flip_04628a(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L046288:
        bra.b   Enemy46_Tail_046220             | +028

| ----------------------------------------------------------------------------
|  Enemy46_PhaseD_Flip_04628a  @ $04628A  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Enemy46_PhaseD_Flip_04628a, "ax", @progbits
        .global Enemy46_PhaseD_Flip_04628a
Enemy46_PhaseD_Flip_04628a:
        addq.w  #0x1,0x80(a6)                   | +000
        lea     0x29bfb8.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L0462a0(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L0462a0:
        jsr     0x2783a.l                       | +016
        jsr     0x28d70.l                       | +01c
        bcc.w   .L0462bc                        | +022
        eori.b  #0x1,0x3a(a6)                   | +026
        lea     Enemy46_RandomPause_0462c0(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L0462bc:
        bra.w   Enemy46_Tail_046220             | +032

| ----------------------------------------------------------------------------
|  Enemy46_RandomPause_0462c0  @ $0462C0  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Enemy46_RandomPause_0462c0, "ax", @progbits
        .global Enemy46_RandomPause_0462c0
Enemy46_RandomPause_0462c0:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x1f,d0                        | +006
        addi.w  #0x14,d0                        | +00a
        move.w  d0,0x70(a6)                     | +00e
        jsr     0x5e9b6.l                       | +012
        andi.w  #0x3,d0                         | +018
        movea.l #0x28dc36,a0                    | +01c
        lsl.w   #0x2,d0                         | +022
        movea.l (a0,d0.w),a0                    | +024
        cmpa.l  #0xffffffff,a0                  | +028
        beq.w   .L0462f8                        | +02e
        jsr     0x28cd4.l                       | +032
.L0462f8:
        lea     .L0462fe(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L0462fe:
        jsr     0x2783a.l                       | +03e
        jsr     0x28d70.l                       | +044
        subq.w  #0x1,0x70(a6)                   | +04a
        cmpi.w  #0x0,0x70(a6)                   | +04e
        bgt.w   .L04631e                        | +054
        lea     Enemy46_Move_0461BC(pc),a1      | +058
        move.l  a1,(a6)                         | +05c
.L04631e:
        bra.w   Enemy46_Tail_046220             | +05e

| ----------------------------------------------------------------------------
|  Drop_SpawnRandom_046322  @ $046322  (144 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_SpawnRandom_046322, "ax", @progbits
        .global Drop_SpawnRandom_046322
Drop_SpawnRandom_046322:
        move.b  0x98(a6),d0                     | +000
        tst.b   d0                              | +004
        bne.w   .L046346                        | +006
        jsr     0x5e9b6.l                       | +00a
        andi.b  #0x1,d0                         | +010
        move.b  d0,0x99(a6)                     | +014
        jsr     0x5e9b6.l                       | +018
        andi.w  #0x7,d0                         | +01e
        addq.b  #0x1,d0                         | +022
.L046346:
        subq.b  #0x1,d0                         | +024
        andi.w  #0xf,d0                         | +026
        move.b  d0,0x98(a6)                     | +02a
        lsl.w   #0x1,d0                         | +02e
        lea     0x28dbc6.l,a0                   | +030
        move.b  0x9a(a6),d1                     | +036
        andi.w  #0x3,d1                         | +03a
        lsl.w   #0x2,d1                         | +03e
        movea.l (a0,d1.w),a0                    | +040
        move.w  (a0,d0.w),d1                    | +044
        lsl.w   #0x1,d0                         | +048
        lea     0x28db86.l,a0                   | +04a
        move.l  (a0,d0.w),0x3c(a6)              | +050
        jsr     0x236e.l                        | +056
        .global Drop_SpawnRandom_046322__L04637e
Drop_SpawnRandom_046322__L04637e:
.L04637e:
        move.b  0x99(a6),d0                     | +05c
        andi.b  #0x1,d0                         | +060
        move.b  d0,0x3a(a6)                     | +064
        lea     .L046390(pc),a1                 | +068
        move.l  a1,(a6)                         | +06c
.L046390:
        jsr     0x2783a.l                       | +06e
        jsr     0x5ca60.l                       | +074
        movea.l #0xffffffff,a0                  | +07a
        lea     0x28db7e.l,a0                   | +080
        jsr     0x5dd5c.l                       | +086
        bcc.w   SetHandlerRts_0463b8            | +08c

| ----------------------------------------------------------------------------
|  Drop_ProbeAndNudgeY_0463c2  @ $0463C2  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_ProbeAndNudgeY_0463c2, "ax", @progbits
        .global Drop_ProbeAndNudgeY_0463c2
Drop_ProbeAndNudgeY_0463c2:
        move.b  0x9a(a6),d0                     | +000
        move.w  #0x9b,d1                        | +004
        jsr     0x9a7aa.l                       | +008
        bcs.w   .L0463d8                        | +00e
        addq.w  #0x4,0x24(a0)                   | +012
.L0463d8:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Drop_Spawn_Tmpl4F_0463da  @ $0463DA  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_Spawn_Tmpl4F_0463da, "ax", @progbits
        .global Drop_Spawn_Tmpl4F_0463da
Drop_Spawn_Tmpl4F_0463da:
        move.w  #0x4f,d1                        | +000
        jsr     0x236e.l                        | +004
        move.l  #0x233c16,0x3c(a6)              | +00a
        bra.b   Drop_SpawnRandom_046322__L04637e | +012

| ----------------------------------------------------------------------------
|  Drop_Spawn_TmplF2_0463ee  @ $0463EE  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_Spawn_TmplF2_0463ee, "ax", @progbits
        .global Drop_Spawn_TmplF2_0463ee
Drop_Spawn_TmplF2_0463ee:
        move.w  #0xf2,d1                        | +000
        jsr     0x236e.l                        | +004
        move.l  #0x24d800,0x3c(a6)              | +00a
        bra.w   Drop_SpawnRandom_046322__L04637e | +012

| ----------------------------------------------------------------------------
|  Drop_Spawn_Tmpl1B_046404  @ $046404  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_Spawn_Tmpl1B_046404, "ax", @progbits
        .global Drop_Spawn_Tmpl1B_046404
Drop_Spawn_Tmpl1B_046404:
        move.w  #0x1b,d1                        | +000
        jsr     0x236e.l                        | +004
        move.l  #0x23603a,0x3c(a6)              | +00a
        bra.w   Drop_SpawnRandom_046322__L04637e | +012

| ----------------------------------------------------------------------------
|  Drop_Spawn_Tmpl1B_B_04641a  @ $04641A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_Spawn_Tmpl1B_B_04641a, "ax", @progbits
        .global Drop_Spawn_Tmpl1B_B_04641a
Drop_Spawn_Tmpl1B_B_04641a:
        move.w  #0x1b,d1                        | +000
        jsr     0x236e.l                        | +004
        move.l  #0x23603a,0x3c(a6)              | +00a
        bra.w   Drop_SpawnRandom_046322__L04637e | +012

| ----------------------------------------------------------------------------
|  Drop_Spawn_Tmpl6D_046430  @ $046430  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_Spawn_Tmpl6D_046430, "ax", @progbits
        .global Drop_Spawn_Tmpl6D_046430
Drop_Spawn_Tmpl6D_046430:
        move.w  #0x6d,d1                        | +000
        jsr     0x236e.l                        | +004
        move.l  #0x23ac84,0x3c(a6)              | +00a
        bra.w   Drop_SpawnRandom_046322__L04637e | +012

| ----------------------------------------------------------------------------
|  Drop_Spawn_Tmpl2C_046446  @ $046446  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_Spawn_Tmpl2C_046446, "ax", @progbits
        .global Drop_Spawn_Tmpl2C_046446
Drop_Spawn_Tmpl2C_046446:
        move.w  #0x2c,d1                        | +000
        jsr     0x236e.l                        | +004
        move.l  #0x23c778,0x3c(a6)              | +00a
        bra.w   Drop_SpawnRandom_046322__L04637e | +012

| ----------------------------------------------------------------------------
|  Drop_Spawn_Tmpl2B_04645c  @ $04645C  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_Spawn_Tmpl2B_04645c, "ax", @progbits
        .global Drop_Spawn_Tmpl2B_04645c
Drop_Spawn_Tmpl2B_04645c:
        move.l  #0x239a4e,d0                    | +000
.L046462:
        move.l  d0,0x3c(a6)                     | +006
        move.w  #0x2b,d1                        | +00a
        jsr     0x236e.l                        | +00e
        bra.w   Drop_SpawnRandom_046322__L04637e | +014
        move.l  #0x23d4a0,d0                    | +018
        bra.b   .L046462                        | +01e
        move.l  #0x24c3e2,d0                    | +020
        bra.b   .L046462                        | +026

| ----------------------------------------------------------------------------
|  Drop_Spawn_Tmpl3A_12F_046484  @ $046484  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_Spawn_Tmpl3A_12F_046484, "ax", @progbits
        .global Drop_Spawn_Tmpl3A_12F_046484
Drop_Spawn_Tmpl3A_12F_046484:
        move.w  #0x3a,d1                        | +000
        tst.b   0x98(a6)                        | +004
        beq.w   .L046494                        | +008
        move.w  #0x12f,d1                       | +00c
.L046494:
        jsr     0x236e.l                        | +010
        move.l  #0x23a068,0x3c(a6)              | +016
        bra.w   Drop_SpawnRandom_046322__L04637e | +01e

| ----------------------------------------------------------------------------
|  Drop_SpawnFromTable_0464a6  @ $0464A6  (114 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_SpawnFromTable_0464a6, "ax", @progbits
        .global Drop_SpawnFromTable_0464a6
Drop_SpawnFromTable_0464a6:
        move.l  #0x249bd4,0x3c(a6)              | +000
        bra.w   .L0464ba                        | +008
        move.l  #0x249c0c,0x3c(a6)              | +00c
.L0464ba:
        tst.b   0x9c(a6)                        | +014
        beq.w   .L0464d8                        | +018
        lea     0x723d2.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        clr.w   0x98(a0)                        | +02e
.L0464d8:
        move.b  0x9b(a6),d0                     | +032
        tst.b   d0                              | +036
        bne.w   .L0464e6                        | +038
        move.b  #0xff,d0                        | +03c
.L0464e6:
        move.b  d0,0x32(a6)                     | +040
        move.b  d0,0x33(a6)                     | +044
        move.b  0x98(a6),d0                     | +048
        andi.w  #0x3,d0                         | +04c
        move.b  0x9a(a6),d1                     | +050
        andi.w  #0x3,d1                         | +054
        lsl.w   #0x2,d0                         | +058
        add.w   d1,d0                           | +05a
        lsl.w   #0x1,d0                         | +05c
        lea     0x28dc4e.l,a0                   | +05e
        move.w  (a0,d0.w),d1                    | +064
        jsr     0x236e.l                        | +068
        bra.w   Drop_SpawnRandom_046322__L04637e | +06e

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_046518  @ $046518  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_046518, "ax", @progbits
        .global Entity_CmpDepthToParent_046518
Entity_CmpDepthToParent_046518:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_04652e                    | +00c

| ----------------------------------------------------------------------------
|  Drop_SpawnThrown_046534  @ $046534  (162 B)
| ----------------------------------------------------------------------------
        .section .text.Drop_SpawnThrown_046534, "ax", @progbits
        .global Drop_SpawnThrown_046534
Drop_SpawnThrown_046534:
        move.b  0x99(a6),d0                     | +000
        ext.w   d0                              | +004
        lsl.w   #0x4,d0                         | +006
        move.w  d0,0x28(a6)                     | +008
        btst    #0x7,0x99(a6)                   | +00c
        beq.w   .L046550                        | +012
        move.b  #0x1,0x3a(a6)                   | +016
.L046550:
        move.b  0x98(a6),d0                     | +01c
        andi.w  #0x3,d0                         | +020
        lsl.w   #0x2,d0                         | +024
        move.b  0x9a(a6),d1                     | +026
        andi.w  #0x3,d1                         | +02a
        add.w   d1,d0                           | +02e
        lsl.w   #0x1,d0                         | +030
        lea     0x28dc4e.l,a0                   | +032
        move.w  (a0,d0.w),d1                    | +038
        jsr     0x236e.l                        | +03c
        move.b  0x9b(a6),d0                     | +042
        tst.b   d0                              | +046
        bne.w   .L046584                        | +048
        move.b  #0xff,d0                        | +04c
.L046584:
        move.b  d0,0x32(a6)                     | +050
        move.b  d0,0x33(a6)                     | +054
        move.w  #0x8000,d0                      | +058
        jsr     0x28134.l                       | +05c
        andi.w  #0xffe3,0x38(a6)                | +062
        ori.w   #0x10,0x38(a6)                  | +068
        lea     0x28dc6e.l,a0                   | +06e
        jsr     0x28cd4.l                       | +074
        lea     .L0465b4(pc),a1                 | +07a
        move.l  a1,(a6)                         | +07e
.L0465b4:
        jsr     0x27cee.l                       | +080
        jsr     0x28d70.l                       | +086
        movea.l #0xffffffff,a0                  | +08c
        lea     0x28dc46.l,a0                   | +092
        jsr     0x5dd5c.l                       | +098
        bcc.w   SetHandlerRts_0465dc            | +09e

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_0465ec  @ $0465EC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_0465ec, "ax", @progbits
        .global Entity_CmpDepthToParent_0465ec
Entity_CmpDepthToParent_0465ec:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_046602                    | +00c

| ----------------------------------------------------------------------------
|  Fade_WhiteFlash_Task_046608  @ $046608  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Fade_WhiteFlash_Task_046608, "ax", @progbits
        .global Fade_WhiteFlash_Task_046608
Fade_WhiteFlash_Task_046608:
        move.w  #0x0,d0                         | +000
        move.w  #0x1f,d1                        | +004
        move.w  #0x1f,d2                        | +008
        move.w  #0x10,d3                        | +00c
        jsr     0x52580.l                       | +010
        move.b  #0xff,0x1081b1.l                | +016
        move.w  #0x20,0x70(a6)                  | +01e
        lea     .L046632(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L046632:
        subq.w  #0x1,0x70(a6)                   | +02a
        cmpi.w  #0x0,0x70(a6)                   | +02e
        bgt.w   SetHandlerRts_046662            | +034
        move.w  #0x1f,d0                        | +038
        move.w  #0x1f,d1                        | +03c
        move.w  #0x1f,d2                        | +040
        move.w  #0x10,d3                        | +044
        jsr     0x52580.l                       | +048
        move.w  #0x10,0x70(a6)                  | +04e

| ----------------------------------------------------------------------------
|  Fade_WhiteFlash_Done_046664  @ $046664  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Fade_WhiteFlash_Done_046664, "ax", @progbits
        .global Fade_WhiteFlash_Done_046664
Fade_WhiteFlash_Done_046664:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L046680                        | +00a
        movea.l 0xc(a6),a1                      | +00e
        clr.b   0x21(a1)                        | +012
        jmp     0x518.l                         | +016
.L046680:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  SceneC_Load_Task_046682  @ $046682  (42 B)
| ----------------------------------------------------------------------------
        .section .text.SceneC_Load_Task_046682, "ax", @progbits
        .global SceneC_Load_Task_046682
SceneC_Load_Task_046682:
        jsr     0x22c8.l                        | +000
        move.b  #0xc,d0                         | +006
        jsr     0x43568.l                       | +00a
        moveq   #0,d0                           | +010
        moveq   #0,d1                           | +012
        jsr     0x437da.l                       | +014
        move.w  #0x3,d0                         | +01a
        jsr     0x523b2.l                       | +01e
        move.w  #0x78,0x70(a6)                  | +024

| ----------------------------------------------------------------------------
|  SceneC_Load_Spawn2_0466b4  @ $0466B4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.SceneC_Load_Spawn2_0466b4, "ax", @progbits
        .global SceneC_Load_Spawn2_0466b4
SceneC_Load_Spawn2_0466b4:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   SetHandlerRts_0466d8            | +00a
        move.w  #0x2,d0                         | +00e
        jsr     0x5239e.l                       | +012
        move.w  #0x3c,0x70(a6)                  | +018

| ----------------------------------------------------------------------------
|  SceneC_Load_Finish_0466da  @ $0466DA  (28 B)
| ----------------------------------------------------------------------------
        .section .text.SceneC_Load_Finish_0466da, "ax", @progbits
        .global SceneC_Load_Finish_0466da
SceneC_Load_Finish_0466da:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L0466f4                        | +00a
        clr.b   0x106ed2.l                      | +00e
        jmp     0x518.l                         | +014
.L0466f4:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  ContinueDigits_P1P2_Task_0466f6  @ $0466F6  (256 B)
| ----------------------------------------------------------------------------
        .section .text.ContinueDigits_P1P2_Task_0466f6, "ax", @progbits
        .global ContinueDigits_P1P2_Task_0466f6
ContinueDigits_P1P2_Task_0466f6:
        move.w  #0x0,0x74(a6)                   | +000
        move.w  #0x7183,d0                      | +006
        move.w  d0,0x22(a6)                     | +00a
        jsr     0x260c4.l                       | +00e
        jsr     0x2604c.l                       | +014
        bra.w   .L04673a                        | +01a
        move.w  #0x1,0x74(a6)                   | +01e
        move.w  #0x7463,d0                      | +024
        move.w  d0,0x22(a6)                     | +028
        jsr     0x260cc.l                       | +02c
        jsr     0x26060.l                       | +032
        move.w  #0x9,0x5c(a6)                   | +038
        move.w  #0x0,0x70(a6)                   | +03e
.L04673a:
        move.w  #0x9,0x5c(a6)                   | +044
        move.w  #0x3b,0x70(a6)                  | +04a
        move.w  0x22(a6),d0                     | +050
        move.w  0x5c(a6),d1                     | +054
        addi.w  #0x3460,d1                      | +058
        movem.w d0-d1,0x3c0000.l                | +05c
        addq.w  #0x1,d0                         | +064
        addi.w  #0x10,d1                        | +066
        movem.w d0-d1,0x3c0000.l                | +06a
        lea     .L04676e(pc),a1                 | +072
        move.l  a1,(a6)                         | +076
.L04676e:
        movea.l 0xc(a6),a1                      | +078
        cmpi.b  #0x0,0x20(a1)                   | +07c
        bne.w   .L046784                        | +082
        jmp     0x518.l                         | +086
        rts                                     | +08c
.L046784:
        cmpi.w  #0x0,0x74(a6)                   | +08e
        bne.w   .L046796                        | +094
        jsr     Continue_IsStartP1_04694a(pc)   | +098
        bra.w   .L04679a                        | +09c
.L046796:
        jsr     Continue_IsStartP2_04698c(pc)   | +0a0
.L04679a:
        bcc.w   .L0467a2                        | +0a4
        clr.w   0x70(a6)                        | +0a8
.L0467a2:
        subq.w  #0x1,0x70(a6)                   | +0ac
        cmpi.w  #0x0,0x70(a6)                   | +0b0
        bgt.w   .L0467f4                        | +0b6
        subq.w  #0x1,0x5c(a6)                   | +0ba
        cmpi.w  #0x0,0x5c(a6)                   | +0be
        bge.w   .L0467cc                        | +0c4
        movea.l 0xc(a6),a1                      | +0c8
        clr.b   0x21(a1)                        | +0cc
        jmp     0x518.l                         | +0d0
.L0467cc:
        move.w  0x22(a6),d0                     | +0d6
        move.w  0x5c(a6),d1                     | +0da
        addi.w  #0x3460,d1                      | +0de
        movem.w d0-d1,0x3c0000.l                | +0e2
        addq.w  #0x1,d0                         | +0ea
        addi.w  #0x10,d1                        | +0ec
        movem.w d0-d1,0x3c0000.l                | +0f0
        move.w  #0x3b,0x70(a6)                  | +0f8
.L0467f4:
        rts                                     | +0fe

| ----------------------------------------------------------------------------
|  ContinueBig_Init_0467f6  @ $0467F6  (130 B)
| ----------------------------------------------------------------------------
        .section .text.ContinueBig_Init_0467f6, "ax", @progbits
        .global ContinueBig_Init_0467f6
ContinueBig_Init_0467f6:
        move.w  #0x0,0x74(a6)                   | +000
        bra.w   .L046806                        | +006
        move.w  #0x1,0x74(a6)                   | +00a
.L046806:
        move.w  #0xffff,0x38(a6)                | +010
        bset    #0x6,0x12(a6)                   | +016
        move.w  #0x5f,d1                        | +01c
        jsr     0x236e.l                        | +020
        move.w  0x14(a6),d1                     | +026
        jsr     0x2c66.l                        | +02a
        lea     0x28de2a.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        move.w  #0xa0,0x22(a6)                  | +03c
        move.w  #0x178,0x24(a6)                 | +042
        clr.w   0x26(a6)                        | +048
        move.w  #0x9,0x5c(a6)                   | +04c
        move.w  #0x3b,0x70(a6)                  | +052
        move.b  #0xff,0x72(a6)                  | +058
        movea.w #0x716a,a1                      | +05e
        lea     0x28de36.l,a2                   | +062
        move.w  #0x4,d1                         | +068
        jsr     0x4784c.l                       | +06c
        lea     ContinueBig_Countdown_046878(pc),a1 | +072
        move.l  a1,(a6)                         | +076
        move.w  #0x10d4,d0                      | +078
        jsr     0x2352.l                        | +07c

| ----------------------------------------------------------------------------
|  ContinueBig_Countdown_046878  @ $046878  (174 B)
| ----------------------------------------------------------------------------
        .section .text.ContinueBig_Countdown_046878, "ax", @progbits
        .global ContinueBig_Countdown_046878
ContinueBig_Countdown_046878:
        movea.l 0xc(a6),a1                      | +000
        cmpi.b  #0x0,0x20(a1)                   | +004
        bne.w   .L04688a                        | +00a
        bra.w   ContinueBig_ClearAndExit_04692e | +00e
.L04688a:
        jsr     Continue_IsStartAny_0469ce(pc)  | +012
        bcc.w   .L04689c                        | +016
        move.b  #0xff,0x72(a6)                  | +01a
        clr.w   0x70(a6)                        | +020
.L04689c:
        subq.w  #0x1,0x70(a6)                   | +024
        cmpi.w  #0x0,0x70(a6)                   | +028
        bgt.w   .L0468ea                        | +02e
        subq.w  #0x1,0x5c(a6)                   | +032
        cmpi.w  #0x0,0x5c(a6)                   | +036
        bge.w   .L0468c4                        | +03c
        movea.l 0xc(a6),a1                      | +040
        clr.b   0x21(a1)                        | +044
        bra.w   ContinueBig_ClearAndExit_04692e | +048
.L0468c4:
        move.w  #0x10d4,d0                      | +04c
        jsr     0x2352.l                        | +050
        move.w  #0x3b,0x70(a6)                  | +056
        lea     0x28dd96.l,a1                   | +05c
        move.w  0x5c(a6),d0                     | +062
        lsl.w   #0x2,d0                         | +066
        movea.l (a1,d0.w),a0                    | +068
        jsr     0x28cd4.l                       | +06c
.L0468ea:
        lea     0x28de54.l,a1                   | +072
        move.w  0x70(a6),d1                     | +078
        move.b  (a1,d1.w),d0                    | +07c
        cmpi.w  #0x37,0x70(a6)                  | +080
        ble.w   .L04691e                        | +086
        cmpi.b  #0x0,0x72(a6)                   | +08a
        bne.w   .L046914                        | +090
        move.b  d0,0x32(a6)                     | +094
        bra.w   .L04691a                        | +098
.L046914:
        move.b  #0xff,0x32(a6)                  | +09c
.L04691a:
        bra.w   JsrAbsThunk_046926              | +0a2
.L04691e:
        clr.b   0x72(a6)                        | +0a6
        move.b  d0,0x32(a6)                     | +0aa

| ----------------------------------------------------------------------------
|  ContinueBig_ClearAndExit_04692e  @ $04692E  (26 B)
| ----------------------------------------------------------------------------
        .section .text.ContinueBig_ClearAndExit_04692e, "ax", @progbits
        .global ContinueBig_ClearAndExit_04692e
ContinueBig_ClearAndExit_04692e:
        movea.w #0x716a,a1                      | +000
        lea     0x28de40.l,a2                   | +004
        move.w  #0x4,d1                         | +00a
        jsr     0x4784c.l                       | +00e
        jmp     0x518.l                         | +014

| ----------------------------------------------------------------------------
|  Rts_046948  @ $046948  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_046948, "ax", @progbits
        .global Rts_046948
Rts_046948:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Continue_IsStartP1_04694a  @ $04694A  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartP1_04694a, "ax", @progbits
        .global Continue_IsStartP1_04694a
Continue_IsStartP1_04694a:
        cmpi.b  #0x2,0x10fdb6.l                 | +000
        beq.w   Continue_IsStartP1_CheckCount_04695c | +008

| ----------------------------------------------------------------------------
|  Continue_IsStartP1_CheckCount_04695c  @ $04695C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartP1_CheckCount_04695c, "ax", @progbits
        .global Continue_IsStartP1_CheckCount_04695c
Continue_IsStartP1_CheckCount_04695c:
        cmpi.w  #0x8,0x5c(a6)                   | +000
        bls.w   Continue_IsStartP1_CheckTask_04696c | +006

| ----------------------------------------------------------------------------
|  Continue_IsStartP1_CheckTask_04696c  @ $04696C  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartP1_CheckTask_04696c, "ax", @progbits
        .global Continue_IsStartP1_CheckTask_04696c
Continue_IsStartP1_CheckTask_04696c:
        move.b  0x10e203.l,d0                   | +000
        andi.b  #0xf0,d0                        | +006
        beq.w   ClearXN_046984                  | +00a

| ----------------------------------------------------------------------------
|  Continue_IsStartP1_Yes_046980  @ $046980  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartP1_Yes_046980, "ax", @progbits
        .global Continue_IsStartP1_Yes_046980
Continue_IsStartP1_Yes_046980:
        bra.w   Stub_0004698A                   | +000

| ----------------------------------------------------------------------------
|  Continue_IsStartP2_04698c  @ $04698C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartP2_04698c, "ax", @progbits
        .global Continue_IsStartP2_04698c
Continue_IsStartP2_04698c:
        cmpi.b  #0x2,0x10fdb7.l                 | +000
        beq.w   Continue_IsStartP2_CheckCount_04699e | +008

| ----------------------------------------------------------------------------
|  Continue_IsStartP2_CheckCount_04699e  @ $04699E  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartP2_CheckCount_04699e, "ax", @progbits
        .global Continue_IsStartP2_CheckCount_04699e
Continue_IsStartP2_CheckCount_04699e:
        cmpi.w  #0x8,0x5c(a6)                   | +000
        bls.w   Continue_IsStartP2_CheckTask_0469ae | +006

| ----------------------------------------------------------------------------
|  Continue_IsStartP2_CheckTask_0469ae  @ $0469AE  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartP2_CheckTask_0469ae, "ax", @progbits
        .global Continue_IsStartP2_CheckTask_0469ae
Continue_IsStartP2_CheckTask_0469ae:
        move.b  0x10e209.l,d0                   | +000
        andi.b  #0xf0,d0                        | +006
        beq.w   ClearXN_0469c6                  | +00a

| ----------------------------------------------------------------------------
|  Continue_IsStartP2_Yes_0469c2  @ $0469C2  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartP2_Yes_0469c2, "ax", @progbits
        .global Continue_IsStartP2_Yes_0469c2
Continue_IsStartP2_Yes_0469c2:
        bra.w   Stub_000469CC                   | +000

| ----------------------------------------------------------------------------
|  Continue_IsStartAny_0469ce  @ $0469CE  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartAny_0469ce, "ax", @progbits
        .global Continue_IsStartAny_0469ce
Continue_IsStartAny_0469ce:
        jsr     Continue_IsStartP1_04694a(pc)   | +000
        bcc.w   JsrPcThunk_0469dc               | +004
        rts                                     | +008

| ----------------------------------------------------------------------------
|  Continue_IsStartAny_Tail_0469d8  @ $0469D8  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_IsStartAny_Tail_0469d8, "ax", @progbits
        .global Continue_IsStartAny_Tail_0469d8
Continue_IsStartAny_Tail_0469d8:
        bra.w   JsrPcRts_0469e0                 | +000

| ----------------------------------------------------------------------------
|  Continue_StoreCount_Exit_0469e2  @ $0469E2  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_StoreCount_Exit_0469e2, "ax", @progbits
        .global Continue_StoreCount_Exit_0469e2
Continue_StoreCount_Exit_0469e2:
        move.b  0x98(a6),d0                     | +000
        cmpi.b  #0xff,d0                        | +004
        bne.w   .L0469f0                        | +008
        clr.b   d0                              | +00c
.L0469f0:
        move.b  d0,0x1081b0.l                   | +00e
        jmp     0x518.l                         | +014

| ----------------------------------------------------------------------------
|  Rts_0469fc  @ $0469FC  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_0469fc, "ax", @progbits
        .global Rts_0469fc
Rts_0469fc:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Fix_ClearMissionBanner_0469fe  @ $0469FE  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_ClearMissionBanner_0469fe, "ax", @progbits
        .global Fix_ClearMissionBanner_0469fe
Fix_ClearMissionBanner_0469fe:
        movea.l #0x7014,a1                      | +000
        lea     0x28de92.l,a2                   | +006
        move.b  #0x9,d1                         | +00c
        jsr     0x47888.l                       | +010
        movea.l #0x7017,a1                      | +016
        lea     0x28de92.l,a2                   | +01c
        move.b  #0x9,d1                         | +022

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_046a2c  @ $046A2C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_046a2c, "ax", @progbits
        .global Entity_CmpDepthToParent_046a2c
Entity_CmpDepthToParent_046a2c:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_046a42                    | +00c

| ----------------------------------------------------------------------------
|  TimeUp_Banner_Task_046a48  @ $046A48  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TimeUp_Banner_Task_046a48, "ax", @progbits
        .global TimeUp_Banner_Task_046a48
TimeUp_Banner_Task_046a48:
        move.w  #0x5a,0x70(a6)                  | +000
        movea.w #0x71af,a1                      | +006
        move.b  #0x4,d1                         | +00a
        lea     0x28debc.l,a2                   | +00e
        jsr     List_ApplyWithSentinelFF_04784C(pc) | +014
        lea     .L046a66(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L046a66:
        subq.w  #0x1,0x70(a6)                   | +01e
        cmpi.w  #0x0,0x70(a6)                   | +022
        bgt.w   .L046a94                        | +028
        movea.w #0x71af,a1                      | +02c
        move.b  #0x4,d1                         | +030
        lea     0x28dec4.l,a2                   | +034
        jsr     List_ApplyWithSentinelFF_04784C(pc) | +03a
        movea.l 0xc(a6),a1                      | +03e
        clr.b   0x21(a1)                        | +042
        jmp     0x518.l                         | +046
.L046a94:
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  FixLayer_ClearTopRows_046a96  @ $046A96  (48 B)
| ----------------------------------------------------------------------------
        .section .text.FixLayer_ClearTopRows_046a96, "ax", @progbits
        .global FixLayer_ClearTopRows_046a96
FixLayer_ClearTopRows_046a96:
        movea.w #0x7000,a1                      | +000
        move.w  #0x20,d0                        | +004
        move.w  #0x28,d1                        | +008
        move.w  #0x2,d2                         | +00c
        jsr     0x5da9c.l                       | +010
        movea.w #0x701a,a1                      | +016
        move.w  #0x20,d0                        | +01a
        move.w  #0x28,d1                        | +01e
        move.w  #0x6,d2                         | +022
        jsr     0x5da9c.l                       | +026
        bra.w   FixLayer_QuadBatch_046AC6__L046af2 | +02c

| ----------------------------------------------------------------------------
|  FixBlit_Row4x1_Step_046c48  @ $046C48  (78 B)
| ----------------------------------------------------------------------------
        .section .text.FixBlit_Row4x1_Step_046c48, "ax", @progbits
        .global FixBlit_Row4x1_Step_046c48
FixBlit_Row4x1_Step_046c48:
        cmpi.w  #0x7000,d0                      | +000
        bcc.w   .L046c5a                        | +004
        addi.w  #0x20,d0                        | +008
        addq.w  #0x1,d1                         | +00c
        bra.w   .L046c94                        | +00e
.L046c5a:
        movem.w d0-d1,0x3c0000.l                | +012
        addq.w  #0x1,d0                         | +01a
        addi.w  #0x10,d1                        | +01c
        movem.w d0-d1,0x3c0000.l                | +020
        addq.w  #0x1,d0                         | +028
        addi.w  #0x10,d1                        | +02a
        movem.w d0-d1,0x3c0000.l                | +02e
        addq.w  #0x1,d0                         | +036
        addi.w  #0x10,d1                        | +038
        movem.w d0-d1,0x3c0000.l                | +03c
        addi.w  #0x1d,d0                        | +044
        subi.w  #0x2f,d1                        | +048
.L046c94:
        bra.b   FixBlit_BatchRow4x1_ColorInc_046BDA__L046c34 | +04c

| ----------------------------------------------------------------------------
|  FixBlit_BatchRow4x1_FromTable_046c96  @ $046C96  (120 B)
| ----------------------------------------------------------------------------
        .section .text.FixBlit_BatchRow4x1_FromTable_046c96, "ax", @progbits
        .global FixBlit_BatchRow4x1_FromTable_046c96
FixBlit_BatchRow4x1_FromTable_046c96:
        lea     0x28def4.l,a1                   | +000
        lsl.w   #0x1,d5                         | +006
        move.w  (a1,d5.w),d1                    | +008
        add.w   d4,d1                           | +00c
        move.w  #0x0,d6                         | +00e
        bra.b   .L046cac                        | +012
.L046caa:
        addq.w  #0x1,d6                         | +014
.L046cac:
        cmpi.w  #0x3,d6                         | +016
        bgt.w   .L046d0c                        | +01a
        cmpi.w  #0x74ff,d0                      | +01e
        bls.w   .L046cbe                        | +022
        rts                                     | +026
.L046cbe:
        cmpi.w  #0x7000,d0                      | +028
        bcc.w   .L046cd0                        | +02c
        addi.w  #0x20,d0                        | +030
        addq.w  #0x1,d1                         | +034
        bra.w   .L046d0a                        | +036
.L046cd0:
        movem.w d0-d1,0x3c0000.l                | +03a
        addq.w  #0x1,d0                         | +042
        addi.w  #0x10,d1                        | +044
        movem.w d0-d1,0x3c0000.l                | +048
        addq.w  #0x1,d0                         | +050
        addi.w  #0x10,d1                        | +052
        movem.w d0-d1,0x3c0000.l                | +056
        addq.w  #0x1,d0                         | +05e
        addi.w  #0x10,d1                        | +060
        movem.w d0-d1,0x3c0000.l                | +064
        addi.w  #0x1d,d0                        | +06c
        subi.w  #0x2f,d1                        | +070
.L046d0a:
        bra.b   .L046caa                        | +074
.L046d0c:
        rts                                     | +076

| ----------------------------------------------------------------------------
|  FixBanner_MissionStart_Blit_046d0e  @ $046D0E  (356 B)
| ----------------------------------------------------------------------------
        .section .text.FixBanner_MissionStart_Blit_046d0e, "ax", @progbits
        .global FixBanner_MissionStart_Blit_046d0e
FixBanner_MissionStart_Blit_046d0e:
        move.w  #0x7000,d0                      | +000
        asl.w   #0x5,d2                         | +004
        add.w   d2,d0                           | +006
        add.w   d3,d0                           | +008
        cmpi.w  #0x0,d4                         | +00a
        bne.w   .L046d28                        | +00e
        move.w  #0x0,d4                         | +012
        bra.w   .L046d2c                        | +016
.L046d28:
        move.w  #0x1000,d4                      | +01a
.L046d2c:
        move.w  #0x3e40,d1                      | +01e
        add.w   d4,d1                           | +022
        move.w  #0x0,d5                         | +024
        bra.b   .L046d3a                        | +028
.L046d38:
        addq.w  #0x1,d5                         | +02a
.L046d3a:
        cmpi.w  #0xf,d5                         | +02c
        bgt.w   .L046d9a                        | +030
        cmpi.w  #0x74ff,d0                      | +034
        bls.w   .L046d4c                        | +038
        rts                                     | +03c
.L046d4c:
        cmpi.w  #0x7000,d0                      | +03e
        bcc.w   .L046d5e                        | +042
        addi.w  #0x20,d0                        | +046
        addq.w  #0x1,d1                         | +04a
        bra.w   .L046d98                        | +04c
.L046d5e:
        movem.w d0-d1,0x3c0000.l                | +050
        addq.w  #0x1,d0                         | +058
        addi.w  #0x10,d1                        | +05a
        movem.w d0-d1,0x3c0000.l                | +05e
        addq.w  #0x1,d0                         | +066
        addi.w  #0x10,d1                        | +068
        movem.w d0-d1,0x3c0000.l                | +06c
        addq.w  #0x1,d0                         | +074
        addi.w  #0x10,d1                        | +076
        movem.w d0-d1,0x3c0000.l                | +07a
        addi.w  #0x1d,d0                        | +082
        subi.w  #0x2f,d1                        | +086
.L046d98:
        bra.b   .L046d38                        | +08a
.L046d9a:
        move.w  #0x3f40,d1                      | +08c
        add.w   d4,d1                           | +090
        move.w  #0x0,d5                         | +092
        bra.b   .L046da8                        | +096
.L046da6:
        addq.w  #0x1,d5                         | +098
.L046da8:
        cmpi.w  #0x1,d5                         | +09a
        bgt.w   .L046e08                        | +09e
        cmpi.w  #0x74ff,d0                      | +0a2
        bls.w   .L046dba                        | +0a6
        rts                                     | +0aa
.L046dba:
        cmpi.w  #0x7000,d0                      | +0ac
        bcc.w   .L046dcc                        | +0b0
        addi.w  #0x20,d0                        | +0b4
        addq.w  #0x1,d1                         | +0b8
        bra.w   .L046e06                        | +0ba
.L046dcc:
        movem.w d0-d1,0x3c0000.l                | +0be
        addq.w  #0x1,d0                         | +0c6
        addi.w  #0x10,d1                        | +0c8
        movem.w d0-d1,0x3c0000.l                | +0cc
        addq.w  #0x1,d0                         | +0d4
        addi.w  #0x10,d1                        | +0d6
        movem.w d0-d1,0x3c0000.l                | +0da
        addq.w  #0x1,d0                         | +0e2
        addi.w  #0x10,d1                        | +0e4
        movem.w d0-d1,0x3c0000.l                | +0e8
        addi.w  #0x1d,d0                        | +0f0
        subi.w  #0x2f,d1                        | +0f4
.L046e06:
        bra.b   .L046da6                        | +0f8
.L046e08:
        cmpi.w  #0x74ff,d0                      | +0fa
        bls.w   .L046e12                        | +0fe
        rts                                     | +102
.L046e12:
        cmpi.w  #0x7000,d0                      | +104
        bcc.w   .L046e1c                        | +108
        rts                                     | +10c
.L046e1c:
        move.w  #0x2320,d1                      | +10e
        movem.w d0-d1,0x3c0000.l                | +112
        addq.w  #0x1,d0                         | +11a
        movem.w d0-d1,0x3c0000.l                | +11c
        addq.w  #0x1,d0                         | +124
        movem.w d0-d1,0x3c0000.l                | +126
        addq.w  #0x1,d0                         | +12e
        movem.w d0-d1,0x3c0000.l                | +130
        addi.w  #0x1d,d0                        | +138
        movem.w d0-d1,0x3c0000.l                | +13c
        addq.w  #0x1,d0                         | +144
        movem.w d0-d1,0x3c0000.l                | +146
        addq.w  #0x1,d0                         | +14e
        movem.w d0-d1,0x3c0000.l                | +150
        addq.w  #0x1,d0                         | +158
        movem.w d0-d1,0x3c0000.l                | +15a
        rts                                     | +162

| ----------------------------------------------------------------------------
|  FixBanner_MissionComplete_Blit_046e72  @ $046E72  (356 B)
| ----------------------------------------------------------------------------
        .section .text.FixBanner_MissionComplete_Blit_046e72, "ax", @progbits
        .global FixBanner_MissionComplete_Blit_046e72
FixBanner_MissionComplete_Blit_046e72:
        move.w  #0x7000,d0                      | +000
        asl.w   #0x5,d2                         | +004
        add.w   d2,d0                           | +006
        add.w   d3,d0                           | +008
        cmpi.w  #0x0,d4                         | +00a
        bne.w   .L046e8c                        | +00e
        move.w  #0x0,d4                         | +012
        bra.w   .L046e90                        | +016
.L046e8c:
        move.w  #0x1000,d4                      | +01a
.L046e90:
        move.w  #0x3e80,d1                      | +01e
        add.w   d4,d1                           | +022
        move.w  #0x0,d5                         | +024
        bra.b   .L046e9e                        | +028
.L046e9c:
        addq.w  #0x1,d5                         | +02a
.L046e9e:
        cmpi.w  #0xf,d5                         | +02c
        bgt.w   .L046efe                        | +030
        cmpi.w  #0x74ff,d0                      | +034
        bls.w   .L046eb0                        | +038
        rts                                     | +03c
.L046eb0:
        cmpi.w  #0x7000,d0                      | +03e
        bcc.w   .L046ec2                        | +042
        addi.w  #0x20,d0                        | +046
        addq.w  #0x1,d1                         | +04a
        bra.w   .L046efc                        | +04c
.L046ec2:
        movem.w d0-d1,0x3c0000.l                | +050
        addq.w  #0x1,d0                         | +058
        addi.w  #0x10,d1                        | +05a
        movem.w d0-d1,0x3c0000.l                | +05e
        addq.w  #0x1,d0                         | +066
        addi.w  #0x10,d1                        | +068
        movem.w d0-d1,0x3c0000.l                | +06c
        addq.w  #0x1,d0                         | +074
        addi.w  #0x10,d1                        | +076
        movem.w d0-d1,0x3c0000.l                | +07a
        addi.w  #0x1d,d0                        | +082
        subi.w  #0x2f,d1                        | +086
.L046efc:
        bra.b   .L046e9c                        | +08a
.L046efe:
        move.w  #0x3f80,d1                      | +08c
        add.w   d4,d1                           | +090
        move.w  #0x0,d5                         | +092
        bra.b   .L046f0c                        | +096
.L046f0a:
        addq.w  #0x1,d5                         | +098
.L046f0c:
        cmpi.w  #0xb,d5                         | +09a
        bgt.w   .L046f6c                        | +09e
        cmpi.w  #0x74ff,d0                      | +0a2
        bls.w   .L046f1e                        | +0a6
        rts                                     | +0aa
.L046f1e:
        cmpi.w  #0x7000,d0                      | +0ac
        bcc.w   .L046f30                        | +0b0
        addi.w  #0x20,d0                        | +0b4
        addq.w  #0x1,d1                         | +0b8
        bra.w   .L046f6a                        | +0ba
.L046f30:
        movem.w d0-d1,0x3c0000.l                | +0be
        addq.w  #0x1,d0                         | +0c6
        addi.w  #0x10,d1                        | +0c8
        movem.w d0-d1,0x3c0000.l                | +0cc
        addq.w  #0x1,d0                         | +0d4
        addi.w  #0x10,d1                        | +0d6
        movem.w d0-d1,0x3c0000.l                | +0da
        addq.w  #0x1,d0                         | +0e2
        addi.w  #0x10,d1                        | +0e4
        movem.w d0-d1,0x3c0000.l                | +0e8
        addi.w  #0x1d,d0                        | +0f0
        subi.w  #0x2f,d1                        | +0f4
.L046f6a:
        bra.b   .L046f0a                        | +0f8
.L046f6c:
        cmpi.w  #0x74ff,d0                      | +0fa
        bls.w   .L046f76                        | +0fe
        rts                                     | +102
.L046f76:
        cmpi.w  #0x7000,d0                      | +104
        bcc.w   .L046f80                        | +108
        rts                                     | +10c
.L046f80:
        move.w  #0x2320,d1                      | +10e
        movem.w d0-d1,0x3c0000.l                | +112
        addq.w  #0x1,d0                         | +11a
        movem.w d0-d1,0x3c0000.l                | +11c
        addq.w  #0x1,d0                         | +124
        movem.w d0-d1,0x3c0000.l                | +126
        addq.w  #0x1,d0                         | +12e
        movem.w d0-d1,0x3c0000.l                | +130
        addi.w  #0x1d,d0                        | +138
        movem.w d0-d1,0x3c0000.l                | +13c
        addq.w  #0x1,d0                         | +144
        movem.w d0-d1,0x3c0000.l                | +146
        addq.w  #0x1,d0                         | +14e
        movem.w d0-d1,0x3c0000.l                | +150
        addq.w  #0x1,d0                         | +158
        movem.w d0-d1,0x3c0000.l                | +15a
        rts                                     | +162

| ----------------------------------------------------------------------------
|  MissionNumBanner_Task_046fd6  @ $046FD6  (92 B)
| ----------------------------------------------------------------------------
        .section .text.MissionNumBanner_Task_046fd6, "ax", @progbits
        .global MissionNumBanner_Task_046fd6
MissionNumBanner_Task_046fd6:
        clr.w   d0                              | +000
        move.b  0x106ecf.l,d0                   | +002
        cmpi.w  #0x6,d0                         | +008
        bls.w   .L046fea                        | +00c
        move.w  #0x5,d0                         | +010
.L046fea:
        move.w  d0,0x5c(a6)                     | +014
        move.w  #0xffe9,0x22(a6)                | +018
        move.w  #0x9,0x24(a6)                   | +01e
        move.w  #0x2,0x28(a6)                   | +024
        lea     .L047006(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L047006:
        move.w  0x28(a6),d0                     | +030
        add.w   d0,0x22(a6)                     | +034
        move.w  0x22(a6),d2                     | +038
        move.w  0x24(a6),d3                     | +03c
        move.w  0x14(a6),d4                     | +040
        move.w  0x5c(a6),d5                     | +044
        jsr     FixBlit_BatchRow4x2_046B20(pc)  | +048
        cmpi.w  #0x7,0x22(a6)                   | +04c
        blt.w   SetHandlerRts_047038            | +052
        move.w  #0x78,0x70(a6)                  | +056

| ----------------------------------------------------------------------------
|  MissionNumBanner_WaitDismiss_04703a  @ $04703A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.MissionNumBanner_WaitDismiss_04703a, "ax", @progbits
        .global MissionNumBanner_WaitDismiss_04703a
MissionNumBanner_WaitDismiss_04703a:
        movea.l 0xc(a6),a1                      | +000
        cmpi.b  #0x0,0x20(a1)                   | +004
        beq.w   SetHandlerRts_04704e            | +00a

| ----------------------------------------------------------------------------
|  MissionNumBanner_ScrollOut_047050  @ $047050  (46 B)
| ----------------------------------------------------------------------------
        .section .text.MissionNumBanner_ScrollOut_047050, "ax", @progbits
        .global MissionNumBanner_ScrollOut_047050
MissionNumBanner_ScrollOut_047050:
        move.w  0x28(a6),d0                     | +000
        add.w   d0,0x22(a6)                     | +004
        move.w  0x22(a6),d2                     | +008
        move.w  0x24(a6),d3                     | +00c
        move.w  0x14(a6),d4                     | +010
        move.w  0x5c(a6),d5                     | +014
        jsr     FixBlit_BatchRow4x2_046B20(pc)  | +018
        cmpi.w  #0x27,0x22(a6)                  | +01c
        ble.w   .L04707c                        | +022
        jmp     0x518.l                         | +026
.L04707c:
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  MissionNum_TileByMission_04707e  @ $04707E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.MissionNum_TileByMission_04707e, "ax", @progbits
        .global MissionNum_TileByMission_04707e
MissionNum_TileByMission_04707e:
        clr.l   d1                              | +000
        move.b  0x106ecf.l,d1                   | +002
        andi.w  #0x7,d1                         | +008
        lsl.w   #0x1,d1                         | +00c
        lea     0x28df00.l,a1                   | +00e
        move.w  (a1,d1.w),d0                    | +014

| ----------------------------------------------------------------------------
|  MissionNum_PalByMission_04709e  @ $04709E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.MissionNum_PalByMission_04709e, "ax", @progbits
        .global MissionNum_PalByMission_04709e
MissionNum_PalByMission_04709e:
        clr.l   d1                              | +000
        move.b  0x106ecf.l,d1                   | +002
        andi.w  #0x7,d1                         | +008
        lsl.w   #0x1,d1                         | +00c
        lea     0x28df10.l,a1                   | +00e
        move.w  (a1,d1.w),d0                    | +014

| ----------------------------------------------------------------------------
|  MissionStart_Task_0470be  @ $0470BE  (96 B)
| ----------------------------------------------------------------------------
        .section .text.MissionStart_Task_0470be, "ax", @progbits
        .global MissionStart_Task_0470be
MissionStart_Task_0470be:
        clr.w   d0                              | +000
        move.w  d0,0x14(a6)                     | +002
        lea     MissionNumBanner_Task_046fd6(pc),a1 | +006
        jsr     0x4ae.l                         | +00a
        move.w  0x14(a6),0x14(a0)               | +010
        clr.b   0x20(a6)                        | +016
        move.w  #0x27,0x22(a6)                  | +01a
        move.w  #0xf,0x24(a6)                   | +020
        move.w  #0xfffe,0x28(a6)                | +026
        lea     .L0470f0(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L0470f0:
        move.w  0x28(a6),d0                     | +032
        add.w   d0,0x22(a6)                     | +036
        move.w  0x22(a6),d2                     | +03a
        move.w  0x24(a6),d3                     | +03e
        move.w  0x14(a6),d4                     | +042
        jsr     FixBanner_MissionStart_Blit_046d0e(pc) | +046
        cmpi.w  #0xb,0x22(a6)                   | +04a
        bgt.w   JsrPcRts_047122                 | +050
        move.w  #0x1e,0x70(a6)                  | +054
        lea     MissionStart_Pause_047124(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e

| ----------------------------------------------------------------------------
|  MissionStart_Pause_047124  @ $047124  (26 B)
| ----------------------------------------------------------------------------
        .section .text.MissionStart_Pause_047124, "ax", @progbits
        .global MissionStart_Pause_047124
MissionStart_Pause_047124:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   SetHandlerRts_047144            | +00a
        move.w  #0xa,0x70(a6)                   | +00e
        move.w  #0x2,0x5c(a6)                   | +014

| ----------------------------------------------------------------------------
|  MissionStart_Clear_047146  @ $047146  (60 B)
| ----------------------------------------------------------------------------
        .section .text.MissionStart_Clear_047146, "ax", @progbits
        .global MissionStart_Clear_047146
MissionStart_Clear_047146:
        move.w  0x22(a6),d2                     | +000
        movea.w #0x7000,a1                      | +004
        asl.w   #0x5,d2                         | +008
        adda.w  d2,a1                           | +00a
        adda.w  0x24(a6),a1                     | +00c
        move.w  #0x2320,d0                      | +010
        move.w  #0x12,d1                        | +014
        move.w  #0x4,d2                         | +018
        jsr     0x5da9c.l                       | +01c
        move.w  #0xa,0x70(a6)                   | +022
        lea     .L047174(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L047174:
        subq.w  #0x1,0x70(a6)                   | +02e
        cmpi.w  #0x0,0x70(a6)                   | +032
        bgt.w   SetHandlerRts_047188            | +038

| ----------------------------------------------------------------------------
|  MissionStart_Redraw_04718a  @ $04718A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.MissionStart_Redraw_04718a, "ax", @progbits
        .global MissionStart_Redraw_04718a
MissionStart_Redraw_04718a:
        move.w  0x22(a6),d2                     | +000
        move.w  0x24(a6),d3                     | +004
        move.w  0x14(a6),d4                     | +008
        jsr     FixBanner_MissionStart_Blit_046d0e(pc) | +00c
        move.w  #0xa,0x70(a6)                   | +010
        lea     .L0471a6(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0471a6:
        subq.w  #0x1,0x70(a6)                   | +01c
        cmpi.w  #0x0,0x70(a6)                   | +020
        bgt.w   SetHandlerRts_0471d8            | +026
        subq.w  #0x1,0x5c(a6)                   | +02a
        cmpi.w  #0x0,0x5c(a6)                   | +02e
        bgt.w   SetTaskHandler_0471d2           | +034
        move.b  #0x1,0x20(a6)                   | +038
        lea     MissionStart_ScrollOut_0471da(pc),a1 | +03e
        move.l  a1,(a6)                         | +042
        bra.w   SetHandlerRts_0471d8            | +044

| ----------------------------------------------------------------------------
|  MissionStart_ScrollOut_0471da  @ $0471DA  (50 B)
| ----------------------------------------------------------------------------
        .section .text.MissionStart_ScrollOut_0471da, "ax", @progbits
        .global MissionStart_ScrollOut_0471da
MissionStart_ScrollOut_0471da:
        move.w  0x28(a6),d0                     | +000
        add.w   d0,0x22(a6)                     | +004
        move.w  0x22(a6),d2                     | +008
        move.w  0x24(a6),d3                     | +00c
        move.w  0x14(a6),d4                     | +010
        jsr     FixBanner_MissionStart_Blit_046d0e(pc) | +014
        cmpi.w  #0xffee,0x22(a6)                | +018
        bgt.w   .L04720a                        | +01e
        movea.l 0xc(a6),a1                      | +022
        clr.b   0x21(a1)                        | +026
        jmp     0x518.l                         | +02a
.L04720a:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  MissionComplete_Task_04720c  @ $04720C  (100 B)
| ----------------------------------------------------------------------------
        .section .text.MissionComplete_Task_04720c, "ax", @progbits
        .global MissionComplete_Task_04720c
MissionComplete_Task_04720c:
        move.w  #0x20,d0                        | +000
        jsr     0x2352.l                        | +004
        clr.w   d0                              | +00a
        move.w  d0,0x14(a6)                     | +00c
        lea     MissionNumBanner_Task_046fd6(pc),a1 | +010
        jsr     0x4ae.l                         | +014
        move.w  0x14(a6),0x14(a0)               | +01a
        clr.b   0x20(a6)                        | +020
        move.w  #0x26,0x22(a6)                  | +024
        move.w  #0xf,0x24(a6)                   | +02a
        move.w  #0xfffe,0x28(a6)                | +030
        lea     .L047248(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L047248:
        move.w  0x28(a6),d0                     | +03c
        add.w   d0,0x22(a6)                     | +040
        move.w  0x22(a6),d2                     | +044
        move.w  0x24(a6),d3                     | +048
        move.w  0x14(a6),d4                     | +04c
        jsr     FixBanner_MissionComplete_Blit_046e72(pc) | +050
        cmpi.w  #0x6,0x22(a6)                   | +054
        bgt.w   SetHandlerRts_047276            | +05a
        move.w  #0x5a,0x70(a6)                  | +05e

| ----------------------------------------------------------------------------
|  MissionComplete_Pause_047278  @ $047278  (14 B)
| ----------------------------------------------------------------------------
        .section .text.MissionComplete_Pause_047278, "ax", @progbits
        .global MissionComplete_Pause_047278
MissionComplete_Pause_047278:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   SetHandlerRts_04728c            | +00a

| ----------------------------------------------------------------------------
|  MissionComplete_Clear_04728e  @ $04728E  (60 B)
| ----------------------------------------------------------------------------
        .section .text.MissionComplete_Clear_04728e, "ax", @progbits
        .global MissionComplete_Clear_04728e
MissionComplete_Clear_04728e:
        move.w  0x22(a6),d2                     | +000
        movea.w #0x7000,a1                      | +004
        asl.w   #0x5,d2                         | +008
        adda.w  d2,a1                           | +00a
        adda.w  0x24(a6),a1                     | +00c
        move.w  #0x2320,d0                      | +010
        move.w  #0x1c,d1                        | +014
        move.w  #0x4,d2                         | +018
        jsr     0x5da9c.l                       | +01c
        move.w  #0xa,0x70(a6)                   | +022
        lea     .L0472bc(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L0472bc:
        subq.w  #0x1,0x70(a6)                   | +02e
        cmpi.w  #0x0,0x70(a6)                   | +032
        bgt.w   SetHandlerRts_0472d0            | +038

| ----------------------------------------------------------------------------
|  MissionComplete_Redraw_0472d2  @ $0472D2  (66 B)
| ----------------------------------------------------------------------------
        .section .text.MissionComplete_Redraw_0472d2, "ax", @progbits
        .global MissionComplete_Redraw_0472d2
MissionComplete_Redraw_0472d2:
        move.w  0x22(a6),d2                     | +000
        move.w  0x24(a6),d3                     | +004
        move.w  0x14(a6),d4                     | +008
        jsr     FixBanner_MissionComplete_Blit_046e72(pc) | +00c
        move.w  #0xa,0x70(a6)                   | +010
        lea     .L0472ee(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0472ee:
        subq.w  #0x1,0x70(a6)                   | +01c
        cmpi.w  #0x0,0x70(a6)                   | +020
        bgt.w   SetHandlerRts_04731a            | +026
        subq.w  #0x1,0x5c(a6)                   | +02a
        cmpi.w  #0x0,0x5c(a6)                   | +02e
        bgt.w   SetTaskHandler_047314           | +034
        lea     MissionComplete_ScrollOut_04731c(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
        bra.w   SetHandlerRts_04731a            | +03e

| ----------------------------------------------------------------------------
|  MissionComplete_ScrollOut_04731c  @ $04731C  (62 B)
| ----------------------------------------------------------------------------
        .section .text.MissionComplete_ScrollOut_04731c, "ax", @progbits
        .global MissionComplete_ScrollOut_04731c
MissionComplete_ScrollOut_04731c:
        move.b  #0x1,0x20(a6)                   | +000
        lea     .L047328(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L047328:
        move.w  0x28(a6),d0                     | +00c
        add.w   d0,0x22(a6)                     | +010
        move.w  0x22(a6),d2                     | +014
        move.w  0x24(a6),d3                     | +018
        move.w  0x14(a6),d4                     | +01c
        jsr     FixBanner_MissionComplete_Blit_046e72(pc) | +020
        cmpi.w  #0xffe4,0x22(a6)                | +024
        bgt.w   SetHandlerRts_047360            | +02a
        move.w  #0x3,d0                         | +02e
        jsr     0x5239e.l                       | +032
        move.w  #0x32,0x70(a6)                  | +038

| ----------------------------------------------------------------------------
|  MissionComplete_Finish_047362  @ $047362  (28 B)
| ----------------------------------------------------------------------------
        .section .text.MissionComplete_Finish_047362, "ax", @progbits
        .global MissionComplete_Finish_047362
MissionComplete_Finish_047362:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L04737c                        | +00a
        clr.b   0x106ed2.l                      | +00e
        jmp     0x518.l                         | +014
.L04737c:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  BigText_Typewriter_04737e  @ $04737E  (48 B)
| ----------------------------------------------------------------------------
        .section .text.BigText_Typewriter_04737e, "ax", @progbits
        .global BigText_Typewriter_04737e
BigText_Typewriter_04737e:
        move.w  0x30(a6),0x5c(a6)               | +000
        lea     .L04738a(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L04738a:
        cmpi.w  #0x0,0x30(a6)                   | +00c
        ble.w   .L04739a                        | +012
        subq.w  #0x1,0x30(a6)                   | +016
        rts                                     | +01a
.L04739a:
        move.w  0x5c(a6),0x30(a6)               | +01c
        movea.l 0x3c(a6),a1                     | +022
        move.b  (a1),d0                         | +026
        cmpi.b  #0xff,d0                        | +028
        bne.w   BigText_Typewriter_CheckVramLo_0473bc | +02c

| ----------------------------------------------------------------------------
|  BigText_Typewriter_CheckVramLo_0473bc  @ $0473BC  (12 B)
| ----------------------------------------------------------------------------
        .section .text.BigText_Typewriter_CheckVramLo_0473bc, "ax", @progbits
        .global BigText_Typewriter_CheckVramLo_0473bc
BigText_Typewriter_CheckVramLo_0473bc:
        movea.w 0x22(a6),a1                     | +000
        cmpa.w  #0x7000,a1                      | +004
        bcc.w   BigText_Typewriter_CheckVramHi_0473d6 | +008

| ----------------------------------------------------------------------------
|  BigText_Typewriter_CheckVramHi_0473d6  @ $0473D6  (8 B)
| ----------------------------------------------------------------------------
        .section .text.BigText_Typewriter_CheckVramHi_0473d6, "ax", @progbits
        .global BigText_Typewriter_CheckVramHi_0473d6
BigText_Typewriter_CheckVramHi_0473d6:
        cmpa.w  #0x74ff,a1                      | +000
        bls.w   BigText_Typewriter_PutGlyph_0473ec | +004

| ----------------------------------------------------------------------------
|  BigText_Typewriter_PutGlyph_0473ec  @ $0473EC  (20 B)
| ----------------------------------------------------------------------------
        .section .text.BigText_Typewriter_PutGlyph_0473ec, "ax", @progbits
        .global BigText_Typewriter_PutGlyph_0473ec
BigText_Typewriter_PutGlyph_0473ec:
        move.b  0x16(a6),d1                     | +000
        jsr     Font_BigGlyphToTile_047822(pc)  | +004
        addq.l  #0x1,0x3c(a6)                   | +008
        addi.w  #0x40,0x22(a6)                  | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  SmallText_Typewriter_047400  @ $047400  (48 B)
| ----------------------------------------------------------------------------
        .section .text.SmallText_Typewriter_047400, "ax", @progbits
        .global SmallText_Typewriter_047400
SmallText_Typewriter_047400:
        move.w  0x30(a6),0x5c(a6)               | +000
        lea     .L04740c(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L04740c:
        cmpi.w  #0x0,0x30(a6)                   | +00c
        ble.w   .L04741c                        | +012
        subq.w  #0x1,0x30(a6)                   | +016
        rts                                     | +01a
.L04741c:
        move.w  0x5c(a6),0x30(a6)               | +01c
        movea.l 0x3c(a6),a1                     | +022
        move.b  (a1),d0                         | +026
        cmpi.b  #0xff,d0                        | +028
        bne.w   SmallText_Typewriter_CheckVramLo_04743e | +02c

| ----------------------------------------------------------------------------
|  SmallText_Typewriter_CheckVramLo_04743e  @ $04743E  (12 B)
| ----------------------------------------------------------------------------
        .section .text.SmallText_Typewriter_CheckVramLo_04743e, "ax", @progbits
        .global SmallText_Typewriter_CheckVramLo_04743e
SmallText_Typewriter_CheckVramLo_04743e:
        movea.w 0x22(a6),a1                     | +000
        cmpa.w  #0x7000,a1                      | +004
        bcc.w   SmallText_Typewriter_CheckVramHi_047458 | +008

| ----------------------------------------------------------------------------
|  SmallText_Typewriter_CheckVramHi_047458  @ $047458  (8 B)
| ----------------------------------------------------------------------------
        .section .text.SmallText_Typewriter_CheckVramHi_047458, "ax", @progbits
        .global SmallText_Typewriter_CheckVramHi_047458
SmallText_Typewriter_CheckVramHi_047458:
        cmpa.w  #0x74ff,a1                      | +000
        bls.w   SmallText_Typewriter_PutGlyph_04746e | +004

| ----------------------------------------------------------------------------
|  SmallText_Typewriter_PutGlyph_04746e  @ $04746E  (20 B)
| ----------------------------------------------------------------------------
        .section .text.SmallText_Typewriter_PutGlyph_04746e, "ax", @progbits
        .global SmallText_Typewriter_PutGlyph_04746e
SmallText_Typewriter_PutGlyph_04746e:
        move.b  0x16(a6),d1                     | +000
        jsr     Font_SmallGlyphToTile_0477d4(pc) | +004
        addq.l  #0x1,0x3c(a6)                   | +008
        addi.w  #0x20,0x22(a6)                  | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Fix_Clear5Tiles_723C_047676  @ $047676  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_Clear5Tiles_723C_047676, "ax", @progbits
        .global Fix_Clear5Tiles_723C_047676
Fix_Clear5Tiles_723C_047676:
        movea.w #0x723c,a1                      | +000
        move.w  #0x238b,d0                      | +004
        moveq   #5,d1                           | +008
        moveq   #1,d2                           | +00a

| ----------------------------------------------------------------------------
|  Fix_DrawBigNumber2Digit_04768a  @ $04768A  (160 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_DrawBigNumber2Digit_04768a, "ax", @progbits
        .global Fix_DrawBigNumber2Digit_04768a
Fix_DrawBigNumber2Digit_04768a:
        cmpi.w  #0x63,d0                        | +000
        bls.w   .L047696                        | +004
        move.w  #0x63,d0                        | +008
.L047696:
        jsr     Sub_Divide10_047656(pc)         | +00c
        lea     0x28dee0.l,a1                   | +010
        lsl.w   #0x1,d0                         | +016
        move.w  (a1,d0.w),d2                    | +018
        lsl.w   #0x1,d1                         | +01c
        move.w  (a1,d1.w),d3                    | +01e
        move.w  d4,d0                           | +022
        move.w  d3,d1                           | +024
        movem.w d0-d1,0x3c0000.l                | +026
        addi.w  #0x20,d0                        | +02e
        addq.w  #0x1,d1                         | +032
        movem.w d0-d1,0x3c0000.l                | +034
        subi.w  #0x1f,d0                        | +03c
        addi.w  #0xf,d1                         | +040
        movem.w d0-d1,0x3c0000.l                | +044
        addi.w  #0x20,d0                        | +04c
        addq.w  #0x1,d1                         | +050
        movem.w d0-d1,0x3c0000.l                | +052
        subi.w  #0x21,d0                        | +05a
        move.w  d4,d0                           | +05e
        addi.w  #0x40,d0                        | +060
        move.w  d2,d1                           | +064
        movem.w d0-d1,0x3c0000.l                | +066
        addi.w  #0x20,d0                        | +06e
        addq.w  #0x1,d1                         | +072
        movem.w d0-d1,0x3c0000.l                | +074
        subi.w  #0x1f,d0                        | +07c
        addi.w  #0xf,d1                         | +080
        movem.w d0-d1,0x3c0000.l                | +084
        addi.w  #0x20,d0                        | +08c
        addq.w  #0x1,d1                         | +090
        movem.w d0-d1,0x3c0000.l                | +092
        subi.w  #0x21,d0                        | +09a
        rts                                     | +09e

| ----------------------------------------------------------------------------
|  Fix_DrawBigNumber2Digit_NoLeadZero_04772a  @ $04772A  (170 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_DrawBigNumber2Digit_NoLeadZero_04772a, "ax", @progbits
        .global Fix_DrawBigNumber2Digit_NoLeadZero_04772a
Fix_DrawBigNumber2Digit_NoLeadZero_04772a:
        cmpi.w  #0x63,d0                        | +000
        bls.w   .L047736                        | +004
        move.w  #0x63,d0                        | +008
.L047736:
        jsr     Sub_Divide10_047656(pc)         | +00c
        lea     0x28dee0.l,a1                   | +010
        lsl.w   #0x1,d0                         | +016
        move.w  (a1,d0.w),d2                    | +018
        lsl.w   #0x1,d1                         | +01c
        move.w  (a1,d1.w),d3                    | +01e
        cmp.w   0x28dee0.l,d3                   | +022
        beq.w   .L047792                        | +028
        move.w  d4,d0                           | +02c
        move.w  d3,d1                           | +02e
        movem.w d0-d1,0x3c0000.l                | +030
        addi.w  #0x20,d0                        | +038
        addq.w  #0x1,d1                         | +03c
        movem.w d0-d1,0x3c0000.l                | +03e
        subi.w  #0x1f,d0                        | +046
        addi.w  #0xf,d1                         | +04a
        movem.w d0-d1,0x3c0000.l                | +04e
        addi.w  #0x20,d0                        | +056
        addq.w  #0x1,d1                         | +05a
        movem.w d0-d1,0x3c0000.l                | +05c
        subi.w  #0x21,d0                        | +064
.L047792:
        move.w  d4,d0                           | +068
        addi.w  #0x40,d0                        | +06a
        move.w  d2,d1                           | +06e
        movem.w d0-d1,0x3c0000.l                | +070
        addi.w  #0x20,d0                        | +078
        addq.w  #0x1,d1                         | +07c
        movem.w d0-d1,0x3c0000.l                | +07e
        subi.w  #0x1f,d0                        | +086
        addi.w  #0xf,d1                         | +08a
        movem.w d0-d1,0x3c0000.l                | +08e
        addi.w  #0x20,d0                        | +096
        addq.w  #0x1,d1                         | +09a
        movem.w d0-d1,0x3c0000.l                | +09c
        subi.w  #0x21,d0                        | +0a4
        rts                                     | +0a8

| ----------------------------------------------------------------------------
|  Font_SmallGlyphToTile_0477d4  @ $0477D4  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Font_SmallGlyphToTile_0477d4, "ax", @progbits
        .global Font_SmallGlyphToTile_0477d4
Font_SmallGlyphToTile_0477d4:
        move.w  d0,d2                           | +000
        andi.w  #0xf0,d2                        | +002
        asl.w   #0x1,d2                         | +006
        andi.w  #0xf,d0                         | +008
        addi.w  #0x400,d0                       | +00c
        add.w   d2,d0                           | +010
        asl.w   #0x8,d1                         | +012
        asl.w   #0x4,d1                         | +014
        or.w    d1,d0                           | +016
        move.w  #0x1,d1                         | +018
        move.w  #0x2,d2                         | +01c

| ----------------------------------------------------------------------------
|  Font_BigGlyphToTile_047822  @ $047822  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Font_BigGlyphToTile_047822, "ax", @progbits
        .global Font_BigGlyphToTile_047822
Font_BigGlyphToTile_047822:
        move.w  d0,d2                           | +000
        andi.w  #0x8,d0                         | +002
        asl.w   #0x5,d0                         | +006
        addi.w  #0xb00,d0                       | +008
        andi.w  #0x77,d2                        | +00c
        asl.w   #0x1,d2                         | +010
        add.w   d2,d0                           | +012
        asl.w   #0x8,d1                         | +014
        asl.w   #0x4,d1                         | +016
        or.w    d1,d0                           | +018
        move.w  #0x2,d1                         | +01a
        move.w  #0x2,d2                         | +01e

| ----------------------------------------------------------------------------
|  Font_TileWithPal_047872  @ $047872  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Font_TileWithPal_047872, "ax", @progbits
        .global Font_TileWithPal_047872
Font_TileWithPal_047872:
        asl.w   #0x8,d1                         | +000
        asl.w   #0x4,d1                         | +002
        or.w    d1,d0                           | +004
        move.w  #0x2,d1                         | +006
        move.w  #0x2,d2                         | +00a

| ----------------------------------------------------------------------------
|  Fix_PutString_PalByHighBit_0478ae  @ $0478AE  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_PutString_PalByHighBit_0478ae, "ax", @progbits
        .global Fix_PutString_PalByHighBit_0478ae
Fix_PutString_PalByHighBit_0478ae:
        clr.w   d1                              | +000
        move.b  (a1)+,d1                        | +002
        cmpi.b  #0xff,d1                        | +004
        beq.w   .L0478de                        | +008
        cmpi.b  #0x80,d1                        | +00c
        bcc.w   .L0478ca                        | +010
        ori.w   #0x2300,d1                      | +014
        bra.w   .L0478ce                        | +018
.L0478ca:
        ori.w   #0x2a00,d1                      | +01c
.L0478ce:
        movem.w d0-d1,0x3c0000.l                | +020
        addi.l  #0x20,d0                        | +028
        bra.b   Fix_PutString_PalByHighBit_0478ae | +02e
.L0478de:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_0478e0  @ $0478E0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_0478e0, "ax", @progbits
        .global Entity_CmpDepthToParent_0478e0
Entity_CmpDepthToParent_0478e0:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0478f6                    | +00c
