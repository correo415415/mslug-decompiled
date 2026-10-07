| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $05E000..$062000  (15,144 B, 221 entradas, 108 huecos)
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
|  AtanTable_Tail_05e000  @ $05E000  (24 B)
| ----------------------------------------------------------------------------
        .section .text.AtanTable_Tail_05e000, "ax", @progbits
        .global AtanTable_Tail_05e000
AtanTable_Tail_05e000:
        .dc.w   0xf6f6                        | +000  (dato / opcode no decodificado)
        .dc.w   0xf7f7                        | +002  (dato / opcode no decodificado)
        .dc.w   0xf7f8                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf8f9                        | +006  (dato / opcode no decodificado)
        .dc.w   0xf9f9                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfafa                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xfbfb                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfcfc                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xfcfd                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfdfe                        | +012  (dato / opcode no decodificado)
        .dc.w   0xfeff                        | +014  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +016  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Atan2_Angle256_05e018  @ $05E018  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Atan2_Angle256_05e018, "ax", @progbits
        .global Atan2_Angle256_05e018
Atan2_Angle256_05e018:
        asr.w   #0x1,d0                         | +000
        smi.b   d2                              | +002
        eor.b   d2,d0                           | +004
        asr.w   #0x1,d1                         | +006
        smi.b   d3                              | +008
        eor.b   d3,d1                           | +00a
        move.w  #0xff,d4                        | +00c
        and.w   d4,d0                           | +010
        and.w   d4,d1                           | +012
        cmp.b   d1,d0                           | +014
        smi.b   d4                              | +016
        bcs.w   .L05e036                        | +018
        exg     d1,d0                           | +01c
.L05e036:
        tst.b   d0                              | +01e
        bne.w   .L05e046                        | +020
        move.b  d4,d0                           | +024
        andi.b  #0x40,d0                        | +026
        bra.w   .L05e062                        | +02a
.L05e046:
        lea     Sub_0005DE18(pc),a0             | +02e  -> $05DE18 (hueco futuro, defsym forward)
        move.b  (a0,d0.w),d0                    | +032
        sub.b   (a0,d1.w),d0                    | +036
        lea     Sub_0005DF18(pc),a0             | +03a  -> $05DF18 (hueco futuro, defsym forward)
        move.b  (a0,d0.w),d0                    | +03e
        eor.b   d4,d0                           | +042
        sub.b   d4,d0                           | +044
        addi.b  #0x20,d0                        | +046
.L05e062:
        eor.b   d2,d3                           | +04a
        eor.b   d3,d0                           | +04c
        sub.b   d3,d0                           | +04e
        andi.b  #0x80,d2                        | +050
        add.b   d2,d0                           | +054
        rts                                     | +056

| ----------------------------------------------------------------------------
|  Target_DeltaThenAtan2_05e070  @ $05E070  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Target_DeltaThenAtan2_05e070, "ax", @progbits
        .global Target_DeltaThenAtan2_05e070
Target_DeltaThenAtan2_05e070:
        move.w  0x22(a0),d0                     | +000
        move.w  0x24(a0),d1                     | +004
        sub.w   0x22(a6),d0                     | +008
        sub.w   0x24(a6),d1                     | +00c

| ----------------------------------------------------------------------------
|  Target_AcquireNearestPlayer_05e086  @ $05E086  (150 B)
| ----------------------------------------------------------------------------
        .section .text.Target_AcquireNearestPlayer_05e086, "ax", @progbits
        .global Target_AcquireNearestPlayer_05e086
Target_AcquireNearestPlayer_05e086:
        move.l  a0,-(a7)                        | +000
        jsr     Players_AliveMask_05e1aa(pc)    | +002
        bcs.w   .L05e0ce                        | +006
        lea     0x100440.l,a2                   | +00a
        lea     0x1004e0.l,a3                   | +010
        movea.l (a7)+,a1                        | +016
        btst    #0x0,d0                         | +018
        beq.w   .L05e0b4                        | +01c
        movea.l a2,a0                           | +020
        jsr     Target_InRangeBox_05e260(pc)    | +022
        bcc.w   .L05e0b4                        | +026
        eori.b  #0x1,d0                         | +02a
.L05e0b4:
        btst    #0x1,d0                         | +02e
        beq.w   .L05e0ca                        | +032
        movea.l a3,a0                           | +036
        jsr     Target_InRangeBox_05e260(pc)    | +038
        bcc.w   .L05e0ca                        | +03c
        eori.b  #0x2,d0                         | +040
.L05e0ca:
        bra.w   .L05e0d8                        | +044
.L05e0ce:
        movea.l (a7)+,a1                        | +048
        bra.w   .L05e0d8                        | +04a
        .global Target_AcquireNearestPlayer_05e086__L05e0d4
Target_AcquireNearestPlayer_05e086__L05e0d4:
.L05e0d4:
        jsr     Players_AliveMask_05e1aa(pc)    | +04e
.L05e0d8:
        tst.b   d0                              | +052
        beq.w   Target_DefaultP2_05e12a         | +054
        lea     0x100440.l,a0                   | +058
        lea     0x1004e0.l,a1                   | +05e
        cmpi.b  #0x1,d0                         | +064
        beq.w   ClearXN_05e11c                  | +068
        cmpi.b  #0x2,d0                         | +06c
        beq.w   Target_UseP2_05e122             | +070
        move.w  0x22(a6),d0                     | +074
        move.w  0x22(a0),d1                     | +078
        move.w  0x22(a1),d2                     | +07c
        sub.w   d0,d1                           | +080
        bge.w   .L05e10e                        | +082
        neg.w   d1                              | +086
.L05e10e:
        sub.w   d0,d2                           | +088
        bge.w   .L05e116                        | +08a
        neg.w   d2                              | +08e
.L05e116:
        cmp.w   d1,d2                           | +090
        blt.w   Target_UseP2_05e122             | +092

| ----------------------------------------------------------------------------
|  Target_UseP2_05e122  @ $05E122  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Target_UseP2_05e122, "ax", @progbits
        .global Target_UseP2_05e122
Target_UseP2_05e122:
        movea.l a1,a0                           | +000

| ----------------------------------------------------------------------------
|  Target_DefaultP2_05e12a  @ $05E12A  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Target_DefaultP2_05e12a, "ax", @progbits
        .global Target_DefaultP2_05e12a
Target_DefaultP2_05e12a:
        lea     0x1004e0.l,a0                   | +000

| ----------------------------------------------------------------------------
|  Target_AngleToPlayer_05e136  @ $05E136  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Target_AngleToPlayer_05e136, "ax", @progbits
        .global Target_AngleToPlayer_05e136
Target_AngleToPlayer_05e136:
        jsr     Target_AcquireNearestPlayer_05e086__L05e0d4(pc) | +000
        bcs.w   SetXN_05e160                    | +004
        move.w  0x22(a0),d0                     | +008
        move.w  0x24(a0),d1                     | +00c
        sub.w   0x22(a6),d0                     | +010
        sub.w   0x24(a6),d1                     | +014
        movem.w a0,-(a7)                        | +018
        jsr     Atan2_Angle256_05e018(pc)       | +01c
        movem.w (a7)+,a0                        | +020

| ----------------------------------------------------------------------------
|  TargetBox_Default_05e166  @ $05E166  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TargetBox_Default_05e166, "ax", @progbits
        .global TargetBox_Default_05e166
TargetBox_Default_05e166:
        .dc.w   0xff20                        | +000  (dato / opcode no decodificado)
        .dc.w   0x00e0                        | +002  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00a  (dato / opcode no decodificado)
        lea     TargetBox_Default_05e166(pc),a0 | +00c
        jsr     Target_AcquireNearestPlayer_05e086(pc) | +010
        bcs.w   SetXN_05e1a4                    | +014
        move.w  0x22(a0),d0                     | +018
        move.w  0x24(a0),d1                     | +01c
        addi.w  #0x1c,d1                        | +020
        sub.w   0x22(a6),d0                     | +024
        sub.w   0x24(a6),d1                     | +028
        movem.w a0,-(a7)                        | +02c
        jsr     Atan2_Angle256_05e018(pc)       | +030
        movem.w (a7)+,a0                        | +034

| ----------------------------------------------------------------------------
|  Players_AliveMask_05e1aa  @ $05E1AA  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Players_AliveMask_05e1aa, "ax", @progbits
        .global Players_AliveMask_05e1aa
Players_AliveMask_05e1aa:
        clr.l   d0                              | +000
        jsr     Player_GetEntity_05e3a2(pc)     | +002
        bcc.w   .L05e1bc                        | +006
        move.w  #0x1,d0                         | +00a
        bra.w   .L05e1be                        | +00e
.L05e1bc:
        clr.w   d0                              | +012
.L05e1be:
        move.l  d0,-(a7)                        | +014
        moveq   #1,d0                           | +016
        jsr     Player_GetEntity_05e3a2(pc)     | +018
        bcc.w   .L05e1d2                        | +01c
        move.w  #0x2,d1                         | +020
        bra.w   .L05e1d4                        | +024
.L05e1d2:
        clr.w   d1                              | +028
.L05e1d4:
        move.l  (a7)+,d0                        | +02a
        or.w    d1,d0                           | +02c
        tst.w   d0                              | +02e
        beq.w   SetXN_05e1e4                    | +030

| ----------------------------------------------------------------------------
|  Players_PickRandomAlive_05e1ea  @ $05E1EA  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Players_PickRandomAlive_05e1ea, "ax", @progbits
        .global Players_PickRandomAlive_05e1ea
Players_PickRandomAlive_05e1ea:
        jsr     Players_AliveMask_05e1aa(pc)    | +000
        cmpi.w  #0x3,d0                         | +004
        bcs.w   .L05e202                        | +008
        jsr     0x5e9b6.l                       | +00c
        andi.w  #0x1,d0                         | +012
        addq.w  #0x1,d0                         | +016
.L05e202:
        lea     0x100440.l,a0                   | +018
        cmpi.w  #0x2,d0                         | +01e
        bne.w   .L05e216                        | +022
        lea     0x1004e0.l,a0                   | +026
.L05e216:
        cmpi.w  #0x0,d0                         | +02c
        beq.w   SetXN_05e224                    | +030

| ----------------------------------------------------------------------------
|  Dist_Approx_05e22a  @ $05E22A  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Dist_Approx_05e22a, "ax", @progbits
        .global Dist_Approx_05e22a
Dist_Approx_05e22a:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        move.w  0x22(a0),d2                     | +008
        move.w  0x24(a0),d3                     | +00c
        sub.w   d2,d0                           | +010
        sub.w   d3,d1                           | +012
        cmpi.w  #0x0,d0                         | +014
        bge.w   .L05e248                        | +018
        neg.w   d0                              | +01c
.L05e248:
        cmpi.w  #0x0,d1                         | +01e
        bge.w   .L05e252                        | +022
        neg.w   d1                              | +026
.L05e252:
        cmp.w   d1,d0                           | +028
        bgt.w   .L05e25a                        | +02a
        exg     d0,d1                           | +02e
.L05e25a:
        lsr.w   #0x1,d1                         | +030
        add.w   d1,d0                           | +032
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Target_InRangeBox_05e260  @ $05E260  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Target_InRangeBox_05e260, "ax", @progbits
        .global Target_InRangeBox_05e260
Target_InRangeBox_05e260:
        move.w  0x22(a0),d1                     | +000
        sub.w   0x22(a6),d1                     | +004
        btst    #0x0,0x3a(a6)                   | +008
        bne.w   .L05e284                        | +00e
        cmp.w   (a1),d1                         | +012
        blt.w   SetXN_05e2d2                    | +014
        cmp.w   0x2(a1),d1                      | +018
        bgt.w   SetXN_05e2d2                    | +01c
        bra.w   .L05e294                        | +020
.L05e284:
        neg.w   d1                              | +024
        cmp.w   (a1),d1                         | +026
        blt.w   SetXN_05e2d2                    | +028
        cmp.w   0x2(a1),d1                      | +02c
        bgt.w   SetXN_05e2d2                    | +030
.L05e294:
        move.w  0x24(a0),d1                     | +034
        sub.w   0x24(a6),d1                     | +038
        btst    #0x1,0x3a(a6)                   | +03c
        bne.w   .L05e2ba                        | +042
        cmp.w   0x4(a1),d1                      | +046
        blt.w   SetXN_05e2d2                    | +04a
        cmp.w   0x6(a1),d1                      | +04e
        bgt.w   SetXN_05e2d2                    | +052
        bra.w   ClearXN_05e2cc                  | +056
.L05e2ba:
        neg.w   d1                              | +05a
        cmp.w   0x4(a1),d1                      | +05c
        blt.w   SetXN_05e2d2                    | +060
        cmp.w   0x6(a1),d1                      | +064
        bgt.w   SetXN_05e2d2                    | +068

| ----------------------------------------------------------------------------
|  Player_IndexFromPtr_05e338  @ $05E338  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Player_IndexFromPtr_05e338, "ax", @progbits
        .global Player_IndexFromPtr_05e338
Player_IndexFromPtr_05e338:
        clr.w   d0                              | +000
        move.l  a0,d1                           | +002
        cmpi.l  #0x100440,d1                    | +004
        beq.w   .L05e352                        | +00a
        addq.w  #0x1,d0                         | +00e
        cmpi.l  #0x1004e0,d1                    | +010
        bne.w   SetXN_05e360                    | +016
.L05e352:
        jsr     Player_ProbeFlag6B_05e42a(pc)   | +01a
        bcc.w   SetXN_05e360                    | +01e

| ----------------------------------------------------------------------------
|  Players_AnyAliveNotHit_05e366  @ $05E366  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Players_AnyAliveNotHit_05e366, "ax", @progbits
        .global Players_AnyAliveNotHit_05e366
Players_AnyAliveNotHit_05e366:
        clr.w   d0                              | +000
        jsr     Player_GetEntity_05e3a2(pc)     | +002
        bcc.w   Player2_AliveNotHit_05e380      | +006
        btst    #0x0,0x13(a0)                   | +00a
        bne.w   Player2_AliveNotHit_05e380      | +010

| ----------------------------------------------------------------------------
|  Player2_AliveNotHit_05e380  @ $05E380  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Player2_AliveNotHit_05e380, "ax", @progbits
        .global Player2_AliveNotHit_05e380
Player2_AliveNotHit_05e380:
        move.w  #0x1,d0                         | +000
        jsr     Player_GetEntity_05e3a2(pc)     | +004
        bcc.w   SetXN_05e39c                    | +008
        btst    #0x0,0x13(a0)                   | +00c
        bne.w   SetXN_05e39c                    | +012

| ----------------------------------------------------------------------------
|  Player_GetEntity_05e3a2  @ $05E3A2  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Player_GetEntity_05e3a2, "ax", @progbits
        .global Player_GetEntity_05e3a2
Player_GetEntity_05e3a2:
        andi.w  #0x1,d0                         | +000
        lea     PlayerEntityPtrs_05e3f4(pc),a0  | +004
        asl.l   #0x2,d0                         | +008
        movea.l (a0,d0.w),a0                    | +00a
        btst    #0x7,0x5b(a0)                   | +00e
        beq.w   Player_GetEntityFail_05e3e8     | +014
        .global Player_GetEntity_05e3a2__L05e3ba
Player_GetEntity_05e3a2__L05e3ba:
.L05e3ba:
        cmpi.l  #0xffffffff,(a0)                | +018
        beq.w   Player_GetEntityFail_05e3e8     | +01e
        cmpi.l  #0x52a,(a0)                     | +022
        beq.w   Player_GetEntityFail_05e3e8     | +028
        cmpi.l  #0x400,(a0)                     | +02c
        beq.w   Player_GetEntityFail_05e3e8     | +032
        cmpi.l  #0x2ae3e,(a0)                   | +036
        beq.w   Player_GetEntityFail_05e3e8     | +03c

| ----------------------------------------------------------------------------
|  Player_GetEntityFail_05e3e8  @ $05E3E8  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Player_GetEntityFail_05e3e8, "ax", @progbits
        .global Player_GetEntityFail_05e3e8
Player_GetEntityFail_05e3e8:
        movea.l #0xffffffff,a0                  | +000

| ----------------------------------------------------------------------------
|  PlayerEntityPtrs_05e3f4  @ $05E3F4  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerEntityPtrs_05e3f4, "ax", @progbits
        .global PlayerEntityPtrs_05e3f4
PlayerEntityPtrs_05e3f4:
        .dc.w   0x0010                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +004  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +006  (dato / opcode no decodificado)
        clr.w   d0                              | +008
        jsr     Player_ProbeFlag6B_05e42a(pc)   | +00a
        bcc.w   .L05e40c                        | +00e
        tst.w   d0                              | +012
        beq.w   SetXN_05e424                    | +014
.L05e40c:
        move.w  #0x1,d0                         | +018
        jsr     Player_ProbeFlag6B_05e42a(pc)   | +01c
        bcc.w   ClearXN_05e41e                  | +020
        tst.w   d0                              | +024
        beq.w   SetXN_05e424                    | +026

| ----------------------------------------------------------------------------
|  Player_ProbeFlag6B_05e42a  @ $05E42A  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ProbeFlag6B_05e42a, "ax", @progbits
        .global Player_ProbeFlag6B_05e42a
Player_ProbeFlag6B_05e42a:
        jsr     Player_GetEntity_05e3a2(pc)     | +000
        bcc.w   ClearXN_05e44c                  | +004
        btst    #0x0,0x6b(a6)                   | +008
        beq.w   .L05e442                        | +00e
        clr.b   d0                              | +012
        bra.w   SetXN_05e446                    | +014
.L05e442:
        move.b  #0xff,d0                        | +018

| ----------------------------------------------------------------------------
|  Parent_IsLiveHandler_05e452  @ $05E452  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Parent_IsLiveHandler_05e452, "ax", @progbits
        .global Parent_IsLiveHandler_05e452
Parent_IsLiveHandler_05e452:
        movea.l 0xc(a6),a0                      | +000
        jmp     Player_GetEntity_05e3a2__L05e3ba(pc) | +004

| ----------------------------------------------------------------------------
|  Parent_IsFreed_05e45a  @ $05E45A  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Parent_IsFreed_05e45a, "ax", @progbits
        .global Parent_IsFreed_05e45a
Parent_IsFreed_05e45a:
        movea.l 0xc(a6),a0                      | +000
        cmpi.l  #0xffffffff,(a0)                | +004
        beq.w   SetXN_05e482                    | +00a
        cmpi.l  #0x52a,(a0)                     | +00e
        beq.w   SetXN_05e482                    | +014
        cmpi.l  #0x400,(a0)                     | +018
        beq.w   SetXN_05e482                    | +01e

| ----------------------------------------------------------------------------
|  Entity_IsFreedA0_05e488  @ $05E488  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_IsFreedA0_05e488, "ax", @progbits
        .global Entity_IsFreedA0_05e488
Entity_IsFreedA0_05e488:
        cmpi.l  #0xffffffff,(a0)                | +000
        beq.w   ClearXN_05e4ac                  | +006
        cmpi.l  #0x52a,(a0)                     | +00a
        beq.w   ClearXN_05e4ac                  | +010
        cmpi.l  #0x400,(a0)                     | +014
        beq.w   ClearXN_05e4ac                  | +01a

| ----------------------------------------------------------------------------
|  Parent_GetPrioPos_05e4ca  @ $05E4CA  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Parent_GetPrioPos_05e4ca, "ax", @progbits
        .global Parent_GetPrioPos_05e4ca
Parent_GetPrioPos_05e4ca:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x38(a0),d0                     | +004
        move.w  0x22(a0),d1                     | +008
        move.w  0x24(a0),d2                     | +00c
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Parent_CopyPos_05e4dc  @ $05E4DC  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Parent_CopyPos_05e4dc, "ax", @progbits
        .global Parent_CopyPos_05e4dc
Parent_CopyPos_05e4dc:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Parent_CopyPosPrio_05e4ee  @ $05E4EE  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Parent_CopyPosPrio_05e4ee, "ax", @progbits
        .global Parent_CopyPosPrio_05e4ee
Parent_CopyPosPrio_05e4ee:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        move.w  0x38(a0),0x38(a6)               | +010
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Parent_CopyPosPrioFacing_05e506  @ $05E506  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Parent_CopyPosPrioFacing_05e506, "ax", @progbits
        .global Parent_CopyPosPrioFacing_05e506
Parent_CopyPosPrioFacing_05e506:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        move.b  0x26(a0),0x26(a6)               | +010
        move.b  0x27(a0),0x27(a6)               | +016
        move.b  0x3a(a0),0x3a(a6)               | +01c
        move.w  0x38(a0),0x38(a6)               | +022
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Rng_PickWordFromTable_05e530  @ $05E530  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Rng_PickWordFromTable_05e530, "ax", @progbits
        .global Rng_PickWordFromTable_05e530
Rng_PickWordFromTable_05e530:
        jsr     0x5e9e4.l                       | +000
        asl.w   #0x1,d0                         | +006
        move.w  (a1,d0.w),d0                    | +008

| ----------------------------------------------------------------------------
|  Rng_PickFromTable5_05e544  @ $05E544  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Rng_PickFromTable5_05e544, "ax", @progbits
        .global Rng_PickFromTable5_05e544
Rng_PickFromTable5_05e544:
        lea     RngTable5_05e552(pc),a1         | +000
        move.w  #0x5,d0                         | +004

| ----------------------------------------------------------------------------
|  RngTable5_05e552  @ $05E552  (86 B)
| ----------------------------------------------------------------------------
        .section .text.RngTable5_05e552, "ax", @progbits
        .global RngTable5_05e552
RngTable5_05e552:
        .dc.w   0x103e                        | +000  (dato / opcode no decodificado)
        .dc.w   0x103f                        | +002  (dato / opcode no decodificado)
        .dc.w   0x1040                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1042                        | +006  (dato / opcode no decodificado)
        .dc.w   0x1043                        | +008  (dato / opcode no decodificado)
        moveq   #0,d7                           | +00a
        clr.w   d0                              | +00c
        jsr     Player_GetEntity_05e3a2(pc)     | +00e
        bcc.w   .L05e580                        | +012
        btst    #0x0,0x13(a0)                   | +016
        bne.w   .L05e580                        | +01c
        btst    #0x7,0x5b(a0)                   | +020
        beq.w   .L05e580                        | +026
        bset    #0x0,d7                         | +02a
.L05e580:
        move.w  #0x1,d0                         | +02e
        jsr     Player_GetEntity_05e3a2(pc)     | +032
        bcc.w   .L05e5a4                        | +036
        btst    #0x0,0x13(a0)                   | +03a
        bne.w   .L05e5a4                        | +040
        btst    #0x7,0x5b(a0)                   | +044
        beq.w   .L05e5a4                        | +04a
        bset    #0x1,d7                         | +04e
.L05e5a4:
        move.b  d7,d0                           | +052
        rts                                     | +054

| ----------------------------------------------------------------------------
|  Players_AnyAhead_05e5e0  @ $05E5E0  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Players_AnyAhead_05e5e0, "ax", @progbits
        .global Players_AnyAhead_05e5e0
Players_AnyAhead_05e5e0:
        clr.w   d0                              | +000
        jsr     Player_GetEntity_05e3a2(pc)     | +002
        bcc.w   Player2_IsAhead_05e5f8          | +006
        jsr     Target_IsAhead_05e618(pc)       | +00a
        bcc.w   Player2_IsAhead_05e5f8          | +00e

| ----------------------------------------------------------------------------
|  Player2_IsAhead_05e5f8  @ $05E5F8  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player2_IsAhead_05e5f8, "ax", @progbits
        .global Player2_IsAhead_05e5f8
Player2_IsAhead_05e5f8:
        move.w  #0x1,d0                         | +000
        jsr     Player_GetEntity_05e3a2(pc)     | +004
        bcc.w   SetXN_05e612                    | +008
        jsr     Target_IsAhead_05e618(pc)       | +00c
        bcc.w   SetXN_05e612                    | +010

| ----------------------------------------------------------------------------
|  Target_IsAhead_05e618  @ $05E618  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Target_IsAhead_05e618, "ax", @progbits
        .global Target_IsAhead_05e618
Target_IsAhead_05e618:
        move.w  0x22(a6),d0                     | +000
        sub.w   0x22(a0),d0                     | +004
        btst    #0x0,0x3a(a6)                   | +008
        bne.w   Target_IsAhead_FacingLeft_05e638 | +00e
        cmpi.w  #0x0,d0                         | +012
        blt.w   SetXN_05e646                    | +016

| ----------------------------------------------------------------------------
|  Target_IsAhead_FacingLeft_05e638  @ $05E638  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Target_IsAhead_FacingLeft_05e638, "ax", @progbits
        .global Target_IsAhead_FacingLeft_05e638
Target_IsAhead_FacingLeft_05e638:
        cmpi.w  #0x0,d0                         | +000
        bgt.w   SetXN_05e646                    | +004

| ----------------------------------------------------------------------------
|  Marker_FollowParent_05e64c  @ $05E64C  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Marker_FollowParent_05e64c, "ax", @progbits
        .global Marker_FollowParent_05e64c
