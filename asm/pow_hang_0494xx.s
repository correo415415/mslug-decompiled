| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $049430..$049FC4  (2,880 B, 25 entradas, 9 huecos)
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
|  PowHang_SpawnItemAndFxAtSprite_049430  @ $049430  (66 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_SpawnItemAndFxAtSprite_049430, "ax", @progbits
        .global PowHang_SpawnItemAndFxAtSprite_049430
PowHang_SpawnItemAndFxAtSprite_049430:
        jsr     0x5dd02.l                       | +000
        movea.l 0x5c(a6),a1                     | +006
        move.w  (a1),d0                         | +00a
        move.w  0x2(a1),d1                      | +00c
        add.w   d0,0x22(a0)                     | +010
        add.w   d1,0x24(a0)                     | +014
        lea     PowItem_Toss_048ba0__L048bca(pc),a1 | +018
        jsr     0x4ae.l                         | +01c
        jsr     0x5dd02.l                       | +022
        movea.l 0x5c(a6),a1                     | +028
        move.w  (a1),d0                         | +02c
        move.w  0x2(a1),d1                      | +02e
        add.w   d0,0x22(a0)                     | +032
        add.w   d1,0x24(a0)                     | +036
        move.b  0x7b(a6),0x7b(a0)               | +03a
        rts                                     | +040

| ----------------------------------------------------------------------------
|  PowHang_SpawnFxAndItemA_049472  @ $049472  (86 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_SpawnFxAndItemA_049472, "ax", @progbits
        .global PowHang_SpawnFxAndItemA_049472
PowHang_SpawnFxAndItemA_049472:
        lea     PowFx_DirSprite_048b56(pc),a1   | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0x20,d0                        | +010
        btst    #0x0,0x3a(a6)                   | +014
        bne.w   .L049492                        | +01a
        neg.w   d0                              | +01e
.L049492:
        add.w   d0,0x22(a0)                     | +020
        addi.w  #0xe,0x24(a0)                   | +024
        lea     PowItem_Toss_048ba0(pc),a1      | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        move.w  #0x20,d0                        | +03a
        btst    #0x0,0x3a(a6)                   | +03e
        bne.w   .L0494bc                        | +044
        neg.w   d0                              | +048
.L0494bc:
        add.w   d0,0x22(a0)                     | +04a
        addi.w  #0xe,0x24(a0)                   | +04e
        rts                                     | +054

| ----------------------------------------------------------------------------
|  PowHang_SpawnFxAndItemB_0494c8  @ $0494C8  (86 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_SpawnFxAndItemB_0494c8, "ax", @progbits
        .global PowHang_SpawnFxAndItemB_0494c8
PowHang_SpawnFxAndItemB_0494c8:
        lea     PowFx_DirSprite_048b56(pc),a1   | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0x1e,d0                        | +010
        btst    #0x0,0x3a(a6)                   | +014
        bne.w   .L0494e8                        | +01a
        neg.w   d0                              | +01e
.L0494e8:
        add.w   d0,0x22(a0)                     | +020
        addi.w  #0x22,0x24(a0)                  | +024
        lea     PowItem_Toss_048ba0(pc),a1      | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        move.w  #0x1e,d0                        | +03a
        btst    #0x0,0x3a(a6)                   | +03e
        bne.w   .L049512                        | +044
        neg.w   d0                              | +048
.L049512:
        add.w   d0,0x22(a0)                     | +04a
        addi.w  #0x22,0x24(a0)                  | +04e
        rts                                     | +054

| ----------------------------------------------------------------------------
|  PowHang_LoadTimerAndSlotCheck_04951e  @ $04951E  (32 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_LoadTimerAndSlotCheck_04951e, "ax", @progbits
        .global PowHang_LoadTimerAndSlotCheck_04951e
PowHang_LoadTimerAndSlotCheck_04951e:
        lea     0x2bfcb4.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        movea.l 0x8(a6),a1                      | +010
        move.b  0x10(a6),d0                     | +014
        cmp.b   0x10(a1),d0                     | +018
        bcs.w   SetXN_049544                    | +01c

| ----------------------------------------------------------------------------
|  PowHang_SpawnVariants_04954a  @ $04954A  (140 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_SpawnVariants_04954a, "ax", @progbits
        .global PowHang_SpawnVariants_04954a
PowHang_SpawnVariants_04954a:
        move.b  #0x0,0x7f(a6)                   | +000
        bra.w   .L04955a                        | +006
        move.b  #0x1,0x7f(a6)                   | +00a
.L04955a:
        move.w  #0x38,d1                        | +010
        jsr     0x236e.l                        | +014
        move.w  #0x1,0x66(a6)                   | +01a
        move.b  #0x3,0x70(a6)                   | +020
        move.w  #0xc,0x34(a6)                   | +026
        move.w  #0x8000,0x38(a6)                | +02c
        jsr     0x267e2.l                       | +032
        jsr     0x27cee.l                       | +038
        lea     0x28ff2a.l,a0                   | +03e
        move.l  a0,0x48(a6)                     | +044
        lea     0x2bff0c.l,a0                   | +048
        jsr     0x799de.l                       | +04e
        move.w  d0,0x76(a6)                     | +054
        lea     0x2bff8e.l,a0                   | +058
        jsr     0x799de.l                       | +05e
        move.w  d0,0x7c(a6)                     | +064
        clr.w   d0                              | +068
        move.b  0x9e(a6),d0                     | +06a
        ext.w   d0                              | +06e
        asl.w   #0x4,d0                         | +070
        move.w  d0,0x7a(a6)                     | +072
        move.b  #0x0,0x79(a6)                   | +076
        lea     PowHang_RopeSpawn_0497ac(pc),a1 | +07c
        jsr     0x4ae.l                         | +080
        jsr     0x5dd02.l                       | +086

| ----------------------------------------------------------------------------
|  PowHang_Swing_0495d6  @ $0495D6  (180 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_Swing_0495d6, "ax", @progbits
        .global PowHang_Swing_0495d6
PowHang_Swing_0495d6:
        clr.l   d0                              | +000
        move.w  0x34(a6),d0                     | +002
        andi.w  #0xf,d0                         | +006
        movea.l #0x28f708,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L0495fc                        | +01c
        jsr     0x28cd4.l                       | +020
.L0495fc:
        move.b  #0x0,0x78(a6)                   | +026
        lea     .L049608(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L049608:
        jsr     PowHang_SwingStep_0499dc(pc)    | +032
        jsr     PowHang_SwingAnimByAngle_049a20(pc) | +036
        jsr     PowHang_HitCheck_049b58(pc)     | +03a
        bcc.w   .L049622                        | +03e
        move.b  #0xfd,0x78(a6)                  | +042
        bra.w   .L04962c                        | +048
.L049622:
        cmpi.b  #0xff,0x79(a6)                  | +04c
        bne.w   .L04964c                        | +052
.L04962c:
        move.w  #0xc,d0                         | +056
        sub.w   0x34(a6),d0                     | +05a
        asr.w   #0x8,d0                         | +05e
        asr.w   #0x7,d0                         | +060
        andi.b  #0x1,d0                         | +062
        move.b  d0,0x3a(a6)                     | +066
        lea     0x4797c.l,a1                    | +06a
        move.l  a1,(a6)                         | +070
        bra.w   .L049688                        | +072
.L04964c:
        jsr     PowHang_DropFxTimer_049b06(pc)  | +076
        jsr     0x2870a.l                       | +07a
        bcc.w   .L049666                        | +080
        move.b  #0xff,0x78(a6)                  | +084
        lea     PowHang_Freed_049742(pc),a1     | +08a
        move.l  a1,(a6)                         | +08e
.L049666:
        movea.l #0xffffffff,a0                  | +090
        lea     0x28ffd2.l,a0                   | +096
        jsr     0x5dd5c.l                       | +09c
        bcc.w   .L049688                        | +0a2
        move.b  #0xfe,0x78(a6)                  | +0a6
        jmp     0x518.l                         | +0ac
.L049688:
        rts                                     | +0b2

| ----------------------------------------------------------------------------
|  PowHang_Struggle_04968a  @ $04968A  (184 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_Struggle_04968a, "ax", @progbits
        .global PowHang_Struggle_04968a
PowHang_Struggle_04968a:
        clr.l   d0                              | +000
        move.b  0x71(a6),d0                     | +002
        movea.l #0x28f838,a0                    | +006
        lsl.w   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a0                    | +00e
        cmpa.l  #0xffffffff,a0                  | +012
        beq.w   .L0496ac                        | +018
        jsr     0x28cd4.l                       | +01c
.L0496ac:
        move.b  #0x1,0x78(a6)                   | +022
        lea     .L0496b8(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L0496b8:
        jsr     PowHang_SwingStep_0499dc(pc)    | +02e
        jsr     0x28d70.l                       | +032
        bcc.w   .L0496cc                        | +038
        lea     PowHang_Swing_0495d6(pc),a1     | +03c
        move.l  a1,(a6)                         | +040
.L0496cc:
        jsr     PowHang_HitCheck_049b58(pc)     | +042
        bcc.w   .L0496de                        | +046
        move.b  #0xfd,0x78(a6)                  | +04a
        bra.w   .L0496e8                        | +050
.L0496de:
        cmpi.b  #0xff,0x79(a6)                  | +054
        bne.w   .L049708                        | +05a
.L0496e8:
        move.w  #0xc,d0                         | +05e
        sub.w   0x34(a6),d0                     | +062
        asr.w   #0x8,d0                         | +066
        asr.w   #0x7,d0                         | +068
        andi.b  #0x1,d0                         | +06a
        move.b  d0,0x3a(a6)                     | +06e
        lea     0x4797c.l,a1                    | +072
        move.l  a1,(a6)                         | +078
        bra.w   .L049740                        | +07a
.L049708:
        jsr     0x2870a.l                       | +07e
        bcc.w   .L04971e                        | +084
        move.b  #0xff,0x78(a6)                  | +088
        lea     PowHang_Freed_049742(pc),a1     | +08e
        move.l  a1,(a6)                         | +092
.L04971e:
        movea.l #0xffffffff,a0                  | +094
        lea     0x28ffd2.l,a0                   | +09a
        jsr     0x5dd5c.l                       | +0a0
        bcc.w   .L049740                        | +0a6
        move.b  #0xfe,0x78(a6)                  | +0aa
        jmp     0x518.l                         | +0b0
.L049740:
        rts                                     | +0b6

| ----------------------------------------------------------------------------
|  PowHang_Freed_049742  @ $049742  (106 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_Freed_049742, "ax", @progbits
        .global PowHang_Freed_049742
PowHang_Freed_049742:
        move.w  #0xc,d0                         | +000
        sub.w   0x34(a6),d0                     | +004
        asr.w   #0x8,d0                         | +008
        asr.w   #0x7,d0                         | +00a
        andi.b  #0x1,d0                         | +00c
        move.b  d0,0x3a(a6)                     | +010
        lea     0x4acfe.l,a0                    | +014
        jsr     0x28cd4.l                       | +01a
        lea     0x48ca4.l,a1                    | +020
        jsr     0x4ae.l                         | +026
        jsr     0x5dd02.l                       | +02c
        addi.w  #0x18,0x24(a0)                  | +032
        move.b  0x3a(a6),0x3a(a0)               | +038
        movea.l 0x50(a6),a1                     | +03e
        move.w  0x28(a1),d0                     | +042
        asr.w   #0x4,d0                         | +046
        move.w  d0,0x28(a0)                     | +048
        lea     .L049794(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L049794:
        jsr     0x2783a.l                       | +052
        jsr     0x28d70.l                       | +058
        bcc.w   .L0497aa                        | +05e
        jmp     0x518.l                         | +062
.L0497aa:
        rts                                     | +068

| ----------------------------------------------------------------------------
|  PowHang_RopeSpawn_0497ac  @ $0497AC  (36 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_RopeSpawn_0497ac, "ax", @progbits
        .global PowHang_RopeSpawn_0497ac
PowHang_RopeSpawn_0497ac:
        move.w  #0x18d,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2bfe8a.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.w  d0,0x66(a6)                     | +016
        lea     0x28ff7e.l,a0                   | +01a
        move.l  a0,0x48(a6)                     | +020

| ----------------------------------------------------------------------------
|  PowHang_RopeIdle_0497d0  @ $0497D0  (180 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_RopeIdle_0497d0, "ax", @progbits
        .global PowHang_RopeIdle_0497d0
PowHang_RopeIdle_0497d0:
        lea     .L0497d6(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L0497d6:
        jsr     0x5e4ee.l                       | +006
        subq.w  #0x1,0x38(a6)                   | +00c
        clr.l   d0                              | +010
        movea.l 0xc(a6),a0                      | +012
        move.b  0x71(a0),d0                     | +016
        movea.l #0x28f8d4,a0                    | +01a
        lsl.w   #0x2,d0                         | +020
        movea.l (a0,d0.w),a0                    | +022
        cmpa.l  #0xffffffff,a0                  | +026
        beq.w   .L049806                        | +02c
        jsr     0x28cd4.l                       | +030
.L049806:
        jsr     0x28d70.l                       | +036
        jsr     0x28758.l                       | +03c
        bcc.w   .L04982a                        | +042
        movea.l 0xc(a6),a0                      | +046
        move.b  #0xff,0x79(a0)                  | +04a
        lea     PowHang_RopeBroken_049976(pc),a1 | +050
        move.l  a1,(a6)                         | +054
        bra.w   .L049882                        | +056
.L04982a:
        movea.l 0xc(a6),a0                      | +05a
        cmpi.b  #0x1,0x78(a0)                   | +05e
        bne.w   .L049842                        | +064
        lea     PowHang_RopeStruggle_049884(pc),a1 | +068
        move.l  a1,(a6)                         | +06c
        bra.w   .L049882                        | +06e
.L049842:
        cmpi.b  #0xfd,0x78(a0)                  | +072
        bne.w   .L04985a                        | +078
        move.l  a6,0xc(a6)                      | +07c
        lea     PowHang_RopeCut_04992e(pc),a1   | +080
        move.l  a1,(a6)                         | +084
        bra.w   .L049882                        | +086
.L04985a:
        cmpi.b  #0xff,0x78(a0)                  | +08a
        bne.w   .L049872                        | +090
        move.l  a6,0xc(a6)                      | +094
        lea     PowHang_RopeBroken_049976(pc),a1 | +098
        move.l  a1,(a6)                         | +09c
        bra.w   .L049882                        | +09e
.L049872:
        cmpi.b  #0xfe,0x78(a0)                  | +0a2
        bne.w   .L049882                        | +0a8
        jmp     0x518.l                         | +0ac
.L049882:
        rts                                     | +0b2

| ----------------------------------------------------------------------------
|  PowHang_RopeStruggle_049884  @ $049884  (170 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_RopeStruggle_049884, "ax", @progbits
        .global PowHang_RopeStruggle_049884
PowHang_RopeStruggle_049884:
        clr.l   d0                              | +000
        movea.l 0xc(a6),a0                      | +002
        move.b  0x71(a0),d0                     | +006
        movea.l #0x28fdbc,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L0498aa                        | +01c
        jsr     0x28cd4.l                       | +020
.L0498aa:
        lea     .L0498b0(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L0498b0:
        jsr     0x5e4ee.l                       | +02c
        subq.w  #0x1,0x38(a6)                   | +032
        jsr     0x28d70.l                       | +036
        bcc.w   .L0498ca                        | +03c
        lea     PowHang_RopeIdle_0497d0(pc),a1  | +040
        move.l  a1,(a6)                         | +044
.L0498ca:
        jsr     0x28758.l                       | +046
        bcc.w   .L0498e8                        | +04c
        movea.l 0xc(a6),a0                      | +050
        move.b  #0xff,0x79(a0)                  | +054
        lea     PowHang_RopeBroken_049976(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
        bra.w   .L04992c                        | +060
.L0498e8:
        movea.l 0xc(a6),a0                      | +064
        cmpi.b  #0xfd,0x78(a0)                  | +068
        bne.w   .L049904                        | +06e
        move.l  a6,0xc(a6)                      | +072
        lea     PowHang_RopeCut_04992e(pc),a1   | +076
        move.l  a1,(a6)                         | +07a
        bra.w   .L04992c                        | +07c
.L049904:
        cmpi.b  #0xff,0x78(a0)                  | +080
        bne.w   .L04991c                        | +086
        move.l  a6,0xc(a6)                      | +08a
        lea     PowHang_RopeBroken_049976(pc),a1 | +08e
        move.l  a1,(a6)                         | +092
        bra.w   .L04992c                        | +094
.L04991c:
        cmpi.b  #0xfe,0x78(a0)                  | +098
        bne.w   .L04992c                        | +09e
        jmp     0x518.l                         | +0a2
.L04992c:
        rts                                     | +0a8

| ----------------------------------------------------------------------------
|  PowHang_RopeCut_04992e  @ $04992E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_RopeCut_04992e, "ax", @progbits
        .global PowHang_RopeCut_04992e
PowHang_RopeCut_04992e:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x3a(a0),0x3a(a6)               | +004
        lea     0x28fe2c.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L04994a(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L04994a:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L049960                        | +028
        jmp     0x518.l                         | +02c
.L049960:
        movea.l 0xc(a6),a0                      | +032
        cmpi.b  #0xfe,0x78(a0)                  | +036
        bne.w   .L049974                        | +03c
        jmp     0x518.l                         | +040
.L049974:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  PowHang_RopeBroken_049976  @ $049976  (102 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_RopeBroken_049976, "ax", @progbits
        .global PowHang_RopeBroken_049976
PowHang_RopeBroken_049976:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x3a(a0),0x3a(a6)               | +004
        move.b  #0x0,0x7e(a6)                   | +00a
        lea     0x28fde0.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L049998(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L049998:
        jsr     0x2783a.l                       | +022
        cmpi.b  #0x0,0x7e(a6)                   | +028
        beq.w   .L0499b6                        | +02e
        move.b  0x106f28.l,d0                   | +032
        btst    #0x0,d0                         | +038
        bne.w   .L0499bc                        | +03c
.L0499b6:
        jsr     0x28d70.l                       | +040
.L0499bc:
        bcc.w   .L0499c6                        | +046
        jmp     0x518.l                         | +04a
.L0499c6:
        movea.l 0xc(a6),a0                      | +050
        cmpi.b  #0xfe,0x78(a0)                  | +054
        bne.w   .L0499da                        | +05a
        jmp     0x518.l                         | +05e
.L0499da:
        rts                                     | +064

| ----------------------------------------------------------------------------
|  PowHang_SwingStep_0499dc  @ $0499DC  (60 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_SwingStep_0499dc, "ax", @progbits
        .global PowHang_SwingStep_0499dc
PowHang_SwingStep_0499dc:
        subi.w  #0x2,0x74(a6)                   | +000
        andi.w  #0xff,0x74(a6)                  | +006
        move.w  0x74(a6),d0                     | +00c
        move.w  #0xb0,d1                        | +010
        jsr     0x13c0e.l                       | +014
        add.w   0x7a(a6),d1                     | +01a
        move.w  d1,0x28(a6)                     | +01e
        asr.w   #0x1,d2                         | +022
        btst    #0xf,d2                         | +024
        bne.w   .L049a0a                        | +028
        neg.w   d2                              | +02c
.L049a0a:
        addi.w  #0x20,d2                        | +02e
        move.w  0x7c(a6),d0                     | +032
        sub.w   d0,d2                           | +036
        move.w  d2,0x2a(a6)                     | +038

| ----------------------------------------------------------------------------
|  PowHang_SwingAnimByAngle_049a20  @ $049A20  (222 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_SwingAnimByAngle_049a20, "ax", @progbits
        .global PowHang_SwingAnimByAngle_049a20
PowHang_SwingAnimByAngle_049a20:
        move.w  0x34(a6),0x72(a6)               | +000
        jsr     0x28d70.l                       | +006
        bcc.w   JsrAbsRts_049b04                | +00c
        jsr     0x5e136.l                       | +010
        bcs.w   JsrAbsRts_049b04                | +016
        subq.b  #0x1,0x70(a6)                   | +01a
        bne.w   JsrAbsRts_049b04                | +01e
        move.b  #0x3,0x70(a6)                   | +022
        asr.w   #0x4,d0                         | +028
        andi.w  #0xf,d0                         | +02a
        btst    #0x3,d0                         | +02e
        bne.w   .L049a5c                        | +032
        andi.w  #0xfc,d0                        | +036
        asl.w   #0x1,d0                         | +03a
.L049a5c:
        cmpi.w  #0x8,0x34(a6)                   | +03c
        bne.w   .L049a72                        | +042
        cmpi.w  #0x0,d0                         | +046
        bne.w   .L049a72                        | +04a
        move.w  #0xf,d0                         | +04e
.L049a72:
        jsr     0x4d4d0.l                       | +052
        add.w   d0,0x34(a6)                     | +058
        move.w  0x34(a6),d0                     | +05c
        andi.w  #0xf,d0                         | +060
        btst    #0x3,d0                         | +064
        bne.w   .L049a92                        | +068
        andi.w  #0xfc,d0                        | +06c
        asl.w   #0x1,d0                         | +070
.L049a92:
        move.w  d0,0x34(a6)                     | +072
        move.w  0x72(a6),d0                     | +076
        move.w  0x34(a6),d1                     | +07a
        cmpi.w  #0xd,d0                         | +07e
        bne.w   .L049abe                        | +082
        cmpi.w  #0xc,d1                         | +086
        bne.w   .L049abe                        | +08a
        lea     0x28f748.l,a0                   | +08e
        jsr     0x28cd4.l                       | +094
        bra.w   JsrAbsRts_049b04                | +09a
.L049abe:
        cmpi.w  #0xc,d0                         | +09e
        bne.w   .L049ade                        | +0a2
        cmpi.w  #0xd,d1                         | +0a6
        bne.w   .L049ade                        | +0aa
        lea     0x28f782.l,a0                   | +0ae
        jsr     0x28cd4.l                       | +0b4
        bra.w   JsrAbsRts_049b04                | +0ba
.L049ade:
        clr.l   d0                              | +0be
        move.w  0x34(a6),d0                     | +0c0
        andi.w  #0xf,d0                         | +0c4
        movea.l #0x28f708,a0                    | +0c8
        lsl.w   #0x2,d0                         | +0ce
        movea.l (a0,d0.w),a0                    | +0d0
        cmpa.l  #0xffffffff,a0                  | +0d4
        beq.w   JsrAbsRts_049b04                | +0da

| ----------------------------------------------------------------------------
|  PowHang_DropFxTimer_049b06  @ $049B06  (74 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_DropFxTimer_049b06, "ax", @progbits
        .global PowHang_DropFxTimer_049b06
PowHang_DropFxTimer_049b06:
        subq.w  #0x1,0x76(a6)                   | +000
        cmpi.w  #0x0,0x76(a6)                   | +004
        bgt.w   SetHandlerRts_049b56            | +00a
        cmpi.b  #0x1,0x20(a6)                   | +00e
        beq.w   SetHandlerRts_049b56            | +014
        lea     0x2bff0c.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x76(a6)                     | +024
        lea     0x48b56.l,a1                    | +028
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        move.w  0x80(a6),d0                     | +03a
        move.w  0x82(a6),d1                     | +03e
        add.w   d0,0x22(a0)                     | +042
        add.w   d1,0x24(a0)                     | +046

| ----------------------------------------------------------------------------
|  PowHang_HitCheck_049b58  @ $049B58  (20 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_HitCheck_049b58, "ax", @progbits
        .global PowHang_HitCheck_049b58
PowHang_HitCheck_049b58:
        cmpi.b  #0x0,0x7f(a6)                   | +000
        bne.w   PowHang_HitCheckAlt_049b78      | +006
        jsr     0x27eba.l                       | +00a
        bcc.w   SetXN_049b72                    | +010

| ----------------------------------------------------------------------------
|  PowHang_HitCheckAlt_049b78  @ $049B78  (10 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_HitCheckAlt_049b78, "ax", @progbits
        .global PowHang_HitCheckAlt_049b78
PowHang_HitCheckAlt_049b78:
        jsr     0x27fac.l                       | +000
        bcc.w   SetXN_049b88                    | +006

| ----------------------------------------------------------------------------
|  PowHang_SlotPrioCheck_049b8e  @ $049B8E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.PowHang_SlotPrioCheck_049b8e, "ax", @progbits
        .global PowHang_SlotPrioCheck_049b8e
PowHang_SlotPrioCheck_049b8e:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_049ba4                    | +00c

| ----------------------------------------------------------------------------
|  PowFall_Spawn_049baa  @ $049BAA  (250 B)
| ----------------------------------------------------------------------------
        .section .text.PowFall_Spawn_049baa, "ax", @progbits
        .global PowFall_Spawn_049baa
PowFall_Spawn_049baa:
        jsr     0x5e7c0.l                       | +000
        move.w  #0xe,d1                         | +006
        jsr     0x236e.l                        | +00a
        move.w  #0x8000,0x38(a6)                | +010
        jsr     0x267e2.l                       | +016
        move.w  #0x1,0x66(a6)                   | +01c
        lea     0x28ffdc.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        lea     0x2bdc88.l,a0                   | +02e
        jsr     0x799de.l                       | +034
        move.w  d0,d0                           | +03a
        move.b  0x98(a6),d1                     | +03c
        move.b  d1,0x3a(a6)                     | +040
        btst    #0x0,d1                         | +044
        bne.w   .L049bf8                        | +048
        neg.w   d0                              | +04c
.L049bf8:
        move.w  d0,0x28(a6)                     | +04e
        move.w  #0xffa0,0x2e(a6)                | +052
        move.l  0x106f54.l,d0                   | +058
        swap    d0                              | +05e
        cmpi.w  #0xd0,d0                        | +060
        blt.w   .L049c30                        | +064
        jsr     0x5e9b6.l                       | +068
        andi.w  #0x7,d0                         | +06e
        bne.w   .L049c30                        | +072
        move.w  0x28(a6),d0                     | +076
        asr.w   #0x1,d0                         | +07a
        add.w   d0,0x28(a6)                     | +07c
        asr.w   #0x1,d0                         | +080
        add.w   d0,0x28(a6)                     | +082
.L049c30:
        move.b  #0x0,0x20(a6)                   | +086
        lea     PowFall_Shadow_049ca4(pc),a1    | +08c
        jsr     0x4ae.l                         | +090
        jsr     0x5dd02.l                       | +096
        move.w  0x28(a6),0x28(a0)               | +09c
        move.w  0x2a(a6),0x2a(a0)               | +0a2
        move.b  0x3a(a6),0x3a(a0)               | +0a8
        lea     .L049c5e(pc),a1                 | +0ae
        move.l  a1,(a6)                         | +0b2
.L049c5e:
        jsr     0x27a92.l                       | +0b4
        jsr     0x28d70.l                       | +0ba
        jsr     0x2870a.l                       | +0c0
        bcc.w   .L049c80                        | +0c6
        move.b  #0xff,0x20(a6)                  | +0ca
        jsr     0x49fd0.l                       | +0d0
.L049c80:
        movea.l #0xffffffff,a0                  | +0d6
        lea     0x2902c4.l,a0                   | +0dc
        jsr     0x5dd5c.l                       | +0e2
        bcc.w   .L049ca2                        | +0e8
        move.b  #0xff,0x20(a6)                  | +0ec
        jmp     0x518.l                         | +0f2
.L049ca2:
        rts                                     | +0f8

| ----------------------------------------------------------------------------
|  PowFall_Shadow_049ca4  @ $049CA4  (202 B)
| ----------------------------------------------------------------------------
        .section .text.PowFall_Shadow_049ca4, "ax", @progbits
        .global PowFall_Shadow_049ca4
PowFall_Shadow_049ca4:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x144,d1                       | +006
        jsr     0x236e.l                        | +00a
        subq.w  #0x1,0x38(a6)                   | +010
        lea     0x290090.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     0x290106.l,a0                   | +020
        move.l  a0,0x4c(a6)                     | +026
        jsr     0x283ca.l                       | +02a
        lea     0x29024e.l,a0                   | +030
        move.l  a0,0x48(a6)                     | +036
        lea     .L049ce4(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L049ce4:
        movea.l 0xc(a6),a0                      | +040
        cmpi.b  #0xff,0x20(a0)                  | +044
        bne.w   .L049cfc                        | +04a
        jsr     0x27a92.l                       | +04e
        bra.w   .L049d32                        | +054
.L049cfc:
        move.w  0x22(a0),0x22(a6)               | +058
        move.w  #0x20,d0                        | +05e
        btst    #0x0,0x3a(a6)                   | +062
        bne.w   .L049d12                        | +068
        neg.w   d0                              | +06c
.L049d12:
        add.w   d0,0x22(a6)                     | +06e
        move.w  0x28(a6),d0                     | +072
        movem.w d0,-(a7)                        | +076
        move.w  #0x0,0x28(a6)                   | +07a
        jsr     0x5e7c0.l                       | +080
        movem.w (a7)+,d0                        | +086
        move.w  d0,0x28(a6)                     | +08a
.L049d32:
        jsr     0x28d70.l                       | +08e
        jsr     0x283d8.l                       | +094
        jsr     0x2870a.l                       | +09a
        move.w  #0x7fff,0x66(a6)                | +0a0
        bclr    #0x3,0x13(a6)                   | +0a6
        movea.l #0xffffffff,a0                  | +0ac
        lea     0x2902ce.l,a0                   | +0b2
        jsr     0x5dd5c.l                       | +0b8
        bcc.w   .L049d6c                        | +0be
        jmp     0x518.l                         | +0c2
.L049d6c:
        rts                                     | +0c8

| ----------------------------------------------------------------------------
|  PowFall_SlotPrioCheck_049d6e  @ $049D6E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.PowFall_SlotPrioCheck_049d6e, "ax", @progbits
        .global PowFall_SlotPrioCheck_049d6e
PowFall_SlotPrioCheck_049d6e:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_049d84                    | +00c

| ----------------------------------------------------------------------------
|  HumanDeath_StateTbls_049d8a  @ $049D8A  (544 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_StateTbls_049d8a, "ax", @progbits
        .global HumanDeath_StateTbls_049d8a
HumanDeath_StateTbls_049d8a:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0xa3f8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0xa3f8                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        .dc.w   0xa604                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xa604                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0xa568                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +014  (dato / opcode no decodificado)
        .dc.w   0xa568                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +018  (dato / opcode no decodificado)
        .dc.w   0xa702                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xa3f8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +020  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +024  (dato / opcode no decodificado)
        .dc.w   0xa72e                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +028  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xa88c                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +030  (dato / opcode no decodificado)
        .dc.w   0xa604                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +034  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +038  (dato / opcode no decodificado)
        .dc.w   0xa604                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xa604                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +040  (dato / opcode no decodificado)
        .dc.w   0xa604                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +044  (dato / opcode no decodificado)
        .dc.w   0xa604                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +048  (dato / opcode no decodificado)
        .dc.w   0xa89a                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xa8fa                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +050  (dato / opcode no decodificado)
        .dc.w   0xa948                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +054  (dato / opcode no decodificado)
        .dc.w   0xa604                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +058  (dato / opcode no decodificado)
        .dc.w   0xa72e                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +060  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +064  (dato / opcode no decodificado)
        .dc.w   0xa604                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +068  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +06c  (dato / opcode no decodificado)
        .dc.w   0xa702                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +070  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +074  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +078  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +080  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +084  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +088  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +090  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +094  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +098  (dato / opcode no decodificado)
        .dc.w   0xa5b4                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +09c  (dato / opcode no decodificado)
        .dc.w   0xa5b4                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0xa88c                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0xa89a                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0xa8fa                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0xa948                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0xa72e                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0xa5a6                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +100  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +104  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +108  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +10c  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +110  (dato / opcode no decodificado)
        .dc.w   0xa680                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +114  (dato / opcode no decodificado)
        .dc.w   0xa680                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +118  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +11c  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +120  (dato / opcode no decodificado)
        .dc.w   0xa568                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +124  (dato / opcode no decodificado)
        .dc.w   0xa568                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +128  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +12c  (dato / opcode no decodificado)
        .dc.w   0xa680                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +130  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +134  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +138  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +13c  (dato / opcode no decodificado)
        .dc.w   0xa88c                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +140  (dato / opcode no decodificado)
        .dc.w   0xa680                        | +142  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +144  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +148  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +14c  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +150  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +154  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +156  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +158  (dato / opcode no decodificado)
        .dc.w   0xa89a                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +15c  (dato / opcode no decodificado)
        .dc.w   0xa8fa                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +160  (dato / opcode no decodificado)
        .dc.w   0xa948                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +164  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +166  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +168  (dato / opcode no decodificado)
        .dc.w   0xa72e                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +16c  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +170  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +174  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +176  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +178  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +17c  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +180  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +184  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +186  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +188  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +18a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +18c  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +190  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +192  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +194  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +196  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +198  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +19c  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +19e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0xa5b4                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0xa5b4                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1be  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0xa88c                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +200  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +202  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +204  (dato / opcode no decodificado)
        .dc.w   0xa68c                        | +206  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +208  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +20c  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +20e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +210  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +212  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +214  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +216  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +218  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +21a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +21c  (dato / opcode no decodificado)
        .dc.w   0xa7b0                        | +21e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HumanDeath_StateTblPtrs_049faa  @ $049FAA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_StateTblPtrs_049faa, "ax", @progbits
        .global HumanDeath_StateTblPtrs_049faa
HumanDeath_StateTblPtrs_049faa:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x9d8a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0x9e12                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        .dc.w   0x9e9a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x9f22                        | +00e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HumanDeath_HitCheckUnlessCutscene_049fba  @ $049FBA  (10 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_HitCheckUnlessCutscene_049fba, "ax", @progbits
        .global HumanDeath_HitCheckUnlessCutscene_049fba
HumanDeath_HitCheckUnlessCutscene_049fba:
        tst.b   0x106ed3.l                      | +000
        bne.w   JmpAbsThunk_049fca              | +006
