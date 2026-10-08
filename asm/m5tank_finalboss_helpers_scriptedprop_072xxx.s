| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $071FFC..$076000  (15,662 B, 177 entradas, 69 huecos)
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
|  FinalBoss_FlameTail_FreeIfOffWorld_071ffc  @ $071FFC  (22 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_FlameTail_FreeIfOffWorld_071ffc, "ax", @progbits
        .global FinalBoss_FlameTail_FreeIfOffWorld_071ffc
FinalBoss_FlameTail_FreeIfOffWorld_071ffc:
        movea.l #0xffffffff,a0                  | +000
        lea     0x2d90bc.l,a0                   | +006
        jsr     0x5dd56.l                       | +00c
        bcc.w   SetHandlerRts_072018            | +012

| ----------------------------------------------------------------------------
|  FinalBoss_WreckA_07201a  @ $07201A  (132 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_WreckA_07201a, "ax", @progbits
        .global FinalBoss_WreckA_07201a
FinalBoss_WreckA_07201a:
        move.w  #0x1,0x84(a6)                   | +000
        move.w  #0x1cf,d1                       | +006
        jsr     0x236e.l                        | +00a
        lea     0x2d9aea.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L07203c(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L07203c:
        jsr     FinalBoss_CopyParentPosPrio_072bce(pc) | +022
        move.w  #0xffc4,d0                      | +026
        jsr     Facing_NegIfLeft_072782(pc)     | +02a
        add.w   d0,0x22(a6)                     | +02e
        addi.w  #0x28,0x24(a6)                  | +032
        jsr     0x28d70.l                       | +038
        bcc.w   .L072062                        | +03e
        lea     FinalBoss_WreckA_Fall_0720a6(pc),a1 | +042
        move.l  a1,(a6)                         | +046
.L072062:
        jsr     0x283d8.l                       | +048
        btst    #0x1,0x13(a6)                   | +04e
        beq.w   .L072078                        | +054
        lea     FinalBoss_FlushAndExplodeB_07233a(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L072078:
        jsr     0x5e45a.l                       | +05e
        bcc.w   .L072088                        | +064
        lea     FinalBoss_FlushAndExplodeB_07233a(pc),a1 | +068
        move.l  a1,(a6)                         | +06c
.L072088:
        movea.l #0xffffffff,a0                  | +06e
        lea     0x2d90bc.l,a0                   | +074
        jsr     0x5dd56.l                       | +07a
        bcc.w   SetHandlerRts_0720a4            | +080

| ----------------------------------------------------------------------------
|  FinalBoss_WreckA_Fall_0720a6  @ $0720A6  (150 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_WreckA_Fall_0720a6, "ax", @progbits
        .global FinalBoss_WreckA_Fall_0720a6
FinalBoss_WreckA_Fall_0720a6:
        bset    #0x4,0x6b(a6)                   | +000
        lea     0x2baaaa.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        neg.w   d0                              | +012
        jsr     Facing_NegIfLeft_072782(pc)     | +014
        move.w  d0,0x28(a6)                     | +018
        move.w  #0xd000,d0                      | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x1c,0x38(a6)                  | +02c
        lea     0x2d9b6a.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L0720ea(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L0720ea:
        jsr     0x27cee.l                       | +044
        bcc.w   .L0720fa                        | +04a
        lea     FinalBoss_ShiftX_Explode_07231e__L072326(pc),a1 | +04e
        move.l  a1,(a6)                         | +052
.L0720fa:
        jsr     0x28d70.l                       | +054
        jsr     0x283d8.l                       | +05a
        btst    #0x1,0x13(a6)                   | +060
        beq.w   .L072116                        | +066
        lea     FinalBoss_ShiftX_Explode_07231e__L072326(pc),a1 | +06a
        move.l  a1,(a6)                         | +06e
.L072116:
        jsr     0x5e45a.l                       | +070
        bcc.w   .L072126                        | +076
        lea     FinalBoss_ShiftX_Explode_07231e__L072326(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L072126:
        movea.l #0xffffffff,a0                  | +080
        lea     0x2d90bc.l,a0                   | +086
        jsr     0x5dd56.l                       | +08c
        bcc.w   SetHandlerRts_072142            | +092

| ----------------------------------------------------------------------------
|  FinalBoss_WreckB_072144  @ $072144  (158 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_WreckB_072144, "ax", @progbits
        .global FinalBoss_WreckB_072144
FinalBoss_WreckB_072144:
        move.w  #0x39,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xd000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x14,0x38(a6)                  | +01a
        lea     0x2da526.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L072176(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L072176:
        movea.l 0xc(a6),a0                      | +032
        cmpi.b  #0x5,0x8a(a0)                   | +036
        beq.w   FinalBoss_FlushAndExplodeB_07233a | +03c
        move.w  0x22(a0),d0                     | +040
        add.w   0x90(a6),d0                     | +044
        move.w  d0,0x22(a6)                     | +048
        move.w  0x24(a0),d0                     | +04c
        add.w   0x92(a6),d0                     | +050
        move.w  d0,0x24(a6)                     | +054
        jsr     0x28d70.l                       | +058
        bcc.w   .L0721ac                        | +05e
        lea     FinalBoss_WreckB_Fall_0721ea(pc),a1 | +062
        move.l  a1,(a6)                         | +066
.L0721ac:
        jsr     0x283d8.l                       | +068
        btst    #0x1,0x13(a6)                   | +06e
        beq.w   .L0721c2                        | +074
        lea     FinalBoss_FlushAndExplodeB_07233a(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L0721c2:
        jsr     0x5e45a.l                       | +07e
        bcc.w   .L0721d2                        | +084
        lea     FinalBoss_FlushAndExplodeB_07233a(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
.L0721d2:
        movea.l #0xffffffff,a0                  | +08e
        jsr     0x5dd56.l                       | +094
        bcc.w   SetHandlerRts_0721e8            | +09a

| ----------------------------------------------------------------------------
|  FinalBoss_WreckB_Fall_0721ea  @ $0721EA  (134 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_WreckB_Fall_0721ea, "ax", @progbits
        .global FinalBoss_WreckB_Fall_0721ea
FinalBoss_WreckB_Fall_0721ea:
        lea     0x2bacb2.l,a0                   | +000
        jsr     0x799de.l                       | +006
        jsr     Facing_NegIfLeft_072782(pc)     | +00c
        move.w  d0,0x28(a6)                     | +010
        lea     0x2bad34.l,a0                   | +014
        jsr     0x799de.l                       | +01a
        jsr     Facing_NegIfLeft_072782(pc)     | +020
        move.w  d0,0x2c(a6)                     | +024
        lea     0x2da58a.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L072224(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L072224:
        jsr     0x27cee.l                       | +03a
        bcc.w   .L072234                        | +040
        lea     FinalBoss_FlushAndExplodeB_07233a(pc),a1 | +044
        move.l  a1,(a6)                         | +048
.L072234:
        jsr     0x28d70.l                       | +04a
        jsr     0x283d8.l                       | +050
        btst    #0x1,0x13(a6)                   | +056
        beq.w   .L072250                        | +05c
        lea     FinalBoss_FlushAndExplodeB_07233a(pc),a1 | +060
        move.l  a1,(a6)                         | +064
.L072250:
        jsr     0x5e45a.l                       | +066
        bcc.w   .L072260                        | +06c
        lea     FinalBoss_FlushAndExplodeB_07233a(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L072260:
        movea.l #0xffffffff,a0                  | +076
        jsr     0x5dd56.l                       | +07c
        bcc.w   SetHandlerRts_072276            | +082

| ----------------------------------------------------------------------------
|  FinalBoss_Spark_Left_072278  @ $072278  (36 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_Spark_Left_072278, "ax", @progbits
        .global FinalBoss_Spark_Left_072278
FinalBoss_Spark_Left_072278:
        move.w  #0xffbc,d0                      | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x32a,0x2a(a6)                 | +00e
        move.w  #0xffca,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        bra.w   FinalBoss_Spark_07229c__L0722bc | +020

| ----------------------------------------------------------------------------
|  FinalBoss_Spark_07229c  @ $07229C  (98 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_Spark_07229c, "ax", @progbits
        .global FinalBoss_Spark_07229c
FinalBoss_Spark_07229c:
        move.w  #0x111,d0                       | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x87f,0x2a(a6)                 | +00e
        move.w  #0xff6f,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        .global FinalBoss_Spark_07229c__L0722bc
FinalBoss_Spark_07229c__L0722bc:
.L0722bc:
        move.w  #0x38,d1                        | +020
        jsr     0x236e.l                        | +024
        lea     0x29ca36.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L0722d8(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L0722d8:
        jsr     0x27bc8.l                       | +03c
        bcc.w   .L0722e8                        | +042
        lea     Jsr5B6ThenJmpScheduler_0716f2(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L0722e8:
        jsr     0x28d70.l                       | +04c
        movea.l #0xffffffff,a0                  | +052
        jsr     0x5dd56.l                       | +058
        bcc.w   SetHandlerRts_072304            | +05e

| ----------------------------------------------------------------------------
|  FinalBoss_RandDispatch4_072306  @ $072306  (24 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_RandDispatch4_072306, "ax", @progbits
        .global FinalBoss_RandDispatch4_072306
FinalBoss_RandDispatch4_072306:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x3,d0                         | +006
        lsl.w   #0x2,d0                         | +00a
        lea     0x2d9322.l,a0                   | +00c
        movea.l (a0,d0.w),a1                    | +012
        jmp     (a1)                            | +016

| ----------------------------------------------------------------------------
|  FinalBoss_ShiftX_Explode_07231e  @ $07231E  (28 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_ShiftX_Explode_07231e, "ax", @progbits
        .global FinalBoss_ShiftX_Explode_07231e
FinalBoss_ShiftX_Explode_07231e:
        move.w  #0xffd8,d0                      | +000
        bra.w   .L07232a                        | +004
        .global FinalBoss_ShiftX_Explode_07231e__L072326
FinalBoss_ShiftX_Explode_07231e__L072326:
.L072326:
        move.w  #0xffd0,d0                      | +008
.L07232a:
        btst    #0x0,0x3a(a6)                   | +00c
        beq.w   .L072336                        | +012
        neg.w   d0                              | +016
.L072336:
        add.w   d0,0x22(a6)                     | +018

| ----------------------------------------------------------------------------
|  FinalBoss_FlushAndExplodeB_07233a  @ $07233A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_FlushAndExplodeB_07233a, "ax", @progbits
        .global FinalBoss_FlushAndExplodeB_07233a
FinalBoss_FlushAndExplodeB_07233a:
        jsr     0x13600.l                       | +000
        move.w  #0x4000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x0,0x38(a6)                   | +016
        move.l  #0xffffffff,0x48(a6)            | +01c
        jmp     0x77f6a.l                       | +024

| ----------------------------------------------------------------------------
|  FinalBoss_ExplodeC_072364  @ $072364  (36 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_ExplodeC_072364, "ax", @progbits
        .global FinalBoss_ExplodeC_072364
FinalBoss_ExplodeC_072364:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77fd6.l                       | +01e

| ----------------------------------------------------------------------------
|  FinalBoss_SmokePuff_072388  @ $072388  (66 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SmokePuff_072388, "ax", @progbits
        .global FinalBoss_SmokePuff_072388
FinalBoss_SmokePuff_072388:
        move.w  #0xc000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x14,0x38(a6)                  | +010
        move.w  #0x148,d1                       | +016
        jsr     0x236e.l                        | +01a
        lea     0x2c3f0a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L0723ba(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L0723ba:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   SetHandlerRts_0723d0            | +03e

| ----------------------------------------------------------------------------
|  FinalBoss_LimbPart_Init_0723d2  @ $0723D2  (92 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_LimbPart_Init_0723d2, "ax", @progbits
        .global FinalBoss_LimbPart_Init_0723d2
FinalBoss_LimbPart_Init_0723d2:
        clr.b   0x9a(a6)                        | +000
        bra.w   .L0723e0                        | +004
        move.b  #0x1,0x9a(a6)                   | +008
.L0723e0:
        movea.l 0xc(a6),a0                      | +00e
        move.w  0x66(a0),d0                     | +012
        lsr.w   #0x1,d0                         | +016
        move.w  d0,0x66(a6)                     | +018
        lea     0x2da750.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        jsr     0x28d70.l                       | +028
        lea     .L072406(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L072406:
        jsr     FinalBoss_LimbPart_SyncWithParent_0724dc(pc) | +034
        bgt.w   .L072414                        | +038
        lea     FinalBoss_LimbPart_Damaged_072436(pc),a1 | +03c
        move.l  a1,(a6)                         | +040
.L072414:
        btst    #0x0,0x13(a6)                   | +042
        beq.w   .L072424                        | +048
        lea     JmpToScheduler_0724d4(pc),a1    | +04c
        move.l  a1,(a6)                         | +050
.L072424:
        jsr     0x5e45a.l                       | +052
        bcc.w   SetHandlerRts_072434            | +058

| ----------------------------------------------------------------------------
|  FinalBoss_LimbPart_Damaged_072436  @ $072436  (76 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_LimbPart_Damaged_072436, "ax", @progbits
        .global FinalBoss_LimbPart_Damaged_072436
FinalBoss_LimbPart_Damaged_072436:
        move.w  0x66(a6),d0                     | +000
        lsr.w   #0x1,d0                         | +004
        move.w  d0,0x66(a6)                     | +006
        lea     .L072446(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L072446:
        jsr     FinalBoss_LimbPart_SyncWithParent_0724dc(pc) | +010
        bgt.w   .L072454                        | +014
        lea     FinalBoss_LimbPart_Critical_07248a(pc),a1 | +018
        move.l  a1,(a6)                         | +01c
.L072454:
        addq.b  #0x1,0x70(a6)                   | +01e
        move.b  0x70(a6),d0                     | +022
        andi.b  #0x7,d0                         | +026
        bne.w   .L072468                        | +02a
        jsr     FinalBoss_SpawnSmokeA_072540__L072554(pc) | +02e
.L072468:
        btst    #0x0,0x13(a6)                   | +032
        beq.w   .L072478                        | +038
        lea     JmpToScheduler_0724d4(pc),a1    | +03c
        move.l  a1,(a6)                         | +040
.L072478:
        jsr     0x5e45a.l                       | +042
        bcc.w   SetHandlerRts_072488            | +048

| ----------------------------------------------------------------------------
|  FinalBoss_LimbPart_Critical_07248a  @ $07248A  (66 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_LimbPart_Critical_07248a, "ax", @progbits
        .global FinalBoss_LimbPart_Critical_07248a
FinalBoss_LimbPart_Critical_07248a:
        jsr     FinalBoss_LimbPart_SyncWithParent_0724dc(pc) | +000
        addq.b  #0x1,0x70(a6)                   | +004
        move.b  0x70(a6),d0                     | +008
        andi.b  #0xf,d0                         | +00c
        bne.w   .L0724a6                        | +010
        jsr     FinalBoss_SpawnSmokeA_072540(pc) | +014
        bra.w   .L0724b2                        | +018
.L0724a6:
        andi.b  #0x3,d0                         | +01c
        bne.w   .L0724b2                        | +020
        jsr     FinalBoss_SpawnSmokeA_072540__L072554(pc) | +024
.L0724b2:
        btst    #0x0,0x13(a6)                   | +028
        beq.w   .L0724c2                        | +02e
        lea     JmpToScheduler_0724d4(pc),a1    | +032
        move.l  a1,(a6)                         | +036
.L0724c2:
        jsr     0x5e45a.l                       | +038
        bcc.w   SetHandlerRts_0724d2            | +03e

| ----------------------------------------------------------------------------
|  FinalBoss_LimbPart_SyncWithParent_0724dc  @ $0724DC  (100 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_LimbPart_SyncWithParent_0724dc, "ax", @progbits
        .global FinalBoss_LimbPart_SyncWithParent_0724dc
FinalBoss_LimbPart_SyncWithParent_0724dc:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x32(a0),0x32(a6)               | +004
        move.b  0x33(a0),0x33(a6)               | +00a
        move.w  0x22(a0),0x22(a6)               | +010
        move.w  0x24(a0),d0                     | +016
        add.w   0x98(a6),d0                     | +01a
        move.w  d0,0x24(a6)                     | +01e
        move.w  0x38(a0),d0                     | +022
        andi.w  #0xffe3,d0                      | +026
        ori.w   #0x0,d0                         | +02a
        move.w  d0,0x38(a6)                     | +02e
        move.b  0x13(a0),0x13(a6)               | +032
        cmpi.b  #0x0,0x9a(a6)                   | +038
        beq.w   .L072536                        | +03e
        bclr    #0x0,0x13(a6)                   | +042
        jsr     0x7b2.l                         | +048
        bcc.w   .L072536                        | +04e
        bset    #0x0,0x13(a6)                   | +052
        rts                                     | +058
.L072536:
        move.w  0x66(a0),d0                     | +05a
        cmp.w   0x66(a6),d0                     | +05e
        rts                                     | +062

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnSmokeA_072540  @ $072540  (138 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnSmokeA_072540, "ax", @progbits
        .global FinalBoss_SpawnSmokeA_072540
FinalBoss_SpawnSmokeA_072540:
        lea     FinalBoss_SmokeA_072606(pc),a1  | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        bra.w   .L072564                        | +010
        .global FinalBoss_SpawnSmokeA_072540__L072554
FinalBoss_SpawnSmokeA_072540__L072554:
.L072554:
        lea     FinalBoss_SmokeB_0725f6(pc),a1  | +014
        jsr     0x4ae.l                         | +018
        jsr     0x5dd02.l                       | +01e
.L072564:
        move.w  #0x1e,0x70(a0)                  | +024
        jsr     0x5e9b6.l                       | +02a
        andi.w  #0x3f,d0                        | +030
        subi.w  #0x20,d0                        | +034
        cmpi.b  #0xff,0x32(a6)                  | +038
        beq.w   .L072590                        | +03e
        move.b  0x32(a6),d1                     | +042
        andi.w  #0xff,d1                        | +046
        addq.w  #0x1,d1                         | +04a
        muls.w  d1,d0                           | +04c
        asr.w   #0x8,d0                         | +04e
.L072590:
        add.w   d0,0x22(a0)                     | +050
        jsr     0x5e9b6.l                       | +054
        andi.w  #0x1f,d0                        | +05a
        addq.w  #0x8,d0                         | +05e
        cmpi.b  #0xff,0x33(a6)                  | +060
        beq.w   .L0725b8                        | +066
        move.b  0x33(a6),d1                     | +06a
        andi.w  #0xff,d1                        | +06e
        addq.w  #0x1,d1                         | +072
        muls.w  d1,d0                           | +074
        asr.w   #0x8,d0                         | +076
.L0725b8:
        add.w   d0,0x24(a0)                     | +078
        move.b  0x32(a6),0x32(a0)               | +07c
        move.b  0x33(a6),0x33(a0)               | +082
        rts                                     | +088

| ----------------------------------------------------------------------------
|  FinalBoss_Smoke_Reposition_0725ca  @ $0725CA  (44 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_Smoke_Reposition_0725ca, "ax", @progbits
        .global FinalBoss_Smoke_Reposition_0725ca
FinalBoss_Smoke_Reposition_0725ca:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x1f,d0                        | +006
        addi.w  #0x14,d0                        | +00a
        move.w  d0,0x70(a6)                     | +00e
        jsr     0x5e9b6.l                       | +012
        andi.w  #0x3f,d0                        | +018
        subi.w  #0x20,d0                        | +01c
        add.w   d0,0x22(a6)                     | +020
        subq.w  #0x1,0x38(a6)                   | +024
        bra.w   FinalBoss_SmokeA_072606__L07262a | +028

| ----------------------------------------------------------------------------
|  FinalBoss_SmokeB_0725f6  @ $0725F6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SmokeB_0725f6, "ax", @progbits
        .global FinalBoss_SmokeB_0725f6
FinalBoss_SmokeB_0725f6:
        lea     0x2da5c4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   FinalBoss_SmokeA_072606__L072612 | +00c

| ----------------------------------------------------------------------------
|  FinalBoss_SmokeA_072606  @ $072606  (198 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SmokeA_072606, "ax", @progbits
        .global FinalBoss_SmokeA_072606
FinalBoss_SmokeA_072606:
        lea     0x2da680.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global FinalBoss_SmokeA_072606__L072612
FinalBoss_SmokeA_072606__L072612:
.L072612:
        move.w  #0xd,d1                         | +00c
        jsr     0x236e.l                        | +010
        addq.w  #0x1,0x38(a6)                   | +016
        cmpi.b  #0xff,0x33(a6)                  | +01a
        bne.w   .L07262a                        | +020
        .global FinalBoss_SmokeA_072606__L07262a
FinalBoss_SmokeA_072606__L07262a:
.L07262a:
        jsr     0x267e2.l                       | +024
        jsr     0x5e9b6.l                       | +02a
        andi.w  #0xff,d0                        | +030
        addi.w  #0x40,d0                        | +034
        move.w  d0,0x2a(a6)                     | +038
        lsr.w   #0x6,d0                         | +03c
        neg.w   d0                              | +03e
        move.w  d0,0x2e(a6)                     | +040
        clr.w   0x8c(a6)                        | +044
        cmpi.w  #0xa,0x70(a6)                   | +048
        bgt.w   .L07265e                        | +04e
        move.w  #0xa,0x70(a6)                   | +052
.L07265e:
        lea     .L072664(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L072664:
        jsr     0x5e9b6.l                       | +05e
        move.w  d0,d1                           | +064
        andi.w  #0x3ff,d0                       | +066
        cmpi.w  #0x0,d1                         | +06a
        bgt.w   .L07267a                        | +06e
        neg.w   d0                              | +072
.L07267a:
        add.w   d0,0x8c(a6)                     | +074
        move.b  0x8c(a6),d0                     | +078
        andi.w  #0xff,d0                        | +07c
        add.w   d0,d0                           | +080
        lea     0x2c072c.l,a0                   | +082
        move.w  (a0,d0.w),d0                    | +088
        lsl.w   #0x1,d0                         | +08c
        move.w  d0,0x28(a6)                     | +08e
        jsr     0x27cee.l                       | +092
        jsr     0x28d70.l                       | +098
        bcc.w   .L0726ae                        | +09e
        lea     Jsr5B6ThenJmpScheduler_0716f2(pc),a1 | +0a2
        move.l  a1,(a6)                         | +0a6
.L0726ae:
        subq.w  #0x1,0x70(a6)                   | +0a8
        bgt.w   .L0726bc                        | +0ac
        lea     Jsr5B6ThenJmpScheduler_0716f2(pc),a1 | +0b0
        move.l  a1,(a6)                         | +0b4
.L0726bc:
        movea.l #0xffffffff,a0                  | +0b6
        jsr     0x5dd56.l                       | +0bc
        bcc.w   SetHandlerRts_0726d2            | +0c2

| ----------------------------------------------------------------------------
|  FinalBoss_InScreenByVel_0726d4  @ $0726D4  (34 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_InScreenByVel_0726d4, "ax", @progbits
        .global FinalBoss_InScreenByVel_0726d4
FinalBoss_InScreenByVel_0726d4:
        cmpi.w  #0x0,0x28(a6)                   | +000
        bgt.w   .L0726ec                        | +006
        cmpi.w  #0x50,0x22(a6)                  | +00a
        blt.w   SetXN_0726fc                    | +010
        bra.w   ClearXN_0726f6                  | +014
.L0726ec:
        cmpi.w  #0xf0,0x22(a6)                  | +018
        bgt.w   SetXN_0726fc                    | +01e

| ----------------------------------------------------------------------------
|  FinalBoss_TargetAheadByVel_072702  @ $072702  (66 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_TargetAheadByVel_072702, "ax", @progbits
        .global FinalBoss_TargetAheadByVel_072702
FinalBoss_TargetAheadByVel_072702:
        move.b  0x106f28.l,d0                   | +000
        andi.b  #0x3,d0                         | +006
        bne.w   ClearXN_07274a                  | +00a
        move.b  0x3a(a6),d0                     | +00e
        cmpi.w  #0x0,0x28(a6)                   | +012
        beq.w   ClearXN_07274a                  | +018
        blt.w   .L072726                        | +01c
        eori.b  #0x1,d0                         | +020
.L072726:
        lea     0x2d90d4.l,a0                   | +024
        btst    #0x0,d0                         | +02a
        beq.w   .L07273a                        | +02e
        lea     0x2d90dc.l,a0                   | +032
.L07273a:
        jsr     0x5e086.l                       | +038
        bcs.w   ClearXN_07274a                  | +03e

| ----------------------------------------------------------------------------
|  FinalBoss_EdgeFlagByVel_072750  @ $072750  (38 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_EdgeFlagByVel_072750, "ax", @progbits
        .global FinalBoss_EdgeFlagByVel_072750
FinalBoss_EdgeFlagByVel_072750:
        cmpi.w  #0x0,0x28(a6)                   | +000
        beq.w   ClearXN_07277c                  | +006
        bgt.w   .L07276c                        | +00a
        btst    #0x1,0x69(a6)                   | +00e
        bne.w   SetXN_072776                    | +014
        bra.w   ClearXN_07277c                  | +018
.L07276c:
        btst    #0x0,0x69(a6)                   | +01c
        beq.w   ClearXN_07277c                  | +022

| ----------------------------------------------------------------------------
|  Facing_NegIfLeft_072782  @ $072782  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Facing_NegIfLeft_072782, "ax", @progbits
        .global Facing_NegIfLeft_072782
Facing_NegIfLeft_072782:
        btst    #0x0,0x3a(a6)                   | +000
        beq.w   ClearXN_072794                  | +006
        neg.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  FinalBoss_SyncStateToParentAndTick_07279a  @ $07279A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SyncStateToParentAndTick_07279a, "ax", @progbits
        .global FinalBoss_SyncStateToParentAndTick_07279a
FinalBoss_SyncStateToParentAndTick_07279a:
        btst    #0x0,0x77(a6)                   | +000
        beq.w   .L0727c2                        | +006
        move.l  0xc(a6),d0                      | +00a
        cmpi.l  #0xffffffff,d0                  | +00e
        beq.w   .L0727c2                        | +014
        movea.l d0,a0                           | +018
        addq.b  #0x1,0x21(a0)                   | +01a
        addq.b  #0x1,0x76(a0)                   | +01e
        move.b  0x20(a0),0x20(a6)               | +022
.L0727c2:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  FinalBoss_SyncStateToParent_0727c4  @ $0727C4  (38 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SyncStateToParent_0727c4, "ax", @progbits
        .global FinalBoss_SyncStateToParent_0727c4
FinalBoss_SyncStateToParent_0727c4:
        btst    #0x0,0x77(a6)                   | +000
        beq.w   .L0727e8                        | +006
        move.l  0xc(a6),d0                      | +00a
        cmpi.l  #0xffffffff,d0                  | +00e
        beq.w   .L0727e8                        | +014
        movea.l d0,a0                           | +018
        addq.b  #0x1,0x21(a0)                   | +01a
        move.b  0x20(a0),0x20(a6)               | +01e
.L0727e8:
        rts                                     | +024

| ----------------------------------------------------------------------------
|  FinalBoss_TickAttackTimers_0727ea  @ $0727EA  (60 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_TickAttackTimers_0727ea, "ax", @progbits
        .global FinalBoss_TickAttackTimers_0727ea
FinalBoss_TickAttackTimers_0727ea:
        subq.w  #0x1,0x72(a6)                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +004
        bgt.w   .L072806                        | +00a
        cmpi.b  #0x0,0x88(a6)                   | +00e
        bne.w   .L072802                        | +014
.L072802:
        jsr     FinalBoss_PickCannonPattern_07282c(pc) | +018
.L072806:
        subq.w  #0x1,0x74(a6)                   | +01c
        cmpi.w  #0x0,0x74(a6)                   | +020
        bgt.w   JsrPcRts_07282a                 | +026
        cmpi.b  #0x1,0x20(a6)                   | +02a
        beq.w   JsrPcThunk_072826               | +030
        jsr     FinalBoss_PickBeamPattern_072996(pc) | +034
        bra.w   JsrPcRts_07282a                 | +038

| ----------------------------------------------------------------------------
|  FinalBoss_PickCannonPattern_07282c  @ $07282C  (362 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_PickCannonPattern_07282c, "ax", @progbits
        .global FinalBoss_PickCannonPattern_07282c
FinalBoss_PickCannonPattern_07282c:
        cmpi.b  #0x0,0x88(a6)                   | +000
        bne.w   .L0728ca                        | +006
        cmpi.b  #0x1,0x20(a6)                   | +00a
        beq.w   .L07288a                        | +010
        lea     0x2ba9a6.l,a0                   | +014
        jsr     0x799de.l                       | +01a
        move.b  d0,0x89(a6)                     | +020
        lea     0x2ba924.l,a0                   | +024
        jsr     0x799de.l                       | +02a
        addi.w  #0x1e,d0                        | +030
        move.w  d0,0x72(a6)                     | +034
        move.b  #0x1,0x88(a6)                   | +038
        move.b  #0x2,0x84(a6)                   | +03e
        move.b  #0x2,0x85(a6)                   | +044
        move.b  #0x2,0x86(a6)                   | +04a
        move.b  #0x0,0x87(a6)                   | +050
        move.b  #0x2,0x83(a6)                   | +056
        rts                                     | +05c
.L07288a:
        move.b  #0x7f,0x89(a6)                  | +05e
        lea     0x2ba924.l,a0                   | +064
        jsr     0x799de.l                       | +06a
        addi.w  #0x10,d0                        | +070
        move.w  d0,0x72(a6)                     | +074
        move.b  #0x1,0x88(a6)                   | +078
        move.b  #0x2,0x84(a6)                   | +07e
        move.b  #0x2,0x85(a6)                   | +084
        move.b  #0x0,0x86(a6)                   | +08a
        move.b  #0x0,0x87(a6)                   | +090
        move.b  #0x2,0x83(a6)                   | +096
        rts                                     | +09c
.L0728ca:
        cmpi.b  #0x1,0x88(a6)                   | +09e
        bne.w   .L072952                        | +0a4
        move.b  #0x3,0x84(a6)                   | +0a8
        move.b  #0x3,0x85(a6)                   | +0ae
        move.b  #0x0,0x86(a6)                   | +0b4
        move.b  #0x0,0x87(a6)                   | +0ba
        move.b  #0x0,0x83(a6)                   | +0c0
        btst    #0x0,0x89(a6)                   | +0c6
        beq.w   .L07291a                        | +0cc
        move.b  #0x0,0x84(a6)                   | +0d0
        move.b  #0x0,0x85(a6)                   | +0d6
        move.b  #0x3,0x86(a6)                   | +0dc
        move.b  #0x0,0x87(a6)                   | +0e2
        move.b  #0x0,0x83(a6)                   | +0e8
.L07291a:
        lea     0x2ba924.l,a0                   | +0ee
        jsr     0x799de.l                       | +0f4
        addi.w  #0x1e,d0                        | +0fa
        cmpi.b  #0x1,0x20(a6)                   | +0fe
        beq.w   .L072938                        | +104
        addi.w  #0xfff2,d0                      | +108
.L072938:
        move.w  d0,0x72(a6)                     | +10c
        subq.b  #0x1,0x89(a6)                   | +110
        cmpi.b  #0x0,0x89(a6)                   | +114
        bge.w   .L072950                        | +11a
        move.b  #0x2,0x88(a6)                   | +11e
.L072950:
        rts                                     | +124
.L072952:
        move.b  #0x0,0x88(a6)                   | +126
        lea     0x2ba8a2.l,a0                   | +12c
        jsr     0x799de.l                       | +132
        move.w  d0,0x72(a6)                     | +138
        jsr     0x5e9b6.l                       | +13c
        andi.w  #0x3f,d0                        | +142
        add.w   d0,0x72(a6)                     | +146
        move.b  #0x4,0x84(a6)                   | +14a
        move.b  #0x4,0x85(a6)                   | +150
        move.b  #0x4,0x86(a6)                   | +156
        move.b  #0x0,0x87(a6)                   | +15c
        move.b  #0x0,0x83(a6)                   | +162
        rts                                     | +168

| ----------------------------------------------------------------------------
|  FinalBoss_PickBeamPattern_072996  @ $072996  (254 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_PickBeamPattern_072996, "ax", @progbits
        .global FinalBoss_PickBeamPattern_072996
FinalBoss_PickBeamPattern_072996:
        cmpi.b  #0x0,0x8a(a6)                   | +000
        bne.w   .L0729ea                        | +006
        lea     0x2bac30.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.b  d0,0x8b(a6)                     | +016
        lea     0x2babae.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        addi.w  #0x1e,d0                        | +026
        move.w  d0,0x74(a6)                     | +02a
        move.b  #0x1,0x8a(a6)                   | +02e
        move.b  #0x0,0x84(a6)                   | +034
        move.b  #0x0,0x85(a6)                   | +03a
        move.b  #0x0,0x86(a6)                   | +040
        move.b  #0x2,0x87(a6)                   | +046
        move.b  #0x3,0x83(a6)                   | +04c
        rts                                     | +052
.L0729ea:
        cmpi.b  #0x1,0x8a(a6)                   | +054
        bne.w   .L072a2e                        | +05a
        move.b  #0x0,0x84(a6)                   | +05e
        move.b  #0x0,0x85(a6)                   | +064
        move.b  #0x0,0x86(a6)                   | +06a
        move.b  #0x3,0x87(a6)                   | +070
        move.b  #0x0,0x83(a6)                   | +076
        lea     0x2babae.l,a0                   | +07c
        jsr     0x799de.l                       | +082
        addi.w  #0x1e,d0                        | +088
        move.w  d0,0x74(a6)                     | +08c
        move.b  #0x2,0x8a(a6)                   | +090
        rts                                     | +096
.L072a2e:
        lea     0x2babae.l,a0                   | +098
        jsr     0x799de.l                       | +09e
        addi.w  #0x1e,d0                        | +0a4
        move.w  d0,0x74(a6)                     | +0a8
        subq.b  #0x1,0x8b(a6)                   | +0ac
        cmpi.b  #0x0,0x8b(a6)                   | +0b0
        bge.w   .L072a6e                        | +0b6
        lea     0x2bab2c.l,a0                   | +0ba
        jsr     0x799de.l                       | +0c0
        move.w  d0,0x74(a6)                     | +0c6
        jsr     0x5e9b6.l                       | +0ca
        andi.w  #0x3f,d0                        | +0d0
        add.w   d0,0x74(a6)                     | +0d4
.L072a6e:
        move.b  #0x0,0x8a(a6)                   | +0d8
        move.b  #0x0,0x84(a6)                   | +0de
        move.b  #0x0,0x85(a6)                   | +0e4
        move.b  #0x0,0x86(a6)                   | +0ea
        move.b  #0x4,0x87(a6)                   | +0f0
        move.b  #0x0,0x83(a6)                   | +0f6
        rts                                     | +0fc

| ----------------------------------------------------------------------------
|  FinalBoss_PickBeamPatternB_072a94  @ $072A94  (198 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_PickBeamPatternB_072a94, "ax", @progbits
        .global FinalBoss_PickBeamPatternB_072a94
FinalBoss_PickBeamPatternB_072a94:
        cmpi.b  #0x0,0x8a(a6)                   | +000
        bne.w   .L072ada                        | +006
        lea     0x2babae.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        asr.w   #0x1,d0                         | +016
        addi.w  #0x14,d0                        | +018
        move.w  d0,0x74(a6)                     | +01c
        move.b  #0x1,0x8a(a6)                   | +020
        move.b  #0x0,0x84(a6)                   | +026
        move.b  #0x0,0x85(a6)                   | +02c
        move.b  #0x0,0x86(a6)                   | +032
        move.b  #0x2,0x87(a6)                   | +038
        move.b  #0x0,0x83(a6)                   | +03e
        rts                                     | +044
.L072ada:
        cmpi.b  #0x1,0x8a(a6)                   | +046
        bne.w   .L072b1e                        | +04c
        move.b  #0x0,0x84(a6)                   | +050
        move.b  #0x0,0x85(a6)                   | +056
        move.b  #0x0,0x86(a6)                   | +05c
        move.b  #0x3,0x87(a6)                   | +062
        move.b  #0x0,0x83(a6)                   | +068
        lea     0x2babae.l,a0                   | +06e
        jsr     0x799de.l                       | +074
        asr.w   #0x1,d0                         | +07a
        addq.w  #0x4,d0                         | +07c
        move.w  d0,0x74(a6)                     | +07e
        move.b  #0x2,0x8a(a6)                   | +082
        rts                                     | +088
.L072b1e:
        lea     0x2babae.l,a0                   | +08a
        jsr     0x799de.l                       | +090
        asr.w   #0x1,d0                         | +096
        addi.w  #0xa,d0                         | +098
        move.w  d0,0x74(a6)                     | +09c
        move.b  #0x1,0x8a(a6)                   | +0a0
        move.b  #0x0,0x84(a6)                   | +0a6
        move.b  #0x0,0x85(a6)                   | +0ac
        move.b  #0x0,0x86(a6)                   | +0b2
        move.b  #0x6,0x87(a6)                   | +0b8
        move.b  #0x0,0x83(a6)                   | +0be
        rts                                     | +0c4

| ----------------------------------------------------------------------------
|  FinalBoss_HeadTargetTimer_072b5a  @ $072B5A  (48 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_HeadTargetTimer_072b5a, "ax", @progbits
        .global FinalBoss_HeadTargetTimer_072b5a
FinalBoss_HeadTargetTimer_072b5a:
        subq.w  #0x1,0x72(a6)                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +004
        bgt.w   ClearXN_072b90                  | +00a
        lea     0x2d90cc.l,a0                   | +00e
        jsr     0x5e086.l                       | +014
        bcs.w   ClearXN_072b90                  | +01a
        jsr     0x5e9b6.l                       | +01e
        andi.w  #0x3f,d0                        | +024
        addi.w  #0x1e,d0                        | +028
        move.w  d0,0x72(a6)                     | +02c

| ----------------------------------------------------------------------------
|  FinalBoss_PlayPartCry_072b96  @ $072B96  (48 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_PlayPartCry_072b96, "ax", @progbits
        .global FinalBoss_PlayPartCry_072b96
FinalBoss_PlayPartCry_072b96:
        move.w  #0x5b,d1                        | +000
        movea.l 0xc(a6),a0                      | +004
        cmpi.b  #0x0,0x80(a0)                   | +008
        beq.w   .L072bac                        | +00e
        move.w  #0x5c,d1                        | +012
.L072bac:
        btst    #0x0,0x77(a6)                   | +016
        bne.w   .L072bba                        | +01c
        move.w  #0x1e5,d1                       | +020
.L072bba:
        jsr     0x236e.l                        | +024
        move.w  #0xc,0x1c(a6)                   | +02a

| ----------------------------------------------------------------------------
|  FinalBoss_CopyParentPosPrio_072bce  @ $072BCE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_CopyParentPosPrio_072bce, "ax", @progbits
        .global FinalBoss_CopyParentPosPrio_072bce
FinalBoss_CopyParentPosPrio_072bce:
        jsr     0x5e506.l                       | +000
        move.w  0x84(a6),d0                     | +006
        add.w   d0,0x38(a6)                     | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  FinalBoss_OffsetByLimbTable_072bde  @ $072BDE  (42 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_OffsetByLimbTable_072bde, "ax", @progbits
        .global FinalBoss_OffsetByLimbTable_072bde
FinalBoss_OffsetByLimbTable_072bde:
        movea.l 0x80(a6),a1                     | +000
        movea.l 0xc(a6),a0                      | +004
        move.b  0x82(a0),d1                     | +008
        subq.b  #0x1,d1                         | +00c
        andi.w  #0xf,d1                         | +00e
        lsl.w   #0x2,d1                         | +012
        move.w  0x2(a1,d1.w),d0                 | +014
        add.w   d0,0x24(a6)                     | +018
        move.w  (a1,d1.w),d0                    | +01c
        jsr     Facing_NegIfLeft_072782(pc)     | +020
        add.w   d0,0x22(a6)                     | +024
        rts                                     | +028

| ----------------------------------------------------------------------------
|  FinalBoss_FetchPartSprite_072c08  @ $072C08  (42 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_FetchPartSprite_072c08, "ax", @progbits
        .global FinalBoss_FetchPartSprite_072c08
FinalBoss_FetchPartSprite_072c08:
        movea.l 0xc(a6),a1                      | +000
        move.w  0x86(a6),d1                     | +004
        move.b  (a1,d1.w),d0                    | +008
        clr.b   (a1,d1.w)                       | +00c
        andi.w  #0xf,d0                         | +010
        beq.w   FinalBoss_NullSprite_072c38     | +014
        lsl.w   #0x2,d0                         | +018
        move.l  (a0,d0.w),d0                    | +01a
        cmpi.l  #0xffffffff,d0                  | +01e
        beq.w   FinalBoss_NullSprite_072c38     | +024
        movea.l d0,a0                           | +028

| ----------------------------------------------------------------------------
|  FinalBoss_NullSprite_072c38  @ $072C38  (6 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_NullSprite_072c38, "ax", @progbits
        .global FinalBoss_NullSprite_072c38
FinalBoss_NullSprite_072c38:
        movea.l #0xffffffff,a0                  | +000

| ----------------------------------------------------------------------------
|  FinalBoss_HitTestPlayers_072c44  @ $072C44  (78 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_HitTestPlayers_072c44, "ax", @progbits
        .global FinalBoss_HitTestPlayers_072c44
FinalBoss_HitTestPlayers_072c44:
        clr.w   d0                              | +000
        jsr     0x5e42a.l                       | +002
        bcc.w   .L072c6c                        | +008
        tst.b   d0                              | +00c
        bne.w   .L072c6c                        | +00e
        movea.l 0x74(a0),a1                     | +012
        btst    #0x0,0x3a(a6)                   | +016
        beq.w   .L072c68                        | +01c
        movea.l 0x78(a0),a1                     | +020
.L072c68:
        jsr     Entity_CheckActiveBoxOverlap_072C98(pc) | +024
.L072c6c:
        move.w  #0x1,d0                         | +028
        jsr     0x5e42a.l                       | +02c
        bcc.w   JsrPcRts_072c96                 | +032
        tst.b   d0                              | +036
        bne.w   JsrPcRts_072c96                 | +038
        movea.l 0x74(a0),a1                     | +03c
        btst    #0x0,0x3a(a6)                   | +040
        beq.w   JsrPcThunk_072c92               | +046
        movea.l 0x78(a0),a1                     | +04a

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnBeamAndCasing_072d28  @ $072D28  (72 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnBeamAndCasing_072d28, "ax", @progbits
        .global FinalBoss_SpawnBeamAndCasing_072d28
FinalBoss_SpawnBeamAndCasing_072d28:
        lea     FinalBoss_Beam_071ee6(pc),a1    | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0xffd6,d0                      | +010
        jsr     Facing_NegIfLeft_072782(pc)     | +014
        add.w   d0,0x22(a0)                     | +018
        addi.w  #0x40,0x24(a0)                  | +01c
        lea     FinalBoss_Casing_071e50(pc),a1  | +022
        jsr     0x4ae.l                         | +026
        jsr     0x5dd02.l                       | +02c
        move.w  #0xfff0,d0                      | +032
        jsr     Facing_NegIfLeft_072782(pc)     | +036
        add.w   d0,0x22(a0)                     | +03a
        addi.w  #0x16,0x24(a0)                  | +03e
        move.w  #0x1064,d0                      | +044

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnWreckAndFlame_072d78  @ $072D78  (36 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnWreckAndFlame_072d78, "ax", @progbits
        .global FinalBoss_SpawnWreckAndFlame_072d78
FinalBoss_SpawnWreckAndFlame_072d78:
        lea     FinalBoss_WreckA_07201a(pc),a1  | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        lea     FinalBoss_Flame_071eb6(pc),a1   | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        move.w  #0x1066,d0                      | +020

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnDebris2D91F8_072da4  @ $072DA4  (12 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnDebris2D91F8_072da4, "ax", @progbits
        .global FinalBoss_SpawnDebris2D91F8_072da4
FinalBoss_SpawnDebris2D91F8_072da4:
        lea     0x2d91f8.l,a1                   | +000
        jsr     0x77c7e.l                       | +006

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnSmokeColumn_072db8  @ $072DB8  (48 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnSmokeColumn_072db8, "ax", @progbits
        .global FinalBoss_SpawnSmokeColumn_072db8
FinalBoss_SpawnSmokeColumn_072db8:
        jsr     0x6e412.l                       | +000
        move.w  #0x10,d0                        | +006
.L072dc2:
        movem.w d0,-(a7)                        | +00a
        lea     0x62536.l,a1                    | +00e
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        addi.w  #0x18,0x24(a0)                  | +020
        movem.w (a7)+,d0                        | +026
        subq.w  #0x1,d0                         | +02a
        bcc.b   .L072dc2                        | +02c
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  FinalBoss_AttackThenDebris_072dea  @ $072DEA  (34 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_AttackThenDebris_072dea, "ax", @progbits
        .global FinalBoss_AttackThenDebris_072dea
FinalBoss_AttackThenDebris_072dea:
        move.l  #0x2d8f1c,0x4c(a6)              | +000
        jsr     0x283ca.l                       | +008
        jsr     0x283d8.l                       | +00e
        move.l  #0xffffffff,0x4c(a6)            | +014
        lea     0x2d920a.l,a1                   | +01c

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnCannonPair_072e14  @ $072E14  (100 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnCannonPair_072e14, "ax", @progbits
        .global FinalBoss_SpawnCannonPair_072e14
FinalBoss_SpawnCannonPair_072e14:
        lea     FinalBoss_CannonB_071dc8(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0x29,d0                        | +010
        jsr     Facing_NegIfLeft_072782(pc)     | +014
        move.w  d0,0x9a(a0)                     | +018
        addi.w  #0x1e,0x9c(a0)                  | +01c
        jsr     0x5e9b6.l                       | +022
        movem.w d0,-(a7)                        | +028
        lea     FinalBoss_Cannon_071be6(pc),a1  | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        movem.w (a7)+,d0                        | +03c
        andi.w  #0x7,d0                         | +040
        move.b  d0,0x98(a0)                     | +044
        move.b  0x20(a6),0x20(a0)               | +048
        move.w  #0x25,d0                        | +04e
        jsr     Facing_NegIfLeft_072782(pc)     | +052
        move.w  d0,0x9a(a0)                     | +056
        move.w  #0x44,0x9c(a0)                  | +05a
        move.w  #0x10c6,d0                      | +060

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnCannonPairB_072e80  @ $072E80  (96 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnCannonPairB_072e80, "ax", @progbits
        .global FinalBoss_SpawnCannonPairB_072e80
FinalBoss_SpawnCannonPairB_072e80:
        lea     FinalBoss_CannonB_071dc8(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0x2c,d0                        | +010
        jsr     Facing_NegIfLeft_072782(pc)     | +014
        move.w  d0,0x9a(a0)                     | +018
        addi.w  #0x1e,0x9c(a0)                  | +01c
        jsr     0x5e9b6.l                       | +022
        movem.w d0,-(a7)                        | +028
        lea     FinalBoss_Cannon_071be6__L071bf6(pc),a1 | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        movem.w (a7)+,d0                        | +03c
        andi.w  #0x7,d0                         | +040
        addq.w  #0x8,d0                         | +044
        move.b  d0,0x98(a6)                     | +046
        move.w  #0x2c,d0                        | +04a
        jsr     Facing_NegIfLeft_072782(pc)     | +04e
        move.w  d0,0x9a(a0)                     | +052
        move.w  #0x44,0x9c(a0)                  | +056
        move.w  #0x10c6,d0                      | +05c

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnSmokeAndWreckB_072ee8  @ $072EE8  (80 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnSmokeAndWreckB_072ee8, "ax", @progbits
        .global FinalBoss_SpawnSmokeAndWreckB_072ee8
FinalBoss_SpawnSmokeAndWreckB_072ee8:
        lea     0x66b6c.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        eori.b  #0x1,0x3a(a0)                   | +012
        move.w  #0x18,d0                        | +018
        jsr     Facing_NegIfLeft_072782(pc)     | +01c
        add.w   d0,0x22(a0)                     | +020
        addi.w  #0x10,0x24(a0)                  | +024
        lea     FinalBoss_WreckB_072144(pc),a1  | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        move.w  #0x20,d0                        | +03a
        jsr     Facing_NegIfLeft_072782(pc)     | +03e
        move.w  d0,0x90(a0)                     | +042
        addi.w  #0x10,0x92(a0)                  | +046
        move.w  #0x10a4,d0                      | +04c

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnPuffA_072f40  @ $072F40  (36 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnPuffA_072f40, "ax", @progbits
        .global FinalBoss_SpawnPuffA_072f40
FinalBoss_SpawnPuffA_072f40:
        lea     FinalBoss_SmokePuff_072388(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0xfff0,d0                      | +010
        jsr     Facing_NegIfLeft_072782(pc)     | +014
        add.w   d0,0x22(a0)                     | +018
        addi.w  #0xe,0x24(a0)                   | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnPuffB_072f64  @ $072F64  (36 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnPuffB_072f64, "ax", @progbits
        .global FinalBoss_SpawnPuffB_072f64
FinalBoss_SpawnPuffB_072f64:
        lea     FinalBoss_SmokePuff_072388(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0xfff4,d0                      | +010
        jsr     Facing_NegIfLeft_072782(pc)     | +014
        add.w   d0,0x22(a0)                     | +018
        addi.w  #0xfffe,0x24(a0)                | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  FinalBoss_SpawnPuffC_072f88  @ $072F88  (36 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_SpawnPuffC_072f88, "ax", @progbits
        .global FinalBoss_SpawnPuffC_072f88
FinalBoss_SpawnPuffC_072f88:
        lea     FinalBoss_SmokePuff_072388(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0x4,d0                         | +010
        jsr     Facing_NegIfLeft_072782(pc)     | +014
        add.w   d0,0x22(a0)                     | +018
        addi.w  #0xfffe,0x24(a0)                | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  FinalBoss_CmpDepthToParent_072fac  @ $072FAC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.FinalBoss_CmpDepthToParent_072fac, "ax", @progbits
        .global FinalBoss_CmpDepthToParent_072fac
FinalBoss_CmpDepthToParent_072fac:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_072fc2                    | +00c

| ----------------------------------------------------------------------------
|  M5Tank_Tmpl76_072fc8  @ $072FC8  (244 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Tmpl76_072fc8, "ax", @progbits
        .global M5Tank_Tmpl76_072fc8
M5Tank_Tmpl76_072fc8:
        move.w  #0x28,d0                        | +000
        jsr     0x2352.l                        | +004
        move.w  #0xec,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x16,0x1c(a6)                  | +014
        jsr     0x138fe.l                       | +01a
        move.l  #0x2dab18,0x4c(a6)              | +020
        move.l  #0x2da784,0x48(a6)              | +028
        move.l  #0x2db018,0x60(a6)              | +030
        move.w  #0x4000,d0                      | +038
        jsr     0x28134.l                       | +03c
        andi.w  #0xffe3,0x38(a6)                | +042
        ori.w   #0x14,0x38(a6)                  | +048
        lea     0x2baef8.l,a0                   | +04e
        jsr     0x799de.l                       | +054
        move.w  d0,0x66(a6)                     | +05a
        lea     0x2baf7a.l,a0                   | +05e
        jsr     0x799de.l                       | +064
        move.w  d0,0x82(a6)                     | +06a
        jsr     0x5e7c0.l                       | +06e
        lea     0x723d2.l,a1                    | +074
        jsr     0x4ae.l                         | +07a
        jsr     0x5dd02.l                       | +080
        move.w  #0x3c,0x98(a0)                  | +086
        lea     M5Tank_Hull_07350a(pc),a1       | +08c
        jsr     0x4ae.l                         | +090
        jsr     0x5dd02.l                       | +096
        clr.w   0x7c(a0)                        | +09c
        lea     M5Tank_Cabin_0735b8(pc),a1      | +0a0
        jsr     0x4ae.l                         | +0a4
        jsr     0x5dd02.l                       | +0aa
        move.w  #0xffff,0x7c(a0)                | +0b0
        lea     M5Tank_Launcher_073688(pc),a1   | +0b6
        jsr     0x4ae.l                         | +0ba
        jsr     0x5dd02.l                       | +0c0
        move.w  #0xffff,0x7c(a0)                | +0c6
        lea     M5Tank_Gun_073952(pc),a1        | +0cc
        jsr     0x4ae.l                         | +0d0
        jsr     0x5dd02.l                       | +0d6
        clr.b   0x73(a6)                        | +0dc
        clr.b   0x75(a6)                        | +0e0
        move.w  #0x140,0x36(a6)                 | +0e4
        move.w  #0x1073,d0                      | +0ea
        jsr     0x2352.l                        | +0ee

| ----------------------------------------------------------------------------
|  M5Tank_Drive_0730bc  @ $0730BC  (140 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Drive_0730bc, "ax", @progbits
        .global M5Tank_Drive_0730bc
M5Tank_Drive_0730bc:
        tst.b   0x75(a6)                        | +000
        bne.w   M5Tank_Retreat_0731b4           | +004
        jsr     M5Tank_InScreen_074122(pc)      | +008
        bcs.w   M5Tank_Turn_073148              | +00c
        move.w  0x36(a6),d0                     | +010
        move.w  #0x4,d1                         | +014
        btst    #0x0,0x73(a6)                   | +018
        bne.w   .L0730e2                        | +01e
        clr.w   d1                              | +022
        neg.w   d0                              | +024
.L0730e2:
        move.w  d0,0x28(a6)                     | +026
        move.b  0x74(a6),d0                     | +02a
        andi.w  #0x3,d0                         | +02e
        add.w   d1,d0                           | +032
        movea.l #0x2db11c,a0                    | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L07310c                        | +046
        jsr     0x28cd4.l                       | +04a
.L07310c:
        move.l  #0x2daa24,0x4c(a6)              | +050
        lea     .L07311a(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L07311a:
        jsr     0x28998.l                       | +05e
        jsr     0x27afc.l                       | +064
        bcc.w   .L073130                        | +06a
        lea     M5Tank_Turn_073148(pc),a1       | +06e
        move.l  a1,(a6)                         | +072
.L073130:
        jsr     M5Tank_InScreenSetC_07410e(pc)  | +074
        bcc.w   .L07313e                        | +078
        lea     M5Tank_Turn_073148(pc),a1       | +07c
        move.l  a1,(a6)                         | +080
.L07313e:
        jsr     0x28d70.l                       | +082
        bra.w   M5Tank_Phase3_0732e8__L073382   | +088

| ----------------------------------------------------------------------------
|  M5Tank_Turn_073148  @ $073148  (108 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Turn_073148, "ax", @progbits
        .global M5Tank_Turn_073148
M5Tank_Turn_073148:
        eori.b  #0x1,0x73(a6)                   | +000
        clr.w   0x28(a6)                        | +006
        move.w  #0x3c,0x70(a6)                  | +00a
        move.b  0x74(a6),d0                     | +010
        andi.w  #0x7,d0                         | +014
        movea.l #0x2db0fc,a0                    | +018
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a0                    | +020
        cmpa.l  #0xffffffff,a0                  | +024
        beq.w   .L07317c                        | +02a
        jsr     0x28cd4.l                       | +02e
.L07317c:
        move.l  #0x2dab18,0x4c(a6)              | +034
        lea     .L07318a(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L07318a:
        jsr     0x28998.l                       | +042
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        subq.w  #0x1,0x70(a6)                   | +054
        cmpi.w  #0x0,0x70(a6)                   | +058
        bgt.w   .L0731b0                        | +05e
        lea     M5Tank_Drive_0730bc(pc),a1      | +062
        move.l  a1,(a6)                         | +066
.L0731b0:
        bra.w   M5Tank_Phase3_0732e8__L073382   | +068

| ----------------------------------------------------------------------------
|  M5Tank_Retreat_0731b4  @ $0731B4  (170 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Retreat_0731b4, "ax", @progbits
        .global M5Tank_Retreat_0731b4
M5Tank_Retreat_0731b4:
        move.w  #0x1050,d0                      | +000
        move.w  #0xcf,d1                        | +004
        jsr     0x440d0.l                       | +008
        move.w  d0,0x5c(a6)                     | +00e
        move.w  d0,d2                           | +012
        move.w  0x36(a6),d0                     | +014
        asr.w   #0x1,d0                         | +018
        move.w  #0x4,d1                         | +01a
        move.b  #0x1,0x73(a6)                   | +01e
        cmp.w   0x22(a6),d2                     | +024
        bge.w   .L0731e8                        | +028
        clr.w   d1                              | +02c
        neg.w   d0                              | +02e
        clr.b   0x73(a6)                        | +030
.L0731e8:
        move.w  d0,0x28(a6)                     | +034
        move.b  0x74(a6),d0                     | +038
        andi.w  #0x3,d0                         | +03c
        add.w   d1,d0                           | +040
        movea.l #0x2db13c,a0                    | +042
        lsl.w   #0x2,d0                         | +048
        movea.l (a0,d0.w),a0                    | +04a
        cmpa.l  #0xffffffff,a0                  | +04e
        beq.w   .L073212                        | +054
        jsr     0x28cd4.l                       | +058
.L073212:
        move.l  #0x2daa24,0x4c(a6)              | +05e
        move.w  #0xb4,0x70(a6)                  | +066
        lea     .L073226(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L073226:
        jsr     0x28998.l                       | +072
        jsr     0x27afc.l                       | +078
        jsr     M5Tank_AtGoalX_074196(pc)       | +07e
        bcc.w   .L073240                        | +082
        lea     M5Tank_Pause_07325e(pc),a1      | +086
        move.l  a1,(a6)                         | +08a
.L073240:
        jsr     0x28d70.l                       | +08c
        subq.w  #0x1,0x70(a6)                   | +092
        cmpi.w  #0x0,0x70(a6)                   | +096
        bgt.w   .L07325a                        | +09c
        lea     M5Tank_Pause_07325e(pc),a1      | +0a0
        move.l  a1,(a6)                         | +0a4
.L07325a:
        bra.w   M5Tank_Phase3_0732e8__L073382   | +0a6

| ----------------------------------------------------------------------------
|  M5Tank_Pause_07325e  @ $07325E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Pause_07325e, "ax", @progbits
        .global M5Tank_Pause_07325e
M5Tank_Pause_07325e:
        move.w  #0x14,0x70(a6)                  | +000
        move.b  #0x2,0x75(a6)                   | +006
        move.l  #0x2dab18,0x4c(a6)              | +00c
        lea     .L073278(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L073278:
        subq.w  #0x1,0x70(a6)                   | +01a
        cmpi.w  #0x0,0x70(a6)                   | +01e
        bgt.w   .L07328c                        | +024
        lea     M5Tank_Phase2_073290(pc),a1     | +028
        move.l  a1,(a6)                         | +02c
.L07328c:
        bra.w   M5Tank_Phase2_073290__L0732ac   | +02e

| ----------------------------------------------------------------------------
|  M5Tank_Phase2_073290  @ $073290  (88 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Phase2_073290, "ax", @progbits
        .global M5Tank_Phase2_073290
M5Tank_Phase2_073290:
        move.w  #0x10a7,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2db5a4.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0732ac(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
        .global M5Tank_Phase2_073290__L0732ac
M5Tank_Phase2_073290__L0732ac:
.L0732ac:
        jsr     0x28998.l                       | +01c
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        bcc.w   .L0732c8                        | +02e
        lea     M5Tank_Phase3_0732e8(pc),a1     | +032
        move.l  a1,(a6)                         | +036
.L0732c8:
        jsr     0x2870a.l                       | +038
        bcc.w   .L0732e4                        | +03e
        bclr    #0x3,0x13(a6)                   | +042
        lea     0x5e766.l,a0                    | +048
        jsr     0x5e770.l                       | +04e
.L0732e4:
        bra.w   M5Tank_Phase3_0732e8__L0733d4   | +054

| ----------------------------------------------------------------------------
|  M5Tank_Phase3_0732e8  @ $0732E8  (282 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Phase3_0732e8, "ax", @progbits
        .global M5Tank_Phase3_0732e8
M5Tank_Phase3_0732e8:
        lea     0x2bb07e.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x90(a6)                     | +00c
        clr.b   0x92(a6)                        | +010
        move.w  #0x92,d0                        | +014
        lea     0xee1b4.l,a1                    | +018
        jsr     0x4429e.l                       | +01e
        lea     .L073312(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L073312:
        jsr     0x28998.l                       | +02a
        jsr     0x2783a.l                       | +030
        jsr     0x28d70.l                       | +036
        jsr     0x2870a.l                       | +03c
        bcc.w   .L073340                        | +042
        bclr    #0x3,0x13(a6)                   | +046
        lea     0x5e766.l,a0                    | +04c
        jsr     0x5e770.l                       | +052
.L073340:
        subq.w  #0x1,0x90(a6)                   | +058
        cmpi.w  #0xffff,0x90(a6)                | +05c
        bgt.w   .L073354                        | +062
        move.w  #0xffff,0x90(a6)                | +066
.L073354:
        jsr     0x28758.l                       | +06c
        bcc.w   .L07337e                        | +072
        cmpi.w  #0x0,0x90(a6)                   | +076
        bgt.w   .L073372                        | +07c
        lea     M5Tank_Die_07340a(pc),a1        | +080
        move.l  a1,(a6)                         | +084
        bra.w   .L07337e                        | +086
.L073372:
        bclr    #0x0,0x13(a6)                   | +08a
        move.w  #0x14,0x66(a6)                  | +090
.L07337e:
        bra.w   .L0733fc                        | +096
        .global M5Tank_Phase3_0732e8__L073382
M5Tank_Phase3_0732e8__L073382:
.L073382:
        jsr     0x2870a.l                       | +09a
        bcc.w   .L0733d4                        | +0a0
        bclr    #0x3,0x13(a6)                   | +0a4
        lea     0x5e766.l,a0                    | +0aa
        jsr     0x5e770.l                       | +0b0
        tst.b   0x75(a6)                        | +0b6
        bne.w   .L0733d4                        | +0ba
        move.w  0x66(a6),d0                     | +0be
        cmp.w   0x82(a6),d0                     | +0c2
        bgt.w   .L0733d4                        | +0c6
        move.b  #0x1,0x75(a6)                   | +0ca
        lea     0x2db268.l,a1                   | +0d0
        jsr     0x77c7e.l                       | +0d6
        move.w  #0x1032,d0                      | +0dc
        jsr     0x2352.l                        | +0e0
        lea     M5Tank_Retreat_0731b4(pc),a1    | +0e6
        move.l  a1,(a6)                         | +0ea
        .global M5Tank_Phase3_0732e8__L0733d4
M5Tank_Phase3_0732e8__L0733d4:
.L0733d4:
        jsr     0x28758.l                       | +0ec
        bcc.w   .L0733fc                        | +0f2
        move.w  #0x14,0x66(a6)                  | +0f6
        bclr    #0x0,0x13(a6)                   | +0fc
        bset    #0x0,0x5a(a6)                   | +102
        lea     0x5e766.l,a0                    | +108
        jsr     0x5e770.l                       | +10e
.L0733fc:
        jsr     0x283ca.l                       | +114

| ----------------------------------------------------------------------------
|  M5Tank_Die_07340a  @ $07340A  (66 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Die_07340a, "ax", @progbits
        .global M5Tank_Die_07340a
M5Tank_Die_07340a:
        jsr     M5Tank_SpawnDebrisD_0745b0(pc)  | +000
        clr.b   0x106ed3.l                      | +004
        move.w  #0x1033,d0                      | +00a
        jsr     0x2352.l                        | +00e
        move.b  #0x6,0x75(a6)                   | +014
        lea     0x2db6ce.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L073436(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L073436:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        bcc.w   SetHandlerRts_073452            | +038
        clr.b   0x106ed2.l                      | +03c

| ----------------------------------------------------------------------------
|  M5Tank_Sink_073454  @ $073454  (48 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Sink_073454, "ax", @progbits
        .global M5Tank_Sink_073454
M5Tank_Sink_073454:
        lea     0x2db6de.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L073466(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L073466:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L07347c                        | +01e
        lea     M5Tank_Explode_073484(pc),a1    | +022
        move.l  a1,(a6)                         | +026
.L07347c:
        move.b  #0xff,0x92(a6)                  | +028
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  M5Tank_Explode_073484  @ $073484  (110 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Explode_073484, "ax", @progbits
        .global M5Tank_Explode_073484
M5Tank_Explode_073484:
        addi.w  #0x20,0x24(a6)                  | +000
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
        jsr     M5Tank_SpawnSmokeColumn_07455a(pc) | +058
        subi.w  #0x20,0x24(a6)                  | +05c
        jsr     0x28d70.l                       | +062
        jmp     0x518.l                         | +068

| ----------------------------------------------------------------------------
|  M5Tank_Rts_0734f2  @ $0734F2  (2 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Rts_0734f2, "ax", @progbits
        .global M5Tank_Rts_0734f2
M5Tank_Rts_0734f2:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  M5Tank_Hull_07350a  @ $07350A  (148 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Hull_07350a, "ax", @progbits
        .global M5Tank_Hull_07350a
M5Tank_Hull_07350a:
        move.l  #0x2db15c,0x8c(a6)              | +000
        move.l  #0x2dadb4,0x48(a6)              | +008
        move.w  #0x64,0x66(a6)                  | +010
        move.w  #0xed,d1                        | +016
        jsr     0x236e.l                        | +01a
        move.w  #0x17,0x1c(a6)                  | +020
        jsr     0x138fe.l                       | +026
        lea     0x2db6ee.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L073548(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L073548:
        jsr     M5Tank_FollowParentSprite_0741f8(pc) | +03e
        jsr     0x28d70.l                       | +042
        jsr     0x2870a.l                       | +048
        bcc.w   .L07356e                        | +04e
        bclr    #0x3,0x13(a6)                   | +052
        lea     0x5e766.l,a0                    | +058
        jsr     0x5e770.l                       | +05e
.L07356e:
        jsr     0x28758.l                       | +064
        bcc.w   .L07357e                        | +06a
        lea     M5Tank_PartExplode_0735a6(pc),a1 | +06e
        move.l  a1,(a6)                         | +072
.L07357e:
        movea.l 0xc(a6),a0                      | +074
        tst.b   0x75(a0)                        | +078
        beq.w   .L073590                        | +07c
        lea     M5Tank_PartExplode_0735a6(pc),a1 | +080
        move.l  a1,(a6)                         | +084
.L073590:
        movea.l 0xc(a6),a0                      | +086
        cmpi.b  #0x6,0x76(a0)                   | +08a
        bne.w   SetHandlerRts_0735a4            | +090

| ----------------------------------------------------------------------------
|  M5Tank_PartExplode_0735a6  @ $0735A6  (18 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_PartExplode_0735a6, "ax", @progbits
        .global M5Tank_PartExplode_0735a6
M5Tank_PartExplode_0735a6:
        move.l  #0xffffffff,0x48(a6)            | +000
        addi.w  #0x40,0x24(a6)                  | +008
        bra.w   Sub_00077FD6                    | +00e  -> $077FD6 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  M5Tank_Cabin_0735b8  @ $0735B8  (200 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Cabin_0735b8, "ax", @progbits
        .global M5Tank_Cabin_0735b8
M5Tank_Cabin_0735b8:
        clr.b   0x20(a6)                        | +000
        lea     0x6a7de.l,a1                    | +004
        jsr     0x4ae.l                         | +00a
        jsr     0x5dd02.l                       | +010
        move.l  a0,0x88(a6)                     | +016
        move.w  #0x120,0x98(a0)                 | +01a
        move.b  #0xe0,0x9a(a0)                  | +020
        move.b  #0x38,0x9b(a0)                  | +026
        move.b  #0xff,0x9c(a0)                  | +02c
        clr.b   0x9d(a0)                        | +032
        move.b  #0x1,0x9e(a0)                   | +036
        move.l  #0x2db178,0x8c(a6)              | +03c
        move.w  #0xed,d1                        | +044
        jsr     0x236e.l                        | +048
        move.w  #0x17,0x1c(a6)                  | +04e
        jsr     0x138fe.l                       | +054
        lea     0x2db732.l,a0                   | +05a
        jsr     0x28cd4.l                       | +060
        lea     .L073624(pc),a1                 | +066
        move.l  a1,(a6)                         | +06a
.L073624:
        cmpi.b  #0x5,0x76(a6)                   | +06c
        bne.w   .L07363e                        | +072
        movea.l 0x88(a6),a0                     | +076
        bset    #0x0,0x13(a0)                   | +07a
        lea     .L07363e(pc),a1                 | +080
        move.l  a1,(a6)                         | +084
.L07363e:
        jsr     M5Tank_FollowParentSprite_0741f8(pc) | +086
        movea.l 0xc(a6),a0                      | +08a
        btst    #0x7,0x5a(a0)                   | +08e
        beq.w   .L073656                        | +094
        bset    #0x0,0x5a(a6)                   | +098
.L073656:
        jsr     0x28d70.l                       | +09e
        cmpi.b  #0x5,0x76(a6)                   | +0a4
        beq.w   .L073676                        | +0aa
        movea.l 0x88(a6),a0                     | +0ae
        move.b  #0x2,0x45(a0)                   | +0b2
        move.b  0x9b(a6),0x9b(a0)               | +0b8
.L073676:
        jsr     0x5e45a.l                       | +0be
        bcc.w   SetHandlerRts_073686            | +0c4

| ----------------------------------------------------------------------------
|  M5Tank_Launcher_073688  @ $073688  (204 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Launcher_073688, "ax", @progbits
        .global M5Tank_Launcher_073688
M5Tank_Launcher_073688:
        clr.b   0x79(a6)                        | +000
        lea     0x2bb286.l,a0                   | +004
        jsr     0x799de.l                       | +00a
        move.b  d0,0x72(a6)                     | +010
        lea     0x2bb308.l,a0                   | +014
        jsr     0x799de.l                       | +01a
        move.w  d0,0x70(a6)                     | +020
        move.l  #0x2db194,0x8c(a6)              | +024
        move.w  #0xee,d1                        | +02c
        jsr     0x236e.l                        | +030
        move.w  #0x18,0x1c(a6)                  | +036
        jsr     0x138fe.l                       | +03c
        lea     0x2db8d6.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        lea     .L0736dc(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L0736dc:
        jsr     M5Tank_FollowParentSprite_0741f8(pc) | +054
        movea.l 0xc(a6),a0                      | +058
        btst    #0x7,0x5a(a0)                   | +05c
        beq.w   .L0736f4                        | +062
        bset    #0x0,0x5a(a6)                   | +066
.L0736f4:
        jsr     0x28d70.l                       | +06c
        subq.w  #0x1,0x70(a6)                   | +072
        cmpi.w  #0x0,0x70(a6)                   | +076
        bgt.w   .L07374a                        | +07c
        jsr     M5Tank_SpawnRocket_074336(pc)   | +080
        lea     0x2bb38a.l,a0                   | +084
        jsr     0x799de.l                       | +08a
        move.w  d0,0x70(a6)                     | +090
        subq.b  #0x1,0x72(a6)                   | +094
        cmpi.b  #0x0,0x72(a6)                   | +098
        bgt.w   .L07374a                        | +09e
        lea     0x2bb286.l,a0                   | +0a2
        jsr     0x799de.l                       | +0a8
        move.b  d0,0x72(a6)                     | +0ae
        lea     0x2bb308.l,a0                   | +0b2
        jsr     0x799de.l                       | +0b8
        move.w  d0,0x70(a6)                     | +0be
.L07374a:
        jsr     0x5e45a.l                       | +0c2
        bcc.w   SetHandlerRts_07375a            | +0c8

| ----------------------------------------------------------------------------
|  M5Tank_Turret_07375c  @ $07375C  (160 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Turret_07375c, "ax", @progbits
        .global M5Tank_Turret_07375c
M5Tank_Turret_07375c:
        jsr     M5Tank_SpawnDebrisC_0745a2(pc)  | +000
        lea     0x2baffc.l,a0                   | +004
        jsr     0x799de.l                       | +00a
        move.w  d0,0x66(a6)                     | +010
        move.l  #0x2db0b0,0x60(a6)              | +014
        move.w  #0xec,d1                        | +01c
        jsr     0x236e.l                        | +020
        move.w  #0x17,0x1c(a6)                  | +026
        jsr     0x138fe.l                       | +02c
        lea     0x2dba40.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L0737a0(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L0737a0:
        jsr     0x28998.l                       | +044
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        bcc.w   .L0737bc                        | +056
        lea     M5Tank_TurretFire_073804(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
        .global M5Tank_Turret_07375c__L0737bc
M5Tank_Turret_07375c__L0737bc:
.L0737bc:
        jsr     0x2870a.l                       | +060
        bcc.w   .L0737de                        | +066
        jsr     0x519be.l                       | +06a
        bclr    #0x3,0x13(a6)                   | +070
        lea     0x5e766.l,a0                    | +076
        jsr     0x5e770.l                       | +07c
.L0737de:
        jsr     0x28758.l                       | +082
        bcc.w   .L0737ee                        | +088
        lea     M5Tank_TurretDie_0738d6(pc),a1  | +08c
        move.l  a1,(a6)                         | +090
.L0737ee:
        movea.l 0xc(a6),a0                      | +092
        cmpi.b  #0x5,0x76(a0)                   | +096
        bne.w   SetHandlerRts_073802            | +09c

| ----------------------------------------------------------------------------
|  M5Tank_TurretFire_073804  @ $073804  (16 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_TurretFire_073804, "ax", @progbits
        .global M5Tank_TurretFire_073804
M5Tank_TurretFire_073804:
        lea     M5Tank_Muzzle_073e5c(pc),a1     | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a

| ----------------------------------------------------------------------------
|  M5Tank_TurretAim_073814  @ $073814  (82 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_TurretAim_073814, "ax", @progbits
        .global M5Tank_TurretAim_073814
M5Tank_TurretAim_073814:
        move.b  #0x0,0x20(a6)                   | +000
        lea     0x2bb182.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.w  d0,0x70(a6)                     | +012
        lea     0x2dba5a.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L07383c(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L07383c:
        jsr     0x28998.l                       | +028
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        subq.w  #0x1,0x70(a6)                   | +03a
        cmpi.w  #0x0,0x70(a6)                   | +03e
        bgt.w   .L073862                        | +044
        lea     M5Tank_TurretRecoil_073866(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L073862:
        bra.w   M5Tank_Turret_07375c__L0737bc   | +04e

| ----------------------------------------------------------------------------
|  M5Tank_TurretRecoil_073866  @ $073866  (112 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_TurretRecoil_073866, "ax", @progbits
        .global M5Tank_TurretRecoil_073866
M5Tank_TurretRecoil_073866:
        clr.b   0x21(a6)                        | +000
        lea     0x2bb204.l,a0                   | +004
        jsr     0x799de.l                       | +00a
        move.w  d0,0x70(a6)                     | +010
        lea     .L073880(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L073880:
        subi.w  #0x1,0x70(a6)                   | +01a
        cmpi.w  #0x0,0x70(a6)                   | +020
        bgt.w   .L0738a8                        | +026
        move.b  #0x1,0x20(a6)                   | +02a
        lea     0x2dba66.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     .L0738a8(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L0738a8:
        jsr     0x28998.l                       | +042
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        tst.b   0x21(a6)                        | +054
        beq.w   .L0738c8                        | +058
        lea     M5Tank_TurretAim_073814(pc),a1  | +05c
        move.l  a1,(a6)                         | +060
.L0738c8:
        movea.l 0xc(a6),a0                      | +062
        move.b  #0x1,0x93(a0)                   | +066
        bra.w   M5Tank_Turret_07375c__L0737bc   | +06c

| ----------------------------------------------------------------------------
|  M5Tank_TurretDie_0738d6  @ $0738D6  (4 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_TurretDie_0738d6, "ax", @progbits
        .global M5Tank_TurretDie_0738d6
M5Tank_TurretDie_0738d6:
        jsr     M5Tank_SpawnScrap3_0744aa(pc)   | +000

| ----------------------------------------------------------------------------
|  M5Tank_TurretWreck_0738da  @ $0738DA  (64 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_TurretWreck_0738da, "ax", @progbits
        .global M5Tank_TurretWreck_0738da
M5Tank_TurretWreck_0738da:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0x5,0x75(a0)                   | +004
        move.l  #0xffffffff,0x60(a6)            | +00a
        move.w  #0x102c,d0                      | +012
        jsr     0x2352.l                        | +016
        move.l  #0xffffffff,0x48(a6)            | +01c
        lea     0x2dba94.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        lea     M5Tank_TurretWreck_Watch_07391a(pc),a1 | +030
        move.l  a1,(a6)                         | +034
        move.b  #0xff,0x92(a6)                  | +036
        bra.w   M5Tank_TurretWreck_Watch_07391a__L073934 | +03c

| ----------------------------------------------------------------------------
|  M5Tank_TurretWreck_Watch_07391a  @ $07391A  (48 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_TurretWreck_Watch_07391a, "ax", @progbits
        .global M5Tank_TurretWreck_Watch_07391a
M5Tank_TurretWreck_Watch_07391a:
        clr.b   0x92(a6)                        | +000
        move.w  #0x92,d0                        | +004
        lea     0xee1da.l,a1                    | +008
        jsr     0x4429e.l                       | +00e
        lea     .L073934(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
        .global M5Tank_TurretWreck_Watch_07391a__L073934
M5Tank_TurretWreck_Watch_07391a__L073934:
.L073934:
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        jsr     0x5e45a.l                       | +026
        bcc.w   SetHandlerRts_073950            | +02c

| ----------------------------------------------------------------------------
|  M5Tank_Gun_073952  @ $073952  (216 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Gun_073952, "ax", @progbits
        .global M5Tank_Gun_073952
M5Tank_Gun_073952:
        lea     0x2bb40c.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x70(a6)                     | +00c
        lea     0x2bb592.l,a0                   | +010
        jsr     0x799de.l                       | +016
        move.b  d0,0x72(a6)                     | +01c
        clr.b   0x79(a6)                        | +020
        lea     0x2bb614.l,a0                   | +024
        jsr     0x799de.l                       | +02a
        move.b  d0,0x98(a6)                     | +030
        move.w  #0xed,d1                        | +034
        jsr     0x236e.l                        | +038
        move.l  #0x2db1b0,0x8c(a6)              | +03e
        lea     0x2dbabc.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        .global M5Tank_Gun_073952__L0739a4
M5Tank_Gun_073952__L0739a4:
.L0739a4:
        lea     .L0739aa(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L0739aa:
        jsr     M5Tank_FollowParentSpriteB_07422c(pc) | +058
        movea.l 0xc(a6),a0                      | +05c
        move.b  0x93(a0),d0                     | +060
        clr.b   0x93(a0)                        | +064
        tst.b   d0                              | +068
        beq.w   .L0739d8                        | +06a
        clr.b   0x79(a6)                        | +06e
        lea     0x2bb614.l,a0                   | +072
        jsr     0x799de.l                       | +078
        move.b  d0,0x98(a6)                     | +07e
        bra.w   .L073a06                        | +082
.L0739d8:
        movea.l 0xc(a6),a0                      | +086
        cmpi.b  #0x2,0x75(a0)                   | +08a
        beq.w   .L073a06                        | +090
        cmpi.b  #0x3,0x75(a0)                   | +094
        beq.w   .L073a06                        | +09a
        subi.w  #0x1,0x70(a6)                   | +09e
        cmpi.w  #0x0,0x70(a6)                   | +0a4
        bgt.w   .L073a06                        | +0aa
        lea     M5Tank_GunFire_073a32(pc),a1    | +0ae
        move.l  a1,(a6)                         | +0b2
.L073a06:
        jsr     0x28d70.l                       | +0b4
        .global M5Tank_Gun_073952__L073a0c
M5Tank_Gun_073952__L073a0c:
.L073a0c:
        movea.l 0xc(a6),a0                      | +0ba
        cmpi.b  #0x6,0x75(a0)                   | +0be
        bne.w   .L073a20                        | +0c4
        lea     JmpToScheduler_0734f4(pc),a1    | +0c8
        move.l  a1,(a6)                         | +0cc
.L073a20:
        jsr     0x5e45a.l                       | +0ce
        bcc.w   SetHandlerRts_073a30            | +0d4

| ----------------------------------------------------------------------------
|  M5Tank_GunFire_073a32  @ $073A32  (188 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_GunFire_073a32, "ax", @progbits
        .global M5Tank_GunFire_073a32
M5Tank_GunFire_073a32:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x2,0x75(a0)                   | +004
        ble.w   .L073a54                        | +00a
        lea     0x2bb48e.l,a0                   | +00e
        jsr     0x799de.l                       | +014
        move.w  d0,0x70(a6)                     | +01a
        bra.w   .L073a64                        | +01e
.L073a54:
        lea     0x2bb510.l,a0                   | +022
        jsr     0x799de.l                       | +028
        move.w  d0,0x70(a6)                     | +02e
.L073a64:
        movea.l 0xc(a6),a0                      | +032
        move.b  0x76(a0),d0                     | +036
        andi.w  #0x1f,d0                        | +03a
        movea.l #0x2db1cc,a0                    | +03e
        lsl.w   #0x2,d0                         | +044
        movea.l (a0,d0.w),a0                    | +046
        cmpa.l  #0xffffffff,a0                  | +04a
        beq.w   .L073a8c                        | +050
        jsr     0x28cd4.l                       | +054
.L073a8c:
        lea     .L073a92(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L073a92:
        jsr     M5Tank_FollowParentSpriteB_07422c__L074234(pc) | +060
        jsr     0x28d70.l                       | +064
        bcc.w   .L073aea                        | +06a
        lea     M5Tank_Gun_073952__L0739a4(pc),a1 | +06e
        move.l  a1,(a6)                         | +072
        subi.b  #0x1,0x72(a6)                   | +074
        cmpi.b  #0x0,0x72(a6)                   | +07a
        bgt.w   .L073aea                        | +080
        clr.b   0x79(a6)                        | +084
        lea     0x2bb614.l,a0                   | +088
        jsr     0x799de.l                       | +08e
        move.b  d0,0x98(a6)                     | +094
        lea     0x2bb40c.l,a0                   | +098
        jsr     0x799de.l                       | +09e
        move.w  d0,0x70(a6)                     | +0a4
        lea     0x2bb592.l,a0                   | +0a8
        jsr     0x799de.l                       | +0ae
        move.b  d0,0x72(a6)                     | +0b4
.L073aea:
        bra.w   M5Tank_Gun_073952__L073a0c      | +0b8

| ----------------------------------------------------------------------------
|  M5Tank_Casing_073aee  @ $073AEE  (108 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Casing_073aee, "ax", @progbits
        .global M5Tank_Casing_073aee
M5Tank_Casing_073aee:
        move.w  #0xec,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x16,0x1c(a6)                  | +00a
        jsr     0x138fe.l                       | +010
        move.w  #0x8000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x14,0x38(a6)                  | +026
        lea     0x2dbd1c.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L073b2c(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L073b2c:
        jsr     0x2783a.l                       | +03e
        movea.l 0xc(a6),a0                      | +044
        btst    #0x7,0x5a(a0)                   | +048
        beq.w   .L073b46                        | +04e
        bset    #0x0,0x5a(a6)                   | +052
.L073b46:
        jsr     0x28d70.l                       | +058
        movea.l 0xc(a6),a0                      | +05e
        cmpi.b  #0x5,0x76(a0)                   | +062
        bne.w   SetHandlerRts_073b60            | +068

| ----------------------------------------------------------------------------
|  M5Tank_CasingGround_073b62  @ $073B62  (40 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_CasingGround_073b62, "ax", @progbits
        .global M5Tank_CasingGround_073b62
M5Tank_CasingGround_073b62:
        lea     0x2dbddc.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L073b74(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L073b74:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x5e45a.l                       | +01e
        bcc.w   SetHandlerRts_073b90            | +024

| ----------------------------------------------------------------------------
|  M5Tank_CasingExplode_073b92  @ $073B92  (26 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_CasingExplode_073b92, "ax", @progbits
        .global M5Tank_CasingExplode_073b92
M5Tank_CasingExplode_073b92:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        bra.w   AnimSeq_00077F6A                | +016

| ----------------------------------------------------------------------------
|  M5Tank_Rocket_Launch_073bac  @ $073BAC  (116 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Rocket_Launch_073bac, "ax", @progbits
        .global M5Tank_Rocket_Launch_073bac
M5Tank_Rocket_Launch_073bac:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0x23,d1                        | +006
        jsr     0x236e.l                        | +00a
        move.w  #0x10,0x7e(a6)                  | +010
        clr.w   0x7c(a6)                        | +016
        lea     0x2dbdee.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L073bd8(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L073bd8:
        jsr     M5Tank_FollowParentGun_074260(pc) | +02c
        jsr     0x28d70.l                       | +030
        bcc.w   .L073bf6                        | +036
        move.w  #0x10a4,d0                      | +03a
        jsr     0x2352.l                        | +03e
        lea     M5Tank_Rocket_Fly_073c28(pc),a1 | +044
        move.l  a1,(a6)                         | +048
        .global M5Tank_Rocket_Launch_073bac__L073bf6
M5Tank_Rocket_Launch_073bac__L073bf6:
.L073bf6:
        movea.l 0xc(a6),a0                      | +04a
        cmpi.b  #0x5,0x76(a0)                   | +04e
        bne.w   .L073c0a                        | +054
        lea     M5Tank_Rocket_Explode_073d76(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L073c0a:
        movea.l #0xffffffff,a0                  | +05e
        lea     0x2db010.l,a0                   | +064
        jsr     0x5dd56.l                       | +06a
        bcc.w   SetHandlerRts_073c26            | +070

| ----------------------------------------------------------------------------
|  M5Tank_Rocket_Fly_073c28  @ $073C28  (334 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Rocket_Fly_073c28, "ax", @progbits
        .global M5Tank_Rocket_Fly_073c28
M5Tank_Rocket_Fly_073c28:
        clr.w   0x80(a6)                        | +000
        move.w  #0x14,0x66(a6)                  | +004
        move.w  #0x400,0x36(a6)                 | +00a
        addi.w  #0xffc8,0x22(a6)                | +010
        addi.w  #0xf,0x24(a6)                   | +016
        lea     0x2db228.l,a0                   | +01c
        tst.b   0x98(a6)                        | +022
        bne.w   .L073c6a                        | +026
        movea.l 0xc(a6),a1                      | +02a
        movea.l 0xc(a1),a1                      | +02e
        cmpi.b  #0x5,0x75(a1)                   | +032
        bne.w   .L073c6a                        | +038
        lea     0x2db248.l,a0                   | +03c
.L073c6a:
        move.b  0x79(a6),d0                     | +042
        andi.w  #0xf,d0                         | +046
        add.w   d0,d0                           | +04a
        move.w  (a0,d0.w),d0                    | +04c
        andi.w  #0xf,d0                         | +050
        lea     0x2db1e8.l,a0                   | +054
        lsl.w   #0x2,d0                         | +05a
        move.l  (a0,d0.w),0x90(a6)              | +05c
        clr.w   0x94(a6)                        | +062
        clr.b   0x96(a6)                        | +066
        move.w  #0xd000,d0                      | +06a
        jsr     0x28134.l                       | +06e
        andi.w  #0xffe3,0x38(a6)                | +074
        ori.w   #0x14,0x38(a6)                  | +07a
        lea     0x2dbe40.l,a0                   | +080
        jsr     0x28cd4.l                       | +086
        lea     .L073cba(pc),a1                 | +08c
        move.l  a1,(a6)                         | +090
.L073cba:
        jsr     0x78f8a.l                       | +092
        bcc.w   .L073cca                        | +098
        lea     M5Tank_Rocket_Explode_073d76(pc),a1 | +09c
        move.l  a1,(a6)                         | +0a0
.L073cca:
        move.b  0x34(a6),d0                     | +0a2
        andi.w  #0xff,d0                        | +0a6
        lsr.w   #0x3,d0                         | +0aa
        move.w  d0,0x7e(a6)                     | +0ac
        jsr     0x28d70.l                       | +0b0
        .global M5Tank_Rocket_Fly_073c28__L073cde
M5Tank_Rocket_Fly_073c28__L073cde:
.L073cde:
        jsr     0x283d8.l                       | +0b6
        btst    #0x1,0x13(a6)                   | +0bc
        beq.w   .L073cf4                        | +0c2
        lea     M5Tank_CasingExplode_073b92(pc),a1 | +0c6
        move.l  a1,(a6)                         | +0ca
.L073cf4:
        jsr     0x2870a.l                       | +0cc
        bcc.w   .L073d10                        | +0d2
        lea     0x5e766.l,a0                    | +0d6
        jsr     0x5e770.l                       | +0dc
        bclr    #0x3,0x13(a6)                   | +0e2
.L073d10:
        jsr     0x28758.l                       | +0e8
        bcc.w   .L073d20                        | +0ee
        lea     M5Tank_Rocket_Explode_073d76(pc),a1 | +0f2
        move.l  a1,(a6)                         | +0f6
.L073d20:
        bra.w   M5Tank_Rocket_Launch_073bac__L073bf6 | +0f8
        move.w  #0xb4,0x70(a6)                  | +0fc
        andi.w  #0x1f,0x7e(a6)                  | +102
        jsr     M5Tank_VelByAngle7E_074306(pc)  | +108
        jsr     0x5e1ea.l                       | +10c
        move.l  a0,0x84(a6)                     | +112
        lea     .L073d44(pc),a1                 | +116
        move.l  a1,(a6)                         | +11a
.L073d44:
        jsr     M5Tank_Rocket_Home_07429e(pc)   | +11c
        jsr     0x27cee.l                       | +120
        bcc.w   .L073d58                        | +126
        lea     M5Tank_Rocket_Explode_073d76(pc),a1 | +12a
        move.l  a1,(a6)                         | +12e
.L073d58:
        jsr     0x28d70.l                       | +130
        subq.w  #0x1,0x70(a6)                   | +136
        cmpi.w  #0x0,0x70(a6)                   | +13a
        bgt.w   .L073d72                        | +140
        lea     M5Tank_Rocket_Explode_073d76(pc),a1 | +144
        move.l  a1,(a6)                         | +148
.L073d72:
        bra.w   .L073cde                        | +14a

| ----------------------------------------------------------------------------
|  M5Tank_Rocket_Explode_073d76  @ $073D76  (172 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Rocket_Explode_073d76, "ax", @progbits
        .global M5Tank_Rocket_Explode_073d76
M5Tank_Rocket_Explode_073d76:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   M5Tank_CasingExplode_073b92     | +00a
        move.w  #0x14,0x66(a6)                  | +00e
        addi.w  #0xffc8,0x22(a6)                | +014
        addi.w  #0xf,0x24(a6)                   | +01a
        move.b  0x79(a6),d0                     | +020
        andi.w  #0x3,d0                         | +024
        lsl.w   #0x2,d0                         | +028
        move.w  d0,0x70(a6)                     | +02a
        move.w  #0xfa00,0x28(a6)                | +02e
        move.w  #0xd000,d0                      | +034
        jsr     0x28134.l                       | +038
        andi.w  #0xffe3,0x38(a6)                | +03e
        ori.w   #0x14,0x38(a6)                  | +044
        lea     0x2dbe40.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     .L073dd2(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L073dd2:
        subq.w  #0x1,0x70(a6)                   | +05c
        cmpi.w  #0x0,0x70(a6)                   | +060
        bgt.w   .L073dec                        | +066
        move.w  #0xff60,0x2e(a6)                | +06a
        lea     .L073dec(pc),a1                 | +070
        move.l  a1,(a6)                         | +074
.L073dec:
        jsr     0x27d50.l                       | +076
        bcc.w   .L073dfc                        | +07c
        lea     M5Tank_Rocket_Explode_073d76(pc),a1 | +080
        move.l  a1,(a6)                         | +084
.L073dfc:
        jsr     0x28d70.l                       | +086
        move.w  0x28(a6),d0                     | +08c
        move.w  0x2a(a6),d1                     | +090
        asr.w   #0x4,d0                         | +094
        asr.w   #0x4,d1                         | +096
        jsr     0x5e018.l                       | +098
        lsr.w   #0x3,d0                         | +09e
        andi.w  #0x1f,d0                        | +0a0
        move.w  d0,0x7e(a6)                     | +0a4
        bra.w   M5Tank_Rocket_Fly_073c28__L073cde | +0a8

| ----------------------------------------------------------------------------
|  M5Tank_SmokeTrail_073e22  @ $073E22  (50 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SmokeTrail_073e22, "ax", @progbits
        .global M5Tank_SmokeTrail_073e22
M5Tank_SmokeTrail_073e22:
        move.w  #0x2000,0x38(a6)                | +000
        move.w  #0xd,d1                         | +006
        jsr     0x236e.l                        | +00a
        lea     0x2dc2e0.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L073e44(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L073e44:
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        bcc.w   SetHandlerRts_073e5a            | +02e

| ----------------------------------------------------------------------------
|  M5Tank_Muzzle_073e5c  @ $073E5C  (108 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Muzzle_073e5c, "ax", @progbits
        .global M5Tank_Muzzle_073e5c
M5Tank_Muzzle_073e5c:
        move.w  #0xef,d1                        | +000
        jsr     0x236e.l                        | +004
        .global M5Tank_Muzzle_073e5c__L073e66
M5Tank_Muzzle_073e5c__L073e66:
.L073e66:
        move.l  #0x2daf54,0x4c(a6)              | +00a
        lea     0x2dc374.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L073e80(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L073e80:
        jsr     M5Tank_FollowParentMuzzle_07428c(pc) | +024
        jsr     0x28d70.l                       | +028
        movea.l 0xc(a6),a0                      | +02e
        cmpi.b  #0x1,0x20(a0)                   | +032
        bne.w   .L073e9e                        | +038
        lea     M5Tank_MuzzleB_073ed0(pc),a1    | +03c
        move.l  a1,(a6)                         | +040
        .global M5Tank_Muzzle_073e5c__L073e9e
M5Tank_Muzzle_073e5c__L073e9e:
.L073e9e:
        jsr     0x283ca.l                       | +042
        jsr     0x283d8.l                       | +048
        movea.l 0xc(a6),a0                      | +04e
        cmpi.l  #0x7391a,(a0)                   | +052
        bne.w   .L073ebe                        | +058
        lea     M5Tank_MuzzleE_073f9a(pc),a1    | +05c
        move.l  a1,(a6)                         | +060
.L073ebe:
        jsr     0x5e45a.l                       | +062
        bcc.w   SetHandlerRts_073ece            | +068

| ----------------------------------------------------------------------------
|  M5Tank_MuzzleB_073ed0  @ $073ED0  (40 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_MuzzleB_073ed0, "ax", @progbits
        .global M5Tank_MuzzleB_073ed0
M5Tank_MuzzleB_073ed0:
        lea     0x2dc406.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L073ee2(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L073ee2:
        jsr     M5Tank_FollowParentMuzzle_07428c(pc) | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L073ef6                        | +01c
        lea     M5Tank_MuzzleC_073ef8(pc),a1    | +020
        move.l  a1,(a6)                         | +024
.L073ef6:
        bra.b   M5Tank_Muzzle_073e5c__L073e9e   | +026

| ----------------------------------------------------------------------------
|  M5Tank_MuzzleC_073ef8  @ $073EF8  (102 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_MuzzleC_073ef8, "ax", @progbits
        .global M5Tank_MuzzleC_073ef8
M5Tank_MuzzleC_073ef8:
        move.w  #0x10c6,d0                      | +000
        jsr     0x2352.l                        | +004
        move.l  #0x2daeb0,0x4c(a6)              | +00a
        lea     0x2bb100.l,a0                   | +012
        jsr     0x799de.l                       | +018
        move.b  d0,0x72(a6)                     | +01e
        lea     0x2dc4bc.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        lea     .L073f2c(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L073f2c:
        jsr     M5Tank_FollowParentMuzzle_07428c(pc) | +034
        jsr     0x28d70.l                       | +038
        bcc.w   .L073f5a                        | +03e
        lea     0x2dc50c.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        subq.b  #0x1,0x72(a6)                   | +04e
        cmpi.b  #0x0,0x72(a6)                   | +052
        bgt.w   .L073f5a                        | +058
        lea     M5Tank_MuzzleD_073f5e(pc),a1    | +05c
        move.l  a1,(a6)                         | +060
.L073f5a:
        bra.w   M5Tank_Muzzle_073e5c__L073e9e   | +062

| ----------------------------------------------------------------------------
|  M5Tank_MuzzleD_073f5e  @ $073F5E  (60 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_MuzzleD_073f5e, "ax", @progbits
        .global M5Tank_MuzzleD_073f5e
M5Tank_MuzzleD_073f5e:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0xff,0x21(a0)                  | +004
        move.l  #0xffffffff,0x4c(a6)            | +00a
        lea     0x2dc5ae.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L073f82(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L073f82:
        jsr     M5Tank_FollowParentMuzzle_07428c(pc) | +024
        jsr     0x28d70.l                       | +028
        bcc.w   .L073f96                        | +02e
        lea     M5Tank_Muzzle_073e5c__L073e66(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L073f96:
        bra.w   M5Tank_Muzzle_073e5c__L073e9e   | +038

| ----------------------------------------------------------------------------
|  M5Tank_MuzzleE_073f9a  @ $073F9A  (40 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_MuzzleE_073f9a, "ax", @progbits
        .global M5Tank_MuzzleE_073f9a
M5Tank_MuzzleE_073f9a:
        move.l  #0xffffffff,0x4c(a6)            | +000
        lea     0x2dc5ae.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        lea     .L073fb4(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L073fb4:
        jsr     M5Tank_FollowParentMuzzle_07428c(pc) | +01a
        jsr     0x28d70.l                       | +01e
        bcc.w   SetHandlerRts_073fc8            | +024

| ----------------------------------------------------------------------------
|  M5Missile_Tmpl77_073fca  @ $073FCA  (146 B)
| ----------------------------------------------------------------------------
        .section .text.M5Missile_Tmpl77_073fca, "ax", @progbits
        .global M5Missile_Tmpl77_073fca
M5Missile_Tmpl77_073fca:
        move.b  0x98(a6),d0                     | +000
        andi.w  #0xff,d0                        | +004
        lsl.w   #0x4,d0                         | +008
        move.w  d0,0x36(a6)                     | +00a
        neg.w   d0                              | +00e
        move.w  d0,0x28(a6)                     | +010
        move.b  0x99(a6),d0                     | +014
        andi.w  #0xff,d0                        | +018
        lsl.w   #0x4,d0                         | +01c
        move.w  d0,0x5c(a6)                     | +01e
        move.w  #0x23,d1                        | +022
        jsr     0x236e.l                        | +026
        bset    #0x4,0x6b(a6)                   | +02c
        lea     0x2dbe40.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L07400e(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L07400e:
        move.w  0x5c(a6),d0                     | +044
        cmp.w   0x22(a6),d0                     | +048
        blt.w   .L074026                        | +04c
        move.w  #0xffe0,0x2e(a6)                | +050
        lea     .L074026(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L074026:
        jsr     0x27d50.l                       | +05c
        bcc.w   .L074036                        | +062
        lea     M5Tank_Rocket_Explode_073d76(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L074036:
        move.w  0x28(a6),d0                     | +06c
        move.w  0x2a(a6),d1                     | +070
        asr.w   #0x4,d0                         | +074
        asr.w   #0x4,d1                         | +076
        jsr     0x5e018.l                       | +078
        lsr.w   #0x3,d0                         | +07e
        andi.w  #0x1f,d0                        | +080
        move.w  d0,0x7e(a6)                     | +084
        jsr     0x28d70.l                       | +088
        bra.w   M5Tank_Rocket_Fly_073c28__L073cde | +08e

| ----------------------------------------------------------------------------
|  M5Tank_Frag_07405c  @ $07405C  (104 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Frag_07405c, "ax", @progbits
        .global M5Tank_Frag_07405c
M5Tank_Frag_07405c:
        move.w  #0xd000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x14,0x38(a6)                  | +010
        movea.l 0xc(a6),a0                      | +016
        move.b  0x98(a0),d0                     | +01a
        andi.w  #0xff,d0                        | +01e
        lea     0x2bb696.l,a0                   | +022
        movea.l 0xc(a6),a1                      | +028
        movea.l 0xc(a1),a1                      | +02c
        cmpi.b  #0x3,0x75(a1)                   | +030
        ble.w   .L07409c                        | +036
        lea     0x2bb6a6.l,a0                   | +03a
.L07409c:
        lsl.w   #0x2,d0                         | +040
        movea.l (a0,d0.w),a0                    | +042
        move.b  0x79(a6),d0                     | +046
        andi.w  #0x7,d0                         | +04a
        lsl.w   #0x2,d0                         | +04e
        move.w  (a0,d0.w),0x28(a6)              | +050
        move.w  0x2(a0,d0.w),0x2e(a6)           | +056
        move.w  #0x100,0x2a(a6)                 | +05c
        jmp     0x6dbd4.l                       | +062

| ----------------------------------------------------------------------------
|  M5Tank_SmokeB_0740c4  @ $0740C4  (66 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SmokeB_0740c4, "ax", @progbits
        .global M5Tank_SmokeB_0740c4
M5Tank_SmokeB_0740c4:
        move.w  #0x193,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2dc616.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0740e0(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0740e0:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L0740f6                        | +028
        lea     Jsr5B6ThenJmpScheduler_0734fc(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L0740f6:
        movea.l #0xffffffff,a0                  | +032
        jsr     0x5dd56.l                       | +038
        bcc.w   SetHandlerRts_07410c            | +03e

| ----------------------------------------------------------------------------
|  M5Tank_InScreenSetC_07410e  @ $07410E  (8 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_InScreenSetC_07410e, "ax", @progbits
        .global M5Tank_InScreenSetC_07410e
M5Tank_InScreenSetC_07410e:
        jsr     M5Tank_InScreen_074122(pc)      | +000
        bcs.w   SetXN_07411c                    | +004

| ----------------------------------------------------------------------------
|  M5Tank_InScreen_074122  @ $074122  (56 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_InScreen_074122, "ax", @progbits
        .global M5Tank_InScreen_074122
M5Tank_InScreen_074122:
        btst    #0x0,0x73(a6)                   | +000
        bne.w   .L07413a                        | +006
        cmpi.w  #0xa0,0x22(a6)                  | +00a
        blt.w   SetXN_074160                    | +010
        bra.w   ClearXN_07415a                  | +014
.L07413a:
        cmpi.w  #0x118,0x22(a6)                 | +018
        bgt.w   SetXN_074160                    | +01e
        move.w  #0x1050,d0                      | +022
        move.w  #0xcf,d1                        | +026
        jsr     0x440d0.l                       | +02a
        cmp.w   0x22(a6),d0                     | +030
        blt.w   SetXN_074160                    | +034

| ----------------------------------------------------------------------------
|  M5Tank_TargetNear_074166  @ $074166  (36 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_TargetNear_074166, "ax", @progbits
        .global M5Tank_TargetNear_074166
M5Tank_TargetNear_074166:
        btst    #0x0,0x73(a6)                   | +000
        bne.w   ClearXN_07418a                  | +006
        jsr     0x5e0d4.l                       | +00a
        bcs.w   ClearXN_07418a                  | +010
        move.w  0x22(a6),d0                     | +014
        sub.w   0x22(a0),d0                     | +018
        cmpi.w  #0x98,d0                        | +01c
        blt.w   SetXN_074190                    | +020

| ----------------------------------------------------------------------------
|  M5Tank_AtGoalX_074196  @ $074196  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_AtGoalX_074196, "ax", @progbits
        .global M5Tank_AtGoalX_074196
M5Tank_AtGoalX_074196:
        move.w  0x5c(a6),d0                     | +000
        move.w  d0,d1                           | +004
        subi.w  #0x10,d0                        | +006
        addi.w  #0x10,d1                        | +00a
        cmp.w   0x22(a6),d0                     | +00e
        bge.w   ClearXN_0741ba                  | +012
        cmp.w   0x22(a6),d1                     | +016
        ble.w   ClearXN_0741ba                  | +01a

| ----------------------------------------------------------------------------
|  M5Tank_FollowParent_0741c0  @ $0741C0  (56 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_FollowParent_0741c0, "ax", @progbits
        .global M5Tank_FollowParent_0741c0
M5Tank_FollowParent_0741c0:
        jsr     0x5e506.l                       | +000
        move.w  0x7c(a6),d0                     | +006
        cmpi.b  #0x3,0x75(a0)                   | +00a
        bcs.w   .L0741d6                        | +010
        addq.w  #0x2,d0                         | +014
.L0741d6:
        add.w   d0,0x38(a6)                     | +016
        move.b  0x77(a0),d0                     | +01a
        move.b  0x78(a0),d1                     | +01e
        move.b  d0,0x77(a6)                     | +022
        move.b  d1,0x78(a6)                     | +026
        ext.w   d0                              | +02a
        ext.w   d1                              | +02c
        add.w   d0,0x22(a6)                     | +02e
        add.w   d1,0x24(a6)                     | +032
        rts                                     | +036

| ----------------------------------------------------------------------------
|  M5Tank_FollowParentSprite_0741f8  @ $0741F8  (44 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_FollowParentSprite_0741f8, "ax", @progbits
        .global M5Tank_FollowParentSprite_0741f8
M5Tank_FollowParentSprite_0741f8:
        jsr     M5Tank_FollowParent_0741c0(pc)  | +000
        move.b  0x76(a0),d0                     | +004
        cmp.b   0x76(a6),d0                     | +008
        beq.w   JsrAbsRts_07422a                | +00c
        move.b  d0,0x76(a6)                     | +010
        andi.w  #0x1f,d0                        | +014
        movea.l 0x8c(a6),a0                     | +018
        lsl.w   #0x2,d0                         | +01c
        movea.l (a0,d0.w),a0                    | +01e
        cmpa.l  #0xffffffff,a0                  | +022
        beq.w   JsrAbsRts_07422a                | +028

| ----------------------------------------------------------------------------
|  M5Tank_FollowParentSpriteB_07422c  @ $07422C  (52 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_FollowParentSpriteB_07422c, "ax", @progbits
        .global M5Tank_FollowParentSpriteB_07422c
M5Tank_FollowParentSpriteB_07422c:
        jsr     M5Tank_FollowParentSprite_0741f8(pc) | +000
        bra.w   .L074238                        | +004
        .global M5Tank_FollowParentSpriteB_07422c__L074234
M5Tank_FollowParentSpriteB_07422c__L074234:
.L074234:
        jsr     M5Tank_FollowParent_0741c0(pc)  | +008
.L074238:
        move.w  #0x58,d0                        | +00c
        movea.l 0xc(a6),a0                      | +010
        cmpi.b  #0x4,0x75(a0)                   | +014
        blt.w   .L07424e                        | +01a
        move.w  #0x76,d0                        | +01e
.L07424e:
        add.w   d0,0x24(a6)                     | +022
        addi.w  #0xffe0,0x22(a6)                | +026
        move.w  #0xc000,0x38(a6)                | +02c
        rts                                     | +032

| ----------------------------------------------------------------------------
|  M5Tank_FollowParentGun_074260  @ $074260  (44 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_FollowParentGun_074260, "ax", @progbits
        .global M5Tank_FollowParentGun_074260
M5Tank_FollowParentGun_074260:
        jsr     M5Tank_FollowParent_0741c0(pc)  | +000
        move.b  0x7a(a0),d0                     | +004
        move.w  #0x2,d1                         | +008
        btst    #0x0,0x79(a6)                   | +00c
        beq.w   .L07427e                        | +012
        move.b  0x7b(a0),d0                     | +016
        move.w  #0xfff6,d1                      | +01a
.L07427e:
        andi.w  #0xff,d0                        | +01e
        add.w   d0,0x24(a6)                     | +022
        add.w   d1,0x22(a6)                     | +026
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  M5Tank_FollowParentMuzzle_07428c  @ $07428C  (18 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_FollowParentMuzzle_07428c, "ax", @progbits
        .global M5Tank_FollowParentMuzzle_07428c
M5Tank_FollowParentMuzzle_07428c:
        jsr     0x5e506.l                       | +000
        subi.w  #0x30,0x22(a6)                  | +006
        subq.w  #0x8,0x24(a6)                   | +00c
        rts                                     | +010

| ----------------------------------------------------------------------------
|  M5Tank_Rocket_Home_07429e  @ $07429E  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Rocket_Home_07429e, "ax", @progbits
        .global M5Tank_Rocket_Home_07429e
M5Tank_Rocket_Home_07429e:
        move.b  0x106f28.l,d0                   | +000
        andi.b  #0x7,d0                         | +006
        bne.w   M5Tank_Rocket_RandFlip_0742ee   | +00a
        movea.l 0x84(a6),a0                     | +00e
        jsr     0x5e070.l                       | +012
        lsr.w   #0x3,d0                         | +018
        move.w  #0x1,d1                         | +01a
        cmp.w   0x7e(a6),d0                     | +01e
        beq.w   M5Tank_Rocket_RandFlip_0742ee   | +022
        bgt.w   .L0742cc                        | +026
        move.w  #0xffff,d1                      | +02a
.L0742cc:
        cmpi.w  #0x0,0x80(a6)                   | +02e
        beq.w   .L0742da                        | +034
        move.w  0x80(a6),d1                     | +038
.L0742da:
        add.w   d1,0x7e(a6)                     | +03c
        andi.w  #0x1f,0x7e(a6)                  | +040
        jsr     M5Tank_VelByAngle7E_074306(pc)  | +046

| ----------------------------------------------------------------------------
|  M5Tank_Rocket_RandFlip_0742ee  @ $0742EE  (18 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_Rocket_RandFlip_0742ee, "ax", @progbits
        .global M5Tank_Rocket_RandFlip_0742ee
M5Tank_Rocket_RandFlip_0742ee:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x1f,d0                        | +006
        bne.w   ClearXN_074300                  | +00a
        neg.w   0x80(a6)                        | +00e

| ----------------------------------------------------------------------------
|  M5Tank_VelByAngle7E_074306  @ $074306  (48 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_VelByAngle7E_074306, "ax", @progbits
        .global M5Tank_VelByAngle7E_074306
M5Tank_VelByAngle7E_074306:
        move.w  0x7e(a6),d0                     | +000
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
|  M5Tank_SpawnRocket_074336  @ $074336  (100 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnRocket_074336, "ax", @progbits
        .global M5Tank_SpawnRocket_074336
M5Tank_SpawnRocket_074336:
        cmpi.b  #0x5,0x76(a6)                   | +000
        beq.w   .L074398                        | +006
        clr.b   0x94(a6)                        | +00a
        movea.l 0xc(a6),a1                      | +00e
        cmpi.b  #0x5,0x75(a1)                   | +012
        blt.w   .L074398                        | +018
        movea.l 0xc(a6),a1                      | +01c
        cmpi.b  #0x2,0x75(a1)                   | +020
        beq.w   .L074398                        | +026
        cmpi.b  #0x3,0x75(a1)                   | +02a
        beq.w   .L074398                        | +030
        cmpi.b  #0x4,0x75(a1)                   | +034
        bne.w   .L074378                        | +03a
        clr.b   0x94(a6)                        | +03e
.L074378:
        lea     M5Tank_Rocket_Launch_073bac(pc),a1 | +042
        jsr     0x4ae.l                         | +046
        jsr     0x5dd02.l                       | +04c
        move.b  0x94(a6),0x98(a0)               | +052
        move.b  0x79(a6),0x79(a0)               | +058
        addq.b  #0x1,0x79(a6)                   | +05e
.L074398:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  M5Tank_SpawnCasing_07439a  @ $07439A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnCasing_07439a, "ax", @progbits
        .global M5Tank_SpawnCasing_07439a
M5Tank_SpawnCasing_07439a:
        lea     M5Tank_Casing_073aee(pc),a1     | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  M5Tank_SpawnTurret_0743ac  @ $0743AC  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnTurret_0743ac, "ax", @progbits
        .global M5Tank_SpawnTurret_0743ac
M5Tank_SpawnTurret_0743ac:
        move.l  #0x2db064,0x60(a6)              | +000
        lea     M5Tank_Turret_07375c(pc),a1     | +008
        jsr     0x4ae.l                         | +00c
        jsr     0x5dd02.l                       | +012
        clr.w   0x7c(a0)                        | +018
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  M5Tank_SpawnExplosion_0743ca  @ $0743CA  (12 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnExplosion_0743ca, "ax", @progbits
        .global M5Tank_SpawnExplosion_0743ca
M5Tank_SpawnExplosion_0743ca:
        lea     0x77f6a.l,a1                    | +000
        jsr     0x4ae.l                         | +006

| ----------------------------------------------------------------------------
|  M5Tank_SpawnExplosionB_0743de  @ $0743DE  (34 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnExplosionB_0743de, "ax", @progbits
        .global M5Tank_SpawnExplosionB_0743de
M5Tank_SpawnExplosionB_0743de:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x77fd6.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        lea     0x2db28c.l,a1                   | +01c

| ----------------------------------------------------------------------------
|  M5Tank_SpawnDebrisB_074408  @ $074408  (6 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnDebrisB_074408, "ax", @progbits
        .global M5Tank_SpawnDebrisB_074408
M5Tank_SpawnDebrisB_074408:
        lea     0x2db27a.l,a1                   | +000

| ----------------------------------------------------------------------------
|  M5Tank_SpawnSmokeAndFrag_074416  @ $074416  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnSmokeAndFrag_074416, "ax", @progbits
        .global M5Tank_SpawnSmokeAndFrag_074416
M5Tank_SpawnSmokeAndFrag_074416:
        move.w  #0x10ea,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     M5Tank_SmokeB_0740c4(pc),a1     | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        lea     M5Tank_Frag_07405c(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        addi.w  #0xffe4,0x22(a0)                | +02a
        addi.w  #0xfffc,0x24(a0)                | +030
        move.b  0x79(a6),0x79(a0)               | +036
        addi.b  #0x1,0x79(a6)                   | +03c
        andi.b  #0x7,0x79(a6)                   | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  M5Tank_SmokeByVel_Lut_074460  @ $074460  (74 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SmokeByVel_Lut_074460, "ax", @progbits
        .global M5Tank_SmokeByVel_Lut_074460
M5Tank_SmokeByVel_Lut_074460:
        .dc.w   0x180e                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0a07                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0504                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0403                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0302                        | +008  (dato / opcode no decodificado)
        clr.b   d1                              | +00a
        move.w  0x36(a6),d0                     | +00c
        cmpi.w  #0x400,d0                       | +010
        bgt.w   .L07448a                        | +014
        lsr.w   #0x7,d0                         | +018
        lea     M5Tank_SmokeByVel_Lut_074460(pc),a1 | +01a
        move.b  (a1,d0.w),d1                    | +01e
        cmp.b   0x3b(a6),d1                     | +022
        bcc.w   .L0744a8                        | +026
.L07448a:
        move.b  d1,0x3b(a6)                     | +02a
        lea     M5Tank_SmokeTrail_073e22(pc),a1 | +02e
        jsr     0x6fe.l                         | +032
        jsr     0x5dd02.l                       | +038
        move.b  0x3b(a6),0x44(a0)               | +03e
        clr.b   0x3b(a6)                        | +044
.L0744a8:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  M5Tank_SpawnScrap3_0744aa  @ $0744AA  (176 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnScrap3_0744aa, "ax", @progbits
        .global M5Tank_SpawnScrap3_0744aa
M5Tank_SpawnScrap3_0744aa:
        lea     0x3fe5a.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        clr.b   0x98(a0)                        | +012
        clr.b   0x9a(a0)                        | +016
        move.b  #0x1,0x9b(a0)                   | +01a
        move.b  #0xc8,0x9c(a0)                  | +020
        clr.b   0x9d(a0)                        | +026
        addi.w  #0x20,0x24(a0)                  | +02a
        subi.w  #0x28,0x22(a0)                  | +030
        lea     0x3fe5a.l,a1                    | +036
        jsr     0x4ae.l                         | +03c
        jsr     0x5dd02.l                       | +042
        clr.b   0x98(a0)                        | +048
        clr.b   0x9a(a0)                        | +04c
        move.b  #0xa,0x9b(a0)                   | +050
        move.b  #0x0,0x9c(a0)                   | +056
        move.b  #0x28,0x9d(a0)                  | +05c
        addi.w  #0x20,0x24(a0)                  | +062
        subq.w  #0x8,0x22(a0)                   | +068
        move.b  #0x4,0x44(a0)                   | +06c
        lea     0x3fe5a.l,a1                    | +072
        jsr     0x4ae.l                         | +078
        jsr     0x5dd02.l                       | +07e
        clr.b   0x98(a0)                        | +084
        clr.b   0x9a(a0)                        | +088
        move.b  #0x6,0x9b(a0)                   | +08c
        move.b  #0xa,0x9c(a0)                   | +092
        clr.b   0x9d(a0)                        | +098
        addi.w  #0x20,0x24(a0)                  | +09c
        addi.w  #0x18,0x22(a0)                  | +0a2
        move.b  #0x8,0x44(a0)                   | +0a8
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  M5Tank_SpawnSmokeColumn_07455a  @ $07455A  (48 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnSmokeColumn_07455a, "ax", @progbits
        .global M5Tank_SpawnSmokeColumn_07455a
M5Tank_SpawnSmokeColumn_07455a:
        jsr     0x6e412.l                       | +000
        move.w  #0x10,d0                        | +006
.L074564:
        movem.w d0,-(a7)                        | +00a
        lea     0x62536.l,a1                    | +00e
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        addi.w  #0x18,0x24(a0)                  | +020
        movem.w (a7)+,d0                        | +026
        subq.w  #0x1,d0                         | +02a
        bcc.b   .L074564                        | +02c
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  M5Tank_SetPrioD000_07458a  @ $07458A  (24 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SetPrioD000_07458a, "ax", @progbits
        .global M5Tank_SetPrioD000_07458a
M5Tank_SetPrioD000_07458a:
        move.w  #0xd000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        rts                                     | +016

| ----------------------------------------------------------------------------
|  M5Tank_SpawnDebrisC_0745a2  @ $0745A2  (6 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnDebrisC_0745a2, "ax", @progbits
        .global M5Tank_SpawnDebrisC_0745a2
M5Tank_SpawnDebrisC_0745a2:
        lea     0x2db2de.l,a1                   | +000

| ----------------------------------------------------------------------------
|  M5Tank_SpawnDebrisD_0745b0  @ $0745B0  (6 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_SpawnDebrisD_0745b0, "ax", @progbits
        .global M5Tank_SpawnDebrisD_0745b0
M5Tank_SpawnDebrisD_0745b0:
        lea     0x2db364.l,a1                   | +000

| ----------------------------------------------------------------------------
|  M5Tank_CmpDepthToParent_0745be  @ $0745BE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.M5Tank_CmpDepthToParent_0745be, "ax", @progbits
        .global M5Tank_CmpDepthToParent_0745be
M5Tank_CmpDepthToParent_0745be:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0745d4                    | +00c

| ----------------------------------------------------------------------------
|  MiniScript_Init_0745da  @ $0745DA  (8 B)
| ----------------------------------------------------------------------------
        .section .text.MiniScript_Init_0745da, "ax", @progbits
        .global MiniScript_Init_0745da
MiniScript_Init_0745da:
        move.l  a1,(a0)                         | +000
        clr.w   0x4(a0)                         | +002
        rts                                     | +006

| ----------------------------------------------------------------------------
|  MiniScript_Step_0745e2  @ $0745E2  (66 B)
| ----------------------------------------------------------------------------
        .section .text.MiniScript_Step_0745e2, "ax", @progbits
        .global MiniScript_Step_0745e2
MiniScript_Step_0745e2:
        movea.l (a0),a1                         | +000
        move.w  (a1)+,d0                        | +002
        lsl.w   #0x2,d0                         | +004
        lea     .L0745f0(pc),a2                 | +006
        jmp     (a2,d0.w)                       | +00a
.L0745f0:
        bra.w   SetC_074624                     | +00e
        bra.w   MiniScript_OpSetTimer_07462a    | +012
        bra.w   MiniScript_OpWait_074632        | +016
        bra.w   MiniScript_OpCall_07464a        | +01a
        bra.w   MiniScript_OpCallLoop_074668    | +01e
        bra.w   MiniScript_OpSetFlag_074652     | +022
        bra.w   MiniScript_OpClrFlag_07465c     | +026
        bra.w   .L074610                        | +02a
.L074610:
        move.w  (a1)+,d0                        | +02e
        move.l  a1,(a0)                         | +030
        movem.l a0,-(a7)                        | +032
        jsr     0x2352.l                        | +036
        movem.l (a7)+,a0                        | +03c
        bra.b   MiniScript_Step_0745e2          | +040

| ----------------------------------------------------------------------------
|  MiniScript_OpSetTimer_07462a  @ $07462A  (8 B)
| ----------------------------------------------------------------------------
        .section .text.MiniScript_OpSetTimer_07462a, "ax", @progbits
        .global MiniScript_OpSetTimer_07462a
MiniScript_OpSetTimer_07462a:
        move.w  (a1)+,0x4(a0)                   | +000
        move.l  a1,(a0)                         | +004
        bra.b   MiniScript_Step_0745e2          | +006

| ----------------------------------------------------------------------------
|  MiniScript_OpWait_074632  @ $074632  (18 B)
| ----------------------------------------------------------------------------
        .section .text.MiniScript_OpWait_074632, "ax", @progbits
        .global MiniScript_OpWait_074632
MiniScript_OpWait_074632:
        move.w  0x4(a0),d0                      | +000
        bne.w   .L07463e                        | +004
        move.l  a1,(a0)                         | +008
        bra.b   MiniScript_Step_0745e2          | +00a
.L07463e:
        subq.w  #0x1,d0                         | +00c
        move.w  d0,0x4(a0)                      | +00e

| ----------------------------------------------------------------------------
|  MiniScript_OpCall_07464a  @ $07464A  (2 B)
| ----------------------------------------------------------------------------
        .section .text.MiniScript_OpCall_07464a, "ax", @progbits
        .global MiniScript_OpCall_07464a
MiniScript_OpCall_07464a:
        jsr     (a1)                            | +000

| ----------------------------------------------------------------------------
|  MiniScript_OpSetFlag_074652  @ $074652  (10 B)
| ----------------------------------------------------------------------------
        .section .text.MiniScript_OpSetFlag_074652, "ax", @progbits
        .global MiniScript_OpSetFlag_074652
MiniScript_OpSetFlag_074652:
        move.w  (a1)+,d0                        | +000
        move.l  a1,(a0)                         | +002
        st.b    (a6,d0.w)                       | +004
        bra.b   MiniScript_Step_0745e2          | +008

| ----------------------------------------------------------------------------
|  MiniScript_OpClrFlag_07465c  @ $07465C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.MiniScript_OpClrFlag_07465c, "ax", @progbits
        .global MiniScript_OpClrFlag_07465c
MiniScript_OpClrFlag_07465c:
        move.w  (a1)+,d0                        | +000
        move.l  a1,(a0)                         | +002
        clr.b   (a6,d0.w)                       | +004
        bra.w   MiniScript_Step_0745e2          | +008

| ----------------------------------------------------------------------------
|  MiniScript_OpCallLoop_074668  @ $074668  (6 B)
| ----------------------------------------------------------------------------
        .section .text.MiniScript_OpCallLoop_074668, "ax", @progbits
        .global MiniScript_OpCallLoop_074668
MiniScript_OpCallLoop_074668:
        jsr     (a1)                            | +000
        bra.w   MiniScript_Step_0745e2          | +002

| ----------------------------------------------------------------------------
|  ScriptedProp_Sprites_07466e  @ $07466E  (928 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Sprites_07466e, "ax", @progbits
        .global ScriptedProp_Sprites_07466e
ScriptedProp_Sprites_07466e:
        .dc.w   0x0003                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .global ScriptedProp_Sprites_07466e__L0746be
ScriptedProp_Sprites_07466e__L0746be:
.L0746be:
        .dc.w   0x0003                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +064  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +070  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x8001                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +0da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +0ee  (dato / opcode no decodificado)
        .global ScriptedProp_Sprites_07466e__L07475e
ScriptedProp_Sprites_07466e__L07475e:
.L07475e:
        .dc.w   0x005b                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x010b                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x010c                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x005d                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x010d                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x005e                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x010e                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x005f                        | +100  (dato / opcode no decodificado)
        .dc.w   0x010f                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0110                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0072                        | +108  (dato / opcode no decodificado)
        .dc.w   0x017d                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0073                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x017e                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +110  (dato / opcode no decodificado)
        .dc.w   0x017f                        | +112  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +114  (dato / opcode no decodificado)
        .global ScriptedProp_Sprites_07466e__L074784
ScriptedProp_Sprites_07466e__L074784:
.L074784:
        .dc.w   0x005b                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0180                        | +118  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0181                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x005d                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0181                        | +120  (dato / opcode no decodificado)
        .dc.w   0x005e                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0182                        | +124  (dato / opcode no decodificado)
        .dc.w   0x005f                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0183                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0184                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0072                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0185                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0073                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0186                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0187                        | +138  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x000f                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +140  (dato / opcode no decodificado)
        .dc.w   0xb478                        | +142  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +144  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +148  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +154  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +156  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +158  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +160  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +162  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +164  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +166  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +168  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +170  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +174  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +176  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +178  (dato / opcode no decodificado)
        .dc.w   0x000b                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +180  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +184  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +186  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +188  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +18a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x000b                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +190  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +192  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +194  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +196  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +198  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +19c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +19e  (dato / opcode no decodificado)
        .dc.w   0x000d                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0x000b                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +1be  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x0017                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x0017                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +200  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +202  (dato / opcode no decodificado)
        .dc.w   0x0017                        | +204  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +206  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +208  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +20c  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +20e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +210  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +212  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +214  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +216  (dato / opcode no decodificado)
        .dc.w   0x0017                        | +218  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +21a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +21c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +21e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +220  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +222  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +224  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +226  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +228  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +22a  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +22c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +22e  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +230  (dato / opcode no decodificado)
        .dc.w   0x000d                        | +232  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +234  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +236  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +238  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +23a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +23c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +23e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +240  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +242  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +244  (dato / opcode no decodificado)
        .dc.w   0x000d                        | +246  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +248  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +24a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +24c  (dato / opcode no decodificado)
        .dc.w   0x47aa                        | +24e  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +250  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +252  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +254  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +256  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +258  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +25a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +25c  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +25e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +260  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +262  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +264  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +266  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +268  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +26a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +26c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +26e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +270  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +272  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +274  (dato / opcode no decodificado)
        .dc.w   0x7efe                        | +276  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +278  (dato / opcode no decodificado)
        .dc.w   0x7f6a                        | +27a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +27c  (dato / opcode no decodificado)
        .dc.w   0x7f6a                        | +27e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +280  (dato / opcode no decodificado)
        .dc.w   0x7efe                        | +282  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +284  (dato / opcode no decodificado)
        .dc.w   0x7fd6                        | +286  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +288  (dato / opcode no decodificado)
        .dc.w   0x7efe                        | +28a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +28c  (dato / opcode no decodificado)
        .dc.w   0x7efe                        | +28e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +290  (dato / opcode no decodificado)
        .dc.w   0x7f6a                        | +292  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +294  (dato / opcode no decodificado)
        .dc.w   0x7efe                        | +296  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +298  (dato / opcode no decodificado)
        .dc.w   0x7f6a                        | +29a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +29c  (dato / opcode no decodificado)
        .dc.w   0x8218                        | +29e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2a0  (dato / opcode no decodificado)
        .dc.w   0x8236                        | +2a2  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2a4  (dato / opcode no decodificado)
        .dc.w   0x8254                        | +2a6  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2a8  (dato / opcode no decodificado)
        .dc.w   0x8272                        | +2aa  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2ac  (dato / opcode no decodificado)
        .dc.w   0x8290                        | +2ae  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2b0  (dato / opcode no decodificado)
        .dc.w   0x82ae                        | +2b2  (dato / opcode no decodificado)
        .dc.w   0xff0b                        | +2b4  (dato / opcode no decodificado)
        .dc.w   0x012c                        | +2b6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +2b8  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +2ba  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +2bc  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +2be  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2c0  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2c2  (dato / opcode no decodificado)
        .dc.w   0x48e2                        | +2c4  (dato / opcode no decodificado)
        .dc.w   0xff0d                        | +2c6  (dato / opcode no decodificado)
        .dc.w   0x012c                        | +2c8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +2ca  (dato / opcode no decodificado)
        .dc.w   0x0050                        | +2cc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +2ce  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +2d0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2d2  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2d4  (dato / opcode no decodificado)
        .dc.w   0x48e2                        | +2d6  (dato / opcode no decodificado)
        .dc.w   0xff11                        | +2d8  (dato / opcode no decodificado)
        .dc.w   0x012c                        | +2da  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +2dc  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +2de  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +2e0  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +2e2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2e4  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2e6  (dato / opcode no decodificado)
        .dc.w   0x48e2                        | +2e8  (dato / opcode no decodificado)
        .dc.w   0xff13                        | +2ea  (dato / opcode no decodificado)
        .dc.w   0x012c                        | +2ec  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +2ee  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +2f0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +2f2  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +2f4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2f6  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2f8  (dato / opcode no decodificado)
        .dc.w   0x48e2                        | +2fa  (dato / opcode no decodificado)
        .dc.w   0xff17                        | +2fc  (dato / opcode no decodificado)
        .dc.w   0x012c                        | +2fe  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +300  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +302  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +304  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +306  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +308  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +30a  (dato / opcode no decodificado)
        .dc.w   0x48e2                        | +30c  (dato / opcode no decodificado)
        .dc.w   0xff1d                        | +30e  (dato / opcode no decodificado)
        .dc.w   0x012c                        | +310  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +312  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +314  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +316  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +318  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +31a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +31c  (dato / opcode no decodificado)
        .dc.w   0x48e2                        | +31e  (dato / opcode no decodificado)
        .dc.w   0xff1f                        | +320  (dato / opcode no decodificado)
        .dc.w   0x012c                        | +322  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +324  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +326  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +328  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +32a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +32c  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +32e  (dato / opcode no decodificado)
        .dc.w   0x48e2                        | +330  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +332  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +334  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +336  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +338  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +33a  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +33c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +33e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +340  (dato / opcode no decodificado)
        .dc.w   0x7fd6                        | +342  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +344  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +346  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +348  (dato / opcode no decodificado)
        .dc.w   0x0050                        | +34a  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +34c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +34e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +350  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +352  (dato / opcode no decodificado)
        .dc.w   0x7f6a                        | +354  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +356  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +358  (dato / opcode no decodificado)
        .dc.w   0xff30                        | +35a  (dato / opcode no decodificado)
        .dc.w   0x00c0                        | +35c  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +35e  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +360  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +362  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +364  (dato / opcode no decodificado)
        .dc.w   0x48e2                        | +366  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +368  (dato / opcode no decodificado)
        .dc.w   0x000f                        | +36a  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +36c  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +36e  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +370  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +372  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +374  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +376  (dato / opcode no decodificado)
        .dc.w   0x8740                        | +378  (dato / opcode no decodificado)
        .global ScriptedProp_Sprites_07466e__L0749e8
ScriptedProp_Sprites_07466e__L0749e8:
.L0749e8:
        .dc.w   0x0005                        | +37a  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +37c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +37e  (dato / opcode no decodificado)
        .dc.w   0x0071                        | +380  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +382  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +384
        movem.l ScriptedProp_Script_Blob0_074a0e(pc),d0/a1 | +388
        jsr     0x4429e.l                       | +38e
        movem.l (a7)+,a0/a6                     | +394
        move.l  #0x74a16,(a0)                   | +398
        rts                                     | +39e

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Blob0_074a0e  @ $074A0E  (28 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Blob0_074a0e, "ax", @progbits
        .global ScriptedProp_Script_Blob0_074a0e
ScriptedProp_Script_Blob0_074a0e:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0071                        | +002  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +004  (dato / opcode no decodificado)
        .dc.w   0x9638                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +008  (dato / opcode no decodificado)
        cmpi.w  #0x80,0x74(a6)                  | +00a
        bhi.w   .L074a28                        | +010
        move.l  #0x74a2a,(a0)                   | +014
.L074a28:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step1_074a2a  @ $074A2A  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step1_074a2a, "ax", @progbits
        .global ScriptedProp_Script_Step1_074a2a
ScriptedProp_Script_Step1_074a2a:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x1076                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Blob1_074a4c(pc),a2 | +00a
        jsr     0x5022a.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74a50,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Blob1_074a4c  @ $074A4C  (24 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Blob1_074a4c, "ax", @progbits
        .global ScriptedProp_Script_Blob1_074a4c
ScriptedProp_Script_Blob1_074a4c:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x4852                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +004  (dato / opcode no decodificado)
        cmpi.w  #0x66,0x74(a6)                  | +006
        bhi.w   .L074a62                        | +00c
        move.l  #0x74a64,(a0)                   | +010
.L074a62:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step2_074a64  @ $074A64  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step2_074a64, "ax", @progbits
        .global ScriptedProp_Script_Step2_074a64
ScriptedProp_Script_Step2_074a64:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x1076                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Blob2_074a86(pc),a2 | +00a
        jsr     0x5022a.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74a8a,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Blob2_074a86  @ $074A86  (24 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Blob2_074a86, "ax", @progbits
        .global ScriptedProp_Script_Blob2_074a86
ScriptedProp_Script_Blob2_074a86:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x487a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +004  (dato / opcode no decodificado)
        cmpi.w  #0x4c,0x74(a6)                  | +006
        bhi.w   .L074a9c                        | +00c
        move.l  #0x74a9e,(a0)                   | +010
.L074a9c:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step3_074a9e  @ $074A9E  (38 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step3_074a9e, "ax", @progbits
        .global ScriptedProp_Script_Step3_074a9e
ScriptedProp_Script_Step3_074a9e:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x102c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +004  (dato / opcode no decodificado)
        .dc.w   0x007a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +00a
        movem.l ScriptedProp_Script_Step4_074ac4(pc),a2 | +00e
        jsr     0x5022a.l                       | +014
        movem.l (a7)+,a0/a6                     | +01a
        move.l  #0x74ac8,(a0)                   | +01e
        rts                                     | +024

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step4_074ac4  @ $074AC4  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step4_074ac4, "ax", @progbits
        .global ScriptedProp_Script_Step4_074ac4
ScriptedProp_Script_Step4_074ac4:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x4866                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step5_074ae6(pc),a2 | +00a
        jsr     0x5022a.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74aea,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step5_074ae6  @ $074AE6  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step5_074ae6, "ax", @progbits
        .global ScriptedProp_Script_Step5_074ae6
ScriptedProp_Script_Step5_074ae6:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x483e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Blob5_074b08(pc),a2 | +00a
        jsr     0x5022a.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74b0c,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Blob5_074b08  @ $074B08  (24 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Blob5_074b08, "ax", @progbits
        .global ScriptedProp_Script_Blob5_074b08
ScriptedProp_Script_Blob5_074b08:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x482a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +004  (dato / opcode no decodificado)
        cmpi.w  #0x33,0x74(a6)                  | +006
        bhi.w   .L074b1e                        | +00c
        move.l  #0x74b20,(a0)                   | +010
.L074b1e:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step6_074b20  @ $074B20  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step6_074b20, "ax", @progbits
        .global ScriptedProp_Script_Step6_074b20
ScriptedProp_Script_Step6_074b20:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x1076                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step7_074b42(pc),d0 | +00a
        jsr     0x2352.l                        | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74b46,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step7_074b42  @ $074B42  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step7_074b42, "ax", @progbits
        .global ScriptedProp_Script_Step7_074b42
ScriptedProp_Script_Step7_074b42:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x1076                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Blob7_074b64(pc),a2 | +00a
        jsr     0x5022a.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74b68,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Blob7_074b64  @ $074B64  (24 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Blob7_074b64, "ax", @progbits
        .global ScriptedProp_Script_Blob7_074b64
ScriptedProp_Script_Blob7_074b64:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x4816                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +004  (dato / opcode no decodificado)
        cmpi.w  #0x19,0x74(a6)                  | +006
        bhi.w   .L074b7a                        | +00c
        move.l  #0x74b7c,(a0)                   | +010
.L074b7a:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step8_074b7c  @ $074B7C  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step8_074b7c, "ax", @progbits
        .global ScriptedProp_Script_Step8_074b7c
ScriptedProp_Script_Step8_074b7c:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x102e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Blob8_074b9e(pc),a2 | +00a
        jsr     0x5022a.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74ba2,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Blob8_074b9e  @ $074B9E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Blob8_074b9e, "ax", @progbits
        .global ScriptedProp_Script_Blob8_074b9e
ScriptedProp_Script_Blob8_074b9e:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x4802                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +004  (dato / opcode no decodificado)
        cmpi.w  #0x0,0x66(a6)                   | +006
        bne.w   .L074bb4                        | +00c
        move.l  #0x74bb6,(a0)                   | +010
.L074bb4:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step9_074bb6  @ $074BB6  (38 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step9_074bb6, "ax", @progbits
        .global ScriptedProp_Script_Step9_074bb6
ScriptedProp_Script_Step9_074bb6:
        .dc.w   0x0005                        | +000  (dato / opcode no decodificado)
        .dc.w   0x007c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0071                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +00a
        movem.l ScriptedProp_Script_Step10_074bdc(pc),a1 | +00e
        jsr     0x4ae.l                         | +014
        movem.l (a7)+,a0/a6                     | +01a
        move.l  #0x74be0,(a0)                   | +01e
        rts                                     | +024

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step10_074bdc  @ $074BDC  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step10_074bdc, "ax", @progbits
        .global ScriptedProp_Script_Step10_074bdc
ScriptedProp_Script_Step10_074bdc:
        .dc.w   0x0008                        | +000  (dato / opcode no decodificado)
        .dc.w   0x656e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step11_074bfe(pc),a1 | +00a
        jsr     0x77c7e.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74c02,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step11_074bfe  @ $074BFE  (32 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step11_074bfe, "ax", @progbits
        .global ScriptedProp_Script_Step11_074bfe
ScriptedProp_Script_Step11_074bfe:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x49a0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +004  (dato / opcode no decodificado)
        .dc.w   0x102d                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +00a
        jsr     0x434ce.l                       | +00e
        movem.l (a7)+,a0/a6                     | +014
        move.l  #0x74c1e,(a0)                   | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step12_074c1e  @ $074C1E  (36 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step12_074c1e, "ax", @progbits
        .global ScriptedProp_Script_Step12_074c1e
ScriptedProp_Script_Step12_074c1e:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +006  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +008
        movem.l ScriptedProp_Script_Step13_074c42(pc),a1 | +00c
        jsr     0x4ae.l                         | +012
        movem.l (a7)+,a0/a6                     | +018
        move.l  #0x74c46,(a0)                   | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step13_074c42  @ $074C42  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step13_074c42, "ax", @progbits
        .global ScriptedProp_Script_Step13_074c42
ScriptedProp_Script_Step13_074c42:
        .dc.w   0x0008                        | +000  (dato / opcode no decodificado)
        .dc.w   0x656e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step14_074c64(pc),a1 | +00a
        jsr     0x77c7e.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74c68,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step14_074c64  @ $074C64  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step14_074c64, "ax", @progbits
        .global ScriptedProp_Script_Step14_074c64
ScriptedProp_Script_Step14_074c64:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x4922                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step15_074c86(pc),a1 | +00a
        jsr     0x77c7e.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74c8a,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step15_074c86  @ $074C86  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step15_074c86, "ax", @progbits
        .global ScriptedProp_Script_Step15_074c86
ScriptedProp_Script_Step15_074c86:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x4934                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step16_074ca8(pc),a1 | +00a
        jsr     0x77c7e.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74cac,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step16_074ca8  @ $074CA8  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step16_074ca8, "ax", @progbits
        .global ScriptedProp_Script_Step16_074ca8
ScriptedProp_Script_Step16_074ca8:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x4946                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step17_074cca(pc),a1 | +00a
        jsr     0x77c7e.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74cce,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step17_074cca  @ $074CCA  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step17_074cca, "ax", @progbits
        .global ScriptedProp_Script_Step17_074cca
ScriptedProp_Script_Step17_074cca:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x4958                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step18_074cec(pc),a1 | +00a
        jsr     0x77c7e.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74cf0,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step18_074cec  @ $074CEC  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step18_074cec, "ax", @progbits
        .global ScriptedProp_Script_Step18_074cec
ScriptedProp_Script_Step18_074cec:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x496a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step19_074d0e(pc),a1 | +00a
        jsr     0x77c7e.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74d12,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step19_074d0e  @ $074D0E  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step19_074d0e, "ax", @progbits
        .global ScriptedProp_Script_Step19_074d0e
ScriptedProp_Script_Step19_074d0e:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x497c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step20_074d30(pc),a1 | +00a
        jsr     0x77c7e.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74d34,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step20_074d30  @ $074D30  (32 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step20_074d30, "ax", @progbits
        .global ScriptedProp_Script_Step20_074d30
ScriptedProp_Script_Step20_074d30:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x498e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1033                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +00a
        jsr     0x434ce.l                       | +00e
        movem.l (a7)+,a0/a6                     | +014
        move.l  #0x74d50,(a0)                   | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step21_074d50  @ $074D50  (36 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step21_074d50, "ax", @progbits
        .global ScriptedProp_Script_Step21_074d50
ScriptedProp_Script_Step21_074d50:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +006  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +008
        movem.l ScriptedProp_Script_Step22_074d74(pc),a1 | +00c
        jsr     0x4ae.l                         | +012
        movem.l (a7)+,a0/a6                     | +018
        move.l  #0x74d78,(a0)                   | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step22_074d74  @ $074D74  (58 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step22_074d74, "ax", @progbits
        .global ScriptedProp_Script_Step22_074d74
ScriptedProp_Script_Step22_074d74:
        .dc.w   0x0008                        | +000  (dato / opcode no decodificado)
        .dc.w   0x656e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0076                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1032                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0077                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +016  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +01c  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +01e
        movem.l ScriptedProp_Script_Step23_074dae(pc),a1 | +022
        jsr     0x77c7e.l                       | +028
        movem.l (a7)+,a0/a6                     | +02e
        move.l  #0x74db2,(a0)                   | +032
        rts                                     | +038

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step23_074dae  @ $074DAE  (32 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step23_074dae, "ax", @progbits
        .global ScriptedProp_Script_Step23_074dae
ScriptedProp_Script_Step23_074dae:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x49b2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1030                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +00a
        jsr     0x434ce.l                       | +00e
        movem.l (a7)+,a0/a6                     | +014
        move.l  #0x74dce,(a0)                   | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step24_074dce  @ $074DCE  (36 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step24_074dce, "ax", @progbits
        .global ScriptedProp_Script_Step24_074dce
ScriptedProp_Script_Step24_074dce:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +006  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +008
        movem.l ScriptedProp_Script_Step25_074df2(pc),a1 | +00c
        jsr     0x4ae.l                         | +012
        movem.l (a7)+,a0/a6                     | +018
        move.l  #0x74df6,(a0)                   | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step25_074df2  @ $074DF2  (36 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step25_074df2, "ax", @progbits
        .global ScriptedProp_Script_Step25_074df2
ScriptedProp_Script_Step25_074df2:
        .dc.w   0x0008                        | +000  (dato / opcode no decodificado)
        .dc.w   0x656e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +00a
        movem.l ScriptedProp_Script_Step26_074e16(pc),a2 | +00e
        jsr     ScriptedProp_BlitParams_0750d4(pc) | +014
        movem.l (a7)+,a0/a6                     | +018
        move.l  #0x74e1a,(a0)                   | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step26_074e16  @ $074E16  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step26_074e16, "ax", @progbits
        .global ScriptedProp_Script_Step26_074e16
ScriptedProp_Script_Step26_074e16:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x48ca                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step27_074e38(pc),a2 | +00a
        jsr     0x5022a.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74e3c,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step27_074e38  @ $074E38  (40 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step27_074e38, "ax", @progbits
        .global ScriptedProp_Script_Step27_074e38
ScriptedProp_Script_Step27_074e38:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x488e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +004  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00a  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +00c
        movem.l ScriptedProp_Script_Step28_074e60(pc),a1 | +010
        jsr     0x77c7e.l                       | +016
        movem.l (a7)+,a0/a6                     | +01c
        move.l  #0x74e64,(a0)                   | +020
        rts                                     | +026

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step28_074e60  @ $074E60  (40 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step28_074e60, "ax", @progbits
        .global ScriptedProp_Script_Step28_074e60
ScriptedProp_Script_Step28_074e60:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x49c4                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1030                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +008  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0079                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +012  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +014
        jsr     ScriptedProp_WarpOriginAndRunScript_0750a4(pc) | +018
        movem.l (a7)+,a0/a6                     | +01c
        move.l  #0x74e88,(a0)                   | +020
        rts                                     | +026

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step29_074e88  @ $074E88  (28 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step29_074e88, "ax", @progbits
        .global ScriptedProp_Script_Step29_074e88
ScriptedProp_Script_Step29_074e88:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +002
        movem.l ScriptedProp_Script_Step30_074ea4(pc),a2 | +006
        jsr     ScriptedProp_BlitParams_0750d4(pc) | +00c
        movem.l (a7)+,a0/a6                     | +010
        move.l  #0x74ea8,(a0)                   | +014
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step30_074ea4  @ $074EA4  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step30_074ea4, "ax", @progbits
        .global ScriptedProp_Script_Step30_074ea4
ScriptedProp_Script_Step30_074ea4:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x48d6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +006
        movem.l ScriptedProp_Script_Step31_074ec6(pc),a2 | +00a
        jsr     0x5022a.l                       | +010
        movem.l (a7)+,a0/a6                     | +016
        move.l  #0x74eca,(a0)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step31_074ec6  @ $074EC6  (42 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step31_074ec6, "ax", @progbits
        .global ScriptedProp_Script_Step31_074ec6
ScriptedProp_Script_Step31_074ec6:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x48a2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1026                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0071                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +00e
        movem.l ScriptedProp_Script_Step32_074ef0(pc),d0/a1 | +012
        jsr     0x4429e.l                       | +018
        movem.l (a7)+,a0/a6                     | +01e
        move.l  #0x74ef8,(a0)                   | +022
        rts                                     | +028

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step32_074ef0  @ $074EF0  (32 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step32_074ef0, "ax", @progbits
        .global ScriptedProp_Script_Step32_074ef0
ScriptedProp_Script_Step32_074ef0:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0071                        | +002  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +004  (dato / opcode no decodificado)
        .dc.w   0x9686                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +00a
        jsr     0x434ce.l                       | +00e
        movem.l (a7)+,a0/a6                     | +014
        move.l  #0x74f10,(a0)                   | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step33_074f10  @ $074F10  (36 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step33_074f10, "ax", @progbits
        .global ScriptedProp_Script_Step33_074f10
ScriptedProp_Script_Step33_074f10:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +006  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +008
        movem.l ScriptedProp_Script_Step34_074f34(pc),a1 | +00c
        jsr     0x4ae.l                         | +012
        movem.l (a7)+,a0/a6                     | +018
        move.l  #0x74f38,(a0)                   | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_Step34_074f34  @ $074F34  (36 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_Step34_074f34, "ax", @progbits
        .global ScriptedProp_Script_Step34_074f34
ScriptedProp_Script_Step34_074f34:
        .dc.w   0x0008                        | +000  (dato / opcode no decodificado)
        .dc.w   0x656e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1026                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +008  (dato / opcode no decodificado)
        .dc.w   0x003c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00e  (dato / opcode no decodificado)
        movem.l a0/a6,-(a7)                     | +010
        jsr     ClrRamWord_0750e2(pc)           | +014
        movem.l (a7)+,a0/a6                     | +018
        move.l  #0x74f58,(a0)                   | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  ScriptedProp_Script_End_074f58  @ $074F58  (2 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Script_End_074f58, "ax", @progbits
        .global ScriptedProp_Script_End_074f58
ScriptedProp_Script_End_074f58:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ScriptedProp_IdleScript_074f5a  @ $074F5A  (330 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_IdleScript_074f5a, "ax", @progbits
        .global ScriptedProp_IdleScript_074f5a
ScriptedProp_IdleScript_074f5a:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +004  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +114  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +120  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +140  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +142  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +148  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ScriptedProp_WarpOriginAndRunScript_0750a4  @ $0750A4  (28 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_WarpOriginAndRunScript_0750a4, "ax", @progbits
        .global ScriptedProp_WarpOriginAndRunScript_0750a4
ScriptedProp_WarpOriginAndRunScript_0750a4:
        clr.w   d0                              | +000
        clr.w   d1                              | +002
        jsr     0x44022.l                       | +004
        move.w  d0,0x22(a6)                     | +00a
        move.w  d1,0x24(a6)                     | +00e
        lea     ScriptedProp_IdleScript_074f5a(pc),a1 | +012
        jmp     0x43fac.l                       | +016

| ----------------------------------------------------------------------------
|  ScriptedProp_ClampLocalX_0750c0  @ $0750C0  (20 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_ClampLocalX_0750c0, "ax", @progbits
        .global ScriptedProp_ClampLocalX_0750c0
ScriptedProp_ClampLocalX_0750c0:
        jsr     0x440d0.l                       | +000
        cmpi.w  #0x180,d0                       | +006
        bcs.w   .L0750d2                        | +00a
        move.w  #0x180,d0                       | +00e
.L0750d2:
        rts                                     | +012

| ----------------------------------------------------------------------------
|  ScriptedProp_BlitParams_0750d4  @ $0750D4  (14 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_BlitParams_0750d4, "ax", @progbits
        .global ScriptedProp_BlitParams_0750d4
ScriptedProp_BlitParams_0750d4:
        movem.l (a2)+,a0                        | +000
        movem.w (a2)+,d0-d1/d4-d5               | +004
        jmp     0x51e74.l                       | +008

| ----------------------------------------------------------------------------
|  ScriptedProp_SnapToAnchor_0750ea  @ $0750EA  (22 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_SnapToAnchor_0750ea, "ax", @progbits
        .global ScriptedProp_SnapToAnchor_0750ea
ScriptedProp_SnapToAnchor_0750ea:
        move.w  #0xfc0,d0                       | +000
        move.w  #0xe0,d1                        | +004
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +008
        move.w  d0,0x22(a6)                     | +00c
        move.w  d1,0x24(a6)                     | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  ScriptedProp_Tmpl97_075100  @ $075100  (158 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Tmpl97_075100, "ax", @progbits
        .global ScriptedProp_Tmpl97_075100
ScriptedProp_Tmpl97_075100:
        lea     0x2bf004.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x66(a6)                     | +00c
        move.w  d0,0x72(a6)                     | +010
        lsr.w   #0x2,d0                         | +014
        move.w  d0,0x7e(a6)                     | +016
        clr.b   0x70(a6)                        | +01a
        clr.b   0x76(a6)                        | +01e
        clr.b   0x78(a6)                        | +022
        clr.b   0x77(a6)                        | +026
        clr.b   0x79(a6)                        | +02a
        clr.b   0x7a(a6)                        | +02e
        clr.b   0x71(a6)                        | +032
        clr.b   0x7c(a6)                        | +036
        lea     Sub_0007690A(pc),a1             | +03a  -> $07690A (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +03e
        lea     Sub_00076012(pc),a1             | +044  -> $076012 (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +048
        lea     Sub_00076ADE(pc),a1             | +04e  -> $076ADE (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +052
        lea     Sub_00076A2A(pc),a1             | +058  -> $076A2A (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +05c
        lea     Sub_00076BE8(pc),a1             | +062  -> $076BE8 (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +066
        move.w  #0xc000,0x38(a6)                | +06c
        move.b  #0x96,0x7d(a6)                  | +072
        lea     0x80(a6),a0                     | +078
        lea     ScriptedProp_Script_End_074f58(pc),a1 | +07c
        jsr     MiniScript_Init_0745da(pc)      | +080
        move.l  #0x74712,0x60(a6)               | +084
        lea     ScriptedProp_Sprites_07466e__L0746be(pc),a0 | +08c
        move.l  a0,0x48(a6)                     | +090
        lea     ScriptedProp_SpriteTable_075270(pc),a0 | +094
        jsr     0x28cd4.l                       | +098

| ----------------------------------------------------------------------------
|  ScriptedProp_Run_0751a6  @ $0751A6  (174 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Run_0751a6, "ax", @progbits
        .global ScriptedProp_Run_0751a6
ScriptedProp_Run_0751a6:
        jsr     0x2870a.l                       | +000
        bcs.w   .L0751c8                        | +006
        cmpi.w  #0xea8,0x106f50.l               | +00a
        blt.w   .L0751ec                        | +012
        subq.b  #0x1,0x7d(a6)                   | +016
        tst.b   0x7d(a6)                        | +01a
        bne.w   .L0751ec                        | +01e
.L0751c8:
        lea     ScriptedProp_Sprites_07466e(pc),a0 | +022
        move.l  a0,0x48(a6)                     | +026
        move.w  #0x28,d0                        | +02a
        jsr     0x2352.l                        | +02e
        lea     0x80(a6),a0                     | +034
        lea     ScriptedProp_Sprites_07466e__L0749e8(pc),a1 | +038
        jsr     MiniScript_Init_0745da(pc)      | +03c
        lea     .L0751ec(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L0751ec:
        bsr.w   ScriptedProp_SnapToAnchor_0750ea | +046
        jsr     0x28998.l                       | +04a
        move.w  0x22(a6),d0                     | +050
        cmpi.w  #0x12c,d0                       | +054
        bcs.w   .L075208                        | +058
        move.b  #0x2,0x45(a6)                   | +05c
.L075208:
        moveq   #0,d0                           | +062
        move.w  0x66(a6),d0                     | +064
        lsl.l   #0x8,d0                         | +068
        divu.w  0x72(a6),d0                     | +06a
        move.w  d0,0x74(a6)                     | +06e
        lea     0x80(a6),a0                     | +072
        jsr     MiniScript_Step_0745e2(pc)      | +076
        jsr     0x2870a.l                       | +07a
        scs.b   0x7b(a6)                        | +080
        bcc.w   .L07523a                        | +084
        bclr    #0x3,0x13(a6)                   | +088
        jsr     0x77d88.l                       | +08e
.L07523a:
        jsr     0x28d70.l                       | +094
        jsr     0x28758.l                       | +09a
        bcc.w   SetHandlerRts_07525a            | +0a0
        clr.b   0x106ed3.l                      | +0a4
        clr.b   0x7b(a6)                        | +0aa

| ----------------------------------------------------------------------------
|  ScriptedProp_Dead_07525c  @ $07525C  (14 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Dead_07525c, "ax", @progbits
        .global ScriptedProp_Dead_07525c
ScriptedProp_Dead_07525c:
        lea     .L075262(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L075262:
        bsr.w   ScriptedProp_SnapToAnchor_0750ea | +006
        lea     0x80(a6),a0                     | +00a

| ----------------------------------------------------------------------------
|  ScriptedProp_SpriteTable_075270  @ $075270  (3268 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_SpriteTable_075270, "ax", @progbits
        .global ScriptedProp_SpriteTable_075270
ScriptedProp_SpriteTable_075270:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +010  (dato / opcode no decodificado)
        .dc.w   0x3290                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +014  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x30fc                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +026  (dato / opcode no decodificado)
        .dc.w   0x3120                        | +028  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +030  (dato / opcode no decodificado)
        .dc.w   0x3144                        | +032  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x316e                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3190                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x31b2                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +058  (dato / opcode no decodificado)
        .dc.w   0x31d4                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +062  (dato / opcode no decodificado)
        .dc.w   0x31fc                        | +064  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x321c                        | +06e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +076  (dato / opcode no decodificado)
        .dc.w   0x3236                        | +078  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +080  (dato / opcode no decodificado)
        .dc.w   0x3250                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x3276                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3290                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x32b0                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x32ca                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x992c                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x9946                        | +0be  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x9960                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x1075                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x30fc                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x316e                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x31d4                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x3236                        | +100  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +108  (dato / opcode no decodificado)
        .dc.w   0x3290                        | +10a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +112  (dato / opcode no decodificado)
        .dc.w   0x992c                        | +114  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x9946                        | +11e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +120  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +126  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +128  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x1075                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +136  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +138  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +140  (dato / opcode no decodificado)
        .dc.w   0x9960                        | +142  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x992c                        | +14c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +154  (dato / opcode no decodificado)
        .dc.w   0x9946                        | +156  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +158  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x32ca                        | +160  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +164  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +166  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +168  (dato / opcode no decodificado)
        .dc.w   0x32b0                        | +16a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +170  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +172  (dato / opcode no decodificado)
        .dc.w   0x3290                        | +174  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +176  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +178  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x3276                        | +17e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +180  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +184  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +186  (dato / opcode no decodificado)
        .dc.w   0x3250                        | +188  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +18a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +190  (dato / opcode no decodificado)
        .dc.w   0x3236                        | +192  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +194  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +196  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +198  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x321c                        | +19c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +19e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x31fc                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0x31d4                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0x31b2                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1be  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0x3190                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x316e                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0x3144                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0x3120                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0x30fc                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0x1075                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +200  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +202  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +204  (dato / opcode no decodificado)
        .dc.w   0x9946                        | +206  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +208  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +20c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +20e  (dato / opcode no decodificado)
        .dc.w   0x3290                        | +210  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +212  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +214  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +216  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +218  (dato / opcode no decodificado)
        .dc.w   0x3236                        | +21a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +21c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +21e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +220  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +222  (dato / opcode no decodificado)
        .dc.w   0x31d4                        | +224  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +226  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +228  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +22a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +22c  (dato / opcode no decodificado)
        .dc.w   0x316e                        | +22e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +230  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +232  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +234  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +236  (dato / opcode no decodificado)
        .dc.w   0x30fc                        | +238  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +23a  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +23c  (dato / opcode no decodificado)
        .dc.w   0x1075                        | +23e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +240  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +242  (dato / opcode no decodificado)
        .dc.w   0x1099                        | +244  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +246  (dato / opcode no decodificado)
        .dc.w   0x0156                        | +248  (dato / opcode no decodificado)
        .dc.w   0x007a                        | +24a  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +24c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +24e  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +250  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +252  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +254  (dato / opcode no decodificado)
        .dc.w   0x9b7a                        | +256  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +258  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +25a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +25c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +25e  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +260  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +262  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +264  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +266  (dato / opcode no decodificado)
        .dc.w   0x9b84                        | +268  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +26a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +26c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +26e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +270  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +272  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +274  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +276  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +278  (dato / opcode no decodificado)
        .dc.w   0x54c2                        | +27a  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +27c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +27e  (dato / opcode no decodificado)
        .dc.w   0x9b94                        | +280  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +282  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +284  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +286  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +288  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +28a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +28c  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +28e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +290  (dato / opcode no decodificado)
        .dc.w   0x9b84                        | +292  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +294  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +296  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +298  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +29a  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +29c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +29e  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +2a0  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2a2  (dato / opcode no decodificado)
        .dc.w   0x54ec                        | +2a4  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +2a6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2a8  (dato / opcode no decodificado)
        .dc.w   0x9b94                        | +2aa  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +2ac  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2ae  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2b0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2b2  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +2b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2b6  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +2b8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2ba  (dato / opcode no decodificado)
        .dc.w   0x9bb6                        | +2bc  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +2be  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2c0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2c2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2c4  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +2c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2c8  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +2ca  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +2cc  (dato / opcode no decodificado)
        .dc.w   0x5516                        | +2ce  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +2d0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2d2  (dato / opcode no decodificado)
        .dc.w   0x9bd6                        | +2d4  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +2d6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2d8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2da  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2dc  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +2de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2e0  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +2e2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2e4  (dato / opcode no decodificado)
        .dc.w   0x9be6                        | +2e6  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +2e8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2ea  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2ec  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2ee  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +2f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2f2  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +2f4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2f6  (dato / opcode no decodificado)
        .dc.w   0x9c04                        | +2f8  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +2fa  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2fc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2fe  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +300  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +302  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +304  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +306  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +308  (dato / opcode no decodificado)
        .dc.w   0x9c1e                        | +30a  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +30c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +30e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +310  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +312  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +314  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +316  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +318  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +31a  (dato / opcode no decodificado)
        .dc.w   0x9c4e                        | +31c  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +31e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +320  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +322  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +324  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +326  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +328  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +32a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +32c  (dato / opcode no decodificado)
        .dc.w   0x9c7e                        | +32e  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +330  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +332  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +334  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +336  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +338  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +33a  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +33c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +33e  (dato / opcode no decodificado)
        .dc.w   0x9cb0                        | +340  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +342  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +344  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +346  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +348  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +34a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +34c  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +34e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +350  (dato / opcode no decodificado)
        .dc.w   0x9cca                        | +352  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +354  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +356  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +358  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +35a  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +35c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +35e  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +360  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +362  (dato / opcode no decodificado)
        .dc.w   0x9ce4                        | +364  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +366  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +368  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +36a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +36c  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +36e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +370  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +372  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +374  (dato / opcode no decodificado)
        .dc.w   0x9d0a                        | +376  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +378  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +37a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +37c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +37e  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +380  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +382  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +384  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +386  (dato / opcode no decodificado)
        .dc.w   0x9d24                        | +388  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +38a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +38c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +38e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +390  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +392  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +394  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +396  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +398  (dato / opcode no decodificado)
        .dc.w   0x9d42                        | +39a  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +39c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +39e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3a0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3a2  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +3a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3a6  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +3a8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3aa  (dato / opcode no decodificado)
        .dc.w   0x9d5c                        | +3ac  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +3ae  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +3b0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3b2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3b4  (dato / opcode no decodificado)
        .dc.w   0x99a0                        | +3b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3b8  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +3ba  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3bc  (dato / opcode no decodificado)
        .dc.w   0x9d76                        | +3be  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +3c0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +3c2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3c4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3c6  (dato / opcode no decodificado)
        .dc.w   0x99c6                        | +3c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3ca  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +3cc  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3ce  (dato / opcode no decodificado)
        .dc.w   0x9da8                        | +3d0  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +3d2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +3d4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3d6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3d8  (dato / opcode no decodificado)
        .dc.w   0x99fc                        | +3da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3dc  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +3de  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +3e0  (dato / opcode no decodificado)
        .dc.w   0x34ce                        | +3e2  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +3e4  (dato / opcode no decodificado)
        .dc.w   0x109c                        | +3e6  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +3e8  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +3ea  (dato / opcode no decodificado)
        .dc.w   0x650e                        | +3ec  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +3ee  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3f0  (dato / opcode no decodificado)
        .dc.w   0x9dd8                        | +3f2  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +3f4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +3f6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3f8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3fa  (dato / opcode no decodificado)
        .dc.w   0x9a2a                        | +3fc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3fe  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +400  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +402  (dato / opcode no decodificado)
        .dc.w   0x9e06                        | +404  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +406  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +408  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +40a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +40c  (dato / opcode no decodificado)
        .dc.w   0x9a2a                        | +40e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +410  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +412  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +414  (dato / opcode no decodificado)
        .dc.w   0x565e                        | +416  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +418  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +41a  (dato / opcode no decodificado)
        .dc.w   0x9b94                        | +41c  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +41e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +420  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +422  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +424  (dato / opcode no decodificado)
        .dc.w   0x9a2a                        | +426  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +428  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +42a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +42c  (dato / opcode no decodificado)
        .dc.w   0x9b84                        | +42e  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +430  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +432  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +434  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +436  (dato / opcode no decodificado)
        .dc.w   0x9a5c                        | +438  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +43a  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +43c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +43e  (dato / opcode no decodificado)
        .dc.w   0x9b7a                        | +440  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +442  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +444  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +446  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +448  (dato / opcode no decodificado)
        .dc.w   0x9a86                        | +44a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +44c  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +44e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +450  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +452  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +454  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +456  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +458  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +45a  (dato / opcode no decodificado)
        .dc.w   0x9ab8                        | +45c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +45e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +460  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +462  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +464  (dato / opcode no decodificado)
        .dc.w   0x9ade                        | +466  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +468  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +46a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +46c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +46e  (dato / opcode no decodificado)
        .dc.w   0x9b04                        | +470  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +472  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +474  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +476  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +478  (dato / opcode no decodificado)
        .dc.w   0x9b2a                        | +47a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +47c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +47e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +480  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +482  (dato / opcode no decodificado)
        .dc.w   0x9b52                        | +484  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +486  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +488  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +48a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +48c  (dato / opcode no decodificado)
        .dc.w   0x997a                        | +48e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +490  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +492  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +494  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +496  (dato / opcode no decodificado)
        .dc.w   0x83ca                        | +498  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +49a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +49c  (dato / opcode no decodificado)
        .dc.w   0x83d8                        | +49e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4a0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +4a2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4a4  (dato / opcode no decodificado)
        .dc.w   0x9e32                        | +4a6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4a8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +4aa  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4ac  (dato / opcode no decodificado)
        .dc.w   0x9e32                        | +4ae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4b0  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +4b2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +4b4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +4b6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4b8  (dato / opcode no decodificado)
        .dc.w   0x9e32                        | +4ba  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4bc  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +4be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +4c0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +4c2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4c4  (dato / opcode no decodificado)
        .dc.w   0x9e32                        | +4c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4c8  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +4ca  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +4cc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +4ce  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4d0  (dato / opcode no decodificado)
        .dc.w   0x9e32                        | +4d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4d4  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +4d6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +4d8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4da  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +4dc  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4de  (dato / opcode no decodificado)
        .dc.w   0x9e4a                        | +4e0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4e2  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +4e4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4e6  (dato / opcode no decodificado)
        .dc.w   0x9e4a                        | +4e8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4ea  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +4ec  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +4ee  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +4f0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4f2  (dato / opcode no decodificado)
        .dc.w   0x9e4a                        | +4f4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4f6  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +4f8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +4fa  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +4fc  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4fe  (dato / opcode no decodificado)
        .dc.w   0x9e4a                        | +500  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +502  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +504  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +506  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +508  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +50a  (dato / opcode no decodificado)
        .dc.w   0x9e4a                        | +50c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +50e  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +510  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +512  (dato / opcode no decodificado)
        .dc.w   0x0308                        | +514  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +516  (dato / opcode no decodificado)
        .dc.w   0x5704                        | +518  (dato / opcode no decodificado)
        .dc.w   0x0501                        | +51a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +51c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +51e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +520  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +522  (dato / opcode no decodificado)
        .dc.w   0x9e62                        | +524  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +526  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +528  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +52a  (dato / opcode no decodificado)
        .dc.w   0x9e62                        | +52c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +52e  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +530  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +532  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +534  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +536  (dato / opcode no decodificado)
        .dc.w   0x9e62                        | +538  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +53a  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +53c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +53e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +540  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +542  (dato / opcode no decodificado)
        .dc.w   0x9e62                        | +544  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +546  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +548  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +54a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +54c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +54e  (dato / opcode no decodificado)
        .dc.w   0x9e62                        | +550  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +552  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +554  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +556  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +558  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +55a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +55c  (dato / opcode no decodificado)
        .dc.w   0x9e7a                        | +55e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +560  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +562  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +564  (dato / opcode no decodificado)
        .dc.w   0x9e7a                        | +566  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +568  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +56a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +56c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +56e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +570  (dato / opcode no decodificado)
        .dc.w   0x9e7a                        | +572  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +574  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +576  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +578  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +57a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +57c  (dato / opcode no decodificado)
        .dc.w   0x9e7a                        | +57e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +580  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +582  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +584  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +586  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +588  (dato / opcode no decodificado)
        .dc.w   0x9e7a                        | +58a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +58c  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +58e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +590  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +592  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +594  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +596  (dato / opcode no decodificado)
        .dc.w   0x9e92                        | +598  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +59a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +59c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +59e  (dato / opcode no decodificado)
        .dc.w   0x9e92                        | +5a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5a2  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +5a4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5a6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5a8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5aa  (dato / opcode no decodificado)
        .dc.w   0x9e92                        | +5ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5ae  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +5b0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5b2  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5b4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5b6  (dato / opcode no decodificado)
        .dc.w   0x9e92                        | +5b8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5ba  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +5bc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5be  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5c0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5c2  (dato / opcode no decodificado)
        .dc.w   0x9e92                        | +5c4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5c6  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +5c8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5ca  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5cc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +5ce  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5d0  (dato / opcode no decodificado)
        .dc.w   0x9ea2                        | +5d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5d4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5d6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5d8  (dato / opcode no decodificado)
        .dc.w   0x9ea2                        | +5da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5dc  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +5de  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5e0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5e2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5e4  (dato / opcode no decodificado)
        .dc.w   0x9ea2                        | +5e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5e8  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +5ea  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5ec  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5ee  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5f0  (dato / opcode no decodificado)
        .dc.w   0x9ea2                        | +5f2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5f4  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +5f6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5f8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5fa  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5fc  (dato / opcode no decodificado)
        .dc.w   0x9ea2                        | +5fe  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +600  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +602  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +604  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +606  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +608  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +60a  (dato / opcode no decodificado)
        .dc.w   0x9eb2                        | +60c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +60e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +610  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +612  (dato / opcode no decodificado)
        .dc.w   0x9eb2                        | +614  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +616  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +618  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +61a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +61c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +61e  (dato / opcode no decodificado)
        .dc.w   0x9eb2                        | +620  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +622  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +624  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +626  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +628  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +62a  (dato / opcode no decodificado)
        .dc.w   0x9eb2                        | +62c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +62e  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +630  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +632  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +634  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +636  (dato / opcode no decodificado)
        .dc.w   0x9eb2                        | +638  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +63a  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +63c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +63e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +640  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +642  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +644  (dato / opcode no decodificado)
        .dc.w   0x9ec2                        | +646  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +648  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +64a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +64c  (dato / opcode no decodificado)
        .dc.w   0x9ec2                        | +64e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +650  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +652  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +654  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +656  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +658  (dato / opcode no decodificado)
        .dc.w   0x9ec2                        | +65a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +65c  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +65e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +660  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +662  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +664  (dato / opcode no decodificado)
        .dc.w   0x9ec2                        | +666  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +668  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +66a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +66c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +66e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +670  (dato / opcode no decodificado)
        .dc.w   0x9ec2                        | +672  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +674  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +676  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +678  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +67a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +67c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +67e  (dato / opcode no decodificado)
        .dc.w   0x9ed2                        | +680  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +682  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +684  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +686  (dato / opcode no decodificado)
        .dc.w   0x9ed2                        | +688  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +68a  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +68c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +68e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +690  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +692  (dato / opcode no decodificado)
        .dc.w   0x9ed2                        | +694  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +696  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +698  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +69a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +69c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +69e  (dato / opcode no decodificado)
        .dc.w   0x9ed2                        | +6a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6a2  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +6a4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +6a6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +6a8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +6aa  (dato / opcode no decodificado)
        .dc.w   0x9ed2                        | +6ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6ae  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +6b0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +6b2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6b4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6b6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +6b8  (dato / opcode no decodificado)
        .dc.w   0x9ee2                        | +6ba  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6bc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +6be  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +6c0  (dato / opcode no decodificado)
        .dc.w   0x9ee2                        | +6c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6c4  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +6c6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +6c8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +6ca  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +6cc  (dato / opcode no decodificado)
        .dc.w   0x9ee2                        | +6ce  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6d0  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +6d2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +6d4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +6d6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +6d8  (dato / opcode no decodificado)
        .dc.w   0x9ee2                        | +6da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6dc  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +6de  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +6e0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +6e2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +6e4  (dato / opcode no decodificado)
        .dc.w   0x9ee2                        | +6e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6e8  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +6ea  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +6ec  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6ee  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6f0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +6f2  (dato / opcode no decodificado)
        .dc.w   0x9ef2                        | +6f4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6f6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +6f8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +6fa  (dato / opcode no decodificado)
        .dc.w   0x9ef2                        | +6fc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6fe  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +700  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +702  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +704  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +706  (dato / opcode no decodificado)
        .dc.w   0x9ef2                        | +708  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +70a  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +70c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +70e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +710  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +712  (dato / opcode no decodificado)
        .dc.w   0x9ef2                        | +714  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +716  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +718  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +71a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +71c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +71e  (dato / opcode no decodificado)
        .dc.w   0x9ef2                        | +720  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +722  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +724  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +726  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +728  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +72a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +72c  (dato / opcode no decodificado)
        .dc.w   0x9f02                        | +72e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +730  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +732  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +734  (dato / opcode no decodificado)
        .dc.w   0x9f02                        | +736  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +738  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +73a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +73c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +73e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +740  (dato / opcode no decodificado)
        .dc.w   0x9f02                        | +742  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +744  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +746  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +748  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +74a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +74c  (dato / opcode no decodificado)
        .dc.w   0x9f02                        | +74e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +750  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +752  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +754  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +756  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +758  (dato / opcode no decodificado)
        .dc.w   0x9f02                        | +75a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +75c  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +75e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +760  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +762  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +764  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +766  (dato / opcode no decodificado)
        .dc.w   0x9f22                        | +768  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +76a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +76c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +76e  (dato / opcode no decodificado)
        .dc.w   0x9f22                        | +770  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +772  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +774  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +776  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +778  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +77a  (dato / opcode no decodificado)
        .dc.w   0x9f22                        | +77c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +77e  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +780  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +782  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +784  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +786  (dato / opcode no decodificado)
        .dc.w   0x9f22                        | +788  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +78a  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +78c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +78e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +790  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +792  (dato / opcode no decodificado)
        .dc.w   0x9f22                        | +794  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +796  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +798  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +79a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +79c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +79e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +7a0  (dato / opcode no decodificado)
        .dc.w   0x9f3a                        | +7a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7a4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +7a6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +7a8  (dato / opcode no decodificado)
        .dc.w   0x9f3a                        | +7aa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7ac  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +7ae  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +7b0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +7b2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +7b4  (dato / opcode no decodificado)
        .dc.w   0x9f3a                        | +7b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7b8  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +7ba  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +7bc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +7be  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +7c0  (dato / opcode no decodificado)
        .dc.w   0x9f3a                        | +7c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7c4  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +7c6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +7c8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +7ca  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +7cc  (dato / opcode no decodificado)
        .dc.w   0x9f3a                        | +7ce  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7d0  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +7d2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +7d4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +7d6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +7d8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +7da  (dato / opcode no decodificado)
        .dc.w   0x9f5e                        | +7dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7de  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +7e0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +7e2  (dato / opcode no decodificado)
        .dc.w   0x9f5e                        | +7e4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7e6  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +7e8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +7ea  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +7ec  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +7ee  (dato / opcode no decodificado)
        .dc.w   0x9f5e                        | +7f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7f2  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +7f4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +7f6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +7f8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +7fa  (dato / opcode no decodificado)
        .dc.w   0x9f5e                        | +7fc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7fe  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +800  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +802  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +804  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +806  (dato / opcode no decodificado)
        .dc.w   0x9f5e                        | +808  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +80a  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +80c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +80e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +810  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +812  (dato / opcode no decodificado)
        .dc.w   0x0156                        | +814  (dato / opcode no decodificado)
        .dc.w   0x007a                        | +816  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +818  (dato / opcode no decodificado)
        .dc.w   0xffdc                        | +81a  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +81c  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +81e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +820  (dato / opcode no decodificado)
        .dc.w   0x9b7a                        | +822  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +824  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +826  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +828  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +82a  (dato / opcode no decodificado)
        .dc.w   0x32e4                        | +82c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +82e  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +830  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +832  (dato / opcode no decodificado)
        .dc.w   0x9b84                        | +834  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +836  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +838  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +83a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +83c  (dato / opcode no decodificado)
        .dc.w   0x30fc                        | +83e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +840  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +842  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +844  (dato / opcode no decodificado)
        .dc.w   0x5a8e                        | +846  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +848  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +84a  (dato / opcode no decodificado)
        .dc.w   0x9b94                        | +84c  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +84e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +850  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +852  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +854  (dato / opcode no decodificado)
        .dc.w   0x32e4                        | +856  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +858  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +85a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +85c  (dato / opcode no decodificado)
        .dc.w   0x9b84                        | +85e  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +860  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +862  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +864  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +866  (dato / opcode no decodificado)
        .dc.w   0x30fc                        | +868  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +86a  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +86c  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +86e  (dato / opcode no decodificado)
        .dc.w   0x5ab8                        | +870  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +872  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +874  (dato / opcode no decodificado)
        .dc.w   0x9b94                        | +876  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +878  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +87a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +87c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +87e  (dato / opcode no decodificado)
        .dc.w   0x32e4                        | +880  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +882  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +884  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +886  (dato / opcode no decodificado)
        .dc.w   0x9bb6                        | +888  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +88a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +88c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +88e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +890  (dato / opcode no decodificado)
        .dc.w   0x30fc                        | +892  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +894  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +896  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +898  (dato / opcode no decodificado)
        .dc.w   0x5ae2                        | +89a  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +89c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +89e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8a0  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +8a2  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +8a4  (dato / opcode no decodificado)
        .dc.w   0x1065                        | +8a6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8a8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8aa  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +8ac  (dato / opcode no decodificado)
        .dc.w   0x32e4                        | +8ae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8b0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8b2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8b4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +8b6  (dato / opcode no decodificado)
        .dc.w   0x3314                        | +8b8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8ba  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8bc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8be  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +8c0  (dato / opcode no decodificado)
        .dc.w   0x3346                        | +8c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8c4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8c6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8c8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +8ca  (dato / opcode no decodificado)
        .dc.w   0x3378                        | +8cc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8ce  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8d0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8d2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +8d4  (dato / opcode no decodificado)
        .dc.w   0x33ac                        | +8d6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8d8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8da  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8dc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +8de  (dato / opcode no decodificado)
        .dc.w   0x33d8                        | +8e0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8e2  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +8e4  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +8e6  (dato / opcode no decodificado)
        .dc.w   0x6536                        | +8e8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8ea  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8ec  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +8ee  (dato / opcode no decodificado)
        .dc.w   0x3404                        | +8f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8f2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8f4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8f6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +8f8  (dato / opcode no decodificado)
        .dc.w   0x3426                        | +8fa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8fc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8fe  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +900  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +902  (dato / opcode no decodificado)
        .dc.w   0x3448                        | +904  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +906  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +908  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +90a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +90c  (dato / opcode no decodificado)
        .dc.w   0x346a                        | +90e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +910  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +912  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +914  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +916  (dato / opcode no decodificado)
        .dc.w   0x3494                        | +918  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +91a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +91c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +91e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +920  (dato / opcode no decodificado)
        .dc.w   0x346a                        | +922  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +924  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +926  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +928  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +92a  (dato / opcode no decodificado)
        .dc.w   0x83ca                        | +92c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +92e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +930  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +932  (dato / opcode no decodificado)
        .dc.w   0x9f7e                        | +934  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +936  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +938  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +93a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +93c  (dato / opcode no decodificado)
        .dc.w   0x9f9a                        | +93e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +940  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +942  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +944  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +946  (dato / opcode no decodificado)
        .dc.w   0x9fac                        | +948  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +94a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +94c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +94e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +950  (dato / opcode no decodificado)
        .dc.w   0x9fbc                        | +952  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +954  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +956  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +958  (dato / opcode no decodificado)
        .dc.w   0x5b98                        | +95a  (dato / opcode no decodificado)
        .dc.w   0x001e                        | +95c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +95e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +960  (dato / opcode no decodificado)
        .dc.w   0x6040                        | +962  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +964  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +966  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +968  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +96a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +96c  (dato / opcode no decodificado)
        .dc.w   0x7c3c                        | +96e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +970  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +972  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +974  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +976  (dato / opcode no decodificado)
        .dc.w   0x7c56                        | +978  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +97a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +97c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +97e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +980  (dato / opcode no decodificado)
        .dc.w   0x7c70                        | +982  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +984  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +986  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +988  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +98a  (dato / opcode no decodificado)
        .dc.w   0x7c8a                        | +98c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +98e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +990  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +992  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +994  (dato / opcode no decodificado)
        .dc.w   0x7cb6                        | +996  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +998  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +99a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +99c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +99e  (dato / opcode no decodificado)
        .dc.w   0x7ce0                        | +9a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9a2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9a4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9a6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +9a8  (dato / opcode no decodificado)
        .dc.w   0x7d06                        | +9aa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9ac  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9ae  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9b0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +9b2  (dato / opcode no decodificado)
        .dc.w   0x7d34                        | +9b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9b6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9b8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9ba  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +9bc  (dato / opcode no decodificado)
        .dc.w   0x7d62                        | +9be  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9c0  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +9c2  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +9c4  (dato / opcode no decodificado)
        .dc.w   0x0156                        | +9c6  (dato / opcode no decodificado)
        .dc.w   0x007a                        | +9c8  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +9ca  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +9cc  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +9ce  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +9d0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +9d2  (dato / opcode no decodificado)
        .dc.w   0x8142                        | +9d4  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +9d6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9d8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9da  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +9dc  (dato / opcode no decodificado)
        .dc.w   0x7d90                        | +9de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9e0  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +9e2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +9e4  (dato / opcode no decodificado)
        .dc.w   0x8152                        | +9e6  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +9e8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9ea  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9ec  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +9ee  (dato / opcode no decodificado)
        .dc.w   0x7dbe                        | +9f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9f2  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +9f4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +9f6  (dato / opcode no decodificado)
        .dc.w   0x8174                        | +9f8  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +9fa  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9fc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9fe  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a00  (dato / opcode no decodificado)
        .dc.w   0x7de4                        | +a02  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a04  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +a06  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +a08  (dato / opcode no decodificado)
        .dc.w   0x5ee8                        | +a0a  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +a0c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a0e  (dato / opcode no decodificado)
        .dc.w   0x8192                        | +a10  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +a12  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a14  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a16  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a18  (dato / opcode no decodificado)
        .dc.w   0x7e0a                        | +a1a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a1c  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +a1e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a20  (dato / opcode no decodificado)
        .dc.w   0x81b4                        | +a22  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +a24  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a26  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a28  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a2a  (dato / opcode no decodificado)
        .dc.w   0x7e30                        | +a2c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a2e  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +a30  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a32  (dato / opcode no decodificado)
        .dc.w   0x81cc                        | +a34  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +a36  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a38  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a3a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a3c  (dato / opcode no decodificado)
        .dc.w   0x7e56                        | +a3e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a40  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +a42  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a44  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a46  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +a48  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a4a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a4c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a4e  (dato / opcode no decodificado)
        .dc.w   0x7d62                        | +a50  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a52  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +a54  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a56  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a58  (dato / opcode no decodificado)
        .dc.w   0x7e7c                        | +a5a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a5c  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +a5e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a60  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a62  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a64  (dato / opcode no decodificado)
        .dc.w   0x7d62                        | +a66  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a68  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a6a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a6c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a6e  (dato / opcode no decodificado)
        .dc.w   0x7d34                        | +a70  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a72  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a74  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a76  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a78  (dato / opcode no decodificado)
        .dc.w   0x7d06                        | +a7a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a7c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a7e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a80  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a82  (dato / opcode no decodificado)
        .dc.w   0x7ce0                        | +a84  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a86  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a88  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a8a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a8c  (dato / opcode no decodificado)
        .dc.w   0x7cb6                        | +a8e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a90  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a92  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a94  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +a96  (dato / opcode no decodificado)
        .dc.w   0x7c8a                        | +a98  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a9a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a9c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a9e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +aa0  (dato / opcode no decodificado)
        .dc.w   0x7c70                        | +aa2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +aa4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +aa6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +aa8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +aaa  (dato / opcode no decodificado)
        .dc.w   0x7c56                        | +aac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +aae  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ab0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +ab2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +ab4  (dato / opcode no decodificado)
        .dc.w   0x7c3c                        | +ab6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ab8  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +aba  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +abc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +abe  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +ac0  (dato / opcode no decodificado)
        .dc.w   0x7eaa                        | +ac2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ac4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ac6  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +ac8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +aca  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +acc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +ace  (dato / opcode no decodificado)
        .dc.w   0x7ecc                        | +ad0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ad2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ad4  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +ad6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ad8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +ada  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +adc  (dato / opcode no decodificado)
        .dc.w   0x7eee                        | +ade  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ae0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ae2  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +ae4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ae6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +ae8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +aea  (dato / opcode no decodificado)
        .dc.w   0x7f10                        | +aec  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +aee  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +af0  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +af2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +af4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +af6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +af8  (dato / opcode no decodificado)
        .dc.w   0x7f3c                        | +afa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +afc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +afe  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +b00  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b02  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +b04  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b06  (dato / opcode no decodificado)
        .dc.w   0x7f68                        | +b08  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b0a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +b0c  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +b0e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b10  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +b12  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b14  (dato / opcode no decodificado)
        .dc.w   0x7f96                        | +b16  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b18  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +b1a  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +b1c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b1e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +b20  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b22  (dato / opcode no decodificado)
        .dc.w   0x7fc4                        | +b24  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b26  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +b28  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +b2a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b2c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +b2e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b30  (dato / opcode no decodificado)
        .dc.w   0x7ff2                        | +b32  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b34  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +b36  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +b38  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +b3a  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +b3c  (dato / opcode no decodificado)
        .dc.w   0x0156                        | +b3e  (dato / opcode no decodificado)
        .dc.w   0x007a                        | +b40  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +b42  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +b44  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +b46  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +b48  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b4a  (dato / opcode no decodificado)
        .dc.w   0x8142                        | +b4c  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +b4e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b50  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +b52  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b54  (dato / opcode no decodificado)
        .dc.w   0x8020                        | +b56  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b58  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +b5a  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +b5c  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +b5e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b60  (dato / opcode no decodificado)
        .dc.w   0x8152                        | +b62  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +b64  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b66  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +b68  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b6a  (dato / opcode no decodificado)
        .dc.w   0x804e                        | +b6c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b6e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +b70  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +b72  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +b74  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b76  (dato / opcode no decodificado)
        .dc.w   0x8174                        | +b78  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +b7a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b7c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +b7e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b80  (dato / opcode no decodificado)
        .dc.w   0x807c                        | +b82  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b84  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +b86  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +b88  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +b8a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +b8c  (dato / opcode no decodificado)
        .dc.w   0x5ee8                        | +b8e  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +b90  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b92  (dato / opcode no decodificado)
        .dc.w   0x8192                        | +b94  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +b96  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b98  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +b9a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +b9c  (dato / opcode no decodificado)
        .dc.w   0x80a2                        | +b9e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ba0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ba2  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +ba4  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +ba6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +ba8  (dato / opcode no decodificado)
        .dc.w   0x81b4                        | +baa  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +bac  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +bae  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +bb0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +bb2  (dato / opcode no decodificado)
        .dc.w   0x80c8                        | +bb4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bb6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +bb8  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +bba  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +bbc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +bbe  (dato / opcode no decodificado)
        .dc.w   0x81cc                        | +bc0  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +bc2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +bc4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +bc6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +bc8  (dato / opcode no decodificado)
        .dc.w   0x80ee                        | +bca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bcc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +bce  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +bd0  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +bd2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bd4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bd6  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +bd8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +bda  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +bdc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +bde  (dato / opcode no decodificado)
        .dc.w   0x7ff2                        | +be0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +be2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +be4  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +be6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +be8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +bea  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +bec  (dato / opcode no decodificado)
        .dc.w   0x8114                        | +bee  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bf0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +bf2  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +bf4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +bf6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +bf8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +bfa  (dato / opcode no decodificado)
        .dc.w   0x7ff2                        | +bfc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bfe  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +c00  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +c02  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +c04  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +c06  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +c08  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +c0a  (dato / opcode no decodificado)
        .dc.w   0x7fc4                        | +c0c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c0e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +c10  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +c12  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +c14  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +c16  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +c18  (dato / opcode no decodificado)
        .dc.w   0x7f96                        | +c1a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c1c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +c1e  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +c20  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +c22  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +c24  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +c26  (dato / opcode no decodificado)
        .dc.w   0x7f68                        | +c28  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c2a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +c2c  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +c2e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +c30  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +c32  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +c34  (dato / opcode no decodificado)
        .dc.w   0x7f3c                        | +c36  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c38  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +c3a  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +c3c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +c3e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +c40  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +c42  (dato / opcode no decodificado)
        .dc.w   0x7f10                        | +c44  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c46  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +c48  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +c4a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +c4c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +c4e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +c50  (dato / opcode no decodificado)
        .dc.w   0x7eee                        | +c52  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c54  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +c56  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +c58  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +c5a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +c5c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +c5e  (dato / opcode no decodificado)
        .dc.w   0x7ecc                        | +c60  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c62  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +c64  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +c66  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +c68  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +c6a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +c6c  (dato / opcode no decodificado)
        .dc.w   0x7eaa                        | +c6e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c70  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +c72  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +c74  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +c76  (dato / opcode no decodificado)
        move.l  a6,-(a7)                        | +c78
        lea     0x100800.l,a6                   | +c7a
        lea     Sub_00076E10(pc),a1             | +c80  -> $076E10 (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +c84
        movea.l 0xc(a6),a1                      | +c8a
        move.w  0x7e(a1),d0                     | +c8e
        cmp.w   0x66(a1),d0                     | +c92
        blt.w   .L075f12                        | +c96
        lea     ScriptedProp_Child_Flash_075f3c(pc),a1 | +c9a
        bra.w   .L075f16                        | +c9e
.L075f12:
        lea     ScriptedProp_Hitbox_075f34(pc),a1 | +ca2
.L075f16:
        move.l  a1,0x7c(a0)                     | +ca6
        movea.l (a7)+,a6                        | +caa
        move.w  0x22(a6),d0                     | +cac
        subq.w  #0x8,d0                         | +cb0
        move.w  d0,0x22(a0)                     | +cb2
        move.w  0x24(a6),d0                     | +cb6
        add.w   0x78(a6),d0                     | +cba
        move.w  d0,0x24(a0)                     | +cbe
        rts                                     | +cc2

| ----------------------------------------------------------------------------
|  ScriptedProp_Hitbox_075f34  @ $075F34  (8 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Hitbox_075f34, "ax", @progbits
        .global ScriptedProp_Hitbox_075f34
ScriptedProp_Hitbox_075f34:
        .dc.w   0xfe80                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ScriptedProp_Child_Flash_075f3c  @ $075F3C  (140 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Child_Flash_075f3c, "ax", @progbits
        .global ScriptedProp_Child_Flash_075f3c
ScriptedProp_Child_Flash_075f3c:
        .dc.w   0xfe80                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +002  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +006  (dato / opcode no decodificado)
        move.w  #0xfe0,d0                       | +008
        move.w  #0xa0,d1                        | +00c
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +010
        move.w  d0,0x22(a6)                     | +014
        move.w  d1,0x24(a6)                     | +018
        move.w  0x70(a6),d0                     | +01c
        beq.w   .L075f66                        | +020
        subq.w  #0x1,d0                         | +024
        move.w  d0,0x70(a6)                     | +026
.L075f66:
        movea.l 0xc(a6),a0                      | +02a
        tst.b   0x7b(a0)                        | +02e
        bne.w   .L075f96                        | +032
        btst    #0x3,0x13(a6)                   | +036
        beq.w   .L075f8c                        | +03c
        lea     ScriptedProp_Sprites_07466e__L07475e(pc),a0 | +040
        jsr     0x2b58.l                        | +044
        bclr    #0x3,0x13(a6)                   | +04a
.L075f8c:
        andi.b  #0x7e,0x5a(a6)                  | +050
        bra.w   .L075fb6                        | +056
.L075f96:
        btst    #0x3,0x13(a6)                   | +05a
        bne.w   .L075fb0                        | +060
        lea     ScriptedProp_Sprites_07466e__L074784(pc),a0 | +064
        jsr     0x2b58.l                        | +068
        bset    #0x3,0x13(a6)                   | +06e
.L075fb0:
        ori.b   #0x81,0x5a(a6)                  | +074
.L075fb6:
        btst    #0x6,0x5b(a0)                   | +07a
        beq.w   .L075fc6                        | +080
        bset    #0x6,0x5b(a6)                   | +084
.L075fc6:
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  ScriptedProp_Child_Timer_075fc8  @ $075FC8  (46 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Child_Timer_075fc8, "ax", @progbits
        .global ScriptedProp_Child_Timer_075fc8
ScriptedProp_Child_Timer_075fc8:
        move.w  0x72(a6),d0                     | +000
        beq.w   .L075fea                        | +004
        subq.w  #0x1,d0                         | +008
        move.w  d0,0x72(a6)                     | +00a
        lea     0x2bf108.l,a0                   | +00e
        jsr     0x799de.l                       | +014
        move.w  d0,0x70(a6)                     | +01a
        bra.w   SetTaskWRts_075ffa              | +01e
.L075fea:
        lea     0x2bf086.l,a0                   | +022
        jsr     0x799de.l                       | +028

| ----------------------------------------------------------------------------
|  ScriptedProp_Child_Parent_075ffc  @ $075FFC  (4 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Child_Parent_075ffc, "ax", @progbits
        .global ScriptedProp_Child_Parent_075ffc
ScriptedProp_Child_Parent_075ffc:
        movea.l 0xc(a6),a0                      | +000