Marker_FollowParent_05e64c:
        movea.l 0xc(a6),a1                      | +000
        move.w  0x16(a1),0x16(a6)               | +004
        move.w  #0xe000,0x38(a6)                | +00a
        lea     MarkerSprite_05e698(pc),a0      | +010
        jsr     0x28cd4.l                       | +014
        lea     .L05e66c(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L05e66c:
        jsr     Parent_CopyPos_05e4dc(pc)       | +020
        jsr     0x28d70.l                       | +024
        movea.l 0xc(a6),a1                      | +02a
        cmpi.l  #0xffffffff,(a1)                | +02e
        beq.w   JmpToScheduler_05e690           | +034
        cmpi.l  #0x52a,(a1)                     | +038
        beq.w   JmpToScheduler_05e690           | +03e
        rts                                     | +042

| ----------------------------------------------------------------------------
|  MarkerSprite_05e698  @ $05E698  (12 B)
| ----------------------------------------------------------------------------
        .section .text.MarkerSprite_05e698, "ax", @progbits
        .global MarkerSprite_05e698
MarkerSprite_05e698:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +004  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +00a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Hit_ClassifyAttack_05e6a4  @ $05E6A4  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Hit_ClassifyAttack_05e6a4, "ax", @progbits
        .global Hit_ClassifyAttack_05e6a4
Hit_ClassifyAttack_05e6a4:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0203                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0203                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0202                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0202                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0202                        | +010  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +014  (dato / opcode no decodificado)
        .dc.w   0x870a                        | +016  (dato / opcode no decodificado)
        .global Hit_ClassifyAttack_05e6a4__L05e6bc
Hit_ClassifyAttack_05e6a4__L05e6bc:
.L05e6bc:
        bcc.w   Hit_ClassNone_05e710            | +018
        clr.l   d0                              | +01c
        move.b  0x58(a6),d0                     | +01e
        andi.l  #0xff,d0                        | +022
        clr.b   d1                              | +028
        cmpi.b  #0x5,d0                         | +02a
        ble.w   .L05e6da                        | +02e
        move.b  #0x1,d1                         | +032
.L05e6da:
        cmpi.w  #0x22,d0                        | +036
        blt.w   .L05e6ee                        | +03a
        nop                                     | +03e
        nop                                     | +040
        cmpi.w  #0x22,d0                        | +042
        nop                                     | +046
        trap    #0xf                            | +048
.L05e6ee:
        lea     Hit_ClassifyAttack_05e6a4(pc),a0 | +04a
        move.b  (a0,d0.w),d0                    | +04e
        cmpi.w  #0x5,d0                         | +052
        blt.w   SetXN_05e70a                    | +056
        nop                                     | +05a
        nop                                     | +05c
        cmpi.w  #0x5,d0                         | +05e
        nop                                     | +062
        trap    #0xf                            | +064

| ----------------------------------------------------------------------------
|  Hit_ClassNone_05e710  @ $05E710  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Hit_ClassNone_05e710, "ax", @progbits
        .global Hit_ClassNone_05e710
Hit_ClassNone_05e710:
        clr.l   d1                              | +000
        clr.l   d0                              | +002

| ----------------------------------------------------------------------------
|  Hit_HPDepletedClassify_05e71a  @ $05E71A  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Hit_HPDepletedClassify_05e71a, "ax", @progbits
        .global Hit_HPDepletedClassify_05e71a
Hit_HPDepletedClassify_05e71a:
        jsr     0x28758.l                       | +000
        bra.b   Hit_ClassifyAttack_05e6a4__L05e6bc | +006

| ----------------------------------------------------------------------------
|  Hit_SoundA_05e728  @ $05E728  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Hit_SoundA_05e728, "ax", @progbits
        .global Hit_SoundA_05e728
Hit_SoundA_05e728:
        btst    #0x0,0x5a(a6)                   | +000
        beq.w   JsrAbsRts_05e74a                | +006
        btst    #0x0,0x13(a6)                   | +00a
        bne.w   JsrAbsRts_05e74a                | +010
        .global Hit_SoundA_05e728__L05e73c
Hit_SoundA_05e728__L05e73c:
.L05e73c:
        cmpi.w  #0xffff,d0                      | +014
        beq.w   JsrAbsRts_05e74a                | +018

| ----------------------------------------------------------------------------
|  Hit_SoundB_05e74c  @ $05E74C  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Hit_SoundB_05e74c, "ax", @progbits
        .global Hit_SoundB_05e74c
Hit_SoundB_05e74c:
        btst    #0x1,0x5a(a6)                   | +000
        beq.w   JsrAbsRts_05e764                | +006
        cmpi.w  #0xffff,d0                      | +00a
        beq.w   JsrAbsRts_05e764                | +00e

| ----------------------------------------------------------------------------
|  HitSoundTable_05e766  @ $05E766  (50 B)
| ----------------------------------------------------------------------------
        .section .text.HitSoundTable_05e766, "ax", @progbits
        .global HitSoundTable_05e766
HitSoundTable_05e766:
        .dc.w   0x108d                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        btst    #0x0,0x13(a6)                   | +00a
        bne.w   .L05e796                        | +010
        move.b  0x58(a6),d0                     | +014
        andi.w  #0xff,d0                        | +018
        subq.w  #0x1,d0                         | +01c
        cmpi.w  #0x5,d0                         | +01e
        bcc.w   .L05e796                        | +022
        asl.w   #0x1,d0                         | +026
        move.w  (a0,d0.w),d0                    | +028
        jmp     Hit_SoundA_05e728(pc)           | +02c
.L05e796:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Hit_SoundByAttackClassTbl_05e798  @ $05E798  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Hit_SoundByAttackClassTbl_05e798, "ax", @progbits
        .global Hit_SoundByAttackClassTbl_05e798
Hit_SoundByAttackClassTbl_05e798:
        btst    #0x0,0x13(a6)                   | +000
        bne.w   .L05e7be                        | +006
        move.b  0x58(a6),d0                     | +00a
        andi.w  #0xff,d0                        | +00e
        subq.w  #0x1,d0                         | +012
        cmpi.w  #0x5,d0                         | +014
        bcc.w   .L05e7be                        | +018
        asl.w   #0x1,d0                         | +01c
        move.w  (a0,d0.w),d0                    | +01e
        jmp     Hit_SoundA_05e728__L05e73c(pc)  | +022
.L05e7be:
        rts                                     | +026

| ----------------------------------------------------------------------------
|  Entity_SnapToGround_05e7c0  @ $05E7C0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SnapToGround_05e7c0, "ax", @progbits
        .global Entity_SnapToGround_05e7c0
Entity_SnapToGround_05e7c0:
        andi.w  #0x3ff,0x24(a6)                 | +000
.L05e7c6:
        cmpi.w  #0x0,0x24(a6)                   | +006
        blt.w   Entity_ResetPosDefault_05e7de   | +00c
        jsr     0x27c8c.l                       | +010
        bcc.b   .L05e7c6                        | +016

| ----------------------------------------------------------------------------
|  Entity_ResetPosDefault_05e7de  @ $05E7DE  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_ResetPosDefault_05e7de, "ax", @progbits
        .global Entity_ResetPosDefault_05e7de
Entity_ResetPosDefault_05e7de:
        move.w  #0xa0,0x22(a6)                  | +000
        move.w  #0x180,0x24(a6)                 | +006

| ----------------------------------------------------------------------------
|  Fix_Write106E8C_05e7f0  @ $05E7F0  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_Write106E8C_05e7f0, "ax", @progbits
        .global Fix_Write106E8C_05e7f0
Fix_Write106E8C_05e7f0:
        movea.l #0x7144,a1                      | +000
        move.w  0x106e8c.l,d0                   | +006

| ----------------------------------------------------------------------------
|  Scroll_IsPastQuarter_05e804  @ $05E804  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Scroll_IsPastQuarter_05e804, "ax", @progbits
        .global Scroll_IsPastQuarter_05e804
Scroll_IsPastQuarter_05e804:
        move.w  0x10e1e8.l,d0                   | +000
        move.w  0x10e1fa.l,d1                   | +006
        sub.w   0x10e1f6.l,d1                   | +00c
        move.w  d1,d2                           | +012
        lsr.w   #0x2,d2                         | +014
        cmp.w   d2,d0                           | +016
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Entity_SetField14By106F28_05e81e  @ $05E81E  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SetField14By106F28_05e81e, "ax", @progbits
        .global Entity_SetField14By106F28_05e81e
Entity_SetField14By106F28_05e81e:
        ori.b   #0x18,(a6)                      | +000
        ori.b   #0x1a,(a6)                      | +004
        moveq   #0,d0                           | +008
        move.b  0x106f28.l,d0                   | +00a
        andi.w  #0x3,d0                         | +010
        lsl.w   #0x1,d0                         | +014
        lea     Entity_SetField14By106F28_05e81e(pc),a0 | +016
        move.w  (a0,d0.w),d1                    | +01a
        move.w  (a6,d1.w),0x14(a6)              | +01e
        rts                                     | +024

| ----------------------------------------------------------------------------
|  Attack_IsHeavyType_05e844  @ $05E844  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Attack_IsHeavyType_05e844, "ax", @progbits
        .global Attack_IsHeavyType_05e844
Attack_IsHeavyType_05e844:
        cmpi.b  #0x2,0x58(a6)                   | +000
        beq.w   SetXN_05e890                    | +006
        cmpi.b  #0x3,0x58(a6)                   | +00a
        beq.w   SetXN_05e890                    | +010
        cmpi.b  #0x15,0x58(a6)                  | +014
        beq.w   SetXN_05e890                    | +01a
        cmpi.b  #0x16,0x58(a6)                  | +01e
        beq.w   SetXN_05e890                    | +024
        cmpi.b  #0x17,0x58(a6)                  | +028
        beq.w   SetXN_05e890                    | +02e
        cmpi.b  #0x18,0x58(a6)                  | +032
        beq.w   SetXN_05e890                    | +038
        cmpi.b  #0x1e,0x58(a6)                  | +03c
        beq.w   SetXN_05e890                    | +042

| ----------------------------------------------------------------------------
|  Players_AnyHit2AC0E_05e896  @ $05E896  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Players_AnyHit2AC0E_05e896, "ax", @progbits
        .global Players_AnyHit2AC0E_05e896
Players_AnyHit2AC0E_05e896:
        lea     0x100440.l,a0                   | +000
        jsr     0x2ac0e.l                       | +006
        bcs.w   Stub_0005E8B8                   | +00c
        lea     0x1004e0.l,a0                   | +010
        jsr     0x2ac0e.l                       | +016
        bcs.w   Stub_0005E8B8                   | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Entity_IsOnscreenMin_05e8ba  @ $05E8BA  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_IsOnscreenMin_05e8ba, "ax", @progbits
        .global Entity_IsOnscreenMin_05e8ba
Entity_IsOnscreenMin_05e8ba:
        cmpi.w  #0x20,0x24(a6)                  | +000
        blt.w   SetXN_05e8d4                    | +006
        cmpi.w  #0xff80,0x22(a6)                | +00a
        blt.w   SetXN_05e8d4                    | +010

| ----------------------------------------------------------------------------
|  Attack_ApplyThenSound10FA_05e8da  @ $05E8DA  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Attack_ApplyThenSound10FA_05e8da, "ax", @progbits
        .global Attack_ApplyThenSound10FA_05e8da
Attack_ApplyThenSound10FA_05e8da:
        jsr     0x283d8.l                       | +000
        btst    #0x1,0x5a(a6)                   | +006
        beq.w   JsrAbsRts_05e8f4                | +00c
        move.w  #0x10fa,d0                      | +010

| ----------------------------------------------------------------------------
|  Owner_GenCheck_05e8f6  @ $05E8F6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Owner_GenCheck_05e8f6, "ax", @progbits
        .global Owner_GenCheck_05e8f6
Owner_GenCheck_05e8f6:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_05e90c                    | +00c

| ----------------------------------------------------------------------------
|  Hud_WriteTimerCounters_05e912  @ $05E912  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Hud_WriteTimerCounters_05e912, "ax", @progbits
        .global Hud_WriteTimerCounters_05e912
Hud_WriteTimerCounters_05e912:
        movea.l #0x70a5,a1                      | +000
        moveq   #0,d0                           | +006
        move.b  0x32(a6),d0                     | +008
        jsr     0x5d6c2.l                       | +00c
        move.w  #0x0,d1                         | +012
        move.w  d1,d2                           | +016
        move.w  0x30(a6),d1                     | +018
        move.b  0x32(a6),d2                     | +01c
        add.w   d2,d1                           | +020
        cmpi.w  #0xff,d1                        | +022
        ble.w   .L05e94a                        | +026
        move.b  #0xff,d1                        | +02a
        move.w  #0xffff,0x30(a6)                | +02e
        bra.w   .L05e95c                        | +034
.L05e94a:
        cmpi.w  #0x1,d1                         | +038
        bge.w   .L05e95c                        | +03c
        move.b  #0x1,d1                         | +040
        move.w  #0x1,0x30(a6)                   | +044
.L05e95c:
        move.b  d1,0x32(a6)                     | +04a
        move.b  d1,0x33(a6)                     | +04e
        movea.l #0x70a7,a1                      | +052
        move.w  0x10e1e4.l,d0                   | +058
        jsr     0x5d6c2.l                       | +05e
        movea.l #0x7147,a1                      | +064
        move.w  0x10e1e6.l,d0                   | +06a

| ----------------------------------------------------------------------------
|  Entity_MarkFlag2TimerMax_05e98a  @ $05E98A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_MarkFlag2TimerMax_05e98a, "ax", @progbits
        .global Entity_MarkFlag2TimerMax_05e98a
Entity_MarkFlag2TimerMax_05e98a:
        ori.b   #0x4,0x12(a6)                   | +000
        move.w  #0xffff,0x30(a6)                | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Rng_Seed_05e998  @ $05E998  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Rng_Seed_05e998, "ax", @progbits
        .global Rng_Seed_05e998
Rng_Seed_05e998:
        lea     0x10e230.l,a0                   | +000
        moveq   #31,d7                          | +006
.L05e9a0:
        move.w  d0,(a0)+                        | +008
        mulu.w  #0xb7c7,d0                      | +00a
        addi.w  #0x81f5,d0                      | +00e
        dbra    d7,.L05e9a0                     | +012
        clr.w   0x10e270.l                      | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Rng_Mask_05ea1c  @ $05EA1C  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Rng_Mask_05ea1c, "ax", @progbits
        .global Rng_Mask_05ea1c
Rng_Mask_05ea1c:
        move.w  d0,d5                           | +000
        lea     0x10e230.l,a4                   | +002
        move.w  0x10e270.l,d7                   | +008
        addi.w  #0x2,d7                         | +00e
        move.w  #0x3e,d6                        | +012
        and.w   d6,d7                           | +016
        move.w  d7,0x10e270.l                   | +018
        move.w  (a4,d7.w),d0                    | +01e
        rol.w   #0x1,d0                         | +022
        subi.w  #0x15,d7                        | +024
        and.w   d6,d7                           | +028
        eor.w   d0,(a4,d7.w)                    | +02a
        move.w  (a4,d7.w),d0                    | +02e
        and.w   d5,d0                           | +032
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Owner_GenCheck_05ea52  @ $05EA52  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Owner_GenCheck_05ea52, "ax", @progbits
        .global Owner_GenCheck_05ea52
Owner_GenCheck_05ea52:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_05ea68                    | +00c

| ----------------------------------------------------------------------------
|  Slot1008A0_SetHandlerIfA5_05ea6e  @ $05EA6E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Slot1008A0_SetHandlerIfA5_05ea6e, "ax", @progbits
        .global Slot1008A0_SetHandlerIfA5_05ea6e
Slot1008A0_SetHandlerIfA5_05ea6e:
        cmpa.l  #0xffffffff,a5                  | +000
        beq.w   .L05ea84                        | +006
        lea     0x1008a0.l,a0                   | +00a
        move.l  #0x5ea96,(a0)                   | +010
.L05ea84:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Slot1008A0_SetHandler400_05ea86  @ $05EA86  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Slot1008A0_SetHandler400_05ea86, "ax", @progbits
        .global Slot1008A0_SetHandler400_05ea86
Slot1008A0_SetHandler400_05ea86:
        lea     0x1008a0.l,a0                   | +000
        move.l  #0x400,(a0)                     | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Task_Clear106F42_05ea96  @ $05EA96  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Task_Clear106F42_05ea96, "ax", @progbits
        .global Task_Clear106F42_05ea96
Task_Clear106F42_05ea96:
        lea     .L05ea9c(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L05ea9c:
        clr.w   0x106f42.l                      | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Slot1008A0_Alloc4AE_05eaa4  @ $05EAA4  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Slot1008A0_Alloc4AE_05eaa4, "ax", @progbits
        .global Slot1008A0_Alloc4AE_05eaa4
Slot1008A0_Alloc4AE_05eaa4:
        move.l  a6,-(a7)                        | +000
        lea     0x1008a0.l,a6                   | +002
        jsr     0x4ae.l                         | +008
        movea.l (a7)+,a6                        | +00e
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Slot1008A0_Alloc6FE_05eab6  @ $05EAB6  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Slot1008A0_Alloc6FE_05eab6, "ax", @progbits
        .global Slot1008A0_Alloc6FE_05eab6
Slot1008A0_Alloc6FE_05eab6:
        move.l  a6,-(a7)                        | +000
        lea     0x1008a0.l,a6                   | +002
        jsr     0x6fe.l                         | +008
        movea.l (a7)+,a6                        | +00e
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Owner_GenCheck_05eac8  @ $05EAC8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Owner_GenCheck_05eac8, "ax", @progbits
        .global Owner_GenCheck_05eac8
Owner_GenCheck_05eac8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_05eade                    | +00c

| ----------------------------------------------------------------------------
|  Fix_DrawMessageRow_05eae4  @ $05EAE4  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_DrawMessageRow_05eae4, "ax", @progbits
        .global Fix_DrawMessageRow_05eae4
Fix_DrawMessageRow_05eae4:
        lea     MessageTileRows_05eb98(pc),a0   | +000
        jsr     0x2b58.l                        | +004
        movea.w #0x704d,a1                      | +00a
        lea     MessageTileRows_05eb98__L05eb9e(pc),a0 | +00e
        asl.l   #0x2,d3                         | +012
        adda.l  d3,a0                           | +014
        movea.l (a0),a0                         | +016
        move.w  #0x2,d1                         | +018
        move.w  #0x2,d2                         | +01c
.L05eb04:
        move.w  (a0),d0                         | +020
        cmpi.w  #0xffff,d0                      | +022
        beq.w   .L05eb2c                        | +026
        ori.w   #0x1000,d0                      | +02a
        andi.w  #0x1fff,d0                      | +02e
        movem.l d0-d2/a0-a1,-(a7)               | +032
        jsr     0x5da56.l                       | +036
        movem.l (a7)+,d0-d2/a0-a1               | +03c
        addq.l  #0x2,a0                         | +040
        adda.w  #0x40,a1                        | +042
        bra.b   .L05eb04                        | +046
.L05eb2c:
        move.w  #0x1,d1                         | +048
        move.w  #0x192,d2                       | +04c
        move.w  #0x1,d3                         | +050
        move.w  #0x1,d4                         | +054

| ----------------------------------------------------------------------------
|  Fix_DrawTileRow_Step40_05eb44  @ $05EB44  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_DrawTileRow_Step40_05eb44, "ax", @progbits
        .global Fix_DrawTileRow_Step40_05eb44
Fix_DrawTileRow_Step40_05eb44:
        move.w  #0x2,d1                         | +000
        move.w  #0x2,d2                         | +004
.L05eb4c:
        move.w  (a0),d0                         | +008
        cmpi.w  #0xffff,d0                      | +00a
        beq.w   .L05eb6c                        | +00e
        movem.l d0-d2/a0-a1,-(a7)               | +012
        jsr     0x5da56.l                       | +016
        movem.l (a7)+,d0-d2/a0-a1               | +01c
        addq.l  #0x2,a0                         | +020
        adda.w  #0x40,a1                        | +022
        bra.b   .L05eb4c                        | +026
.L05eb6c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Fix_DrawTileRow_Step20_05eb6e  @ $05EB6E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_DrawTileRow_Step20_05eb6e, "ax", @progbits
        .global Fix_DrawTileRow_Step20_05eb6e
Fix_DrawTileRow_Step20_05eb6e:
        move.w  #0x1,d1                         | +000
        move.w  #0x2,d2                         | +004
.L05eb76:
        move.w  (a0),d0                         | +008
        cmpi.w  #0xffff,d0                      | +00a
        beq.w   .L05eb96                        | +00e
        movem.l d0-d2/a0-a1,-(a7)               | +012
        jsr     0x5da56.l                       | +016
        movem.l (a7)+,d0-d2/a0-a1               | +01c
        addq.l  #0x2,a0                         | +020
        adda.w  #0x20,a1                        | +022
        bra.b   .L05eb76                        | +026
.L05eb96:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  MessageTileRows_05eb98  @ $05EB98  (1038 B)
| ----------------------------------------------------------------------------
        .section .text.MessageTileRows_05eb98, "ax", @progbits
        .global MessageTileRows_05eb98
MessageTileRows_05eb98:
        .dc.w   0x0801                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .global MessageTileRows_05eb98__L05eb9e
MessageTileRows_05eb98__L05eb9e:
.L05eb9e:
        .dc.w   0x0005                        | +006  (dato / opcode no decodificado)
        .dc.w   0xec16                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xec34                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xec52                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +012  (dato / opcode no decodificado)
        .dc.w   0xec70                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +016  (dato / opcode no decodificado)
        .dc.w   0xec8e                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xecac                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xecca                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +022  (dato / opcode no decodificado)
        .dc.w   0xece8                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +026  (dato / opcode no decodificado)
        .dc.w   0xed06                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xed24                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xed42                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +032  (dato / opcode no decodificado)
        .dc.w   0xed60                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +036  (dato / opcode no decodificado)
        .dc.w   0xed7e                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xed9c                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +03e  (dato / opcode no decodificado)
        .dc.w   0xedba                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +042  (dato / opcode no decodificado)
        .dc.w   0xedd8                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +046  (dato / opcode no decodificado)
        .dc.w   0xedf6                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xee14                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xee32                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +052  (dato / opcode no decodificado)
        .dc.w   0xee50                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +056  (dato / opcode no decodificado)
        .dc.w   0xee6e                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xee8c                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +05e  (dato / opcode no decodificado)
        .dc.w   0xeeaa                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +062  (dato / opcode no decodificado)
        .dc.w   0xeec8                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +066  (dato / opcode no decodificado)
        .dc.w   0xeee6                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xef04                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +06e  (dato / opcode no decodificado)
        .dc.w   0xef22                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +072  (dato / opcode no decodificado)
        .dc.w   0xef40                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +076  (dato / opcode no decodificado)
        .dc.w   0xef5e                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xef7c                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x4c8a                        | +090  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x4c82                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x4c88                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x4c8a                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x4c8a                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x4c8a                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +100  (dato / opcode no decodificado)
        .dc.w   0x4c8a                        | +102  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +104  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +106  (dato / opcode no decodificado)
        .dc.w   0x4baa                        | +108  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x4c88                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +110  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +114  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x4b86                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +120  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +122  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +124  (dato / opcode no decodificado)
        .dc.w   0x4c82                        | +126  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +128  (dato / opcode no decodificado)
        .dc.w   0x4baa                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +12e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x4c8a                        | +140  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +142  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +144  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +146  (dato / opcode no decodificado)
        .dc.w   0x4bac                        | +148  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x4ba4                        | +14c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +154  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +156  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +158  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x4ba4                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +160  (dato / opcode no decodificado)
        .dc.w   0x4c86                        | +162  (dato / opcode no decodificado)
        .dc.w   0x4c82                        | +164  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +166  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +168  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +16a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +170  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +174  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +176  (dato / opcode no decodificado)
        .dc.w   0x4b86                        | +178  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x4ba4                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +180  (dato / opcode no decodificado)
        .dc.w   0x4c88                        | +182  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +184  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +186  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +188  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +18a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +190  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +192  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +194  (dato / opcode no decodificado)
        .dc.w   0x4b86                        | +196  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +198  (dato / opcode no decodificado)
        .dc.w   0x4ba4                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +19c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +19e  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0x4bac                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0x4b62                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +1be  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0x4b64                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0x4b66                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +200  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +202  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +204  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +206  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +208  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +20c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +20e  (dato / opcode no decodificado)
        .dc.w   0x4b68                        | +210  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +212  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +214  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +216  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +218  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +21a  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +21c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +21e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +220  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +222  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +224  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +226  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +228  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +22a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +22c  (dato / opcode no decodificado)
        .dc.w   0x4b6a                        | +22e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +230  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +232  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +234  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +236  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +238  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +23a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +23c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +23e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +240  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +242  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +244  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +246  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +248  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +24a  (dato / opcode no decodificado)
        .dc.w   0x4b6c                        | +24c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +24e  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +250  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +252  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +254  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +256  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +258  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +25a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +25c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +25e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +260  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +262  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +264  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +266  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +268  (dato / opcode no decodificado)
        .dc.w   0x4b6e                        | +26a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +26c  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +26e  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +270  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +272  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +274  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +276  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +278  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +27a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +27c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +27e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +280  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +282  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +284  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +286  (dato / opcode no decodificado)
        .dc.w   0x4c60                        | +288  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +28a  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +28c  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +28e  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +290  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +292  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +294  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +296  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +298  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +29a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +29c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +29e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2a0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2a2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2a4  (dato / opcode no decodificado)
        .dc.w   0x4c62                        | +2a6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2a8  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +2aa  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +2ac  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +2ae  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +2b0  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +2b2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2b6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2b8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2ba  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2bc  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2be  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2c0  (dato / opcode no decodificado)
        .dc.w   0x4b62                        | +2c2  (dato / opcode no decodificado)
        .dc.w   0x4b64                        | +2c4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2c6  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +2c8  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +2ca  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +2cc  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +2ce  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +2d0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2d4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2d6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2d8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2da  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2dc  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2de  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2e0  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +2e2  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +2e4  (dato / opcode no decodificado)
        .dc.w   0x4baa                        | +2e6  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +2e8  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +2ea  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2ec  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2ee  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2f2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2f4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2f6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2f8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2fa  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2fc  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +2fe  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +300  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +302  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +304  (dato / opcode no decodificado)
        .dc.w   0x4c82                        | +306  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +308  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +30a  (dato / opcode no decodificado)
        .dc.w   0x4b62                        | +30c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +30e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +310  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +312  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +314  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +316  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +318  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +31a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +31c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +31e  (dato / opcode no decodificado)
        .dc.w   0x4c88                        | +320  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +322  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +324  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +326  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +328  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +32a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +32c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +32e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +330  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +332  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +334  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +336  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +338  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +33a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +33c  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +33e  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +340  (dato / opcode no decodificado)
        .dc.w   0x4ca4                        | +342  (dato / opcode no decodificado)
        .dc.w   0x4b86                        | +344  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +346  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +348  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +34a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +34c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +34e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +350  (dato / opcode no decodificado)
        .dc.w   0x4bac                        | +352  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +354  (dato / opcode no decodificado)
        .dc.w   0x4c88                        | +356  (dato / opcode no decodificado)
        .dc.w   0x4b86                        | +358  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +35a  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +35c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +35e  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +360  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +362  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +364  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +366  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +368  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +36a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +36c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +36e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +370  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +372  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +374  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +376  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +378  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +37a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +37c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +37e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +380  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +382  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +384  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +386  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +388  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +38a  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +38c  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +38e  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +390  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +392  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +394  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +396  (dato / opcode no decodificado)
        .dc.w   0x4ba0                        | +398  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +39a  (dato / opcode no decodificado)
        .dc.w   0x4c82                        | +39c  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +39e  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +3a0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3a2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3a6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3a8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3aa  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3ac  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3ae  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3b0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3b2  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +3b4  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +3b6  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +3b8  (dato / opcode no decodificado)
        .dc.w   0x4c82                        | +3ba  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +3bc  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +3be  (dato / opcode no decodificado)
        .dc.w   0x4b64                        | +3c0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3c4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3c6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3c8  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +3ca  (dato / opcode no decodificado)
        .dc.w   0x4c80                        | +3cc  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +3ce  (dato / opcode no decodificado)
        .dc.w   0x4bae                        | +3d0  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3d2  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +3d4  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +3d6  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +3d8  (dato / opcode no decodificado)
        .dc.w   0x4c82                        | +3da  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +3dc  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +3de  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3e0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3e2  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3e4  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3e6  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3e8  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3ea  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3ec  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3ee  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +3f0  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +3f2  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +3f4  (dato / opcode no decodificado)
        .dc.w   0x4c82                        | +3f6  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +3f8  (dato / opcode no decodificado)
        .dc.w   0x4b8e                        | +3fa  (dato / opcode no decodificado)
        .dc.w   0x4b66                        | +3fc  (dato / opcode no decodificado)
        .dc.w   0x0b00                        | +3fe  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +400  (dato / opcode no decodificado)
        clr.l   d0                              | +402
        move.b  0x10fdda.l,d0                   | +404
        movea.w #0x7064,a1                      | +40a

| ----------------------------------------------------------------------------
|  Owner_GenCheck_05efae  @ $05EFAE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Owner_GenCheck_05efae, "ax", @progbits
        .global Owner_GenCheck_05efae
Owner_GenCheck_05efae:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_05efc4                    | +00c

| ----------------------------------------------------------------------------
|  DebugColl_Init_05efca  @ $05EFCA  (64 B)
| ----------------------------------------------------------------------------
        .section .text.DebugColl_Init_05efca, "ax", @progbits
        .global DebugColl_Init_05efca
DebugColl_Init_05efca:
        btst    #0x1,0x100001.l                 | +000
        beq.w   .L05efe0                        | +008
        lea     DebugColl_Cursor_05f212(pc),a1  | +00c
        jsr     0x4ae.l                         | +010
.L05efe0:
        move.w  #0x10,0x22(a6)                  | +016
        move.w  #0x1e8,0x24(a6)                 | +01c
        move.w  #0x7043,0x70(a6)                | +022
        move.b  #0xff,0x32(a6)                  | +028
        move.b  #0xff,0x33(a6)                  | +02e
        move.b  #0x0,0x3a(a6)                   | +034
        move.w  #0x1,0x38(a6)                   | +03a

| ----------------------------------------------------------------------------
|  DebugColl_Idle_05f00a  @ $05F00A  (52 B)
| ----------------------------------------------------------------------------
        .section .text.DebugColl_Idle_05f00a, "ax", @progbits
        .global DebugColl_Idle_05f00a
DebugColl_Idle_05f00a:
        move.w  #0xffff,0x74(a6)                | +000
        lea     .L05f016(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L05f016:
        jsr     0x5cd06.l                       | +00c
        bcc.w   SetHandlerRts_05f044            | +012
        jsr     0x5ce14.l                       | +016
        bcc.w   .L05f030                        | +01c
        lea     DebugColl_ShowLowNibble_05f046(pc),a1 | +020
        move.l  a1,(a6)                         | +024
.L05f030:
        bcs.w   SetHandlerRts_05f044            | +026
        jsr     0x5ce26.l                       | +02a
        bcc.w   SetHandlerRts_05f044            | +030

| ----------------------------------------------------------------------------
|  DebugColl_ShowLowNibble_05f046  @ $05F046  (98 B)
| ----------------------------------------------------------------------------
        .section .text.DebugColl_ShowLowNibble_05f046, "ax", @progbits
        .global DebugColl_ShowLowNibble_05f046
DebugColl_ShowLowNibble_05f046:
        move.w  #0xffff,0x74(a6)                | +000
        lea     .L05f052(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L05f052:
        clr.l   d0                              | +00c
        clr.l   d1                              | +00e
        move.w  0x74(a6),d0                     | +010
        move.l  0x106f50.l,d1                   | +014
        swap    d1                              | +01a
        cmp.w   d1,d0                           | +01c
        beq.w   .L05f07e                        | +01e
        jsr     0xc004c2.l                      | +022
        jsr     DebugColl_DrawLowNibble_05f11a(pc) | +028
        move.l  0x106f50.l,d0                   | +02c
        swap    d0                              | +032
        move.w  d0,0x74(a6)                     | +034
.L05f07e:
        jsr     0x5cd06.l                       | +038
        bcc.w   SetHandlerRts_05f0ae            | +03e
        jsr     0x5cce2.l                       | +042
        bcc.w   .L05f098                        | +048
        lea     DebugColl_ShowHighNibble_05f0b0(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L05f098:
        jsr     0x5ce38.l                       | +052
        bcc.w   SetHandlerRts_05f0ae            | +058
        jsr     0xc004c2.l                      | +05c

| ----------------------------------------------------------------------------
|  DebugColl_ShowHighNibble_05f0b0  @ $05F0B0  (98 B)
| ----------------------------------------------------------------------------
        .section .text.DebugColl_ShowHighNibble_05f0b0, "ax", @progbits
        .global DebugColl_ShowHighNibble_05f0b0
DebugColl_ShowHighNibble_05f0b0:
        move.w  #0xffff,0x74(a6)                | +000
        lea     .L05f0bc(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L05f0bc:
        clr.l   d0                              | +00c
        clr.l   d1                              | +00e
        move.w  0x74(a6),d0                     | +010
        move.l  0x106f50.l,d1                   | +014
        swap    d1                              | +01a
        cmp.w   d1,d0                           | +01c
        beq.w   .L05f0e8                        | +01e
        jsr     0xc004c2.l                      | +022
        jsr     DebugColl_DrawHighNibble_05f18e(pc) | +028
        move.l  0x106f50.l,d0                   | +02c
        swap    d0                              | +032
        move.w  d0,0x74(a6)                     | +034
.L05f0e8:
        jsr     0x5cd06.l                       | +038
        bcc.w   SetHandlerRts_05f118            | +03e
        jsr     0x5ccd0.l                       | +042
        bcc.w   .L05f102                        | +048
        lea     DebugColl_ShowLowNibble_05f046(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L05f102:
        jsr     0x5ce38.l                       | +052
        bcc.w   SetHandlerRts_05f118            | +058
        jsr     0xc004c2.l                      | +05c

| ----------------------------------------------------------------------------
|  DebugColl_DrawLowNibble_05f11a  @ $05F11A  (116 B)
| ----------------------------------------------------------------------------
        .section .text.DebugColl_DrawLowNibble_05f11a, "ax", @progbits
        .global DebugColl_DrawLowNibble_05f11a
DebugColl_DrawLowNibble_05f11a:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        jsr     0x43f02.l                       | +008
        lea     DebugColl_Tiles_05f30a(pc),a0   | +00e
        andi.l  #0xf,d0                         | +012
        cmpi.l  #0x0,d0                         | +018
        beq.w   .L05f154                        | +01e
        asl.l   #0x1,d0                         | +022
        adda.l  d0,a0                           | +024
        move.w  (a0),d0                         | +026
        movea.w 0x70(a6),a1                     | +028
        move.w  #0x1,d1                         | +02c
        move.w  #0x1,d2                         | +030
        jsr     0x5da56.l                       | +034
.L05f154:
        addq.w  #0x1,0x70(a6)                   | +03a
        subq.w  #0x8,0x24(a6)                   | +03e
        cmpi.w  #0x118,0x24(a6)                 | +042
        bcc.b   DebugColl_DrawLowNibble_05f11a  | +048
        move.w  #0x1e8,0x24(a6)                 | +04a
        addq.w  #0x5,0x70(a6)                   | +050
        addq.w  #0x8,0x22(a6)                   | +054
        cmpi.w  #0x128,0x22(a6)                 | +058
        bls.b   DebugColl_DrawLowNibble_05f11a  | +05e
        move.w  #0x10,0x22(a6)                  | +060
        move.w  #0x1e8,0x24(a6)                 | +066
        move.w  #0x7043,0x70(a6)                | +06c
        rts                                     | +072

| ----------------------------------------------------------------------------
|  DebugColl_DrawHighNibble_05f18e  @ $05F18E  (132 B)
| ----------------------------------------------------------------------------
        .section .text.DebugColl_DrawHighNibble_05f18e, "ax", @progbits
        .global DebugColl_DrawHighNibble_05f18e
DebugColl_DrawHighNibble_05f18e:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        jsr     0x43f02.l                       | +008
        lea     DebugColl_Tiles_05f30a(pc),a0   | +00e
        andi.l  #0xff,d0                        | +012
        cmpi.l  #0x0,d0                         | +018
        beq.w   .L05f1d8                        | +01e
        andi.l  #0xf0,d0                        | +022
        asr.l   #0x4,d0                         | +028
        asl.l   #0x1,d0                         | +02a
        adda.l  d0,a0                           | +02c
        move.w  (a0),d0                         | +02e
        andi.w  #0xfff,d0                       | +030
        addi.w  #0x5000,d0                      | +034
        movea.w 0x70(a6),a1                     | +038
        move.w  #0x1,d1                         | +03c
        move.w  #0x1,d2                         | +040
        jsr     0x5da56.l                       | +044
.L05f1d8:
        addq.w  #0x1,0x70(a6)                   | +04a
        subq.w  #0x8,0x24(a6)                   | +04e
        cmpi.w  #0x118,0x24(a6)                 | +052
        bcc.b   DebugColl_DrawHighNibble_05f18e | +058
        move.w  #0x1e8,0x24(a6)                 | +05a
        addq.w  #0x5,0x70(a6)                   | +060
        addq.w  #0x8,0x22(a6)                   | +064
        cmpi.w  #0x128,0x22(a6)                 | +068
        bls.b   DebugColl_DrawHighNibble_05f18e | +06e
        move.w  #0x10,0x22(a6)                  | +070
        move.w  #0x1e8,0x24(a6)                 | +076
        move.w  #0x7043,0x70(a6)                | +07c
        rts                                     | +082

| ----------------------------------------------------------------------------
|  DebugColl_Cursor_05f212  @ $05F212  (240 B)
| ----------------------------------------------------------------------------
        .section .text.DebugColl_Cursor_05f212, "ax", @progbits
        .global DebugColl_Cursor_05f212
DebugColl_Cursor_05f212:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0x10,0x22(a6)                  | +00a
        move.w  #0x1e8,0x24(a6)                 | +010
        move.b  #0xff,0x32(a6)                  | +016
        move.b  #0xff,0x33(a6)                  | +01c
        move.b  #0x0,0x3a(a6)                   | +022
        move.w  #0xffff,0x38(a6)                | +028
        bset    #0x6,0x12(a6)                   | +02e
        lea     DebugColl_Tiles_05f30a__L05f352(pc),a0 | +034
        jsr     0x28cd4.l                       | +038
        lea     .L05f256(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L05f256:
        jsr     0x5cd06.l                       | +044
        bcc.w   .L05f298                        | +04a
        jsr     0x5cea4.l                       | +04e
        bcc.w   .L05f26e                        | +054
        addq.w  #0x1,0x24(a6)                   | +058
.L05f26e:
        jsr     0x5ceb6.l                       | +05c
        bcc.w   .L05f27c                        | +062
        subq.w  #0x1,0x24(a6)                   | +066
.L05f27c:
        jsr     0x5cec8.l                       | +06a
        bcc.w   .L05f28a                        | +070
        addq.w  #0x1,0x22(a6)                   | +074
.L05f28a:
        jsr     0x5ceda.l                       | +078
        bcc.w   .L05f298                        | +07e
        subq.w  #0x1,0x22(a6)                   | +082
.L05f298:
        jsr     0x28d70.l                       | +086
        move.w  0x22(a6),d0                     | +08c
        move.w  0x24(a6),d1                     | +090
        movem.l d1,-(a7)                        | +094
        jsr     0x440bc.l                       | +098
        movea.w #0x7425,a1                      | +09e
        jsr     0x5d6c2.l                       | +0a2
        movem.l (a7)+,d0                        | +0a8
        movea.w #0x7426,a1                      | +0ac
        jsr     0x5d6c2.l                       | +0b0
        move.w  0x22(a6),d0                     | +0b6
        move.w  0x24(a6),d1                     | +0ba
        jsr     0x43f02.l                       | +0be
        andi.w  #0xff,d0                        | +0c4
        movea.w #0x7427,a1                      | +0c8
        jsr     0x5d6c2.l                       | +0cc
        lea     DebugColl_Tiles_05f30a__L05f35e(pc),a0 | +0d2
        jsr     0x5dd56.l                       | +0d6
        bcc.w   .L05f2fa                        | +0dc
        move.w  #0xffff,d0                      | +0e0
        bra.w   .L05f2fe                        | +0e4
.L05f2fa:
        move.w  #0x0,d0                         | +0e8
.L05f2fe:
        movea.w #0x7428,a1                      | +0ec

| ----------------------------------------------------------------------------
|  DebugColl_Tiles_05f30a  @ $05F30A  (110 B)
| ----------------------------------------------------------------------------
        .section .text.DebugColl_Tiles_05f30a, "ax", @progbits
        .global DebugColl_Tiles_05f30a
DebugColl_Tiles_05f30a:
        .dc.w   0x2030                        | +000  (dato / opcode no decodificado)
        .dc.w   0x2031                        | +002  (dato / opcode no decodificado)
        .dc.w   0x2032                        | +004  (dato / opcode no decodificado)
        .dc.w   0x2033                        | +006  (dato / opcode no decodificado)
        .dc.w   0x2034                        | +008  (dato / opcode no decodificado)
        .dc.w   0x2035                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x2036                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2037                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x2038                        | +010  (dato / opcode no decodificado)
        .dc.w   0x2039                        | +012  (dato / opcode no decodificado)
        .dc.w   0x2041                        | +014  (dato / opcode no decodificado)
        .dc.w   0x2042                        | +016  (dato / opcode no decodificado)
        .dc.w   0x2043                        | +018  (dato / opcode no decodificado)
        .dc.w   0x2044                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x2045                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x2046                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x2047                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2048                        | +022  (dato / opcode no decodificado)
        .dc.w   0x2049                        | +024  (dato / opcode no decodificado)
        .dc.w   0x204a                        | +026  (dato / opcode no decodificado)
        .dc.w   0x204b                        | +028  (dato / opcode no decodificado)
        .dc.w   0x204c                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x204d                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x204e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x204f                        | +030  (dato / opcode no decodificado)
        .dc.w   0x2050                        | +032  (dato / opcode no decodificado)
        .dc.w   0x2051                        | +034  (dato / opcode no decodificado)
        .dc.w   0x2052                        | +036  (dato / opcode no decodificado)
        .dc.w   0x2053                        | +038  (dato / opcode no decodificado)
        .dc.w   0x2054                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x2055                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x2056                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x2057                        | +040  (dato / opcode no decodificado)
        .dc.w   0x2058                        | +042  (dato / opcode no decodificado)
        .dc.w   0x2059                        | +044  (dato / opcode no decodificado)
        .dc.w   0x205a                        | +046  (dato / opcode no decodificado)
        .global DebugColl_Tiles_05f30a__L05f352
DebugColl_Tiles_05f30a__L05f352:
.L05f352:
        .dc.w   0x0001                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +052  (dato / opcode no decodificado)
        .global DebugColl_Tiles_05f30a__L05f35e
DebugColl_Tiles_05f30a__L05f35e:
.L05f35e:
        .dc.w   0xffe0                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0050                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05c  (dato / opcode no decodificado)
        movea.l 0x8(a6),a1                      | +05e
        move.b  0x10(a6),d0                     | +062
        cmp.b   0x10(a1),d0                     | +066
        bcs.w   SetXN_05f37e                    | +06a

| ----------------------------------------------------------------------------
|  TowerSoldier_Init_05f384  @ $05F384  (102 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Init_05f384, "ax", @progbits
        .global TowerSoldier_Init_05f384
TowerSoldier_Init_05f384:
        move.b  #0x88,0x20(a6)                  | +000
        move.w  #0x27,d1                        | +006
        jsr     0x236e.l                        | +00a
        move.b  #0xff,0x32(a6)                  | +010
        move.b  #0xff,0x33(a6)                  | +016
        move.w  #0x4000,0x38(a6)                | +01c
        addi.w  #0x1,0x38(a6)                   | +022
        jsr     0x267e2.l                       | +028
        lea     0x2c0be2.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        move.b  #0x0,0x71(a6)                   | +03a
        lea     .L05f3ca(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L05f3ca:
        jsr     0x2783a.l                       | +046
        cmpi.w  #0x160,0x22(a6)                 | +04c
        bgt.w   SetHandlerRts_05f3f0            | +052
        lea     TowerSoldier_Watch_05f3f2(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
        cmpi.b  #0x88,0x20(a6)                  | +05c
        bne.w   SetHandlerRts_05f3f0            | +062

| ----------------------------------------------------------------------------
|  TowerSoldier_Watch_05f3f2  @ $05F3F2  (144 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Watch_05f3f2, "ax", @progbits
        .global TowerSoldier_Watch_05f3f2
TowerSoldier_Watch_05f3f2:
        move.b  #0x0,0x70(a6)                   | +000
        lea     .L05f3fe(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L05f3fe:
        jsr     0x2783a.l                       | +00c
        jsr     0x28d70.l                       | +012
        jsr     0x2870a.l                       | +018
        bclr    #0x3,0x13(a6)                   | +01e
        lea     0x2c0ef2.l,a0                   | +024
        btst    #0x0,0x3a(a6)                   | +02a
        beq.w   .L05f42c                        | +030
        lea     0x2c0efc.l,a0                   | +034
.L05f42c:
        jsr     0x5e086.l                       | +03a
        bcs.w   .L05f45e                        | +040
        cmpi.b  #0x0,0x71(a6)                   | +044
        bgt.w   .L05f45a                        | +04a
        lea     TowerSoldier_Grenade_05f6c4(pc),a1 | +04e
        jsr     0x4ae.l                         | +052
        jsr     0x5dd02.l                       | +058
        lea     TowerSoldier_Fire_05f482(pc),a1 | +05e
        move.l  a1,(a6)                         | +062
        bra.w   .L05f45e                        | +064
.L05f45a:
        subq.b  #0x1,0x71(a6)                   | +068
.L05f45e:
        movea.l 0xc(a6),a0                      | +06c
        cmpi.b  #0xff,0x21(a0)                  | +070
        bne.w   .L05f472                        | +076
        lea     TowerSoldier_Die_05f508(pc),a1  | +07a
        move.l  a1,(a6)                         | +07e
.L05f472:
        jsr     Offworld_Box2C0EDE_05f9ac(pc)   | +080
        bcc.w   .L05f480                        | +084
        jmp     0x518.l                         | +088
.L05f480:
        rts                                     | +08e

| ----------------------------------------------------------------------------
|  TowerSoldier_Fire_05f482  @ $05F482  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Fire_05f482, "ax", @progbits
        .global TowerSoldier_Fire_05f482
TowerSoldier_Fire_05f482:
        move.b  #0x0,0x21(a6)                   | +000
        lea     .L05f48e(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L05f48e:
        jsr     0x2783a.l                       | +00c
        jsr     0x28d70.l                       | +012
        jsr     0x2870a.l                       | +018
        bclr    #0x3,0x13(a6)                   | +01e
        cmpi.b  #0x88,0x20(a6)                  | +024
        bne.w   .L05f4be                        | +02a
        movea.l 0xc(a6),a0                      | +02e
        cmpi.b  #0xff,0x21(a0)                  | +032
        beq.w   .L05f4c8                        | +038
.L05f4be:
        cmpi.b  #0xff,0x21(a6)                  | +03c
        bne.w   .L05f4ce                        | +042
.L05f4c8:
        lea     TowerSoldier_Die_05f508(pc),a1  | +046
        move.l  a1,(a6)                         | +04a
.L05f4ce:
        cmpi.b  #0xee,0x21(a6)                  | +04c
        bne.w   .L05f4e4                        | +052
        move.b  #0x1e,0x71(a6)                  | +056
        lea     TowerSoldier_Watch_05f3f2(pc),a1 | +05c
        move.l  a1,(a6)                         | +060
.L05f4e4:
        movea.l 0xc(a6),a0                      | +062
        cmpi.b  #0xff,0x21(a0)                  | +066
        bne.w   .L05f4f8                        | +06c
        lea     TowerSoldier_Die_05f508(pc),a1  | +070
        move.l  a1,(a6)                         | +074
.L05f4f8:
        jsr     Offworld_Box2C0EDE_05f9ac(pc)   | +076
        bcc.w   .L05f506                        | +07a
        jmp     0x518.l                         | +07e
.L05f506:
        rts                                     | +084

| ----------------------------------------------------------------------------
|  TowerSoldier_Die_05f508  @ $05F508  (162 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Die_05f508, "ax", @progbits
        .global TowerSoldier_Die_05f508
TowerSoldier_Die_05f508:
        lea     TowerSoldier_InitB_05f668(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        jsr     0x267e2.l                       | +010
        move.w  #0x0,0x28(a6)                   | +016
        move.w  #0x0,0x2a(a6)                   | +01c
        move.w  #0x0,0x2c(a6)                   | +022
        move.w  #0xff40,0x2e(a6)                | +028
        move.b  #0xff,0x70(a6)                  | +02e
        lea     0x2c0c5e.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        lea     0x2c0dba.l,a0                   | +040
        move.l  a0,0x4c(a6)                     | +046
        jsr     0x283ca.l                       | +04a
        lea     .L05f55e(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L05f55e:
        jsr     0x27d50.l                       | +056
        bcc.w   .L05f56e                        | +05c
        lea     TowerSoldier_Flee_05f5fa(pc),a1 | +060
        move.l  a1,(a6)                         | +064
.L05f56e:
        jsr     0x28d70.l                       | +066
        jsr     TowerSoldier_OffsetPlus10_05f91e(pc) | +06c
        jsr     0x2870a.l                       | +070
        bclr    #0x3,0x13(a6)                   | +076
        jsr     0x283d8.l                       | +07c
        btst    #0x1,0x13(a6)                   | +082
        beq.w   .L05f59a                        | +088
        lea     TowerSoldier_Fall_05f5aa(pc),a1 | +08c
        move.l  a1,(a6)                         | +090
.L05f59a:
        jsr     Offworld_Box2C0EDE_05f9ac(pc)   | +092
        bcc.w   .L05f5a8                        | +096
        jmp     0x518.l                         | +09a
.L05f5a8:
        rts                                     | +0a0

| ----------------------------------------------------------------------------
|  TowerSoldier_Fall_05f5aa  @ $05F5AA  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Fall_05f5aa, "ax", @progbits
        .global TowerSoldier_Fall_05f5aa
TowerSoldier_Fall_05f5aa:
        move.w  #0xd000,0x38(a6)                | +000
        move.w  0x2e(a6),d0                     | +006
        asr.w   #0x1,d0                         | +00a
        move.w  d0,0x2e(a6)                     | +00c
        move.w  #0x0,0x2a(a6)                   | +010
        move.l  #0xffffffff,0x4c(a6)            | +016
        lea     0x2c0cc4.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L05f5da(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L05f5da:
        jsr     0x27d50.l                       | +030
        jsr     0x28d70.l                       | +036
        bcs.w   .L05f5f2                        | +03c
        jsr     Offworld_Box2C0EDE_05f9ac(pc)   | +040
        bcc.w   .L05f5f8                        | +044
.L05f5f2:
        jmp     0x518.l                         | +048
.L05f5f8:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  TowerSoldier_Flee_05f5fa  @ $05F5FA  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Flee_05f5fa, "ax", @progbits
        .global TowerSoldier_Flee_05f5fa
TowerSoldier_Flee_05f5fa:
        jsr     0x2783a.l                       | +000
        move.w  #0x1053,d0                      | +006
        jsr     0x2352.l                        | +00a
        move.w  #0x8000,0x38(a6)                | +010
        lea     0x789f0.l,a1                    | +016
        jsr     0x4ae.l                         | +01c
        jsr     0x5dd02.l                       | +022
        addi.w  #0x1,0x38(a0)                   | +028
        lea     0x2c0f06.l,a1                   | +02e
        jsr     0x43fac.l                       | +034
        jsr     0x267e2.l                       | +03a
        lea     0x2c0c98.l,a0                   | +040
        jsr     0x28cd4.l                       | +046
        lea     .L05f64c(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L05f64c:
        jsr     0x27afc.l                       | +052
        jsr     0x28d70.l                       | +058
        jsr     Offworld_Box2C0EDE_05f9ac(pc)   | +05e
        bcc.w   .L05f666                        | +062
        jmp     0x518.l                         | +066
.L05f666:
        rts                                     | +06c

| ----------------------------------------------------------------------------
|  TowerSoldier_InitB_05f668  @ $05F668  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_InitB_05f668, "ax", @progbits
        .global TowerSoldier_InitB_05f668
TowerSoldier_InitB_05f668:
        move.w  #0x27,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        jsr     0x267e2.l                       | +016
        lea     0x2c0daa.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L05f696(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L05f696:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        movea.l 0xc(a6),a0                      | +03a
        movea.l 0xc(a0),a0                      | +03e
        cmpi.b  #0xff,0x21(a0)                  | +042
        beq.w   .L05f6bc                        | +048
        jsr     Offworld_Box2C0EE8_05f9c8(pc)   | +04c
        bcc.w   .L05f6c2                        | +050
.L05f6bc:
        jmp     0x518.l                         | +054
.L05f6c2:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  TowerSoldier_Grenade_05f6c4  @ $05F6C4  (142 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Grenade_05f6c4, "ax", @progbits
        .global TowerSoldier_Grenade_05f6c4
TowerSoldier_Grenade_05f6c4:
        move.w  #0x1d,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4a,d0                        | +00a
        btst    #0x0,0x3a(a6)                   | +00e
        beq.w   .L05f6e0                        | +014
        move.w  #0xffc4,d0                      | +018
.L05f6e0:
        add.w   d0,0x22(a6)                     | +01c
        addi.w  #0x30,0x24(a6)                  | +020
        jsr     0x267e2.l                       | +026
        lea     0x2c09ac.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        move.w  #0xff80,0x2a(a6)                | +038
.L05f702:
        subq.w  #0x1,0x24(a6)                   | +03e
        move.w  0x22(a6),d1                     | +042
        move.w  0x24(a6),d2                     | +046
        jsr     0x280c6.l                       | +04a
        bcc.b   .L05f702                        | +050
        move.b  #0x0,0x70(a6)                   | +052
        lea     .L05f722(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L05f722:
        jsr     0x2783a.l                       | +05e
        jsr     0x28d70.l                       | +064
        bcc.w   .L05f738                        | +06a
        lea     TowerSoldier_GrenadeBurst_05f752(pc),a1 | +06e
        move.l  a1,(a6)                         | +072
.L05f738:
        jsr     Offworld_Box2C0ED4_05f990(pc)   | +074
        bcc.w   .L05f750                        | +078
        movea.l 0xc(a6),a0                      | +07c
        move.b  #0xee,0x21(a0)                  | +080
        jmp     0x518.l                         | +086
.L05f750:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  TowerSoldier_GrenadeBurst_05f752  @ $05F752  (162 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_GrenadeBurst_05f752, "ax", @progbits
        .global TowerSoldier_GrenadeBurst_05f752
TowerSoldier_GrenadeBurst_05f752:
        jsr     0x267e2.l                       | +000
        lea     0x29b744.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        move.w  #0xfe00,d0                      | +012
        btst    #0x0,0x3a(a6)                   | +016
        beq.w   .L05f774                        | +01c
        neg.w   d0                              | +020
.L05f774:
        move.w  d0,0x28(a6)                     | +022
        lea     .L05f77e(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L05f77e:
        jsr     0x27cee.l                       | +02c
        jsr     0x28d70.l                       | +032
        jsr     0x2870a.l                       | +038
        bcc.w   .L05f79e                        | +03e
        movea.l 0xc(a6),a0                      | +042
        move.b  #0xee,0x21(a0)                  | +046
.L05f79e:
        movea.l 0xc(a6),a0                      | +04c
        cmpi.b  #0xff,0x70(a0)                  | +050
        bne.w   .L05f7c6                        | +056
        lea     0x58f82.l,a1                    | +05a
        move.l  a1,(a6)                         | +060
        lea     0x776e2.l,a1                    | +062
        jsr     0x4ae.l                         | +068
        jsr     0x5dd02.l                       | +06e
.L05f7c6:
        jsr     TowerSoldier_SpawnDebris_05f94c(pc) | +074
        bcc.w   .L05f7d4                        | +078
        lea     TowerSoldier_Muzzle_05f7f4(pc),a1 | +07c
        move.l  a1,(a6)                         | +080
.L05f7d4:
        jsr     0x49fd0.l                       | +082
        jsr     Offworld_Box2C0ED4_05f990(pc)   | +088
        bcc.w   .L05f7f2                        | +08c
        movea.l 0xc(a6),a0                      | +090
        move.b  #0xee,0x21(a0)                  | +094
        jmp     0x518.l                         | +09a
.L05f7f2:
        rts                                     | +0a0

| ----------------------------------------------------------------------------
|  TowerSoldier_Muzzle_05f7f4  @ $05F7F4  (180 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Muzzle_05f7f4, "ax", @progbits
        .global TowerSoldier_Muzzle_05f7f4
TowerSoldier_Muzzle_05f7f4:
        jsr     0x5e4dc.l                       | +000
        move.w  #0x18,d0                        | +006
        btst    #0x0,0x3a(a6)                   | +00a
        beq.w   .L05f80a                        | +010
        neg.w   d0                              | +014
.L05f80a:
        add.w   d0,0x22(a6)                     | +016
        addi.w  #0x10,0x24(a6)                  | +01a
        jsr     0x267e2.l                       | +020
        lea     0x2c0a3a.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L05f82c(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L05f82c:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bcc.w   .L05f842                        | +044
        lea     TowerSoldier_Shell_05f8a8(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L05f842:
        jsr     0x2870a.l                       | +04e
        bcc.w   .L05f856                        | +054
        movea.l 0xc(a6),a0                      | +058
        move.b  #0xee,0x21(a0)                  | +05c
.L05f856:
        movea.l 0xc(a6),a0                      | +062
        cmpi.b  #0xff,0x70(a0)                  | +066
        bne.w   .L05f888                        | +06c
        cmpi.b  #0xff,0x70(a6)                  | +070
        beq.w   .L05f888                        | +076
        lea     0x58f82.l,a1                    | +07a
        move.l  a1,(a6)                         | +080
        lea     0x776e2.l,a1                    | +082
        jsr     0x4ae.l                         | +088
        jsr     0x5dd02.l                       | +08e
.L05f888:
        jsr     0x49fd0.l                       | +094
        jsr     Offworld_Box2C0ED4_05f990(pc)   | +09a
        bcc.w   .L05f8a6                        | +09e
        movea.l 0xc(a6),a0                      | +0a2
        move.b  #0xee,0x21(a0)                  | +0a6
        jmp     0x518.l                         | +0ac
.L05f8a6:
        rts                                     | +0b2

| ----------------------------------------------------------------------------
|  TowerSoldier_Shell_05f8a8  @ $05F8A8  (118 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Shell_05f8a8, "ax", @progbits
        .global TowerSoldier_Shell_05f8a8
TowerSoldier_Shell_05f8a8:
        jsr     0x267e2.l                       | +000
        lea     0x2c0bb6.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L05f8c0(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L05f8c0:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L05f8ea                        | +024
        lea     0x58f82.l,a1                    | +028
        move.l  a1,(a6)                         | +02e
        lea     0x776e2.l,a1                    | +030
        jsr     0x4ae.l                         | +036
        jsr     0x5dd02.l                       | +03c
.L05f8ea:
        jsr     0x2870a.l                       | +042
        bcc.w   .L05f8fe                        | +048
        movea.l 0xc(a6),a0                      | +04c
        move.b  #0xee,0x21(a0)                  | +050
.L05f8fe:
        jsr     0x49fd0.l                       | +056
        jsr     Offworld_Box2C0ED4_05f990(pc)   | +05c
        bcc.w   .L05f91c                        | +060
        movea.l 0xc(a6),a0                      | +064
        move.b  #0xee,0x21(a0)                  | +068
        jmp     0x518.l                         | +06e
.L05f91c:
        rts                                     | +074

| ----------------------------------------------------------------------------
|  TowerSoldier_OffsetPlus10_05f91e  @ $05F91E  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_OffsetPlus10_05f91e, "ax", @progbits
        .global TowerSoldier_OffsetPlus10_05f91e
TowerSoldier_OffsetPlus10_05f91e:
        move.w  0x22(a6),d0                     | +000
        subi.w  #0x20,d0                        | +004
        move.w  0x24(a6),d1                     | +008
        addi.w  #0x10,d1                        | +00c
        move.w  #0x40,d2                        | +010

| ----------------------------------------------------------------------------
|  TowerSoldier_Helper_05f93a  @ $05F93A  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_Helper_05f93a, "ax", @progbits
        .global TowerSoldier_Helper_05f93a
TowerSoldier_Helper_05f93a:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0xff,0x21(a0)                  | +004
        move.b  #0xff,0x70(a6)                  | +00a
        rts                                     | +010

| ----------------------------------------------------------------------------
|  TowerSoldier_SpawnDebris_05f94c  @ $05F94C  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TowerSoldier_SpawnDebris_05f94c, "ax", @progbits
        .global TowerSoldier_SpawnDebris_05f94c
TowerSoldier_SpawnDebris_05f94c:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),d0                     | +004
        sub.w   0x22(a6),d0                     | +008
        btst    #0xf,d0                         | +00c
        beq.w   .L05f962                        | +010
        neg.w   d0                              | +014
.L05f962:
        cmpi.w  #0xa,d0                         | +016
        bgt.w   ClearC_05f98a                   | +01a
        move.w  0x24(a0),d0                     | +01e
        sub.w   0x24(a6),d0                     | +022
        btst    #0xf,d0                         | +026
        beq.w   .L05f97c                        | +02a
        neg.w   d0                              | +02e
.L05f97c:
        cmpi.w  #0x30,d0                        | +030
        bgt.w   ClearC_05f98a                   | +034

| ----------------------------------------------------------------------------
|  Offworld_Box2C0ED4_05f990  @ $05F990  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C0ED4_05f990, "ax", @progbits
        .global Offworld_Box2C0ED4_05f990
Offworld_Box2C0ED4_05f990:
        lea     0x2c0ed4.l,a0                   | +000
        jsr     0x5dd56.l                       | +006
        bcc.w   ClearC_05f9a6                   | +00c

| ----------------------------------------------------------------------------
|  Offworld_Box2C0EDE_05f9ac  @ $05F9AC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C0EDE_05f9ac, "ax", @progbits
        .global Offworld_Box2C0EDE_05f9ac
Offworld_Box2C0EDE_05f9ac:
        lea     0x2c0ede.l,a0                   | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   ClearC_05f9c2                   | +00c

| ----------------------------------------------------------------------------
|  Offworld_Box2C0EE8_05f9c8  @ $05F9C8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C0EE8_05f9c8, "ax", @progbits
        .global Offworld_Box2C0EE8_05f9c8
Offworld_Box2C0EE8_05f9c8:
        lea     0x2c0ee8.l,a0                   | +000
        jsr     0x5dd56.l                       | +006
        bcc.w   ClearC_05f9de                   | +00c

| ----------------------------------------------------------------------------
|  Offworld_Box2C0EF2_05f9e4  @ $05F9E4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C0EF2_05f9e4, "ax", @progbits
        .global Offworld_Box2C0EF2_05f9e4
Offworld_Box2C0EF2_05f9e4:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_05f9fa                    | +00c

| ----------------------------------------------------------------------------
|  HutOccupant_Init_05fa00  @ $05FA00  (78 B)
| ----------------------------------------------------------------------------
        .section .text.HutOccupant_Init_05fa00, "ax", @progbits
        .global HutOccupant_Init_05fa00
HutOccupant_Init_05fa00:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        addq.w  #0x2,0x38(a6)                   | +016
        move.b  #0x0,0x20(a6)                   | +01a
        move.b  #0x0,0x21(a6)                   | +020
        move.w  #0x10,0x66(a6)                  | +026
        jsr     0x267e2.l                       | +02c
        lea     0x2c0f20.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     HutDoor_Init_05fba0(pc),a1      | +03e
        jsr     0x4ae.l                         | +042
        jsr     0x5dd02.l                       | +048

| ----------------------------------------------------------------------------
|  HutOccupant_Idle_05fa56  @ $05FA56  (72 B)
| ----------------------------------------------------------------------------
        .section .text.HutOccupant_Idle_05fa56, "ax", @progbits
        .global HutOccupant_Idle_05fa56
HutOccupant_Idle_05fa56:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        cmpi.b  #0xff,0x21(a6)                  | +00c
        bne.w   .L05fa72                        | +012
        lea     HutOccupant_Emerge_05faa6(pc),a1 | +016
        move.l  a1,(a6)                         | +01a
.L05fa72:
        jsr     HutOccupant_PlayersNear_05fd08(pc) | +01c
        bcc.w   .L05fa80                        | +020
        lea     HutOccupant_Emerge_05faa6(pc),a1 | +024
        move.l  a1,(a6)                         | +028
.L05fa80:
        jsr     0x2870a.l                       | +02a
        bcc.w   .L05fa90                        | +030
        move.b  #0xff,0x20(a6)                  | +034
.L05fa90:
        jsr     0x49fd0.l                       | +03a
        jsr     Offworld_Box2C168E_05fd40(pc)   | +040
        bcc.w   SetHandlerRts_05faa4            | +044

| ----------------------------------------------------------------------------
|  HutOccupant_Emerge_05faa6  @ $05FAA6  (118 B)
| ----------------------------------------------------------------------------
        .section .text.HutOccupant_Emerge_05faa6, "ax", @progbits
        .global HutOccupant_Emerge_05faa6
HutOccupant_Emerge_05faa6:
        jsr     0x267e2.l                       | +000
        lea     0x2c0f9c.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L05fabe(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L05fabe:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L05fafe                        | +024
        move.b  #0x80,0x20(a6)                  | +028
        lea     HutOccupant_Out_05fb24(pc),a1   | +02e
        move.l  a1,(a6)                         | +032
        cmpi.b  #0xff,0x21(a6)                  | +034
        bne.w   .L05fafe                        | +03a
        lea     0x58f82.l,a1                    | +03e
        move.l  a1,(a6)                         | +044
        lea     0x776e2.l,a1                    | +046
        jsr     0x4ae.l                         | +04c
        jsr     0x5dd02.l                       | +052
.L05fafe:
        jsr     0x2870a.l                       | +058
        bcc.w   .L05fb0e                        | +05e
        move.b  #0xff,0x20(a6)                  | +062
.L05fb0e:
        jsr     0x49fd0.l                       | +068
        jsr     Offworld_Box2C168E_05fd40(pc)   | +06e
        bcc.w   SetHandlerRts_05fb22            | +072

| ----------------------------------------------------------------------------
|  HutOccupant_Out_05fb24  @ $05FB24  (92 B)
| ----------------------------------------------------------------------------
        .section .text.HutOccupant_Out_05fb24, "ax", @progbits
        .global HutOccupant_Out_05fb24
HutOccupant_Out_05fb24:
        jsr     0x267e2.l                       | +000
        lea     0x2c10d8.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L05fb3c(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L05fb3c:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        cmpi.b  #0xff,0x21(a6)                  | +024
        bne.w   .L05fb58                        | +02a
        lea     HutOccupant_Emerge_05faa6(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L05fb58:
        jsr     0x2870a.l                       | +034
        bcc.w   .L05fb72                        | +03a
        move.b  #0xff,0x20(a6)                  | +03e
        move.w  #0x10ae,d0                      | +044
        jsr     0x2222.l                        | +048
.L05fb72:
        jsr     0x49fd0.l                       | +04e
        jsr     Offworld_Box2C168E_05fd40(pc)   | +054
        bcc.w   SetHandlerRts_05fb86            | +058

| ----------------------------------------------------------------------------
|  HutOccupant_WaitParent_05fb88  @ $05FB88  (24 B)
| ----------------------------------------------------------------------------
        .section .text.HutOccupant_WaitParent_05fb88, "ax", @progbits
        .global HutOccupant_WaitParent_05fb88
HutOccupant_WaitParent_05fb88:
        lea     .L05fb8e(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L05fb8e:
        cmpi.b  #0xff,0x21(a6)                  | +006
        bne.w   .L05fb9e                        | +00c
        jmp     0x518.l                         | +010
.L05fb9e:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  HutDoor_Init_05fba0  @ $05FBA0  (62 B)
| ----------------------------------------------------------------------------
        .section .text.HutDoor_Init_05fba0, "ax", @progbits
        .global HutDoor_Init_05fba0
HutDoor_Init_05fba0:
        move.w  #0x19,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0x0,0x46(a6)                   | +00a
        move.b  #0xff,0x32(a6)                  | +010
        move.b  #0xff,0x33(a6)                  | +016
        move.b  #0x0,0x44(a6)                   | +01c
        move.w  #0x10,0x66(a6)                  | +022
        subq.w  #0x1,0x38(a6)                   | +028
        jsr     0x267e2.l                       | +02c
        lea     0x2c14cc.l,a0                   | +032
        jsr     0x28cd4.l                       | +038

| ----------------------------------------------------------------------------
|  HutDoor_Idle_05fbe6  @ $05FBE6  (56 B)
| ----------------------------------------------------------------------------
        .section .text.HutDoor_Idle_05fbe6, "ax", @progbits
        .global HutDoor_Idle_05fbe6
HutDoor_Idle_05fbe6:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        movea.l 0xc(a6),a0                      | +00c
        cmpi.b  #0x80,0x20(a0)                  | +010
        bne.w   .L05fc06                        | +016
        lea     HutDoor_Open_05fc26(pc),a1      | +01a
        move.l  a1,(a6)                         | +01e
.L05fc06:
        jsr     0x2870a.l                       | +020
        bcc.w   .L05fc16                        | +026
        lea     HutDoor_Break_05fc78(pc),a1     | +02a
        move.l  a1,(a6)                         | +02e
.L05fc16:
        jsr     Offworld_Box2C168E_05fd40(pc)   | +030
        bcc.w   SetHandlerRts_05fc24            | +034

| ----------------------------------------------------------------------------
|  HutDoor_Open_05fc26  @ $05FC26  (74 B)
| ----------------------------------------------------------------------------
        .section .text.HutDoor_Open_05fc26, "ax", @progbits
        .global HutDoor_Open_05fc26
HutDoor_Open_05fc26:
        lea     0x2c14e2.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L05fc38(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L05fc38:
        jsr     0x2783a.l                       | +012
        movea.l 0xc(a6),a0                      | +018
        cmpi.b  #0xff,0x20(a0)                  | +01c
        bne.w   .L05fc52                        | +022
        move.b  #0x2,0x46(a6)                   | +026
.L05fc52:
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L05fc68                        | +038
        lea     HutDoor_Break_05fc78(pc),a1     | +03c
        move.l  a1,(a6)                         | +040
.L05fc68:
        jsr     Offworld_Box2C168E_05fd40(pc)   | +042
        bcc.w   SetHandlerRts_05fc76            | +046

| ----------------------------------------------------------------------------
|  HutDoor_Break_05fc78  @ $05FC78  (102 B)
| ----------------------------------------------------------------------------
        .section .text.HutDoor_Break_05fc78, "ax", @progbits
        .global HutDoor_Break_05fc78
HutDoor_Break_05fc78:
        move.w  #0x10ae,d0                      | +000
        jsr     0x2222.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.b  #0xff,0x21(a0)                  | +00e
        move.w  #0x108e,d0                      | +014
        jsr     0x2352.l                        | +018
        lea     0x2c1546.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L05fca8(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L05fca8:
        jsr     0x2783a.l                       | +030
        jsr     0x28d70.l                       | +036
        bcc.w   .L05fcd6                        | +03c
        lea     0x77efe.l,a1                    | +040
        jsr     0x6fe.l                         | +046
        jsr     0x5dd02.l                       | +04c
        subi.w  #0x10,0x22(a0)                  | +052
        lea     HutDoor_Free_05fce6(pc),a1      | +058
        move.l  a1,(a6)                         | +05c
.L05fcd6:
        jsr     Offworld_Box2C168E_05fd40(pc)   | +05e
        bcc.w   SetHandlerRts_05fce4            | +062

| ----------------------------------------------------------------------------
|  HutDoor_Free_05fce6  @ $05FCE6  (32 B)
| ----------------------------------------------------------------------------
        .section .text.HutDoor_Free_05fce6, "ax", @progbits
        .global HutDoor_Free_05fce6
HutDoor_Free_05fce6:
        move.w  #0x10ae,d0                      | +000
        jsr     0x2222.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.b  #0xff,0x21(a0)                  | +00e
        lea     .L05fd00(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L05fd00:
        jmp     0x518.l                         | +01a

| ----------------------------------------------------------------------------
|  HutDoor_Rts_05fd06  @ $05FD06  (2 B)
| ----------------------------------------------------------------------------
        .section .text.HutDoor_Rts_05fd06, "ax", @progbits
        .global HutDoor_Rts_05fd06
HutDoor_Rts_05fd06:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  HutOccupant_PlayersNear_05fd08  @ $05FD08  (44 B)
| ----------------------------------------------------------------------------
        .section .text.HutOccupant_PlayersNear_05fd08, "ax", @progbits
        .global HutOccupant_PlayersNear_05fd08
HutOccupant_PlayersNear_05fd08:
        lea     0x100440.l,a0                   | +000
        move.w  0x22(a6),d0                     | +006
        sub.w   0x22(a0),d0                     | +00a
        cmpi.w  #0xa0,d0                        | +00e
        blt.w   SetC_05fd3a                     | +012
        lea     0x1004e0.l,a0                   | +016
        move.w  0x22(a6),d0                     | +01c
        sub.w   0x22(a0),d0                     | +020
        cmpi.w  #0xa0,d0                        | +024
        blt.w   SetC_05fd3a                     | +028

| ----------------------------------------------------------------------------
|  Offworld_Box2C168E_05fd40  @ $05FD40  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C168E_05fd40, "ax", @progbits
        .global Offworld_Box2C168E_05fd40
Offworld_Box2C168E_05fd40:
        lea     0x2c168e.l,a0                   | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   ClearC_05fd56                   | +00c

| ----------------------------------------------------------------------------
|  Offworld_Box2C168E_B_05fd5c  @ $05FD5C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C168E_B_05fd5c, "ax", @progbits
        .global Offworld_Box2C168E_B_05fd5c
Offworld_Box2C168E_B_05fd5c:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_05fd72                    | +00c

| ----------------------------------------------------------------------------
|  Breakable_Tmpl1F_05fd78  @ $05FD78  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Tmpl1F_05fd78, "ax", @progbits
        .global Breakable_Tmpl1F_05fd78
Breakable_Tmpl1F_05fd78:
        lea     0x2c175e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L05fda4                        | +00c
        lea     0x2c17ac.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        bra.w   .L05fda4                        | +01c
        lea     0x2c1886.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
.L05fda4:
        cmpi.w  #0x140,0x22(a6)                 | +02c
        blt.w   Breakable_Init_05fdb6           | +032

| ----------------------------------------------------------------------------
|  Breakable_Init_05fdb6  @ $05FDB6  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Init_05fdb6, "ax", @progbits
        .global Breakable_Init_05fdb6
Breakable_Init_05fdb6:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  0x98(a6),d0                     | +016
        andi.b  #0x1,d0                         | +01a
        move.b  d0,0x3a(a6)                     | +01e
        move.w  #0x8000,0x38(a6)                | +022
        move.w  #0x1,0x66(a6)                   | +028
        jsr     0x267e2.l                       | +02e
        lea     .L05fdf0(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L05fdf0:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        bcc.w   .L05fe06                        | +046
        lea     Breakable_Idle_05fe26(pc),a1    | +04a
        move.l  a1,(a6)                         | +04e
.L05fe06:
        jsr     0x2870a.l                       | +050
        bcc.w   .L05fe16                        | +056
        lea     Breakable_SpawnPiece_060210(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
.L05fe16:
        jsr     Offworld_Box2C21BC_06037c(pc)   | +060
        bcc.w   .L05fe24                        | +064
        jmp     0x518.l                         | +068
.L05fe24:
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  Breakable_Idle_05fe26  @ $05FE26  (154 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Idle_05fe26, "ax", @progbits
        .global Breakable_Idle_05fe26
Breakable_Idle_05fe26:
        lea     0x2b79a0.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x70(a6)                     | +00c
        lea     0x2c196e.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L05fe48(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L05fe48:
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        cmpi.w  #0xff,0x70(a6)                  | +02e
        beq.w   .L05fe78                        | +034
        subi.w  #0x1,0x70(a6)                   | +038
        beq.w   .L05fe72                        | +03e
        subi.w  #0x1,0x70(a6)                   | +042
        bne.w   .L05fe78                        | +048
.L05fe72:
        lea     Breakable_Turn_05fec0(pc),a1    | +04c
        move.l  a1,(a6)                         | +050
.L05fe78:
        jsr     0x5e0d4.l                       | +052
        move.w  0x22(a0),d0                     | +058
        sub.w   0x22(a6),d0                     | +05c
        rol.w   #0x1,d0                         | +060
        andi.w  #0x1,d0                         | +062
        move.b  0x3a(a6),d1                     | +066
        andi.b  #0x1,d1                         | +06a
        cmp.b   d0,d1                           | +06e
        bne.w   .L05fea0                        | +070
        lea     Breakable_Destroyed_05ff1c(pc),a1 | +074
        move.l  a1,(a6)                         | +078
.L05fea0:
        jsr     0x2870a.l                       | +07a
        bcc.w   .L05feb0                        | +080
        lea     Breakable_SpawnPiece_060210(pc),a1 | +084
        move.l  a1,(a6)                         | +088
.L05feb0:
        jsr     Offworld_Box2C21BC_06037c(pc)   | +08a
        bcc.w   .L05febe                        | +08e
        jmp     0x518.l                         | +092
.L05febe:
        rts                                     | +098

| ----------------------------------------------------------------------------
|  Breakable_Turn_05fec0  @ $05FEC0  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Turn_05fec0, "ax", @progbits
        .global Breakable_Turn_05fec0
Breakable_Turn_05fec0:
        jsr     0x5e0d4.l                       | +000
        move.w  0x22(a0),d0                     | +006
        sub.w   0x22(a6),d0                     | +00a
        asl.w   #0x1,d0                         | +00e
        move.w  d0,0x28(a6)                     | +010
        lea     0x2c19ce.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L05fee6(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L05fee6:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        bcc.w   .L05fefc                        | +032
        lea     Breakable_Idle_05fe26(pc),a1    | +036
        move.l  a1,(a6)                         | +03a
.L05fefc:
        jsr     0x2870a.l                       | +03c
        bcc.w   .L05ff0c                        | +042
        lea     Breakable_SpawnPiece_060210(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L05ff0c:
        jsr     Offworld_Box2C21BC_06037c(pc)   | +04c
        bcc.w   .L05ff1a                        | +050
        jmp     0x518.l                         | +054
.L05ff1a:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Breakable_Destroyed_05ff1c  @ $05FF1C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Destroyed_05ff1c, "ax", @progbits
        .global Breakable_Destroyed_05ff1c
Breakable_Destroyed_05ff1c:
        lea     0x2c1b86.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L05ff2e(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L05ff2e:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L05ff44                        | +01e
        lea     Breakable_Idle_05fe26(pc),a1    | +022
        move.l  a1,(a6)                         | +026
.L05ff44:
        jsr     0x2870a.l                       | +028
        bcc.w   .L05ff54                        | +02e
        lea     Breakable_SpawnPiece_060210(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L05ff54:
        jsr     Offworld_Box2C21BC_06037c(pc)   | +038
        bcc.w   .L05ff62                        | +03c
        jmp     0x518.l                         | +040
.L05ff62:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Breakable_Tmpl22_05ff64  @ $05FF64  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Tmpl22_05ff64, "ax", @progbits
        .global Breakable_Tmpl22_05ff64
Breakable_Tmpl22_05ff64:
        cmpi.w  #0x140,0x22(a6)                 | +000
        blt.w   Breakable22_Init_05ff76         | +006

| ----------------------------------------------------------------------------
|  Breakable22_Init_05ff76  @ $05FF76  (146 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable22_Init_05ff76, "ax", @progbits
        .global Breakable22_Init_05ff76
Breakable22_Init_05ff76:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  0x98(a6),d0                     | +016
        andi.b  #0x1,d0                         | +01a
        move.b  d0,0x3a(a6)                     | +01e
        move.w  #0x8000,0x38(a6)                | +022
        move.w  #0x1,0x66(a6)                   | +028
        jsr     0x267e2.l                       | +02e
        lea     0x2c1b02.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        lea     .L05ffbc(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L05ffbc:
        jsr     0x2783a.l                       | +046
        jsr     0x28d70.l                       | +04c
        jsr     0x5e0d4.l                       | +052
        move.w  0x22(a0),d0                     | +058
        sub.w   0x22(a6),d0                     | +05c
        jsr     Facing_FromAngleHi_0603b4(pc)   | +060
        cmpi.w  #0x20,d0                        | +064
        bgt.w   .L05ffe8                        | +068
        lea     Breakable22_Destroyed_060008(pc),a1 | +06c
        move.l  a1,(a6)                         | +070
.L05ffe8:
        jsr     0x2870a.l                       | +072
        bcc.w   .L05fff8                        | +078
        lea     Breakable_SpawnPiece_060210(pc),a1 | +07c
        move.l  a1,(a6)                         | +080
.L05fff8:
        jsr     Offworld_Box2C21BC_06037c(pc)   | +082
        bcc.w   .L060006                        | +086
        jmp     0x518.l                         | +08a
.L060006:
        rts                                     | +090

| ----------------------------------------------------------------------------
|  Breakable22_Destroyed_060008  @ $060008  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable22_Destroyed_060008, "ax", @progbits
        .global Breakable22_Destroyed_060008
Breakable22_Destroyed_060008:
        lea     0x2c1b2a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L06001a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06001a:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L060030                        | +01e
        lea     Breakable_Idle_05fe26(pc),a1    | +022
        move.l  a1,(a6)                         | +026
.L060030:
        jsr     0x2870a.l                       | +028
        bcc.w   .L060040                        | +02e
        lea     Breakable_SpawnPiece_060210(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L060040:
        jsr     Offworld_Box2C21BC_06037c(pc)   | +038
        bcc.w   .L06004e                        | +03c
        jmp     0x518.l                         | +040
.L06004e:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Breakable_Tmpl23_060050  @ $060050  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Tmpl23_060050, "ax", @progbits
        .global Breakable_Tmpl23_060050
Breakable_Tmpl23_060050:
        cmpi.w  #0x140,0x22(a6)                 | +000
        blt.w   Breakable23_Init_060062         | +006

| ----------------------------------------------------------------------------
|  Breakable23_Init_060062  @ $060062  (146 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable23_Init_060062, "ax", @progbits
        .global Breakable23_Init_060062
Breakable23_Init_060062:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  0x98(a6),d0                     | +016
        andi.b  #0x1,d0                         | +01a
        move.b  d0,0x3a(a6)                     | +01e
        move.w  #0x8000,0x38(a6)                | +022
        move.w  #0x1,0x66(a6)                   | +028
        jsr     0x267e2.l                       | +02e
        lea     0x2c1d44.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        lea     .L0600a8(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L0600a8:
        jsr     0x2783a.l                       | +046
        jsr     0x28d70.l                       | +04c
        jsr     0x5e0d4.l                       | +052
        move.w  0x22(a0),d0                     | +058
        sub.w   0x22(a6),d0                     | +05c
        jsr     Facing_FromAngleHi_0603b4(pc)   | +060
        cmpi.w  #0x20,d0                        | +064
        bgt.w   .L0600d4                        | +068
        lea     Breakable23_Destroyed_0600f4(pc),a1 | +06c
        move.l  a1,(a6)                         | +070
.L0600d4:
        jsr     0x2870a.l                       | +072
        bcc.w   .L0600e4                        | +078
        lea     Breakable_SpawnPiece_060210(pc),a1 | +07c
        move.l  a1,(a6)                         | +080
.L0600e4:
        jsr     Offworld_Box2C21BC_06037c(pc)   | +082
        bcc.w   .L0600f2                        | +086
        jmp     0x518.l                         | +08a
.L0600f2:
        rts                                     | +090

| ----------------------------------------------------------------------------
|  Breakable23_Destroyed_0600f4  @ $0600F4  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable23_Destroyed_0600f4, "ax", @progbits
        .global Breakable23_Destroyed_0600f4
Breakable23_Destroyed_0600f4:
        lea     0x2c1d88.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L060106(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L060106:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L06011c                        | +01e
        lea     Breakable_Idle_05fe26(pc),a1    | +022
        move.l  a1,(a6)                         | +026
.L06011c:
        jsr     0x2870a.l                       | +028
        bcc.w   .L06012c                        | +02e
        lea     Breakable_SpawnPiece_060210(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L06012c:
        jsr     Offworld_Box2C21BC_06037c(pc)   | +038
        bcc.w   .L06013a                        | +03c
        jmp     0x518.l                         | +040
.L06013a:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Breakable_Tmpl24_06013c  @ $06013C  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Tmpl24_06013c, "ax", @progbits
        .global Breakable_Tmpl24_06013c
Breakable_Tmpl24_06013c:
        lea     0x2c1d98.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L060178                        | +00c
        lea     0x2c1df8.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        bra.w   .L060178                        | +01c
        lea     0x2c1e58.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        bra.w   .L060178                        | +02c
        lea     0x2c1f28.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
.L060178:
        cmpi.w  #0x140,0x22(a6)                 | +03c
        blt.w   Breakable24_Init_06018a         | +042

| ----------------------------------------------------------------------------
|  Breakable24_Init_06018a  @ $06018A  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable24_Init_06018a, "ax", @progbits
        .global Breakable24_Init_06018a
Breakable24_Init_06018a:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  0x98(a6),d0                     | +016
        andi.b  #0x1,d0                         | +01a
        move.b  d0,0x3a(a6)                     | +01e
        move.w  #0x8000,0x38(a6)                | +022
        move.w  #0x1,0x66(a6)                   | +028
        jsr     0x267e2.l                       | +02e
        lea     .L0601c4(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L0601c4:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        jsr     0x5e0d4.l                       | +046
        move.w  0x22(a0),d0                     | +04c
        sub.w   0x22(a6),d0                     | +050
        jsr     Facing_FromAngleHi_0603b4(pc)   | +054
        cmpi.w  #0x20,d0                        | +058
        bgt.w   .L0601f0                        | +05c
        lea     Breakable_Idle_05fe26(pc),a1    | +060
        move.l  a1,(a6)                         | +064
.L0601f0:
        jsr     0x2870a.l                       | +066
        bcc.w   .L060200                        | +06c
        lea     Breakable_SpawnPiece_060210(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L060200:
        jsr     Offworld_Box2C21BC_06037c(pc)   | +076
        bcc.w   .L06020e                        | +07a
        jmp     0x518.l                         | +07e
.L06020e:
        rts                                     | +084

| ----------------------------------------------------------------------------
|  Breakable_SpawnPiece_060210  @ $060210  (78 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_SpawnPiece_060210, "ax", @progbits
        .global Breakable_SpawnPiece_060210
Breakable_SpawnPiece_060210:
        jsr     0x519be.l                       | +000
        move.w  #0x2,d0                         | +006
        jsr     0x5e9e4.l                       | +00a
        movea.l #0x2c20b8,a0                    | +010
        lsl.w   #0x2,d0                         | +016
        movea.l (a0,d0.w),a0                    | +018
        cmpa.l  #0xffffffff,a0                  | +01c
        beq.w   .L06023c                        | +022
        jsr     0x28cd4.l                       | +026
.L06023c:
        lea     .L060242(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L060242:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   .L06025c                        | +03e
        jsr     Breakable_PieceRng_0603c4(pc)   | +042
        jmp     0x518.l                         | +046
.L06025c:
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  Breakable_Shard_06025e  @ $06025E  (196 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Shard_06025e, "ax", @progbits
        .global Breakable_Shard_06025e
Breakable_Shard_06025e:
        move.w  #0x15d,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x164,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x165,d1                       | +014
        jsr     0x236e.l                        | +018
        move.b  #0xff,0x32(a6)                  | +01e
        move.b  #0xff,0x33(a6)                  | +024
        move.w  #0xd000,0x38(a6)                | +02a
        movea.l 0xc(a6),a0                      | +030
        clr.b   0x3a(a6)                        | +034
        lea     0x2c2168.l,a0                   | +038
        move.l  a0,0x4c(a6)                     | +03e
        jsr     0x283ca.l                       | +042
        move.w  #0x0,0x70(a6)                   | +048
        lea     0x2c72c0.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
        lea     .L0602be(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L0602be:
        jsr     0x27cee.l                       | +060
        bcs.w   .L060302                        | +066
        jsr     Breakable_ShardVel_060322(pc)   | +06a
        move.w  0x28(a6),d0                     | +06e
        move.w  0x2a(a6),d1                     | +072
        asr.w   #0x4,d0                         | +076
        asr.w   #0x4,d1                         | +078
        jsr     0x5e018.l                       | +07a
        lsr.w   #0x3,d0                         | +080
        move.w  d0,0x30(a6)                     | +082
        jsr     0x28d70.l                       | +086
        jsr     0x283d8.l                       | +08c
        btst    #0x1,0x13(a6)                   | +092
        bne.w   .L060302                        | +098
        jsr     Offworld_Box2C21C6_060398(pc)   | +09c
        bcc.w   .L060320                        | +0a0
.L060302:
        lea     0x77eda.l,a1                    | +0a4
        jsr     0x4ae.l                         | +0aa
        jsr     0x5dd02.l                       | +0b0
        move.w  #0x4000,0x38(a0)                | +0b6
        jmp     0x518.l                         | +0bc
.L060320:
        rts                                     | +0c2

| ----------------------------------------------------------------------------
|  Breakable_ShardVel_060322  @ $060322  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_ShardVel_060322, "ax", @progbits
        .global Breakable_ShardVel_060322
Breakable_ShardVel_060322:
        move.w  0x70(a6),d0                     | +000
        andi.l  #0x3,d0                         | +004
        asl.b   #0x1,d0                         | +00a
        move.w  0x16(a6,d0.w),0x14(a6)          | +00c
        addi.w  #0x1,0x70(a6)                   | +012
        cmpi.w  #0x3,0x70(a6)                   | +018
        blt.w   .L06034a                        | +01e
        move.w  #0x0,0x70(a6)                   | +022
.L06034a:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Breakable_SpawnShards_06034c  @ $06034C  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_SpawnShards_06034c, "ax", @progbits
        .global Breakable_SpawnShards_06034c
Breakable_SpawnShards_06034c:
        lea     Breakable_Shard_06025e(pc),a1   | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0x18,0x24(a0)                  | +010
        move.w  0x28(a6),0x28(a0)               | +016
        move.w  #0x400,0x2a(a0)                 | +01c
        move.w  #0x0,0x2c(a0)                   | +022
        addi.w  #0xffe0,0x2e(a0)                | +028
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Offworld_Box2C21BC_06037c  @ $06037C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C21BC_06037c, "ax", @progbits
        .global Offworld_Box2C21BC_06037c
Offworld_Box2C21BC_06037c:
        lea     0x2c21bc.l,a0                   | +000
        jsr     0x5dd56.l                       | +006
        bcc.w   ClearC_060392                   | +00c

| ----------------------------------------------------------------------------
|  Offworld_Box2C21C6_060398  @ $060398  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C21C6_060398, "ax", @progbits
        .global Offworld_Box2C21C6_060398
Offworld_Box2C21C6_060398:
        lea     0x2c21c6.l,a0                   | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   ClearC_0603ae                   | +00c

| ----------------------------------------------------------------------------
|  Facing_FromAngleHi_0603b4  @ $0603B4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Facing_FromAngleHi_0603b4, "ax", @progbits
        .global Facing_FromAngleHi_0603b4
Facing_FromAngleHi_0603b4:
        move.w  d0,d1                           | +000
        asr.w   #0x8,d1                         | +002
        btst    #0x7,d1                         | +004
        beq.w   .L0603c2                        | +008
        neg.w   d0                              | +00c
.L0603c2:
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Breakable_PieceRng_0603c4  @ $0603C4  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_PieceRng_0603c4, "ax", @progbits
        .global Breakable_PieceRng_0603c4
Breakable_PieceRng_0603c4:
        subq.b  #0x1,0x99(a6)                   | +000
        bmi.w   .L060402                        | +004
        jsr     0x5e9b6.l                       | +008
        andi.l  #0x3,d0                         | +00e
        cmpi.l  #0x3,d0                         | +014
        bne.w   .L0603e4                        | +01a
        moveq   #2,d0                           | +01e
.L0603e4:
        asl.l   #0x2,d0                         | +020
        lea     0x2c21d0.l,a0                   | +022
        movea.l (a0,d0.w),a1                    | +028
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd02.l                       | +032
        move.b  0x99(a6),0x99(a0)               | +038
.L060402:
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  Breakable_Stub_060404  @ $060404  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Stub_060404, "ax", @progbits
        .global Breakable_Stub_060404
Breakable_Stub_060404:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06041a                    | +00c

| ----------------------------------------------------------------------------
|  Sign_Tmpl28_060420  @ $060420  (266 B)
| ----------------------------------------------------------------------------
        .section .text.Sign_Tmpl28_060420, "ax", @progbits
        .global Sign_Tmpl28_060420
Sign_Tmpl28_060420:
        ori.b   #0x1,0x3a(a6)                   | +000
        lea     0x2c21dc.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     0x573a2.l,a0                    | +012
        move.l  a0,0x70(a6)                     | +018
        bra.w   .L0604bc                        | +01c
        ori.b   #0x1,0x3a(a6)                   | +020
        lea     0x2c21dc.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     0x572be.l,a0                    | +032
        move.l  a0,0x70(a6)                     | +038
        bra.w   .L0604bc                        | +03c
        ori.b   #0x1,0x3a(a6)                   | +040
        lea     0x2c09ac.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        lea     0x573a2.l,a0                    | +052
        move.l  a0,0x70(a6)                     | +058
        bra.w   .L0604bc                        | +05c
        ori.b   #0x1,0x3a(a6)                   | +060
        lea     0x2c09ac.l,a0                   | +066
        jsr     0x28cd4.l                       | +06c
        lea     0x572be.l,a0                    | +072
        move.l  a0,0x70(a6)                     | +078
        bra.w   .L0604bc                        | +07c
        ori.b   #0x1,0x3a(a6)                   | +080
        lea     0x2c226a.l,a0                   | +086
        jsr     0x28cd4.l                       | +08c
        lea     0x58f58.l,a0                    | +092
        move.l  a0,0x70(a6)                     | +098
.L0604bc:
        move.w  #0xe,d1                         | +09c
        jsr     0x236e.l                        | +0a0
        move.b  #0xff,0x32(a6)                  | +0a6
        move.b  #0xff,0x33(a6)                  | +0ac
        move.w  #0x8000,0x38(a6)                | +0b2
        jsr     0x267e2.l                       | +0b8
        lea     .L0604e4(pc),a1                 | +0be
        move.l  a1,(a6)                         | +0c2
.L0604e4:
        jsr     0x2783a.l                       | +0c4
        jsr     0x28d70.l                       | +0ca
        bcc.w   .L06051a                        | +0d0
        movea.l 0x70(a6),a1                     | +0d4
        jsr     0x4498e.l                       | +0d8
        jsr     0x5dd02.l                       | +0de
        move.l  0x98(a6),0x98(a0)               | +0e4
        move.l  0x9c(a6),0x9c(a0)               | +0ea
        lea     Sign_Idle_06052a(pc),a1         | +0f0
        move.l  a1,(a6)                         | +0f4
        bra.w   .L060528                        | +0f6
.L06051a:
        jsr     Offworld_Box2C2314_06053e(pc)   | +0fa
        bcc.w   .L060528                        | +0fe
        jmp     0x518.l                         | +102
.L060528:
        rts                                     | +108

| ----------------------------------------------------------------------------
|  Sign_Idle_06052a  @ $06052A  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sign_Idle_06052a, "ax", @progbits
        .global Sign_Idle_06052a
Sign_Idle_06052a:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        jmp     0x518.l                         | +00c

| ----------------------------------------------------------------------------
|  Sign_Rts_06053c  @ $06053C  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Sign_Rts_06053c, "ax", @progbits
        .global Sign_Rts_06053c
Sign_Rts_06053c:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Offworld_Box2C2314_06053e  @ $06053E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C2314_06053e, "ax", @progbits
        .global Offworld_Box2C2314_06053e
Offworld_Box2C2314_06053e:
        lea     0x2c2314.l,a0                   | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   ClearC_060554                   | +00c

| ----------------------------------------------------------------------------
|  Sign_Free_06055a  @ $06055A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Sign_Free_06055a, "ax", @progbits
        .global Sign_Free_06055a
Sign_Free_06055a:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_060570                    | +00c

| ----------------------------------------------------------------------------
|  HomingMarker_Targets_060576  @ $060576  (196 B)
| ----------------------------------------------------------------------------
        .section .text.HomingMarker_Targets_060576, "ax", @progbits
        .global HomingMarker_Targets_060576
HomingMarker_Targets_060576:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +016  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +018  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +028  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +038  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        move.w  #0xe,d1                         | +040
        jsr     0x236e.l                        | +044
        move.l  #0x2507fa,0x3c(a6)              | +04a
        lea     .L0605ce(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L0605ce:
        move.b  0x10e208.l,d0                   | +058
        andi.w  #0xf,d0                         | +05e
        add.w   d0,d0                           | +062
        add.w   d0,d0                           | +064
        lea     HomingMarker_Targets_060576(pc),a0 | +066
        move.w  (a0,d0.w),d1                    | +06a
        move.w  0x2(a0,d0.w),d2                 | +06e
        sub.w   0x28(a6),d1                     | +072
        beq.w   .L0605fc                        | +076
        asr.w   #0x4,d1                         | +07a
        bne.w   .L0605f8                        | +07c
        moveq   #1,d1                           | +080
.L0605f8:
        add.w   d1,0x28(a6)                     | +082
.L0605fc:
        sub.w   0x2a(a6),d2                     | +086
        beq.w   .L060610                        | +08a
        asr.w   #0x4,d2                         | +08e
        bne.w   .L06060c                        | +090
        moveq   #1,d2                           | +094
.L06060c:
        add.w   d2,0x2a(a6)                     | +096
.L060610:
        jsr     0x2783a.l                       | +09a
        jsr     0x9c072.l                       | +0a0
        move.w  0x22(a6),d0                     | +0a6
        move.w  0x24(a6),d1                     | +0aa
        addi.w  #0xfff0,d0                      | +0ae
        addq.w  #0x4,d1                         | +0b2
        move.w  #0x20,d2                        | +0b4
        jsr     0x99812.l                       | +0b8
        jmp     0x5ca2a.l                       | +0be

| ----------------------------------------------------------------------------
|  ItemProp_Tmpl114_06063a  @ $06063A  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ItemProp_Tmpl114_06063a, "ax", @progbits
        .global ItemProp_Tmpl114_06063a
ItemProp_Tmpl114_06063a:
        lea     0x2c25fa.l,a1                   | +000
        jsr     0x43fac.l                       | +006
        move.w  #0xe,d1                         | +00c
        jsr     0x236e.l                        | +010
        move.l  #0x2507fa,0x3c(a6)              | +016
        bra.w   SetTaskHandler_0606e6           | +01e

| ----------------------------------------------------------------------------
|  ItemProp_Tmpl115_06065c  @ $06065C  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ItemProp_Tmpl115_06065c, "ax", @progbits
        .global ItemProp_Tmpl115_06065c
ItemProp_Tmpl115_06065c:
        lea     0x2c2606.l,a1                   | +000
        jsr     0x43fac.l                       | +006
        move.w  #0xe,d1                         | +00c
        jsr     0x236e.l                        | +010
        move.l  #0x25086e,0x3c(a6)              | +016
        bra.w   SetTaskHandler_0606e6           | +01e

| ----------------------------------------------------------------------------
|  ItemProp_Tmpl116_06067e  @ $06067E  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ItemProp_Tmpl116_06067e, "ax", @progbits
        .global ItemProp_Tmpl116_06067e
ItemProp_Tmpl116_06067e:
        lea     0x2c2622.l,a1                   | +000
        jsr     0x43fac.l                       | +006
        move.w  #0xe,d1                         | +00c
        jsr     0x236e.l                        | +010
        move.l  #0x250838,0x3c(a6)              | +016
        bra.w   SetTaskHandler_0606e6           | +01e

| ----------------------------------------------------------------------------
|  ItemProp_Tmpl117_0606a0  @ $0606A0  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ItemProp_Tmpl117_0606a0, "ax", @progbits
        .global ItemProp_Tmpl117_0606a0
ItemProp_Tmpl117_0606a0:
        lea     0x2c2614.l,a1                   | +000
        jsr     0x43fac.l                       | +006
        move.w  #0xe,d1                         | +00c
        jsr     0x236e.l                        | +010
        move.l  #0x25084a,0x3c(a6)              | +016
        bra.w   SetTaskHandler_0606e6           | +01e

| ----------------------------------------------------------------------------
|  ItemProp_Tmpl118_0606c2  @ $0606C2  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ItemProp_Tmpl118_0606c2, "ax", @progbits
        .global ItemProp_Tmpl118_0606c2
ItemProp_Tmpl118_0606c2:
        lea     0x2c2630.l,a1                   | +000
        jsr     0x43fac.l                       | +006
        move.w  #0xe,d1                         | +00c
        jsr     0x236e.l                        | +010
        move.l  #0x25085c,0x3c(a6)              | +016
        bra.w   SetTaskHandler_0606e6           | +01e

| ----------------------------------------------------------------------------
|  ItemProp_Rts_0606e4  @ $0606E4  (2 B)
| ----------------------------------------------------------------------------
        .section .text.ItemProp_Rts_0606e4, "ax", @progbits
        .global ItemProp_Rts_0606e4
ItemProp_Rts_0606e4:
        nop                                     | +000

| ----------------------------------------------------------------------------
|  ItemProp_Idle_0606ee  @ $0606EE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ItemProp_Idle_0606ee, "ax", @progbits
        .global ItemProp_Idle_0606ee
ItemProp_Idle_0606ee:
        jsr     0x2783a.l                       | +000
        jsr     0x5ca2a.l                       | +006
        jmp     ItemProp_Wait_0606fe__L060706(pc) | +00c

| ----------------------------------------------------------------------------
|  ItemProp_Wait_0606fe  @ $0606FE  (30 B)
| ----------------------------------------------------------------------------
        .section .text.ItemProp_Wait_0606fe, "ax", @progbits
        .global ItemProp_Wait_0606fe
ItemProp_Wait_0606fe:
        dc.w    Sub_0000FFD0                    | +000
        ori.b   #0x0,0x60(a0,d0.w)              | +002
        .global ItemProp_Wait_0606fe__L060706
ItemProp_Wait_0606fe__L060706:
.L060706:
        lea     ItemProp_Wait_0606fe(pc),a0     | +008
        jsr     0x5dd5c.l                       | +00c
        bcc.w   .L06071a                        | +012
        jmp     0x518.l                         | +016
.L06071a:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Obstacle_CommonInit_06071c  @ $06071C  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_CommonInit_06071c, "ax", @progbits
        .global Obstacle_CommonInit_06071c
Obstacle_CommonInit_06071c:
        move.w  #0x6c,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x8000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0xc,0x38(a6)                   | +01a
        lea     0x2c231e.l,a0                   | +020
        move.l  a0,0x48(a6)                     | +026
        lea     0x2bf414.l,a0                   | +02a
        jsr     0x799de.l                       | +030
        move.w  d0,0x66(a6)                     | +036
        lea     0x2c252e.l,a1                   | +03a

| ----------------------------------------------------------------------------
|  Obstacle_Tmpl110_060764  @ $060764  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Tmpl110_060764, "ax", @progbits
        .global Obstacle_Tmpl110_060764
Obstacle_Tmpl110_060764:
        move.b  #0x2,0x70(a6)                   | +000
        bra.w   .L06077c                        | +006
        move.b  #0x1,0x70(a6)                   | +00a
        bra.w   .L06077c                        | +010
        clr.b   0x70(a6)                        | +014
.L06077c:
        jsr     Obstacle_CommonInit_06071c(pc)  | +018
        lea     0x2c2416.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022

| ----------------------------------------------------------------------------
|  Obstacle_Idle_06078c  @ $06078C  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Idle_06078c, "ax", @progbits
        .global Obstacle_Idle_06078c
Obstacle_Idle_06078c:
        bclr    #0x3,0x13(a6)                   | +000
        lea     .L060798(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L060798:
        jsr     0x2783a.l                       | +00c
        jsr     0x28d70.l                       | +012
        jsr     0x2870a.l                       | +018
        bcc.w   .L0607b4                        | +01e
        lea     Obstacle_Damaged_0607d0(pc),a1  | +022
        move.l  a1,(a6)                         | +026
.L0607b4:
        bclr    #0x3,0x13(a6)                   | +028
        jsr     0x28758.l                       | +02e
        bcc.w   .L0607ca                        | +034
        lea     Obstacle_Destroyed_060814(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L0607ca:
        bsr.w   ItemProp_Wait_0606fe__L060706   | +03e
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Obstacle_Damaged_0607d0  @ $0607D0  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Damaged_0607d0, "ax", @progbits
        .global Obstacle_Damaged_0607d0
Obstacle_Damaged_0607d0:
        lea     0x2c2422.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0607e2(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0607e2:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L0607f8                        | +01e
        lea     Obstacle_Idle_06078c(pc),a1     | +022
        move.l  a1,(a6)                         | +026
.L0607f8:
        bclr    #0x3,0x13(a6)                   | +028
        jsr     0x28758.l                       | +02e
        bcc.w   .L06080e                        | +034
        lea     Obstacle_Destroyed_060814(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L06080e:
        bsr.w   ItemProp_Wait_0606fe__L060706   | +03e
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Obstacle_Destroyed_060814  @ $060814  (114 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Destroyed_060814, "ax", @progbits
        .global Obstacle_Destroyed_060814
Obstacle_Destroyed_060814:
        jsr     0x2783a.l                       | +000
        move.b  0x70(a6),d0                     | +006
        bne.w   .L06082c                        | +00a
        lea     0x2c2572.l,a1                   | +00e
        bra.w   .L06084c                        | +014
.L06082c:
        cmpi.b  #0x1,d0                         | +018
        bne.w   .L06083e                        | +01c
        lea     0x2c2550.l,a1                   | +020
        bra.w   .L06084c                        | +026
.L06083e:
        cmpi.b  #0x2,d0                         | +02a
        bne.w   .L06084c                        | +02e
        lea     0x2c2550.l,a1                   | +032
.L06084c:
        jsr     0x43fac.l                       | +038
        bclr    #0x0,0x13(a6)                   | +03e
        lea     0x2c244e.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        lea     .L06086a(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L06086a:
        jsr     0x2783a.l                       | +056
        jsr     0x28d70.l                       | +05c
        bcc.w   .L060880                        | +062
        lea     Obstacle_Rearm_060886(pc),a1    | +066
        move.l  a1,(a6)                         | +06a
.L060880:
        bsr.w   ItemProp_Wait_0606fe__L060706   | +06c
        rts                                     | +070

| ----------------------------------------------------------------------------
|  Obstacle_Rearm_060886  @ $060886  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Rearm_060886, "ax", @progbits
        .global Obstacle_Rearm_060886
Obstacle_Rearm_060886:
        bclr    #0x3,0x13(a6)                   | +000
        jsr     0x2783a.l                       | +006
        lea     0x2bf496.l,a0                   | +00c
        jsr     0x799de.l                       | +012
        move.w  d0,0x66(a6)                     | +018

| ----------------------------------------------------------------------------
|  Obstacle_IdleB_0608a2  @ $0608A2  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_IdleB_0608a2, "ax", @progbits
        .global Obstacle_IdleB_0608a2
Obstacle_IdleB_0608a2:
        lea     .L0608a8(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L0608a8:
        jsr     0x2783a.l                       | +006
        jsr     0x28d70.l                       | +00c
        jsr     0x2870a.l                       | +012
        bcc.w   .L0608c4                        | +018
        lea     Obstacle_DamagedB_0608e0(pc),a1 | +01c
        move.l  a1,(a6)                         | +020
.L0608c4:
        bclr    #0x3,0x13(a6)                   | +022
        jsr     0x28758.l                       | +028
        bcc.w   .L0608da                        | +02e
        lea     Obstacle_DestroyedB_060924(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L0608da:
        bsr.w   ItemProp_Wait_0606fe__L060706   | +038
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  Obstacle_DamagedB_0608e0  @ $0608E0  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_DamagedB_0608e0, "ax", @progbits
        .global Obstacle_DamagedB_0608e0
Obstacle_DamagedB_0608e0:
        lea     0x2c2438.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0608f2(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0608f2:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L060908                        | +01e
        lea     Obstacle_IdleB_0608a2(pc),a1    | +022
        move.l  a1,(a6)                         | +026
.L060908:
        bclr    #0x3,0x13(a6)                   | +028
        jsr     0x28758.l                       | +02e
        bcc.w   .L06091e                        | +034
        lea     Obstacle_DestroyedB_060924(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L06091e:
        bsr.w   ItemProp_Wait_0606fe__L060706   | +03e
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Obstacle_DestroyedB_060924  @ $060924  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_DestroyedB_060924, "ax", @progbits
        .global Obstacle_DestroyedB_060924
Obstacle_DestroyedB_060924:
        bclr    #0x0,0x13(a6)                   | +000
        jsr     0x2783a.l                       | +006
        lea     0x2c2504.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        move.b  0x70(a6),d0                     | +018
        bne.w   .L06094e                        | +01c
        lea     0x2c2594.l,a1                   | +020
        bra.w   .L06096e                        | +026
.L06094e:
        cmpi.b  #0x1,d0                         | +02a
        bne.w   .L060960                        | +02e
        lea     0x2c25b6.l,a1                   | +032
        bra.w   .L06096e                        | +038
.L060960:
        cmpi.b  #0x2,d0                         | +03c
        bne.w   .L06096e                        | +040
        lea     0x2c25d8.l,a1                   | +044
.L06096e:
        jsr     0x43fac.l                       | +04a
        jsr     0x8f308.l                       | +050
        lea     .L060980(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L060980:
        jsr     0x2783a.l                       | +05c
        jsr     0x28d70.l                       | +062
        bcc.w   .L060996                        | +068
        lea     Obstacle_Wreck_06099c(pc),a1    | +06c
        move.l  a1,(a6)                         | +070
.L060996:
        bsr.w   ItemProp_Wait_0606fe__L060706   | +072
        rts                                     | +076

| ----------------------------------------------------------------------------
|  Obstacle_Wreck_06099c  @ $06099C  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Wreck_06099c, "ax", @progbits
        .global Obstacle_Wreck_06099c
Obstacle_Wreck_06099c:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        bclr    #0x3,0x13(a6)                   | +008
        lea     .L0609b0(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L0609b0:
        jsr     0x2783a.l                       | +014
        jsr     0x28d70.l                       | +01a
        bsr.w   ItemProp_Wait_0606fe__L060706   | +020
        rts                                     | +024

| ----------------------------------------------------------------------------
|  Obstacle_SpawnChild_0609c2  @ $0609C2  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_SpawnChild_0609c2, "ax", @progbits
        .global Obstacle_SpawnChild_0609c2
Obstacle_SpawnChild_0609c2:
        lea     Obstacle_Smoke_0609d4(pc),a1    | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  Obstacle_Smoke_0609d4  @ $0609D4  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Smoke_0609d4, "ax", @progbits
        .global Obstacle_Smoke_0609d4
Obstacle_Smoke_0609d4:
        move.w  #0x6c,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd37e.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x2000,0x38(a6)                | +016
        lea     .L0609f6(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L0609f6:
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        bcc.w   .L060a0c                        | +02e
        jmp     0x518.l                         | +032
.L060a0c:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  Obstacle_Tmpl111_060a0e  @ $060A0E  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Tmpl111_060a0e, "ax", @progbits
        .global Obstacle_Tmpl111_060a0e
Obstacle_Tmpl111_060a0e:
        move.w  #0x82,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xf000,0x38(a6)                | +00a
        lea     0x2c26b4.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L060a30(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L060a30:
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        bclr    #0x3,0x13(a6)                   | +02e
        bclr    #0x0,0x13(a6)                   | +034
        bra.w   ItemProp_Wait_0606fe__L060706   | +03a

| ----------------------------------------------------------------------------
|  Obstacle_Tmpl094_060a4c  @ $060A4C  (198 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Tmpl094_060a4c, "ax", @progbits
        .global Obstacle_Tmpl094_060a4c
Obstacle_Tmpl094_060a4c:
        move.b  #0x2,0x70(a6)                   | +000
        bra.w   .L060a64                        | +006
        move.b  #0x1,0x70(a6)                   | +00a
        bra.w   .L060a64                        | +010
        clr.b   0x70(a6)                        | +014
.L060a64:
        move.w  #0x82,d1                        | +018
        jsr     0x236e.l                        | +01c
        lea     0x2c27a6.l,a1                   | +022
        cmpi.b  #0x1,0x70(a6)                   | +028
        bne.w   .L060a88                        | +02e
        lea     0x2c2796.l,a1                   | +032
        bra.w   .L060a98                        | +038
.L060a88:
        cmpi.b  #0x2,0x70(a6)                   | +03c
        bne.w   .L060a98                        | +042
        lea     0x2c27b6.l,a1                   | +046
.L060a98:
        jsr     0x43fac.l                       | +04c
        lea     0x2bf518.l,a0                   | +052
        jsr     0x799de.l                       | +058
        move.w  d0,0x66(a6)                     | +05e
        move.w  #0x8000,d0                      | +062
        jsr     0x28134.l                       | +066
        andi.w  #0xffe3,0x38(a6)                | +06c
        ori.w   #0xc,0x38(a6)                   | +072
        lea     0x2c26b4.l,a0                   | +078
        jsr     0x28cd4.l                       | +07e
        lea     .L060ad6(pc),a1                 | +084
        move.l  a1,(a6)                         | +088
.L060ad6:
        jsr     0x2783a.l                       | +08a
        jsr     0x28758.l                       | +090
        bcc.w   .L060af0                        | +096
        lea     Obstacle_Flames_060b40(pc),a1   | +09a
        move.l  a1,(a6)                         | +09e
        bra.w   .L060b06                        | +0a0
.L060af0:
        jsr     0x2870a.l                       | +0a4
        bcc.w   .L060b06                        | +0aa
        lea     0x2c26c6.l,a0                   | +0ae
        jsr     0x28cd4.l                       | +0b4
.L060b06:
        jsr     0x28d70.l                       | +0ba
        bsr.w   ItemProp_Wait_0606fe__L060706   | +0c0
        rts                                     | +0c4

| ----------------------------------------------------------------------------
|  Obstacle094_Register_060b12  @ $060B12  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle094_Register_060b12, "ax", @progbits
        .global Obstacle094_Register_060b12
Obstacle094_Register_060b12:
        lea     0x2c27c6.l,a1                   | +000
        cmpi.b  #0x1,0x70(a6)                   | +006
        bne.w   .L060b28                        | +00c
        lea     0x2c27d6.l,a1                   | +010
.L060b28:
        cmpi.b  #0x2,0x70(a6)                   | +016
        bne.w   .L060b38                        | +01c
        lea     0x2c27e6.l,a1                   | +020
.L060b38:
        jmp     0x43fac.l                       | +026

| ----------------------------------------------------------------------------
|  Obstacle_Rts_060b3e  @ $060B3E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Rts_060b3e, "ax", @progbits
        .global Obstacle_Rts_060b3e
Obstacle_Rts_060b3e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Obstacle_Flames_060b40  @ $060B40  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_Flames_060b40, "ax", @progbits
        .global Obstacle_Flames_060b40
Obstacle_Flames_060b40:
        lea     0x2c26ea.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L060b52(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L060b52:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L060b6e                        | +01e
        jsr     0x5b6.l                         | +022
        jmp     0x518.l                         | +028
.L060b6e:
        bsr.w   ItemProp_Wait_0606fe__L060706   | +02e
        rts                                     | +032

| ----------------------------------------------------------------------------
|  Obstacle095_SoundTable_060b74  @ $060B74  (122 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle095_SoundTable_060b74, "ax", @progbits
        .global Obstacle095_SoundTable_060b74
Obstacle095_SoundTable_060b74:
        .dc.w   0x0011                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0130                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0131                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1d7c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0070                        | +00a  (dato / opcode no decodificado)
        bra.w   .L060b88                        | +00c
        clr.b   0x70(a6)                        | +010
.L060b88:
        moveq   #0,d0                           | +014
        move.b  0x98(a6),d0                     | +016
        cmpi.b  #0x3,d0                         | +01a
        bcs.w   .L060b98                        | +01e
        moveq   #0,d0                           | +022
.L060b98:
        add.w   d0,d0                           | +024
        lea     Obstacle095_SoundTable_060b74(pc),a0 | +026
        move.w  (a0,d0.w),d1                    | +02a
        jsr     0x236e.l                        | +02e
        lea     0x2c29bc.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        tst.b   0x70(a6)                        | +040
        beq.w   .L060bc6                        | +044
        lea     0x2c28fc.l,a1                   | +048
        bra.w   .L060bcc                        | +04e
.L060bc6:
        lea     0x2c28bc.l,a1                   | +052
.L060bcc:
        jsr     0x43fac.l                       | +058
        move.w  #0x64,0x66(a6)                  | +05e
        move.w  #0x8000,d0                      | +064
        jsr     0x28134.l                       | +068
        andi.w  #0xffe3,0x38(a6)                | +06e
        ori.w   #0xc,0x38(a6)                   | +074

| ----------------------------------------------------------------------------
|  Obstacle095_Idle_060bf6  @ $060BF6  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle095_Idle_060bf6, "ax", @progbits
        .global Obstacle095_Idle_060bf6
Obstacle095_Idle_060bf6:
        jsr     0x2783a.l                       | +000
        jsr     0x28758.l                       | +006
        bcc.w   .L060c0e                        | +00c
        bsr.w   Obstacle095_Wreck_060c6a        | +010
        bra.w   .L060c36                        | +014
.L060c0e:
        jsr     0x2870a.l                       | +018
        bcc.w   .L060c36                        | +01e
        cmpi.b  #0x4,0x58(a6)                   | +022
        bne.w   .L060c2a                        | +028
        bsr.w   Obstacle095_Wreck_060c6a        | +02c
        bra.w   .L060c36                        | +030
.L060c2a:
        lea     0x2c29ce.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
.L060c36:
        jsr     0x28d70.l                       | +040
        bra.w   ItemProp_Wait_0606fe__L060706   | +046

| ----------------------------------------------------------------------------
|  Obstacle_FlushMusicJmp77FD6_060c40  @ $060C40  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle_FlushMusicJmp77FD6_060c40, "ax", @progbits
        .global Obstacle_FlushMusicJmp77FD6_060c40
Obstacle_FlushMusicJmp77FD6_060c40:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L060c66                        | +00c
        jsr     0x13600.l                       | +010
        move.w  #0x109f,d0                      | +016
        jsr     0x2352.l                        | +01a
        jmp     0x77fd6.l                       | +020
.L060c66:
        bra.w   ItemProp_Wait_0606fe__L060706   | +026

| ----------------------------------------------------------------------------
|  Obstacle095_Wreck_060c6a  @ $060C6A  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle095_Wreck_060c6a, "ax", @progbits
        .global Obstacle095_Wreck_060c6a
Obstacle095_Wreck_060c6a:
        move.w  0x22(a6),d0                     | +000
        sub.w   0x54(a6),d0                     | +004
        bmi.w   .L060c86                        | +008
        lea     0x2c2a14.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        bra.w   SetTaskHandler_060c92           | +018
.L060c86:
        lea     0x2c2aca.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022

| ----------------------------------------------------------------------------
|  Obstacle095_Register_060c9a  @ $060C9A  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle095_Register_060c9a, "ax", @progbits
        .global Obstacle095_Register_060c9a
Obstacle095_Register_060c9a:
        tst.b   0x70(a6)                        | +000
        beq.w   .L060cac                        | +004
        lea     0x2c297c.l,a1                   | +008
        bra.w   .L060cb2                        | +00e
.L060cac:
        lea     0x2c293c.l,a1                   | +012
.L060cb2:
        jmp     0x43fac.l                       | +018

| ----------------------------------------------------------------------------
|  Obstacle095_Stub_060cb8  @ $060CB8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Obstacle095_Stub_060cb8, "ax", @progbits
        .global Obstacle095_Stub_060cb8
Obstacle095_Stub_060cb8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_060cce                    | +00c

| ----------------------------------------------------------------------------
|  Crate_Tmpl07C_060cd4  @ $060CD4  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Tmpl07C_060cd4, "ax", @progbits
        .global Crate_Tmpl07C_060cd4
Crate_Tmpl07C_060cd4:
        move.w  #0x0,0x80(a6)                   | +000
        bra.w   .L060cee                        | +006
        move.w  #0x1,0x80(a6)                   | +00a
        bra.w   .L060cee                        | +010
        move.w  #0x2,0x80(a6)                   | +014
.L060cee:
        move.w  #0x4f,d1                        | +01a
        jsr     0x236e.l                        | +01e
        moveq   #0,d0                           | +024
        move.b  d0,0x20(a6)                     | +026
        move.b  d0,0x21(a6)                     | +02a
        lea     0x2c2b80.l,a0                   | +02e
        move.w  0x80(a6),d0                     | +034
        add.w   d0,d0                           | +038
        add.w   d0,d0                           | +03a
        movea.l (a0,d0.w),a0                    | +03c
        jsr     (a0)                            | +040
        lea     0x2bdc06.l,a0                   | +042
        jsr     0x799de.l                       | +048
        move.w  d0,0x66(a6)                     | +04e
        bclr    #0x1,0x12(a6)                   | +052
        jsr     0x267e2.l                       | +058

| ----------------------------------------------------------------------------
|  Crate_Idle_060d3a  @ $060D3A  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Idle_060d3a, "ax", @progbits
        .global Crate_Idle_060d3a
Crate_Idle_060d3a:
        move.w  0x38(a6),d0                     | +000
        addq.w  #0x1,d0                         | +004
        jsr     0x28134.l                       | +006
        lea     0x29c536.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L060d58(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L060d58:
        jsr     0x2783a.l                       | +01e
        jsr     0x28d70.l                       | +024
        jsr     0x2870a.l                       | +02a
        bcc.w   .L060d80                        | +030
        lea     0x5e766.l,a0                    | +034
        jsr     0x5e770.l                       | +03a
        bclr    #0x3,0x13(a6)                   | +040
.L060d80:
        jsr     0x28758.l                       | +046
        bcc.w   .L060d90                        | +04c
        lea     Crate_Destroyed_060da0(pc),a1   | +050
        move.l  a1,(a6)                         | +054
.L060d90:
        jsr     Offworld_Box2C2C60_060df0(pc)   | +056
        bcc.w   SetHandlerRts_060d9e            | +05a

| ----------------------------------------------------------------------------
|  Crate_Destroyed_060da0  @ $060DA0  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Destroyed_060da0, "ax", @progbits
        .global Crate_Destroyed_060da0
Crate_Destroyed_060da0:
        bclr    #0x1,0x12(a6)                   | +000
        lea     0x2c2b8c.l,a0                   | +006
        move.w  0x80(a6),d0                     | +00c
        add.w   d0,d0                           | +010
        add.w   d0,d0                           | +012
        movea.l (a0,d0.w),a0                    | +014
        jsr     (a0)                            | +018
        lea     0x29c54c.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L060dcc(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L060dcc:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        jsr     Offworld_Box2C2C60_060df0(pc)   | +038
        bcc.w   SetHandlerRts_060de6            | +03c

| ----------------------------------------------------------------------------
|  Offworld_Box2C2C60_060df0  @ $060DF0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Offworld_Box2C2C60_060df0, "ax", @progbits
        .global Offworld_Box2C2C60_060df0
Offworld_Box2C2C60_060df0:
        lea     0x2c2c60.l,a0                   | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   ClearC_060e06                   | +00c

| ----------------------------------------------------------------------------
|  Crate_Register0_060e0c  @ $060E0C  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Register0_060e0c, "ax", @progbits
        .global Crate_Register0_060e0c
Crate_Register0_060e0c:
        lea     0x2c2b98.l,a1                   | +000

| ----------------------------------------------------------------------------
|  Crate_Register1_060e1a  @ $060E1A  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Register1_060e1a, "ax", @progbits
        .global Crate_Register1_060e1a
Crate_Register1_060e1a:
        lea     0x2c2bc4.l,a1                   | +000

| ----------------------------------------------------------------------------
|  Crate_Register2_060e28  @ $060E28  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Register2_060e28, "ax", @progbits
        .global Crate_Register2_060e28
Crate_Register2_060e28:
        lea     0x2c2bf0.l,a1                   | +000

| ----------------------------------------------------------------------------
|  Crate_Register3_060e36  @ $060E36  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Register3_060e36, "ax", @progbits
        .global Crate_Register3_060e36
Crate_Register3_060e36:
        lea     0x2c2c28.l,a1                   | +000

| ----------------------------------------------------------------------------
|  Crate_Stub_060e46  @ $060E46  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Stub_060e46, "ax", @progbits
        .global Crate_Stub_060e46
Crate_Stub_060e46:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_060e5c                    | +00c

| ----------------------------------------------------------------------------
|  AimTurret_Tmpl08E_060e62  @ $060E62  (162 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Tmpl08E_060e62, "ax", @progbits
        .global AimTurret_Tmpl08E_060e62
AimTurret_Tmpl08E_060e62:
        move.w  #0x187,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x188,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x18a,d1                       | +014
        jsr     0x236e.l                        | +018
        move.w  #0x10e8,d0                      | +01e
        jsr     0x2352.l                        | +022
        move.b  #0x0,0x7c(a6)                   | +028
        move.w  #0x8000,0x38(a6)                | +02e
        jsr     0x267e2.l                       | +034
        jsr     0x27cee.l                       | +03a
        move.b  0x98(a6),d0                     | +040
        andi.l  #0x3,d0                         | +044
        move.b  d0,0x3a(a6)                     | +04a
        move.w  0x22(a6),0x72(a6)               | +04e
        move.w  0x24(a6),0x74(a6)               | +054
        jsr     AimTurret_Acquire_06179e(pc)    | +05a
        lea     0x2bdd0a.l,a0                   | +05e
        jsr     0x799de.l                       | +064
        move.w  d0,0x66(a6)                     | +06a
        lea     0x2bde0e.l,a0                   | +06e
        jsr     0x799de.l                       | +074
        move.w  d0,0x78(a6)                     | +07a
        move.b  #0xff,0x7b(a6)                  | +07e
        lea     0x2c32d6.l,a0                   | +084
        move.l  a0,0x48(a6)                     | +08a
        move.b  #0x0,0x20(a6)                   | +08e
        jsr     AimTurret_ScrollGate_061952(pc) | +094
        lea     AimTurret_Gun_061254(pc),a1     | +098
        jsr     0x4ae.l                         | +09c

| ----------------------------------------------------------------------------
|  AimTurret_Active_060f04  @ $060F04  (132 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Active_060f04, "ax", @progbits
        .global AimTurret_Active_060f04
AimTurret_Active_060f04:
        lea     .L060f0a(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L060f0a:
        jsr     AimTurret_Acquire_06179e(pc)    | +006
        jsr     AimTurret_AimAtPlayer_06164c(pc) | +00a
        jsr     AimTurret_HitCheck_061580(pc)   | +00e
        jsr     AimTurret_Anim_0617d2(pc)       | +012
        jsr     AimTurret_Cooldown_0614b6(pc)   | +016
        jsr     0x28d70.l                       | +01a
        jsr     0x28758.l                       | +020
        bcc.w   .L060f44                        | +026
        lea     AimTurret_Die_06100c(pc),a1     | +02a
        move.l  a1,(a6)                         | +02e
        cmpi.b  #0xff,0x9a(a6)                  | +030
        bne.w   .L060f44                        | +036
        lea     AimTurret_DieB_0610f4(pc),a1    | +03a
        move.l  a1,(a6)                         | +03e
.L060f44:
        jsr     AimTurret_RngDifficulty_0618b0(pc) | +040
        jsr     AimTurret_StepAngle_06174a(pc)  | +044
        bcc.w   .L060f56                        | +048
        lea     AimTurret_ActiveB_060f88(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L060f56:
        jsr     AimTurret_ScrollGateB_06197c(pc) | +052
        bcc.w   .L060f64                        | +056
        lea     AimTurret_Dormant_061204(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
.L060f64:
        movea.l #0xffffffff,a0                  | +060
        lea     0x2c33a0.l,a0                   | +066
        jsr     0x5dd5c.l                       | +06c
        bcc.w   .L060f86                        | +072
        move.b  #0xff,0x20(a6)                  | +076
        jmp     0x518.l                         | +07c
.L060f86:
        rts                                     | +082

| ----------------------------------------------------------------------------
|  AimTurret_ActiveB_060f88  @ $060F88  (132 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_ActiveB_060f88, "ax", @progbits
        .global AimTurret_ActiveB_060f88
AimTurret_ActiveB_060f88:
        lea     .L060f8e(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L060f8e:
        jsr     AimTurret_Acquire_06179e(pc)    | +006
        jsr     AimTurret_SinCos40_06161e(pc)   | +00a
        jsr     AimTurret_HitCheck_061580(pc)   | +00e
        jsr     AimTurret_Anim_0617d2(pc)       | +012
        jsr     AimTurret_Cooldown_0614b6(pc)   | +016
        jsr     0x28d70.l                       | +01a
        jsr     0x28758.l                       | +020
        bcc.w   .L060fc8                        | +026
        lea     AimTurret_Die_06100c(pc),a1     | +02a
        move.l  a1,(a6)                         | +02e
        cmpi.b  #0xff,0x9a(a6)                  | +030
        bne.w   .L060fc8                        | +036
        lea     AimTurret_DieB_0610f4(pc),a1    | +03a
        move.l  a1,(a6)                         | +03e
.L060fc8:
        jsr     AimTurret_RngDifficulty_0618b0(pc) | +040
        jsr     AimTurret_StepAngleB_061774(pc) | +044
        bcs.w   .L060fda                        | +048
        lea     AimTurret_Active_060f04(pc),a1  | +04c
        move.l  a1,(a6)                         | +050
.L060fda:
        jsr     AimTurret_ScrollGateB_06197c(pc) | +052
        bcc.w   .L060fe8                        | +056
        lea     AimTurret_Dormant_061204(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
.L060fe8:
        movea.l #0xffffffff,a0                  | +060
        lea     0x2c33a0.l,a0                   | +066
        jsr     0x5dd5c.l                       | +06c
        bcc.w   .L06100a                        | +072
        move.b  #0xff,0x20(a6)                  | +076
        jmp     0x518.l                         | +07c
.L06100a:
        rts                                     | +082

| ----------------------------------------------------------------------------
|  AimTurret_Die_06100c  @ $06100C  (232 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Die_06100c, "ax", @progbits
        .global AimTurret_Die_06100c
AimTurret_Die_06100c:
        move.w  #0x1ff,0x74(a6)                 | +000
        lea     0x2c2cd8.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L061024(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L061024:
        jsr     AimTurret_AimAtPlayerB_0616ca(pc) | +018
        jsr     AimTurret_HitCheck_061580(pc)   | +01c
        jsr     AimTurret_Anim_0617d2(pc)       | +020
        jsr     0x28d70.l                       | +024
        cmpi.w  #0x1b0,0x24(a6)                 | +02a
        blt.w   .L061088                        | +030
        lea     0x77fd6.l,a1                    | +034
        jsr     0x4ae.l                         | +03a
        jsr     0x5dd02.l                       | +040
        subi.w  #0x10,0x24(a0)                  | +046
        lea     0x2c3406.l,a1                   | +04c
        jsr     0x77c7e.l                       | +052
        move.w  #0x102f,d0                      | +058
        jsr     0x2352.l                        | +05c
        move.w  #0x10e8,d0                      | +062
        jsr     0x2222.l                        | +066
        move.b  #0xff,0x20(a6)                  | +06c
        jsr     AimTurret_DropItem_061482(pc)   | +072
        jmp     0x518.l                         | +076
.L061088:
        move.b  0x106f28.l,d0                   | +07c
        andi.b  #0x3,d0                         | +082
        bne.w   .L0610bc                        | +086
        lea     0x2c33b4.l,a1                   | +08a
        jsr     0x77c7e.l                       | +090
        move.w  #0x28,d0                        | +096
        btst    #0x0,0x3a(a6)                   | +09a
        beq.w   .L0610b2                        | +0a0
        neg.w   d0                              | +0a4
.L0610b2:
        add.w   d0,0x22(a0)                     | +0a6
        subi.w  #0x18,0x24(a0)                  | +0aa
.L0610bc:
        movea.l #0xffffffff,a0                  | +0b0
        lea     0x2c33a0.l,a0                   | +0b6
        jsr     0x5dd5c.l                       | +0bc
        bcc.w   .L0610f2                        | +0c2
        move.w  #0x102f,d0                      | +0c6
        jsr     0x2352.l                        | +0ca
        move.w  #0x10e8,d0                      | +0d0
        jsr     0x2222.l                        | +0d4
        move.b  #0xff,0x20(a6)                  | +0da
        jmp     0x518.l                         | +0e0
.L0610f2:
        rts                                     | +0e6

| ----------------------------------------------------------------------------
|  AimTurret_DieB_0610f4  @ $0610F4  (272 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_DieB_0610f4, "ax", @progbits
        .global AimTurret_DieB_0610f4
AimTurret_DieB_0610f4:
        move.w  #0x140,0x72(a6)                 | +000
        move.w  #0x160,0x74(a6)                 | +006
        move.b  0x9c(a6),d1                     | +00c
        asl.b   #0x1,d1                         | +010
        move.b  d1,0x9c(a6)                     | +012
        lea     0x2c2cd8.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L06111c(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L06111c:
        jsr     AimTurret_AimAtPlayer_06164c(pc) | +028
        jsr     AimTurret_HitCheck_061580(pc)   | +02c
        jsr     AimTurret_Anim_0617d2(pc)       | +030
        jsr     0x28d70.l                       | +034
        move.w  0x22(a6),d0                     | +03a
        move.w  0x24(a6),d1                     | +03e
        jsr     0x4400e.l                       | +042
        cmpi.w  #0x3e0,d0                       | +048
        blt.w   .L061198                        | +04c
        lea     0x77fd6.l,a1                    | +050
        jsr     0x4ae.l                         | +056
        jsr     0x5dd02.l                       | +05c
        subi.w  #0x10,0x24(a0)                  | +062
        lea     0x2c3406.l,a1                   | +068
        jsr     0x77c7e.l                       | +06e
        move.w  #0x102f,d0                      | +074
        jsr     0x2352.l                        | +078
        move.w  #0x10e8,d0                      | +07e
        jsr     0x2222.l                        | +082
        move.b  #0xff,0x20(a6)                  | +088
        lea     AimTurret_GunAttack_0612a8(pc),a1 | +08e
        jsr     0x4ae.l                         | +092
        jsr     0x5dd02.l                       | +098
        jmp     0x518.l                         | +09e
.L061198:
        move.b  0x106f28.l,d0                   | +0a4
        andi.b  #0x3,d0                         | +0aa
        bne.w   .L0611cc                        | +0ae
        lea     0x2c33b4.l,a1                   | +0b2
        jsr     0x77c7e.l                       | +0b8
        move.w  #0x28,d0                        | +0be
        btst    #0x0,0x3a(a6)                   | +0c2
        beq.w   .L0611c2                        | +0c8
        neg.w   d0                              | +0cc
.L0611c2:
        add.w   d0,0x22(a0)                     | +0ce
        subi.w  #0x18,0x24(a0)                  | +0d2
.L0611cc:
        movea.l #0xffffffff,a0                  | +0d8
        lea     0x2c33a0.l,a0                   | +0de
        jsr     0x5dd5c.l                       | +0e4
        bcc.w   .L061202                        | +0ea
        move.w  #0x102f,d0                      | +0ee
        jsr     0x2352.l                        | +0f2
        move.w  #0x10e8,d0                      | +0f8
        jsr     0x2222.l                        | +0fc
        move.b  #0xff,0x20(a6)                  | +102
        jmp     0x518.l                         | +108
.L061202:
        rts                                     | +10e

| ----------------------------------------------------------------------------
|  AimTurret_Dormant_061204  @ $061204  (80 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Dormant_061204, "ax", @progbits
        .global AimTurret_Dormant_061204
AimTurret_Dormant_061204:
        move.w  #0x1ff,0x74(a6)                 | +000
        lea     .L061210(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L061210:
        jsr     AimTurret_AimAtPlayerB_0616ca(pc) | +00c
        jsr     AimTurret_HitCheck_061580(pc)   | +010
        jsr     AimTurret_Anim_0617d2(pc)       | +014
        jsr     AimTurret_Cooldown_0614b6(pc)   | +018
        jsr     0x28d70.l                       | +01c
        movea.l #0xffffffff,a0                  | +022
        lea     0x2c33a0.l,a0                   | +028
        jsr     0x5dd56.l                       | +02e
        bcc.w   .L061252                        | +034
        move.b  #0xff,0x20(a6)                  | +038
        move.w  #0x10e8,d0                      | +03e
        jsr     0x2222.l                        | +042
        jmp     0x518.l                         | +048
.L061252:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  AimTurret_Gun_061254  @ $061254  (84 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Gun_061254, "ax", @progbits
        .global AimTurret_Gun_061254
AimTurret_Gun_061254:
        move.w  #0x189,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x18b,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0x0,0x46(a6)                   | +014
        move.b  #0x0,0x3b(a6)                   | +01a
        lea     .L06127a(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L06127a:
        jsr     0x5e4ee.l                       | +026
        movea.l 0xc(a6),a0                      | +02c
        move.b  0x3a(a0),0x3a(a6)               | +030
        jsr     AimTurret_GunOffset_0615f6(pc)  | +036
        jsr     AimTurret_AnimScript_0614e6(pc) | +03a
        movea.l 0xc(a6),a0                      | +03e
        cmpi.b  #0xff,0x20(a0)                  | +042
        bne.w   .L0612a6                        | +048
        jmp     0x518.l                         | +04c
.L0612a6:
        rts                                     | +052

| ----------------------------------------------------------------------------
|  AimTurret_GunAttack_0612a8  @ $0612A8  (62 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_GunAttack_0612a8, "ax", @progbits
        .global AimTurret_GunAttack_0612a8
AimTurret_GunAttack_0612a8:
        move.w  #0x4,0x86(a6)                   | +000
        lea     .L0612b4(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0612b4:
        jsr     0x2783a.l                       | +00c
        lea     0x2c3282.l,a0                   | +012
        move.l  a0,0x4c(a6)                     | +018
        jsr     0x283ca.l                       | +01c
        jsr     0x283ca.l                       | +022
        jsr     0x283d8.l                       | +028
        subq.w  #0x1,0x86(a6)                   | +02e
        bpl.w   .L0612e4                        | +032
        jmp     0x518.l                         | +036
.L0612e4:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  AimTurret_Barrel_0612e6  @ $0612E6  (94 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Barrel_0612e6, "ax", @progbits
        .global AimTurret_Barrel_0612e6
AimTurret_Barrel_0612e6:
        move.w  #0x189,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x5e4ee.l                       | +00a
        subq.w  #0x1,0x38(a6)                   | +010
        movea.l 0xc(a6),a0                      | +014
        move.b  0x3a(a0),0x3a(a6)               | +018
        lea     0x2c2d86.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L061316(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L061316:
        jsr     0x2783a.l                       | +030
        jsr     0x28d70.l                       | +036
        bcs.w   .L06133c                        | +03c
        movea.l #0xffffffff,a0                  | +040
        lea     0x2c33aa.l,a0                   | +046
        jsr     0x5dd56.l                       | +04c
        bcc.w   .L061342                        | +052
.L06133c:
        jmp     0x518.l                         | +056
.L061342:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  AimTurret_Shell_061344  @ $061344  (270 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Shell_061344, "ax", @progbits
        .global AimTurret_Shell_061344
AimTurret_Shell_061344:
        move.w  #0x109b,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  #0x187,d1                       | +00a
        jsr     0x236e.l                        | +00e
        lea     0x2c2df8.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        jsr     0x267e2.l                       | +020
        lea     0x2bdf94.l,a0                   | +026
        jsr     0x799de.l                       | +02c
        move.w  d0,d0                           | +032
        neg.w   d0                              | +034
        move.w  d0,0x2e(a6)                     | +036
        movea.l 0xc(a6),a0                      | +03a
        move.w  0x28(a0),0x28(a6)               | +03e
        move.w  0x22(a0),0x22(a6)               | +044
        move.w  0x24(a0),0x24(a6)               | +04a
        subi.w  #0x10,0x24(a6)                  | +050
        subq.w  #0x1,0x38(a6)                   | +056
        move.w  #0x3c,0x66(a6)                  | +05a
        lea     0x2c334c.l,a0                   | +060
        move.l  a0,0x48(a6)                     | +066
        lea     0x2c322e.l,a0                   | +06a
        move.l  a0,0x4c(a6)                     | +070
        jsr     0x283ca.l                       | +074
        lea     .L0613c4(pc),a1                 | +07a
        move.l  a1,(a6)                         | +07e
.L0613c4:
        jsr     0x27d50.l                       | +080
        bcc.w   .L0613d4                        | +086
        lea     AimTurret_ShellBurst_061452(pc),a1 | +08a
        move.l  a1,(a6)                         | +08e
.L0613d4:
        jsr     0x28d70.l                       | +090
        movea.l 0xc(a6),a0                      | +096
        cmpi.b  #0xff,0x20(a0)                  | +09a
        bne.w   .L0613ee                        | +0a0
        lea     AimTurret_ShellBurst_061452(pc),a1 | +0a4
        move.l  a1,(a6)                         | +0a8
.L0613ee:
        jsr     0x2870a.l                       | +0aa
        bcc.w   .L061408                        | +0b0
        bclr    #0x3,0x13(a6)                   | +0b4
        move.w  #0x108d,d0                      | +0ba
        jsr     0x2352.l                        | +0be
.L061408:
        jsr     0x28758.l                       | +0c4
        bcs.w   .L061422                        | +0ca
        jsr     0x283d8.l                       | +0ce
        btst    #0x1,0x13(a6)                   | +0d4
        beq.w   .L061428                        | +0da
.L061422:
        lea     AimTurret_ShellBurst_061452(pc),a1 | +0de
        move.l  a1,(a6)                         | +0e2
.L061428:
        clr.w   d0                              | +0e4
        sub.w   0x28(a6),d0                     | +0e6
        asr.w   #0x4,d0                         | +0ea
        move.w  d0,0x2c(a6)                     | +0ec
        movea.l #0xffffffff,a0                  | +0f0
        lea     0x2c33aa.l,a0                   | +0f6
        jsr     0x5dd56.l                       | +0fc
        bcc.w   .L061450                        | +102
        jmp     0x518.l                         | +106
.L061450:
        rts                                     | +10c

| ----------------------------------------------------------------------------
|  AimTurret_ShellBurst_061452  @ $061452  (46 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_ShellBurst_061452, "ax", @progbits
        .global AimTurret_ShellBurst_061452
AimTurret_ShellBurst_061452:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x77f6a.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        lea     .L061474(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L061474:
        jsr     0x28d70.l                       | +022
        jmp     0x518.l                         | +028

| ----------------------------------------------------------------------------
|  AimTurret_Rts_061480  @ $061480  (2 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Rts_061480, "ax", @progbits
        .global AimTurret_Rts_061480
AimTurret_Rts_061480:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  AimTurret_DropItem_061482  @ $061482  (46 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_DropItem_061482, "ax", @progbits
        .global AimTurret_DropItem_061482
AimTurret_DropItem_061482:
        move.w  0x22(a6),d0                     | +000
        movem.w d0,-(a7)                        | +004
        move.w  #0x10,d0                        | +008
        btst    #0x0,0x3a(a6)                   | +00c
        beq.w   .L06149a                        | +012
        neg.w   d0                              | +016
.L06149a:
        add.w   d0,0x22(a6)                     | +018
        move.b  0x9d(a6),d0                     | +01c
        move.w  #0x9e,d1                        | +020
        jsr     0x9a7cc.l                       | +024
        movem.w (a7)+,d0                        | +02a

| ----------------------------------------------------------------------------
|  AimTurret_Cooldown_0614b6  @ $0614B6  (40 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Cooldown_0614b6, "ax", @progbits
        .global AimTurret_Cooldown_0614b6
AimTurret_Cooldown_0614b6:
        clr.l   d0                              | +000
        move.b  0x71(a6),d0                     | +002
        cmpi.b  #0x6,d0                         | +006
        bls.w   .L0614c8                        | +00a
        move.b  #0x0,d0                         | +00e
.L0614c8:
        movea.l #0x2c2cbc,a0                    | +012
        lsl.w   #0x2,d0                         | +018
        movea.l (a0,d0.w),a0                    | +01a
        cmpa.l  #0xffffffff,a0                  | +01e
        beq.w   JsrAbsRts_0614e4                | +024

| ----------------------------------------------------------------------------
|  AimTurret_AnimScript_0614e6  @ $0614E6  (146 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_AnimScript_0614e6, "ax", @progbits
        .global AimTurret_AnimScript_0614e6
AimTurret_AnimScript_0614e6:
        movea.l 0xc(a6),a0                      | +000
        clr.l   d0                              | +004
        move.b  0x71(a0),d0                     | +006
        cmpi.b  #0x6,d0                         | +00a
        bls.w   .L0614fc                        | +00e
        move.b  #0x0,d0                         | +012
.L0614fc:
        asl.l   #0x2,d0                         | +016
        lea     0x2c3212.l,a0                   | +018
        movea.l (a0,d0.w),a1                    | +01e
        move.l  a1,0x3c(a6)                     | +022
        subq.b  #0x1,0x46(a6)                   | +026
        cmpi.b  #0x0,0x46(a6)                   | +02a
        bgt.w   .L06154c                        | +030
        addq.b  #0x1,0x3b(a6)                   | +034
        movea.l 0x3c(a6),a0                     | +038
        clr.l   d0                              | +03c
        move.b  0x3b(a6),d0                     | +03e
        asl.l   #0x3,d0                         | +042
        movea.l (a0,d0.w),a1                    | +044
        move.l  a1,d5                           | +048
        cmpi.l  #0xffffffff,d5                  | +04a
        bne.w   .L061540                        | +050
        move.b  #0x0,0x3b(a6)                   | +054
.L061540:
        move.b  0x3b(a6),d0                     | +05a
        asl.l   #0x3,d0                         | +05e
        move.b  0x4(a0,d0.w),0x46(a6)           | +060
.L06154c:
        movea.l 0x3c(a6),a0                     | +066
        clr.l   d0                              | +06a
        move.b  0x3b(a6),d0                     | +06c
        asl.l   #0x3,d0                         | +070
        movea.l (a0,d0.w),a0                    | +072
        move.b  0x3a(a6),d5                     | +076
        andi.b  #0x3,d5                         | +07a
        move.l  0x22(a6),d0                     | +07e
        move.w  0x38(a6),d1                     | +082
        move.w  0x14(a6),d2                     | +086
        move.b  0x33(a6),d3                     | +08a
        move.b  0x32(a6),d4                     | +08e

| ----------------------------------------------------------------------------
|  AimTurret_HitCheck_061580  @ $061580  (118 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_HitCheck_061580, "ax", @progbits
        .global AimTurret_HitCheck_061580
AimTurret_HitCheck_061580:
        addq.b  #0x1,0x70(a6)                   | +000
        move.b  0x70(a6),d0                     | +004
        cmpi.b  #0x2,d0                         | +008
        bne.w   .L0615a8                        | +00c
        move.w  0x18(a6),d0                     | +010
        move.w  0x16(a6),0x18(a6)               | +014
        move.w  d0,0x16(a6)                     | +01a
        move.w  d0,0x14(a6)                     | +01e
        move.b  #0x0,0x70(a6)                   | +022
.L0615a8:
        move.b  0x7c(a6),d0                     | +028
        asl.b   #0x1,d0                         | +02c
        move.b  d0,0x7c(a6)                     | +02e
        jsr     0x2870a.l                       | +032
        bcc.w   .L0615d2                        | +038
        bset    #0x0,0x7c(a6)                   | +03c
        bclr    #0x3,0x13(a6)                   | +042
        move.w  #0x108d,d0                      | +048
        jsr     0x2352.l                        | +04c
.L0615d2:
        btst    #0x0,0x7c(a6)                   | +052
        beq.w   .L0615ee                        | +058
        btst    #0x1,0x7c(a6)                   | +05c
        bne.w   .L0615ee                        | +062
        move.w  0x1a(a6),0x14(a6)               | +066
        rts                                     | +06c
.L0615ee:
        move.w  0x16(a6),0x14(a6)               | +06e
        rts                                     | +074

| ----------------------------------------------------------------------------
|  AimTurret_GunOffset_0615f6  @ $0615F6  (40 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_GunOffset_0615f6, "ax", @progbits
        .global AimTurret_GunOffset_0615f6
AimTurret_GunOffset_0615f6:
        movea.l 0xc(a6),a0                      | +000
        btst    #0x0,0x7c(a0)                   | +004
        beq.w   .L061616                        | +00a
        btst    #0x1,0x7c(a0)                   | +00e
        bne.w   .L061616                        | +014
        move.w  0x18(a6),0x14(a6)               | +018
        rts                                     | +01e
.L061616:
        move.w  0x16(a6),0x14(a6)               | +020
        rts                                     | +026

| ----------------------------------------------------------------------------
|  AimTurret_SinCos40_06161e  @ $06161E  (38 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_SinCos40_06161e, "ax", @progbits
        .global AimTurret_SinCos40_06161e
AimTurret_SinCos40_06161e:
        subi.w  #0x4,0x34(a6)                   | +000
        andi.w  #0xff,0x34(a6)                  | +006
        move.w  0x34(a6),d0                     | +00c
        move.w  #0x40,d1                        | +010
        jsr     0x13c0e.l                       | +014
        asl.w   #0x1,d1                         | +01a
        move.w  #0x0,0x28(a6)                   | +01c
        move.w  d2,0x2a(a6)                     | +022

| ----------------------------------------------------------------------------
|  AimTurret_AimAtPlayer_06164c  @ $06164C  (118 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_AimAtPlayer_06164c, "ax", @progbits
        .global AimTurret_AimAtPlayer_06164c
AimTurret_AimAtPlayer_06164c:
        move.w  0x72(a6),d0                     | +000
        move.w  0x74(a6),d1                     | +004
        sub.w   0x22(a6),d0                     | +008
        sub.w   0x24(a6),d1                     | +00c
        asr.w   #0x1,d0                         | +010
        asr.w   #0x1,d1                         | +012
        jsr     0x5e018.l                       | +014
        clr.w   d1                              | +01a
        move.b  0x9c(a6),d1                     | +01c
        andi.w  #0x1f,d1                        | +020
        btst    #0x0,0x3a(a6)                   | +024
        beq.w   .L06167c                        | +02a
        neg.w   d1                              | +02e
.L06167c:
        add.w   d1,d0                           | +030
        andi.w  #0xff,d0                        | +032
        move.w  d0,0x34(a6)                     | +036
        move.w  0x72(a6),d2                     | +03a
        move.w  0x74(a6),d3                     | +03e
        move.w  0x22(a6),d0                     | +042
        move.w  0x24(a6),d1                     | +046
        jsr     0x5e23a.l                       | +04a
        asl.w   #0x3,d0                         | +050
        move.w  d0,d1                           | +052
        move.w  0x34(a6),d0                     | +054
        andi.w  #0xff,d0                        | +058
        jsr     0x13c0e.l                       | +05c
        sub.w   0x28(a6),d1                     | +062
        sub.w   0x2a(a6),d2                     | +066
        asr.w   #0x3,d1                         | +06a
        asr.w   #0x4,d2                         | +06c
        move.w  d1,0x2c(a6)                     | +06e
        move.w  d2,0x2e(a6)                     | +072

| ----------------------------------------------------------------------------
|  AimTurret_AimAtPlayerB_0616ca  @ $0616CA  (120 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_AimAtPlayerB_0616ca, "ax", @progbits
        .global AimTurret_AimAtPlayerB_0616ca
AimTurret_AimAtPlayerB_0616ca:
        move.w  0x72(a6),d0                     | +000
        move.w  0x74(a6),d1                     | +004
        sub.w   0x22(a6),d0                     | +008
        sub.w   0x24(a6),d1                     | +00c
        asr.w   #0x1,d0                         | +010
        asr.w   #0x1,d1                         | +012
        jsr     0x5e018.l                       | +014
        clr.w   d1                              | +01a
        move.b  0x9c(a6),d1                     | +01c
        andi.w  #0x1f,d1                        | +020
        btst    #0x0,0x3a(a6)                   | +024
        beq.w   .L0616fa                        | +02a
        neg.w   d1                              | +02e
.L0616fa:
        add.w   d1,d0                           | +030
        andi.w  #0xff,d0                        | +032
        move.w  d0,0x34(a6)                     | +036
        move.w  0x72(a6),d2                     | +03a
        move.w  0x74(a6),d3                     | +03e
        move.w  0x22(a6),d0                     | +042
        move.w  0x24(a6),d1                     | +046
        jsr     0x5e23a.l                       | +04a
        asl.w   #0x4,d0                         | +050
        move.w  d0,d1                           | +052
        move.w  0x34(a6),d0                     | +054
        andi.w  #0xff,d0                        | +058
        jsr     0x13c0e.l                       | +05c
        sub.w   0x28(a6),d1                     | +062
        sub.w   0x2a(a6),d2                     | +066
        asr.w   #0x4,d1                         | +06a
        asr.w   #0x4,d2                         | +06c
        move.w  d1,0x2c(a6)                     | +06e
        move.w  #0x10,0x2e(a6)                  | +072

| ----------------------------------------------------------------------------
|  AimTurret_StepAngle_06174a  @ $06174A  (30 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_StepAngle_06174a, "ax", @progbits
        .global AimTurret_StepAngle_06174a
AimTurret_StepAngle_06174a:
        move.w  0x72(a6),d2                     | +000
        move.w  0x74(a6),d3                     | +004
        move.w  0x22(a6),d0                     | +008
        move.w  0x24(a6),d1                     | +00c
        jsr     0x5e23a.l                       | +010
        cmpi.w  #0x8,d0                         | +016
        bgt.w   ClearXN_06176e                  | +01a

| ----------------------------------------------------------------------------
|  AimTurret_StepAngleB_061774  @ $061774  (30 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_StepAngleB_061774, "ax", @progbits
        .global AimTurret_StepAngleB_061774
AimTurret_StepAngleB_061774:
        move.w  0x72(a6),d2                     | +000
        move.w  0x74(a6),d3                     | +004
        move.w  0x22(a6),d0                     | +008
        move.w  0x24(a6),d1                     | +00c
        jsr     0x5e23a.l                       | +010
        cmpi.w  #0x10,d0                        | +016
        bgt.w   ClearXN_061798                  | +01a

| ----------------------------------------------------------------------------
|  AimTurret_Acquire_06179e  @ $06179E  (52 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Acquire_06179e, "ax", @progbits
        .global AimTurret_Acquire_06179e
AimTurret_Acquire_06179e:
        move.b  0x99(a6),d0                     | +000
        andi.b  #0xf,d0                         | +004
        move.w  #0x100,d1                       | +008
.L0617aa:
        cmpi.b  #0x0,d0                         | +00c
        ble.w   .L0617ba                        | +010
        addi.w  #0x10,d1                        | +014
        subq.b  #0x1,d0                         | +018
        bra.b   .L0617aa                        | +01a
.L0617ba:
        move.w  d1,0x74(a6)                     | +01c
        jsr     0x5e0d4.l                       | +020
        bcs.w   .L0617d0                        | +026
        move.w  0x22(a0),d1                     | +02a
        move.w  d1,0x72(a6)                     | +02e
.L0617d0:
        rts                                     | +032

| ----------------------------------------------------------------------------
|  AimTurret_Anim_0617d2  @ $0617D2  (222 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Anim_0617d2, "ax", @progbits
        .global AimTurret_Anim_0617d2
AimTurret_Anim_0617d2:
        move.w  0x28(a6),d0                     | +000
        btst    #0x0,0x3a(a6)                   | +004
        beq.w   .L061848                        | +00a
        cmpi.w  #0xff40,d0                      | +00e
        bge.w   .L0617f0                        | +012
        move.b  #0x6,0x71(a6)                   | +016
        rts                                     | +01c
.L0617f0:
        cmpi.w  #0xff80,d0                      | +01e
        bge.w   .L061800                        | +022
        move.b  #0x5,0x71(a6)                   | +026
        rts                                     | +02c
.L061800:
        cmpi.w  #0xffc0,d0                      | +02e
        bge.w   .L061810                        | +032
        move.b  #0x4,0x71(a6)                   | +036
        rts                                     | +03c
.L061810:
        cmpi.w  #0x40,d0                        | +03e
        bge.w   .L061820                        | +042
        move.b  #0x3,0x71(a6)                   | +046
        rts                                     | +04c
.L061820:
        cmpi.w  #0x80,d0                        | +04e
        bge.w   .L061830                        | +052
        move.b  #0x2,0x71(a6)                   | +056
        rts                                     | +05c
.L061830:
        cmpi.w  #0xc0,d0                        | +05e
        bge.w   .L061840                        | +062
        move.b  #0x1,0x71(a6)                   | +066
        rts                                     | +06c
.L061840:
        move.b  #0x0,0x71(a6)                   | +06e
        rts                                     | +074
.L061848:
        cmpi.w  #0xff40,d0                      | +076
        bge.w   .L061858                        | +07a
        move.b  #0x0,0x71(a6)                   | +07e
        rts                                     | +084
.L061858:
        cmpi.w  #0xff80,d0                      | +086
        bge.w   .L061868                        | +08a
        move.b  #0x1,0x71(a6)                   | +08e
        rts                                     | +094
.L061868:
        cmpi.w  #0xffc0,d0                      | +096
        bge.w   .L061878                        | +09a
        move.b  #0x2,0x71(a6)                   | +09e
        rts                                     | +0a4
.L061878:
        cmpi.w  #0x40,d0                        | +0a6
        bge.w   .L061888                        | +0aa
        move.b  #0x3,0x71(a6)                   | +0ae
        rts                                     | +0b4
.L061888:
        cmpi.w  #0x80,d0                        | +0b6
        bge.w   .L061898                        | +0ba
        move.b  #0x4,0x71(a6)                   | +0be
        rts                                     | +0c4
.L061898:
        cmpi.w  #0xc0,d0                        | +0c6
        bge.w   .L0618a8                        | +0ca
        move.b  #0x5,0x71(a6)                   | +0ce
        rts                                     | +0d4
.L0618a8:
        move.b  #0x6,0x71(a6)                   | +0d6
        rts                                     | +0dc

| ----------------------------------------------------------------------------
|  AimTurret_RngDifficulty_0618b0  @ $0618B0  (90 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_RngDifficulty_0618b0, "ax", @progbits
        .global AimTurret_RngDifficulty_0618b0
AimTurret_RngDifficulty_0618b0:
        cmpi.w  #0x0,0x78(a6)                   | +000
        bgt.w   AimTurret_RtsB_06194c           | +006
        cmpi.b  #0xff,0x7b(a6)                  | +00a
        bne.w   .L0618e4                        | +010
        lea     0x2bde90.l,a0                   | +014
        jsr     0x799de.l                       | +01a
        move.b  d0,0x7a(a6)                     | +020
        lea     0x2bdf12.l,a0                   | +024
        jsr     0x799de.l                       | +02a
        move.b  d0,0x7b(a6)                     | +030
.L0618e4:
        cmpi.b  #0x0,0x7a(a6)                   | +034
        bgt.w   AimTurret_Rts_061946            | +03a
        cmpi.b  #0x0,0x7b(a6)                   | +03e
        bne.w   AimTurret_SpawnPair_061910      | +044
        move.b  #0xff,0x7b(a6)                  | +048
        lea     0x2bdd8c.l,a0                   | +04e
        jsr     0x799de.l                       | +054

| ----------------------------------------------------------------------------
|  AimTurret_SpawnPair_061910  @ $061910  (54 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_SpawnPair_061910, "ax", @progbits
        .global AimTurret_SpawnPair_061910
AimTurret_SpawnPair_061910:
        lea     AimTurret_Shell_061344(pc),a1   | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        lea     AimTurret_Barrel_0612e6(pc),a1  | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        lea     0x2bde90.l,a0                   | +020
        jsr     0x799de.l                       | +026
        move.b  d0,0x7a(a6)                     | +02c
        subq.b  #0x1,0x7b(a6)                   | +030
        rts                                     | +034

| ----------------------------------------------------------------------------
|  AimTurret_Rts_061946  @ $061946  (6 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Rts_061946, "ax", @progbits
        .global AimTurret_Rts_061946
AimTurret_Rts_061946:
        subq.b  #0x1,0x7a(a6)                   | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  AimTurret_RtsB_06194c  @ $06194C  (6 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_RtsB_06194c, "ax", @progbits
        .global AimTurret_RtsB_06194c
AimTurret_RtsB_06194c:
        subq.w  #0x1,0x78(a6)                   | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  AimTurret_ScrollGate_061952  @ $061952  (36 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_ScrollGate_061952, "ax", @progbits
        .global AimTurret_ScrollGate_061952
AimTurret_ScrollGate_061952:
        clr.w   d0                              | +000
        move.b  0x9a(a6),d0                     | +002
        asl.w   #0x6,d0                         | +006
        andi.w  #0x7fff,d0                      | +008
        move.w  d0,0x80(a6)                     | +00c
        clr.w   d0                              | +010
        move.b  0x9b(a6),d0                     | +012
        asl.w   #0x4,d0                         | +016
        move.w  d0,0x82(a6)                     | +018
        move.l  0x106f50.l,d0                   | +01c
        swap    d0                              | +022

| ----------------------------------------------------------------------------
|  AimTurret_ScrollGateB_06197c  @ $06197C  (38 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_ScrollGateB_06197c, "ax", @progbits
        .global AimTurret_ScrollGateB_06197c
AimTurret_ScrollGateB_06197c:
        subi.w  #0x2,0x80(a6)                   | +000
        cmpi.w  #0x0,0x80(a6)                   | +006
        ble.w   SetXN_0619a8                    | +00c
        move.l  0x106f50.l,d0                   | +010
        swap    d0                              | +016
        move.w  0x84(a6),d1                     | +018
        sub.w   d1,d0                           | +01c
        cmp.w   0x82(a6),d0                     | +01e
        bgt.w   SetXN_0619a8                    | +022

| ----------------------------------------------------------------------------
|  AimTurret_Stub_0619ae  @ $0619AE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.AimTurret_Stub_0619ae, "ax", @progbits
        .global AimTurret_Stub_0619ae
AimTurret_Stub_0619ae:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0619c4                    | +00c

| ----------------------------------------------------------------------------
|  GroundNest_Tmpl32_0619ca  @ $0619CA  (216 B)
| ----------------------------------------------------------------------------
        .section .text.GroundNest_Tmpl32_0619ca, "ax", @progbits
        .global GroundNest_Tmpl32_0619ca
GroundNest_Tmpl32_0619ca:
        clr.b   0x83(a6)                        | +000
        clr.b   0x84(a6)                        | +004
        bra.w   .L0619ee                        | +008
        clr.b   0x83(a6)                        | +00c
        move.b  #0x1,0x84(a6)                   | +010
        bra.w   .L0619ee                        | +016
        move.b  #0x1,0x83(a6)                   | +01a
        clr.b   0x84(a6)                        | +020
.L0619ee:
        jsr     0x5e7c0.l                       | +024
        jsr     0x267e2.l                       | +02a
        move.w  #0xf2,d1                        | +030
        tst.b   0x9e(a6)                        | +034
        beq.w   .L061a0a                        | +038
        move.w  #0x1cc,d1                       | +03c
.L061a0a:
        jsr     0x236e.l                        | +040
        move.w  #0x2,0x1c(a6)                   | +046
        jsr     0x138fe.l                       | +04c
        move.w  #0x8000,d0                      | +052
        jsr     0x28134.l                       | +056
        andi.w  #0xffe3,0x38(a6)                | +05c
        ori.w   #0x8,0x38(a6)                   | +062
        lea     0x2b7a22.l,a0                   | +068
        jsr     0x799de.l                       | +06e
        move.w  d0,0x66(a6)                     | +074
        lea     0x2b7b26.l,a0                   | +078
        jsr     0x799de.l                       | +07e
        move.w  d0,0x74(a6)                     | +084
        move.l  #0x2c3458,0x48(a6)              | +088
        move.l  #0x2c3704,0x60(a6)              | +090
        move.w  #0xa0,0x72(a6)                  | +098
        clr.b   0x77(a6)                        | +09e
        lea     0x723d2.l,a1                    | +0a2
        jsr     0x4ae.l                         | +0a8
        jsr     0x5dd02.l                       | +0ae
        clr.w   0x98(a0)                        | +0b4
        lea     0x7773e.l,a1                    | +0b8
        jsr     0x4ae.l                         | +0be
        jsr     0x5dd02.l                       | +0c4
        bset    #0x2,0x6b(a6)                   | +0ca
        move.l  #0x2c3554,0x4c(a6)              | +0d0

| ----------------------------------------------------------------------------
|  LateProp_Init_061aa2  @ $061AA2  (200 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_Init_061aa2, "ax", @progbits
        .global LateProp_Init_061aa2
LateProp_Init_061aa2:
        tst.b   0x98(a6)                        | +000
        bne.w   LateProp_Hit_061bf2             | +004
        move.w  #0x109d,d0                      | +008
        jsr     0x2352.l                        | +00c
        jsr     0x267e2.l                       | +012
        jsr     Sub_000626D8(pc)                | +018  -> $0626D8 (hueco futuro, defsym forward)
        bcs.w   .L061adc                        | +01c
        lea     0x2c3812.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        move.w  #0xff00,d0                      | +02c
        move.l  #0x2c37ea,d1                    | +030
        bra.w   .L061af2                        | +036
.L061adc:
        lea     0x2c3880.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        move.w  #0x100,d0                       | +046
        move.l  #0x2c3858,d1                    | +04a
.L061af2:
        btst    #0x0,0x3a(a6)                   | +050
        beq.w   .L061afe                        | +056
        neg.w   d0                              | +05a
.L061afe:
        move.l  d1,0x7c(a6)                     | +05c
        move.w  d0,0x28(a6)                     | +060
        clr.w   0x78(a6)                        | +064
        jsr     Sub_00062732(pc)                | +068  -> $062732 (hueco futuro, defsym forward)
        lea     .L061b14(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L061b14:
        jsr     0x28998.l                       | +072
        jsr     0x27afc.l                       | +078
        bcc.w   .L061b2a                        | +07e
        lea     LateProp_Die_061c54(pc),a1      | +082
        move.l  a1,(a6)                         | +086
.L061b2a:
        jsr     0x28d70.l                       | +088
        jsr     Sub_000626F0(pc)                | +08e  -> $0626F0 (hueco futuro, defsym forward)
        bcc.w   .L061b3e                        | +092
        lea     LateProp_Rts_061c4e(pc),a1      | +096
        move.l  a1,(a6)                         | +09a
.L061b3e:
        jsr     Sub_00062710(pc)                | +09c  -> $062710 (hueco futuro, defsym forward)
        bcc.w   .L061b4c                        | +0a0
        lea     LateProp_Die_061c54(pc),a1      | +0a4
        move.l  a1,(a6)                         | +0a8
.L061b4c:
        jsr     Sub_00062758(pc)                | +0aa  -> $062758 (hueco futuro, defsym forward)
        bcc.w   .L061b5a                        | +0ae
        lea     LateProp_Die_061c54(pc),a1      | +0b2
        move.l  a1,(a6)                         | +0b6
.L061b5a:
        jsr     0x283ca.l                       | +0b8
        jsr     0x283d8.l                       | +0be
        bra.w   Sub_00062014                    | +0c4  -> $062014 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  LateProp_Idle_061b6a  @ $061B6A  (136 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_Idle_061b6a, "ax", @progbits
        .global LateProp_Idle_061b6a
LateProp_Idle_061b6a:
        move.w  #0x109d,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2b7cac.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.w  d0,0x80(a6)                     | +016
        lea     0x2b7d2e.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        andi.w  #0x3,d0                         | +026
        lsl.w   #0x2,d0                         | +02a
        movea.l #0x2c38c6,a0                    | +02c
        move.l  (a0,d0.w),0x5c(a6)              | +032
        lea     0x2c390a.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        lea     .L061bb4(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L061bb4:
        jsr     0x28998.l                       | +04a
        jsr     0x2783a.l                       | +050
        jsr     0x28d70.l                       | +056
        bcc.w   .L061bd0                        | +05c
        lea     LateProp_Hit_061bf2(pc),a1      | +060
        move.l  a1,(a6)                         | +064
.L061bd0:
        bclr    #0x3,0x13(a6)                   | +066
        lea     0x5e766.l,a0                    | +06c
        jsr     0x5e770.l                       | +072
        jsr     0x283ca.l                       | +078
        jsr     0x283d8.l                       | +07e
        bra.w   Sub_00062046                    | +084  -> $062046 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  LateProp_Hit_061bf2  @ $061BF2  (92 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_Hit_061bf2, "ax", @progbits
        .global LateProp_Hit_061bf2
LateProp_Hit_061bf2:
        lea     0x2c399c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L061c04(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L061c04:
        jsr     0x28998.l                       | +012
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L061c2e                        | +024
        lea     LateProp_Piece_061d04(pc),a1    | +028
        move.l  a1,(a6)                         | +02c
        tst.b   0x98(a6)                        | +02e
        beq.w   .L061c2e                        | +032
        lea     LateProp_Hit_061bf2(pc),a1      | +036
        move.l  a1,(a6)                         | +03a
.L061c2e:
        jsr     Sub_00062710(pc)                | +03c  -> $062710 (hueco futuro, defsym forward)
        bcc.w   .L061c3c                        | +040
        lea     LateProp_PieceIdle_061d84(pc),a1 | +044
        move.l  a1,(a6)                         | +048
.L061c3c:
        jsr     Sub_00062758(pc)                | +04a  -> $062758 (hueco futuro, defsym forward)
        bcc.w   .L061c4a                        | +04e
        lea     LateProp_Idle_061b6a(pc),a1     | +052
        move.l  a1,(a6)                         | +056
.L061c4a:
        bra.w   Sub_00062014                    | +058  -> $062014 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  LateProp_Rts_061c4e  @ $061C4E  (6 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_Rts_061c4e, "ax", @progbits
        .global LateProp_Rts_061c4e
LateProp_Rts_061c4e:
        eori.b  #0x1,0x77(a6)                   | +000

| ----------------------------------------------------------------------------
|  LateProp_Die_061c54  @ $061C54  (176 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_Die_061c54, "ax", @progbits
        .global LateProp_Die_061c54
LateProp_Die_061c54:
        move.w  0x28(a6),d0                     | +000
        btst    #0x0,0x3a(a6)                   | +004
        beq.w   .L061c64                        | +00a
        neg.w   d0                              | +00e
.L061c64:
        cmpi.w  #0x0,d0                         | +010
        bgt.w   .L061c82                        | +014
        lea     0x2c39da.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        move.l  #0x2c37ea,d0                    | +024
        bra.w   .L061c94                        | +02a
.L061c82:
        lea     0x2c3a22.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        move.l  #0x2c3858,d0                    | +03a
.L061c94:
        move.l  d0,0x7c(a6)                     | +040
        move.w  0x28(a6),d0                     | +044
        asr.w   #0x5,d0                         | +048
        clr.w   0x78(a6)                        | +04a
        neg.w   d0                              | +04e
        move.w  d0,0x2c(a6)                     | +050
        move.w  #0x109d,d0                      | +054
        jsr     0x2222.l                        | +058
        lea     .L061cb8(pc),a1                 | +05e
        move.l  a1,(a6)                         | +062
.L061cb8:
        jsr     0x28998.l                       | +064
        jsr     0x27afc.l                       | +06a
        tst.w   0x2c(a6)                        | +070
        beq.w   .L061ce8                        | +074
        tst.w   0x28(a6)                        | +078
        bne.w   .L061ce8                        | +07c
        move.w  0x2c(a6),d0                     | +080
        clr.w   0x2c(a6)                        | +084
        cmpi.w  #0x0,d0                         | +088
        blt.w   .L061ce8                        | +08c
        bra.w   .L061ce8                        | +090
.L061ce8:
        jsr     Sub_00062732(pc)                | +094  -> $062732 (hueco futuro, defsym forward)
        jsr     0x28d70.l                       | +098
        bcc.w   .L061cfc                        | +09e
        lea     LateProp_Hit_061bf2(pc),a1      | +0a2
        move.l  a1,(a6)                         | +0a6
.L061cfc:
        jsr     Sub_00062710(pc)                | +0a8  -> $062710 (hueco futuro, defsym forward)
        bra.w   Sub_00062014                    | +0ac  -> $062014 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  LateProp_Piece_061d04  @ $061D04  (128 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_Piece_061d04, "ax", @progbits
        .global LateProp_Piece_061d04
LateProp_Piece_061d04:
        jsr     Sub_00062710(pc)                | +000  -> $062710 (hueco futuro, defsym forward)
        bcs.w   LateProp_PieceIdle_061d84       | +004
        tst.b   0x98(a6)                        | +008
        bne.w   LateProp_Hit_061bf2             | +00c
        jsr     Sub_000626D8(pc)                | +010  -> $0626D8 (hueco futuro, defsym forward)
        bcs.w   .L061d30                        | +014
        lea     0x2c3a6a.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        move.w  #0xff00,d0                      | +024
        bra.w   .L061d40                        | +028
.L061d30:
        lea     0x2c3aec.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        move.w  #0x100,d0                       | +038
.L061d40:
        btst    #0x0,0x3a(a6)                   | +03c
        beq.w   .L061d4c                        | +042
        neg.w   d0                              | +046
.L061d4c:
        asr.w   #0x5,d0                         | +048
        move.w  d0,0x2c(a6)                     | +04a
        lea     .L061d58(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L061d58:
        jsr     0x28998.l                       | +054
        jsr     0x27afc.l                       | +05a
        jsr     0x28d70.l                       | +060
        bcc.w   .L061d74                        | +066
        lea     LateProp_Init_061aa2(pc),a1     | +06a
        move.l  a1,(a6)                         | +06e
.L061d74:
        jsr     0x283ca.l                       | +070
        jsr     0x283d8.l                       | +076
        bra.w   Sub_00062014                    | +07c  -> $062014 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  LateProp_PieceIdle_061d84  @ $061D84  (50 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_PieceIdle_061d84, "ax", @progbits
        .global LateProp_PieceIdle_061d84
LateProp_PieceIdle_061d84:
        lea     0x2c3b6e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L061d96(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L061d96:
        jsr     0x28998.l                       | +012
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L061db2                        | +024
        lea     LateProp_RngHP_061db6(pc),a1    | +028
        move.l  a1,(a6)                         | +02c
.L061db2:
        bra.w   Sub_00062084                    | +02e  -> $062084 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  LateProp_RngHP_061db6  @ $061DB6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_RngHP_061db6, "ax", @progbits
        .global LateProp_RngHP_061db6
LateProp_RngHP_061db6:
        lea     0x2b7ba8.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x76(a6)                     | +00c

| ----------------------------------------------------------------------------
|  LateProp_Debris_061dc6  @ $061DC6  (172 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_Debris_061dc6, "ax", @progbits
        .global LateProp_Debris_061dc6
LateProp_Debris_061dc6:
        lea     0x2b7c2a.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x74(a6)                     | +00c
        cmpi.b  #0x1,0x76(a6)                   | +010
        ble.w   .L061dfa                        | +016
        cmpi.w  #0x3c,0x74(a6)                  | +01a
        bgt.w   .L061dfa                        | +020
        lea     0x2c3c74.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        bra.w   .L061e06                        | +030
.L061dfa:
        lea     0x2c3be0.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
.L061e06:
        lea     .L061e0c(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L061e0c:
        jsr     0x28998.l                       | +046
        jsr     0x2783a.l                       | +04c
        jsr     0x28d70.l                       | +052
        bcc.w   .L061e42                        | +058
        cmpi.b  #0x0,0x76(a6)                   | +05c
        bgt.w   .L061e42                        | +062
        lea     LateProp_Hit_061bf2(pc),a1      | +066
        move.l  a1,(a6)                         | +06a
        lea     0x2b7b26.l,a0                   | +06c
        jsr     0x799de.l                       | +072
        move.w  d0,0x74(a6)                     | +078
.L061e42:
        cmpi.b  #0x0,0x76(a6)                   | +07c
        ble.w   .L061e6e                        | +082
        subq.w  #0x1,0x74(a6)                   | +086
        cmpi.w  #0x0,0x74(a6)                   | +08a
        bgt.w   .L061e6e                        | +090
        subq.b  #0x1,0x76(a6)                   | +094
        cmpi.b  #0x0,0x76(a6)                   | +098
        ble.w   .L061e6e                        | +09e
        lea     LateProp_Debris_061dc6(pc),a1   | +0a2
        move.l  a1,(a6)                         | +0a6
.L061e6e:
        bra.w   Sub_00062084                    | +0a8  -> $062084 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  LateProp_Spark_061e72  @ $061E72  (76 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_Spark_061e72, "ax", @progbits
        .global LateProp_Spark_061e72
LateProp_Spark_061e72:
        move.w  #0x109d,d0                      | +000
        jsr     0x2222.l                        | +004
        bclr    #0x3,0x13(a6)                   | +00a
        jsr     0x267e2.l                       | +010
        lea     0x2c3d7a.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L061e9a(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L061e9a:
        jsr     0x28998.l                       | +028
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        bcc.w   .L061eb6                        | +03a
        lea     LateProp_Piece_061d04(pc),a1    | +03e
        move.l  a1,(a6)                         | +042
.L061eb6:
        jsr     Sub_00062710(pc)                | +044  -> $062710 (hueco futuro, defsym forward)
        bra.w   Sub_00062014                    | +048  -> $062014 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  LateProp_SparkHit_061ebe  @ $061EBE  (104 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_SparkHit_061ebe, "ax", @progbits
        .global LateProp_SparkHit_061ebe
LateProp_SparkHit_061ebe:
        bclr    #0x3,0x13(a6)                   | +000
        jsr     0x267e2.l                       | +006
        move.w  #0x109d,d0                      | +00c
        jsr     0x2222.l                        | +010
        lea     0x2c3cec.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L061ee6(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L061ee6:
        jsr     0x28998.l                       | +028
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        bcc.w   .L061f02                        | +03a
        lea     LateProp_Piece_061d04(pc),a1    | +03e
        move.l  a1,(a6)                         | +042
.L061f02:
        jsr     Sub_00062710(pc)                | +044  -> $062710 (hueco futuro, defsym forward)
        jsr     0x2870a.l                       | +048
        bcc.w   .L061f22                        | +04e
        lea     0x5e766.l,a0                    | +052
        jsr     0x5e770.l                       | +058
        bclr    #0x3,0x13(a6)                   | +05e
.L061f22:
        bra.w   Sub_00062046                    | +064  -> $062046 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  LateProp_Shot_061f26  @ $061F26  (112 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_Shot_061f26, "ax", @progbits
        .global LateProp_Shot_061f26
LateProp_Shot_061f26:
        jsr     0x267e2.l                       | +000
        lea     0x77fd6.l,a1                    | +006
        jsr     0x4ae.l                         | +00c
        jsr     0x5dd02.l                       | +012
        move.w  #0x18,d0                        | +018
        jsr     Sub_000626B8(pc)                | +01c  -> $0626B8 (hueco futuro, defsym forward)
        add.w   d0,0x22(a0)                     | +020
        move.b  #0xff,0x20(a6)                  | +024
        lea     0x2c3da4.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L061f62(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L061f62:
        jsr     0x2783a.l                       | +03c
        jsr     0x28d70.l                       | +042
        bcc.w   .L061f80                        | +048
        tst.b   0x20(a6)                        | +04c
        bne.w   .L061f80                        | +050
        lea     LateProp_ShotB_061f9e(pc),a1    | +054
        move.l  a1,(a6)                         | +058
.L061f80:
        movea.l #0xffffffff,a0                  | +05a
        lea     0x2c36f4.l,a0                   | +060
        jsr     0x5dd5c.l                       | +066
        bcc.w   SetHandlerRts_061f9c            | +06c

| ----------------------------------------------------------------------------
|  LateProp_ShotB_061f9e  @ $061F9E  (98 B)
| ----------------------------------------------------------------------------
        .section .text.LateProp_ShotB_061f9e, "ax", @progbits
        .global LateProp_ShotB_061f9e
LateProp_ShotB_061f9e:
        lea     0x2c3e48.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L061fba                        | +00c
        lea     0x2c3e9c.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
.L061fba:
        bclr    #0x1,0x12(a6)                   | +01c
        jsr     0x267e2.l                       | +022
        lea     .L061fcc(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L061fcc:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        bcc.w   .L061fea                        | +03a
        tst.b   0x83(a6)                        | +03e
        beq.w   .L061fea                        | +042
        lea     Sub_00062008(pc),a1             | +046  -> $062008 (hueco futuro, defsym forward)
        move.l  a1,(a6)                         | +04a
.L061fea:
        movea.l #0xffffffff,a0                  | +04c
        lea     0x2c36f4.l,a0                   | +052
        jsr     0x5dd5c.l                       | +058
        bcc.w   SetHandlerRts_062006            | +05e
