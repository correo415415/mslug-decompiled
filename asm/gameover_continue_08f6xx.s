| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave PPP — Pantalla de Game Over / Continue, anclas de slot de jugador
|  Región: $08F6D2..$0916B8  (7,678 B, 92 entradas, 50 huecos)
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
|  PlayerSlot_MaskF0F0_08f6d2  @ $08F6D2  (8 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_MaskF0F0_08f6d2, "ax", @progbits
        .global PlayerSlot_MaskF0F0_08f6d2
PlayerSlot_MaskF0F0_08f6d2:
        move.w  #0xf0f0,0x96(a6)                | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  PlayerSlot_MaskCurF0_08f6da  @ $08F6DA  (24 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_MaskCurF0_08f6da, "ax", @progbits
        .global PlayerSlot_MaskCurF0_08f6da
PlayerSlot_MaskCurF0_08f6da:
        move.b  0x106f28.l,d0                   | +000
        not.b   d0                              | +006
        andi.w  #0x1,d0                         | +008
        addi.w  #0x96,d0                        | +00c
        move.b  #0xf0,(a6,d0.w)                 | +010
        rts                                     | +016

| ----------------------------------------------------------------------------
|  PlayerSlot_SetLowNibble_08f6f2  @ $08F6F2  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_SetLowNibble_08f6f2, "ax", @progbits
        .global PlayerSlot_SetLowNibble_08f6f2
PlayerSlot_SetLowNibble_08f6f2:
        move.b  0x106f28.l,d0                   | +000
        not.b   d0                              | +006
        andi.w  #0x1,d0                         | +008
        addi.w  #0x96,d0                        | +00c
        lsl.b   #0x4,d1                         | +010
        move.b  (a6,d0.w),d2                    | +012
        andi.b  #0xf0,d2                        | +016
        or.b    d2,d1                           | +01a
        or.b    d1,(a6,d0.w)                    | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  PlayerSlot_TestMaskCur_08f714  @ $08F714  (24 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_TestMaskCur_08f714, "ax", @progbits
        .global PlayerSlot_TestMaskCur_08f714
PlayerSlot_TestMaskCur_08f714:
        move.b  0x106f28.l,d0                   | +000
        andi.w  #0x1,d0                         | +006
        addi.w  #0x96,d0                        | +00a
        move.b  (a6,d0.w),d0                    | +00e
        and.b   d0,d1                           | +012
        beq.w   ClearXN_08f732                  | +014

| ----------------------------------------------------------------------------
|  Anchor_GetWorldPos_08f738  @ $08F738  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Anchor_GetWorldPos_08f738, "ax", @progbits
        .global Anchor_GetWorldPos_08f738
Anchor_GetWorldPos_08f738:
        move.l  0x70(a0),d1                     | +000
        bmi.w   ClearC_08f774                   | +004
        movea.l d1,a1                           | +008
        movea.l (a1),a2                         | +00a
        andi.w  #0xff,d0                        | +00c
        add.w   d0,d0                           | +010
        add.w   d0,d0                           | +012
        lea     (a2,d0.w),a2                    | +014
        move.w  (a2),d0                         | +018
        move.w  0x2(a2),d1                      | +01a
        move.w  d0,d2                           | +01e
        or.w    d1,d2                           | +020
        beq.w   ClearC_08f774                   | +022
        add.w   0x22(a0),d0                     | +026
        add.w   0x24(a0),d1                     | +02a
        add.w   0x4(a1),d0                      | +02e
        add.w   0x6(a1),d1                      | +032

| ----------------------------------------------------------------------------
|  Anchor_GetWorldPosOrDefault_08f77a  @ $08F77A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Anchor_GetWorldPosOrDefault_08f77a, "ax", @progbits
        .global Anchor_GetWorldPosOrDefault_08f77a
Anchor_GetWorldPosOrDefault_08f77a:
        move.b  d0,d7                           | +000
        bsr.b   Anchor_GetWorldPos_08f738       | +002
        bcc.w   SetCMid_08f794                  | +004
        tst.b   d7                              | +008
        bne.w   SetC_08f790                     | +00a
        addi.w  #0x10,d0                        | +00e
        addi.w  #0xe,d1                         | +012

| ----------------------------------------------------------------------------
|  PlayerSlot_FoldMask_08f79c  @ $08F79C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_FoldMask_08f79c, "ax", @progbits
        .global PlayerSlot_FoldMask_08f79c
PlayerSlot_FoldMask_08f79c:
        move.b  0x96(a0),d0                     | +000
        or.b    0x97(a0),d0                     | +004
        move.b  d0,d1                           | +008
        lsr.b   #0x4,d1                         | +00a
        or.b    d1,d0                           | +00c
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  PlayerSlot_Resolve_08f7ac  @ $08F7AC  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_Resolve_08f7ac, "ax", @progbits
        .global PlayerSlot_Resolve_08f7ac
PlayerSlot_Resolve_08f7ac:
        move.l  a0,d0                           | +000
        beq.w   .L08f7cc                        | +002
        jsr     0x2ac0e.l                       | +006
        bcc.w   .L08f7c6                        | +00c
        lea     0x100580.l,a0                   | +010
        bra.w   .L08f7cc                        | +016
.L08f7c6:
        movea.l #0x0,a0                         | +01a
.L08f7cc:
        rts                                     | +020

| ----------------------------------------------------------------------------
|  PlayerSlot_FindFreeAnchor_08f7ce  @ $08F7CE  (88 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_FindFreeAnchor_08f7ce, "ax", @progbits
        .global PlayerSlot_FindFreeAnchor_08f7ce
PlayerSlot_FindFreeAnchor_08f7ce:
        bsr.b   PlayerSlot_Resolve_08f7ac       | +000
        move.l  a0,d0                           | +002
        beq.w   RetMinus1_0008F826              | +004
        bsr.b   PlayerSlot_FoldMask_08f79c      | +008
        move.w  d0,d7                           | +00a
        btst    #0x0,d7                         | +00c
        bne.w   .L08f7f2                        | +010
        move.b  #0x0,d0                         | +014
        bsr.w   Anchor_GetWorldPos_08f738       | +018
        bcs.w   .L08f7f2                        | +01c
        bset    #0x0,d7                         | +020
.L08f7f2:
        btst    #0x1,d7                         | +024
        bne.w   .L08f80a                        | +028
        move.b  #0x1,d0                         | +02c
        bsr.w   Anchor_GetWorldPos_08f738       | +030
        bcs.w   .L08f80a                        | +034
        bset    #0x1,d7                         | +038
.L08f80a:
        btst    #0x2,d7                         | +03c
        bne.w   .L08f822                        | +040
        move.b  #0x2,d0                         | +044
        bsr.w   Anchor_GetWorldPos_08f738       | +048
        bcs.w   .L08f822                        | +04c
        bset    #0x2,d7                         | +050
.L08f822:
        move.b  d7,d0                           | +054
        rts                                     | +056

| ----------------------------------------------------------------------------
|  PlayerSlot_TryAnchor_08f82a  @ $08F82A  (44 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_TryAnchor_08f82a, "ax", @progbits
        .global PlayerSlot_TryAnchor_08f82a
PlayerSlot_TryAnchor_08f82a:
        movem.w d0,-(a7)                        | +000
        bsr.w   PlayerSlot_Resolve_08f7ac       | +004
        move.l  a0,d0                           | +008
        bne.w   .L08f840                        | +00a
        movem.w (a7)+,d0                        | +00e
        bra.w   ClearC_08f856                   | +012
.L08f840:
        bsr.w   PlayerSlot_FoldMask_08f79c      | +016
        movem.w (a7)+,d1                        | +01a
        btst    d1,d0                           | +01e
        bne.w   ClearC_08f856                   | +020
        move.b  d1,d0                           | +024
        bsr.w   Anchor_GetWorldPosOrDefault_08f77a | +026
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  PlayerSlot_ClaimAnchor_08f85c  @ $08F85C  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_ClaimAnchor_08f85c, "ax", @progbits
        .global PlayerSlot_ClaimAnchor_08f85c
PlayerSlot_ClaimAnchor_08f85c:
        movem.w d0,-(a7)                        | +000
        bsr.b   PlayerSlot_TryAnchor_08f82a     | +004
        movem.w (a7)+,d2                        | +006
        bcc.w   SetCMid_08f882                  | +00a
        move.b  0x106f28.l,d3                   | +00e
        not.b   d3                              | +014
        andi.w  #0x1,d3                         | +016
        addi.w  #0x96,d3                        | +01a
        bset    d2,(a0,d3.w)                    | +01e

| ----------------------------------------------------------------------------
|  PlayerSlot_ClaimAnchorIfFree_08f884  @ $08F884  (56 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_ClaimAnchorIfFree_08f884, "ax", @progbits
        .global PlayerSlot_ClaimAnchorIfFree_08f884
PlayerSlot_ClaimAnchorIfFree_08f884:
        move.w  d0,-(a7)                        | +000
        bsr.w   PlayerSlot_Resolve_08f7ac       | +002
        move.w  (a7)+,d0                        | +006
        move.l  a0,d7                           | +008
        beq.w   ClearC_08f8bc                   | +00a
        move.b  0x96(a0),d2                     | +00e
        or.b    0x97(a0),d2                     | +012
        lsr.b   #0x4,d2                         | +016
        btst    d0,d2                           | +018
        bne.w   ClearC_08f8bc                   | +01a
        move.b  0x106f28.l,d3                   | +01e
        not.b   d3                              | +024
        andi.w  #0x1,d3                         | +026
        addi.w  #0x96,d3                        | +02a
        bset    d0,(a0,d3.w)                    | +02e
        bsr.w   Anchor_GetWorldPosOrDefault_08f77a | +032
        rts                                     | +036

| ----------------------------------------------------------------------------
|  PlayerSlot_SetBit3Cur_08f8c2  @ $08F8C2  (32 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_SetBit3Cur_08f8c2, "ax", @progbits
        .global PlayerSlot_SetBit3Cur_08f8c2
PlayerSlot_SetBit3Cur_08f8c2:
        bsr.w   PlayerSlot_Resolve_08f7ac       | +000
        move.l  a0,d0                           | +004
        beq.b   ClearC_08f8bc                   | +006
        move.b  0x106f28.l,d0                   | +008
        not.b   d0                              | +00e
        andi.w  #0x1,d0                         | +010
        addi.w  #0x96,d0                        | +014
        bset    #0x3,(a0,d0.w)                  | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  PlayerSlot_ClearP3Cur_08f8e2  @ $08F8E2  (28 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlot_ClearP3Cur_08f8e2, "ax", @progbits
        .global PlayerSlot_ClearP3Cur_08f8e2
PlayerSlot_ClearP3Cur_08f8e2:
        move.b  0x106f28.l,d0                   | +000
        not.b   d0                              | +006
        andi.w  #0x1,d0                         | +008
        addi.w  #0x96,d0                        | +00c
        lea     0x100580.l,a0                   | +010
        clr.b   (a0,d0.w)                       | +016
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_08f8fe  @ $08F8FE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_08f8fe, "ax", @progbits
        .global Entity_CmpPrioWithSibling_08f8fe
Entity_CmpPrioWithSibling_08f8fe:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_08f914                    | +00c

| ----------------------------------------------------------------------------
|  GameOver_Boot_08f91a  @ $08F91A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_Boot_08f91a, "ax", @progbits
        .global GameOver_Boot_08f91a
GameOver_Boot_08f91a:
        move.b  #0xa,d0                         | +000
        jsr     0x43568.l                       | +004
        lea     0x2f4a32.l,a0                   | +00a
        jsr     0x2b58.l                        | +010
        move.w  #0x90,0x22(a6)                  | +016
        move.w  #0x1c0,0x24(a6)                 | +01c
        move.w  #0xff00,0x2a(a6)                | +022
        moveq   #0,d0                           | +028
        move.b  d0,0x20(a6)                     | +02a
        move.b  d0,0x21(a6)                     | +02e
        move.w  #0xe,0x82(a6)                   | +032
        move.w  d0,0x86(a6)                     | +038
        move.w  d0,0x84(a6)                     | +03c
        moveq   #2,d0                           | +040
        jsr     0x523b2.l                       | +042

| ----------------------------------------------------------------------------
|  GameOver_Spawn_08f96a  @ $08F96A  (824 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_Spawn_08f96a, "ax", @progbits
        .global GameOver_Spawn_08f96a
GameOver_Spawn_08f96a:
        lea     Continue_Text_Init_091514(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        lea     GO_Figure_A_090db0(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        lea     GO_Figure_B_090de2(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        lea     GO_Scroller_A_090f00(pc),a1     | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        move.w  #0x0,0x80(a0)                   | +03a
        lea     GO_Scroller_B_090f68(pc),a1     | +040
        jsr     0x4ae.l                         | +044
        jsr     0x5dd02.l                       | +04a
        move.w  #0x0,0x80(a0)                   | +050
        lea     GO_Scroller_A_090f00(pc),a1     | +056
        jsr     0x4ae.l                         | +05a
        jsr     0x5dd02.l                       | +060
        move.w  #0x1,0x80(a0)                   | +066
        lea     GO_Figure_Part_090e86(pc),a1    | +06c
        jsr     0x4ae.l                         | +070
        jsr     0x5dd02.l                       | +076
        move.w  #0x0,0x80(a0)                   | +07c
        lea     GO_Figure_Part_090e86(pc),a1    | +082
        jsr     0x4ae.l                         | +086
        jsr     0x5dd02.l                       | +08c
        move.w  #0x1,0x80(a0)                   | +092
        lea     GO_Figure_Part_090e86(pc),a1    | +098
        jsr     0x4ae.l                         | +09c
        jsr     0x5dd02.l                       | +0a2
        move.w  #0x2,0x80(a0)                   | +0a8
        lea     GO_Figure_Part_090e86(pc),a1    | +0ae
        jsr     0x4ae.l                         | +0b2
        jsr     0x5dd02.l                       | +0b8
        move.w  #0x3,0x80(a0)                   | +0be
        lea     GO_Figure_Part_090e86(pc),a1    | +0c4
        jsr     0x4ae.l                         | +0c8
        jsr     0x5dd02.l                       | +0ce
        move.w  #0x4,0x80(a0)                   | +0d4
        lea     GO_Figure_Part_090e86(pc),a1    | +0da
        jsr     0x4ae.l                         | +0de
        jsr     0x5dd02.l                       | +0e4
        move.w  #0x5,0x80(a0)                   | +0ea
        lea     GO_Figure_Part_090e86(pc),a1    | +0f0
        jsr     0x4ae.l                         | +0f4
        jsr     0x5dd02.l                       | +0fa
        move.w  #0x6,0x80(a0)                   | +100
        lea     GO_Figure_Part_090e86(pc),a1    | +106
        jsr     0x4ae.l                         | +10a
        jsr     0x5dd02.l                       | +110
        move.w  #0x7,0x80(a0)                   | +116
        lea     GO_Figure_Part_090e86(pc),a1    | +11c
        jsr     0x4ae.l                         | +120
        jsr     0x5dd02.l                       | +126
        move.w  #0x8,0x80(a0)                   | +12c
        lea     GO_Figure_Part_090e86(pc),a1    | +132
        jsr     0x4ae.l                         | +136
        jsr     0x5dd02.l                       | +13c
        move.w  #0x9,0x80(a0)                   | +142
        lea     GO_Letter_V1_09076e(pc),a1      | +148
        jsr     0x4ae.l                         | +14c
        jsr     0x5dd02.l                       | +152
        lea     GO_Letter_V0_090728(pc),a1      | +158
        jsr     0x4ae.l                         | +15c
        jsr     0x5dd02.l                       | +162
        lea     GO_Letter_V2_0907b4(pc),a1      | +168
        jsr     0x4ae.l                         | +16c
        jsr     0x5dd02.l                       | +172
        lea     GO_Letter_V3_0907fa(pc),a1      | +178
        jsr     0x4ae.l                         | +17c
        jsr     0x5dd02.l                       | +182
        lea     GO_Letter_V4_090840(pc),a1      | +188
        jsr     0x4ae.l                         | +18c
        jsr     0x5dd02.l                       | +192
        lea     GO_Letter_V5_09088e(pc),a1      | +198
        jsr     0x4ae.l                         | +19c
        jsr     0x5dd02.l                       | +1a2
        lea     GO_Letter_V6_0908d4(pc),a1      | +1a8
        jsr     0x4ae.l                         | +1ac
        jsr     0x5dd02.l                       | +1b2
        lea     GO_Letter_V7_09091a(pc),a1      | +1b8
        jsr     0x4ae.l                         | +1bc
        jsr     0x5dd02.l                       | +1c2
        lea     GO_Letter_V8_090968(pc),a1      | +1c8
        jsr     0x4ae.l                         | +1cc
        jsr     0x5dd02.l                       | +1d2
        lea     GO_Letter_V9_0909ae(pc),a1      | +1d8
        jsr     0x4ae.l                         | +1dc
        jsr     0x5dd02.l                       | +1e2
        lea     GO_Letter_V10_0909f4(pc),a1     | +1e8
        jsr     0x4ae.l                         | +1ec
        jsr     0x5dd02.l                       | +1f2
        lea     GO_Letter_V11_090a3a(pc),a1     | +1f8
        jsr     0x4ae.l                         | +1fc
        jsr     0x5dd02.l                       | +202
        lea     GO_Letter_V12_090a80(pc),a1     | +208
        jsr     0x4ae.l                         | +20c
        jsr     0x5dd02.l                       | +212
        lea     GO_Letter_V13_090ac6(pc),a1     | +218
        jsr     0x4ae.l                         | +21c
        jsr     0x5dd02.l                       | +222
        move.w  #0x10df,d0                      | +228
        jsr     0x2352.l                        | +22c
        lea     .L08fba2(pc),a1                 | +232
        move.l  a1,(a6)                         | +236
.L08fba2:
        addq.w  #0x1,0x86(a6)                   | +238
        move.w  0x86(a6),d0                     | +23c
        andi.w  #0x1,d0                         | +240
        beq.w   .L08fc8c                        | +244
        cmpi.w  #0x178,0x24(a6)                 | +248
        ble.w   .L08fc8c                        | +24e
        jsr     GameOver_IntegrateY_08fcaa(pc)  | +252
        cmpi.w  #0x1a8,0x24(a6)                 | +256
        bne.w   .L08fc26                        | +25c
        cmpi.b  #0x2,0x21(a6)                   | +260
        beq.w   .L08fc26                        | +266
        move.b  #0x2,0x21(a6)                   | +26a
        move.w  #0x10df,d0                      | +270
        jsr     0x2222.l                        | +274
        lea     GO_Banner_0905c2(pc),a1         | +27a
        jsr     0x4ae.l                         | +27e
        jsr     0x5dd02.l                       | +284
        move.w  #0x20,0x82(a0)                  | +28a
        lea     GO_Banner_C_09068c(pc),a1       | +290
        jsr     0x4ae.l                         | +294
        jsr     0x5dd02.l                       | +29a
        move.w  #0x20,0x82(a0)                  | +2a0
        lea     GO_Zoom_09034e(pc),a1           | +2a6
        jsr     0x4ae.l                         | +2aa
        jsr     0x5dd02.l                       | +2b0
        move.w  #0x20,0x82(a0)                  | +2b6
.L08fc26:
        cmpi.w  #0x190,0x24(a6)                 | +2bc
        bne.w   .L08fc8c                        | +2c2
        cmpi.b  #0x2,0x21(a6)                   | +2c6
        beq.w   .L08fc8c                        | +2cc
        move.b  #0x2,0x21(a6)                   | +2d0
        move.w  #0x10df,d0                      | +2d6
        jsr     0x2222.l                        | +2da
        lea     GO_Banner_0905c2(pc),a1         | +2e0
        jsr     0x4ae.l                         | +2e4
        jsr     0x5dd02.l                       | +2ea
        move.w  #0x1e,0x82(a0)                  | +2f0
        lea     GO_Banner_D_0906e8(pc),a1       | +2f6
        jsr     0x4ae.l                         | +2fa
        jsr     0x5dd02.l                       | +300
        move.w  #0x1e,0x82(a0)                  | +306
        lea     GO_Shake_0904e4(pc),a1          | +30c
        jsr     0x4ae.l                         | +310
        jsr     0x5dd02.l                       | +316
        move.w  #0x1e,0x82(a0)                  | +31c
.L08fc8c:
        cmpi.w  #0x178,0x24(a6)                 | +322
        bgt.w   SetHandlerRts_08fca8            | +328
        move.b  #0x1,0x21(a6)                   | +32c
        move.w  #0x10,0x82(a6)                  | +332

| ----------------------------------------------------------------------------
|  GameOver_IntegrateY_08fcaa  @ $08FCAA  (32 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_IntegrateY_08fcaa, "ax", @progbits
        .global GameOver_IntegrateY_08fcaa
GameOver_IntegrateY_08fcaa:
        cmpi.w  #0x140,0x24(a6)                 | +000
        ble.w   .L08fcc8                        | +006
        move.w  0x2a(a6),d0                     | +00a
        move.w  d0,d1                           | +00e
        asr.w   #0x8,d1                         | +010
        add.b   d0,0x27(a6)                     | +012
        moveq   #0,d0                           | +016
        addx.w  d1,d0                           | +018
        add.w   d0,0x24(a6)                     | +01a
.L08fcc8:
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  GameOver_Wait_08fcca  @ $08FCCA  (92 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_Wait_08fcca, "ax", @progbits
        .global GameOver_Wait_08fcca
GameOver_Wait_08fcca:
        jsr     GameOver_IntegrateY_08fcaa(pc)  | +000
        subq.w  #0x1,0x82(a6)                   | +004
        bne.w   SetHandlerRts_08fd2c            | +008
        move.w  #0x10df,d0                      | +00c
        jsr     0x2222.l                        | +010
        move.b  #0x2,0x21(a6)                   | +016
        moveq   #0,d0                           | +01c
        moveq   #0,d1                           | +01e
        jsr     0x437da.l                       | +020
        move.w  #0x18,0x82(a6)                  | +026
        lea     GO_Banner_B_090630(pc),a1       | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        lea     GO_Prop_A_0900e4(pc),a1         | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd02.l                       | +046
        lea     GO_Prop_B_09016c(pc),a1         | +04c
        jsr     0x4ae.l                         | +050
        jsr     0x5dd02.l                       | +056

| ----------------------------------------------------------------------------
|  GameOver_Wait2_08fd2e  @ $08FD2E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_Wait2_08fd2e, "ax", @progbits
        .global GameOver_Wait2_08fd2e
GameOver_Wait2_08fd2e:
        jsr     GameOver_IntegrateY_08fcaa(pc)  | +000
        subq.w  #0x1,0x82(a6)                   | +004
        bne.w   SetHandlerRts_08fd66            | +008
        move.w  #0x10df,d0                      | +00c
        jsr     0x2352.l                        | +010
        move.b  #0x3,0x20(a6)                   | +016
        clr.b   0x21(a6)                        | +01c
        move.w  #0x1b,0x82(a6)                  | +020
        lea     0x106f6c.l,a0                   | +026
        jsr     0x51ed6.l                       | +02c

| ----------------------------------------------------------------------------
|  GameOver_Final_08fd68  @ $08FD68  (58 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_Final_08fd68, "ax", @progbits
        .global GameOver_Final_08fd68
GameOver_Final_08fd68:
        jsr     GameOver_IntegrateY_08fcaa(pc)  | +000
        cmpi.w  #0x3,0x82(a6)                   | +004
        bne.w   .L08fd84                        | +00a
        move.b  #0x1,0x10a2d1.l                 | +00e
        move.b  #0x4,0x21(a6)                   | +016
.L08fd84:
        subq.w  #0x1,0x82(a6)                   | +01c
        bne.w   SetHandlerRts_08fda8            | +020
        move.w  #0x10de,d0                      | +024
        jsr     0x2352.l                        | +028
        move.b  #0x3,0x21(a6)                   | +02e
        move.w  #0x10,0x82(a6)                  | +034

| ----------------------------------------------------------------------------
|  GameOver_WaitCredit_08fdaa  @ $08FDAA  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_WaitCredit_08fdaa, "ax", @progbits
        .global GameOver_WaitCredit_08fdaa
GameOver_WaitCredit_08fdaa:
        tst.b   0x10a2cf.l                      | +000
        beq.w   SetHandlerRts_08fdf6            | +006
        subq.w  #0x1,0x82(a6)                   | +00a
        bne.w   SetHandlerRts_08fdf6            | +00e
        jsr     0x5b6.l                         | +012
        clr.b   0x20(a6)                        | +018
        lea     Continue_Text_Init_091514(pc),a1 | +01c
        jsr     0x4ae.l                         | +020
        lea     GO_Sprite_MidB_09004c(pc),a1    | +026
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd02.l                       | +030
        lea     GO_Glow_Drift_090cba(pc),a1     | +036
        jsr     0x4ae.l                         | +03a
        jsr     0x5dd02.l                       | +040

| ----------------------------------------------------------------------------
|  GameOver_Continue_08fdf8  @ $08FDF8  (50 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_Continue_08fdf8, "ax", @progbits
        .global GameOver_Continue_08fdf8
GameOver_Continue_08fdf8:
        cmpi.b  #0x1,0x20(a6)                   | +000
        bne.w   SetHandlerRts_08fe30            | +006
        lea     GO_Sprite_Mid_08fff6(pc),a1     | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        move.w  0x22(a6),0x88(a6)               | +01a
        move.w  0x24(a6),0x8a(a6)               | +020
        move.w  #0x11,0x82(a6)                  | +026
        move.w  #0x0,0x86(a6)                   | +02c

| ----------------------------------------------------------------------------
|  GameOver_ContinuePath_08fe32  @ $08FE32  (98 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_ContinuePath_08fe32, "ax", @progbits
        .global GameOver_ContinuePath_08fe32
GameOver_ContinuePath_08fe32:
        cmpi.w  #0x14,0x86(a6)                  | +000
        bge.w   .L08fe66                        | +006
        lea     0x2f4bf8.l,a0                   | +00a
        move.w  0x86(a6),d0                     | +010
        add.w   d0,d0                           | +014
        add.w   d0,d0                           | +016
        move.w  (a0,d0.w),d1                    | +018
        add.w   0x88(a6),d1                     | +01c
        move.w  d1,0x22(a6)                     | +020
        move.w  0x2(a0,d0.w),d1                 | +024
        add.w   0x8a(a6),d1                     | +028
        move.w  d1,0x24(a6)                     | +02c
        addq.w  #0x1,0x86(a6)                   | +030
.L08fe66:
        subq.w  #0x1,0x82(a6)                   | +034
        bne.w   SetHandlerRts_08fe9a            | +038
        move.w  #0x18,0x82(a6)                  | +03c
        lea     GO_Sprite_Right_08fecc(pc),a1   | +042
        jsr     0x4ae.l                         | +046
        jsr     0x5dd02.l                       | +04c
        lea     GO_Sprite_Left_08ff2c(pc),a1    | +052
        jsr     0x4ae.l                         | +056
        jsr     0x5dd02.l                       | +05c

| ----------------------------------------------------------------------------
|  GameOver_ContinueHold_08fe9c  @ $08FE9C  (18 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_ContinueHold_08fe9c, "ax", @progbits
        .global GameOver_ContinueHold_08fe9c
GameOver_ContinueHold_08fe9c:
        subq.w  #0x1,0x82(a6)                   | +000
        bne.w   SetHandlerRts_08feb4            | +004
        move.w  #0x96,0x82(a6)                  | +008
        jsr     Continue_Text_DrawTitle_091618(pc) | +00e

| ----------------------------------------------------------------------------
|  GameOver_ContinueEnd_08feb6  @ $08FEB6  (14 B)
| ----------------------------------------------------------------------------
        .section .text.GameOver_ContinueEnd_08feb6, "ax", @progbits
        .global GameOver_ContinueEnd_08feb6
GameOver_ContinueEnd_08feb6:
        subq.w  #0x1,0x82(a6)                   | +000
        bne.w   TaskHandler_08feca              | +004
        clr.b   0x106ed2.l                      | +008

| ----------------------------------------------------------------------------
|  GO_Sprite_Right_08fecc  @ $08FECC  (96 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Sprite_Right_08fecc, "ax", @progbits
        .global GO_Sprite_Right_08fecc
GO_Sprite_Right_08fecc:
        move.w  #0xd9,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x130,0x22(a6)                 | +00a
        move.w  #0x180,0x24(a6)                 | +010
        move.w  0x22(a6),0x88(a6)               | +016
        move.w  #0xd8,0x8c(a6)                  | +01c
        move.w  #0xffe8,0x28(a6)                | +022
        move.w  #0x0,0x80(a6)                   | +028
        move.w  #0x0,0x86(a6)                   | +02e
        move.w  #0xd000,d0                      | +034
        jsr     0x28134.l                       | +038
        andi.w  #0xffe3,0x38(a6)                | +03e
        ori.w   #0x0,0x38(a6)                   | +044
        lea     0x2f503e.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     GO_Sprite_Slide_08ff88(pc),a1   | +056
        move.l  a1,(a6)                         | +05a
        bra.w   GO_Sprite_Slide_08ff88          | +05c

| ----------------------------------------------------------------------------
|  GO_Sprite_Left_08ff2c  @ $08FF2C  (92 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Sprite_Left_08ff2c, "ax", @progbits
        .global GO_Sprite_Left_08ff2c
GO_Sprite_Left_08ff2c:
        move.w  #0xda,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x20,0x22(a6)                  | +00a
        move.w  #0x138,0x24(a6)                 | +010
        move.w  0x22(a6),0x88(a6)               | +016
        move.w  #0xff28,0x8c(a6)                | +01c
        move.w  #0x18,0x28(a6)                  | +022
        move.w  #0x2,0x80(a6)                   | +028
        move.w  #0x0,0x86(a6)                   | +02e
        move.w  #0xe000,d0                      | +034
        jsr     0x28134.l                       | +038
        andi.w  #0xffe3,0x38(a6)                | +03e
        ori.w   #0x0,0x38(a6)                   | +044
        lea     0x2f504a.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     GO_Sprite_Slide_08ff88(pc),a1   | +056
        move.l  a1,(a6)                         | +05a

| ----------------------------------------------------------------------------
|  GO_Sprite_Slide_08ff88  @ $08FF88  (102 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Sprite_Slide_08ff88, "ax", @progbits
        .global GO_Sprite_Slide_08ff88
GO_Sprite_Slide_08ff88:
        move.w  0x88(a6),d0                     | +000
        add.w   0x8c(a6),d0                     | +004
        move.w  d0,0x22(a6)                     | +008
        tst.w   0x8c(a6)                        | +00c
        beq.w   .L08ffa8                        | +010
        move.w  0x28(a6),d0                     | +014
        add.w   d0,0x8c(a6)                     | +018
        bra.w   JsrAbsThunk_08ffee              | +01c
.L08ffa8:
        cmpi.w  #0x14,0x86(a6)                  | +020
        bge.w   JsrAbsThunk_08ffee              | +026
        tst.w   0x86(a6)                        | +02a
        bne.w   .L08ffcc                        | +02e
        move.b  #0x6,0x10a2d1.l                 | +032
        move.w  #0x10e0,d0                      | +03a
        jsr     0x2352.l                        | +03e
.L08ffcc:
        lea     0x2f4c48.l,a0                   | +044
        moveq   #0,d0                           | +04a
        move.w  0x86(a6),d0                     | +04c
        add.w   d0,d0                           | +050
        add.w   d0,d0                           | +052
        adda.l  d0,a0                           | +054
        move.w  0x80(a6),d0                     | +056
        move.w  (a0,d0.w),d1                    | +05a
        add.w   d1,0x22(a6)                     | +05e
        addq.w  #0x1,0x86(a6)                   | +062

| ----------------------------------------------------------------------------
|  GO_Sprite_Mid_08fff6  @ $08FFF6  (78 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Sprite_Mid_08fff6, "ax", @progbits
        .global GO_Sprite_Mid_08fff6
GO_Sprite_Mid_08fff6:
        move.w  #0xdb,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x10,0x8c(a6)                  | +00a
        move.w  #0x40,0x8e(a6)                  | +010
        move.w  #0x0,0x80(a6)                   | +016
        move.w  #0x0,0x84(a6)                   | +01c
        move.w  #0x2000,d0                      | +022
        jsr     0x28134.l                       | +026
        andi.w  #0xffe3,0x38(a6)                | +02c
        ori.w   #0x0,0x38(a6)                   | +032
        move.w  #0x10e1,d0                      | +038
        jsr     0x2352.l                        | +03c
        lea     0x2f5056.l,a0                   | +042
        jsr     0x28cd4.l                       | +048

| ----------------------------------------------------------------------------
|  GO_Sprite_MidB_09004c  @ $09004C  (68 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Sprite_MidB_09004c, "ax", @progbits
        .global GO_Sprite_MidB_09004c
GO_Sprite_MidB_09004c:
        move.w  #0xdd,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x0,0x8c(a6)                   | +00a
        move.w  #0x20,0x8e(a6)                  | +010
        move.w  #0x1,0x80(a6)                   | +016
        move.w  #0x0,0x84(a6)                   | +01c
        move.w  #0x0,d0                         | +022
        jsr     0x28134.l                       | +026
        andi.w  #0xffe3,0x38(a6)                | +02c
        ori.w   #0x0,0x38(a6)                   | +032
        lea     0x2f5062.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e

| ----------------------------------------------------------------------------
|  GO_Sprite_FollowParent_090098  @ $090098  (68 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Sprite_FollowParent_090098, "ax", @progbits
        .global GO_Sprite_FollowParent_090098
GO_Sprite_FollowParent_090098:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),d0                     | +004
        add.w   0x8c(a6),d0                     | +008
        move.w  d0,0x22(a6)                     | +00c
        move.w  0x24(a0),d0                     | +010
        add.w   0x8e(a6),d0                     | +014
        move.w  d0,0x24(a6)                     | +018
        jsr     0x28d70.l                       | +01c
        tst.w   0x80(a6)                        | +022
        bne.w   JsrAbsRts_0900e2                | +026
        tst.w   0x84(a6)                        | +02a
        bne.w   JsrAbsRts_0900e2                | +02e
        move.w  #0xffff,0x84(a6)                | +032
        move.w  0x14(a6),d1                     | +038
        move.w  #0xde,d2                        | +03c
        moveq   #2,d3                           | +040
        moveq   #1,d4                           | +042

| ----------------------------------------------------------------------------
|  GO_Prop_A_0900e4  @ $0900E4  (128 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Prop_A_0900e4, "ax", @progbits
        .global GO_Prop_A_0900e4
GO_Prop_A_0900e4:
        move.w  #0xbe,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xae,0x22(a6)                  | +00a
        move.w  #0x148,0x24(a6)                 | +010
        move.w  #0x4000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x0,0x38(a6)                   | +026
        move.w  #0x0,0x82(a6)                   | +02c
        lea     0x2f5328.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L090128(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L090128:
        jsr     GO_ParentState20Is3_0910d6(pc)  | +044
        bcc.w   .L090138                        | +048
        jmp     0x518.l                         | +04c
        rts                                     | +052
.L090138:
        cmpi.w  #0x10,0x82(a6)                  | +054
        bge.w   JsrAbsThunk_090164              | +05a
        lea     0x2f4b68.l,a0                   | +05e
        move.w  0x82(a6),d0                     | +064
        add.w   d0,d0                           | +068
        add.w   d0,d0                           | +06a
        move.w  (a0,d0.w),d1                    | +06c
        add.w   d1,0x22(a6)                     | +070
        move.w  0x2(a0,d0.w),d1                 | +074
        add.w   d1,0x24(a6)                     | +078
        addq.w  #0x1,0x82(a6)                   | +07c

| ----------------------------------------------------------------------------
|  GO_Prop_B_09016c  @ $09016C  (394 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Prop_B_09016c, "ax", @progbits
        .global GO_Prop_B_09016c
GO_Prop_B_09016c:
        move.w  #0xbd,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xac,0x22(a6)                  | +00a
        move.w  #0x148,0x24(a6)                 | +010
        move.w  #0x8000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x0,0x38(a6)                   | +026
        move.w  #0x0,0x82(a6)                   | +02c
        move.w  #0x0,0x86(a6)                   | +032
        lea     0x2f531c.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        lea     .L0901b6(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L0901b6:
        jsr     GO_ParentState20Is3_0910d6(pc)  | +04a
        bcc.w   .L0901c6                        | +04e
        jmp     0x518.l                         | +052
        rts                                     | +058
.L0901c6:
        cmpi.w  #0x10,0x82(a6)                  | +05a
        bge.w   .L090248                        | +060
        cmpi.w  #0xa,0x82(a6)                   | +064
        bne.w   .L090222                        | +06a
        move.w  0x14(a6),d1                     | +06e
        move.w  #0x100,d2                       | +072
        moveq   #2,d3                           | +076
        moveq   #3,d4                           | +078
        jsr     0x2c30.l                        | +07a
        move.w  #0x10,d1                        | +080
        move.w  #0x19b,d2                       | +084
        moveq   #2,d3                           | +088
        moveq   #3,d4                           | +08a
        jsr     0x2c26.l                        | +08c
        move.w  #0x11,d1                        | +092
        move.w  #0x19c,d2                       | +096
        moveq   #2,d3                           | +09a
        moveq   #3,d4                           | +09c
        jsr     0x2c26.l                        | +09e
        move.w  #0x12,d1                        | +0a4
        move.w  #0x19d,d2                       | +0a8
        moveq   #2,d3                           | +0ac
        moveq   #3,d4                           | +0ae
        jsr     0x2c26.l                        | +0b0
.L090222:
        lea     0x2f4ba8.l,a0                   | +0b6
        move.w  0x82(a6),d0                     | +0bc
        add.w   d0,d0                           | +0c0
        add.w   d0,d0                           | +0c2
        move.w  (a0,d0.w),d1                    | +0c4
        add.w   d1,0x22(a6)                     | +0c8
        move.w  0x2(a0,d0.w),d1                 | +0cc
        add.w   d1,0x24(a6)                     | +0d0
        addq.w  #0x1,0x82(a6)                   | +0d4
        bra.w   JsrAbsThunk_0902f6              | +0d8
.L090248:
        cmpi.w  #0x4,0x86(a6)                   | +0dc
        bge.w   JsrAbsThunk_0902f6              | +0e2
        cmpi.w  #0x0,0x86(a6)                   | +0e6
        bne.w   .L090282                        | +0ec
        lea     GO_Prop_C_0902fe(pc),a1         | +0f0
        jsr     0x4ae.l                         | +0f4
        jsr     0x5dd02.l                       | +0fa
        move.w  #0x101,d1                       | +100
        jsr     0x236e.l                        | +104
        lea     0x2f4a38.l,a0                   | +10a
        jsr     0x2b58.l                        | +110
.L090282:
        cmpi.w  #0x2,0x86(a6)                   | +116
        bne.w   .L0902d4                        | +11c
        move.w  0x14(a6),d1                     | +120
        move.w  #0x100,d2                       | +124
        moveq   #1,d3                           | +128
        moveq   #6,d4                           | +12a
        jsr     0x2c30.l                        | +12c
        move.w  #0x10,d1                        | +132
        move.w  #0x19b,d2                       | +136
        moveq   #1,d3                           | +13a
        moveq   #6,d4                           | +13c
        jsr     0x2c26.l                        | +13e
        move.w  #0x11,d1                        | +144
        move.w  #0x19c,d2                       | +148
        moveq   #1,d3                           | +14c
        moveq   #6,d4                           | +14e
        jsr     0x2c26.l                        | +150
        move.w  #0x12,d1                        | +156
        move.w  #0x19d,d2                       | +15a
        moveq   #1,d3                           | +15e
        moveq   #6,d4                           | +160
        jsr     0x2c26.l                        | +162
.L0902d4:
        lea     0x2f4be8.l,a0                   | +168
        move.w  0x86(a6),d0                     | +16e
        add.w   d0,d0                           | +172
        add.w   d0,d0                           | +174
        move.w  (a0,d0.w),d1                    | +176
        add.w   d1,0x22(a6)                     | +17a
        move.w  0x2(a0,d0.w),d1                 | +17e
        add.w   d1,0x24(a6)                     | +182
        addq.w  #0x1,0x86(a6)                   | +186

| ----------------------------------------------------------------------------
|  GO_Prop_C_0902fe  @ $0902FE  (80 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Prop_C_0902fe, "ax", @progbits
        .global GO_Prop_C_0902fe
GO_Prop_C_0902fe:
        move.w  #0xff,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x98,0x22(a6)                  | +00a
        move.w  #0x198,0x24(a6)                 | +010
        move.w  #0xe000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x0,0x38(a6)                   | +026
        lea     0x2f5368.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L09033c(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L09033c:
        jsr     0x28d70.l                       | +03e
        bcc.w   .L09034c                        | +044
        jmp     0x518.l                         | +048
.L09034c:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  GO_Zoom_09034e  @ $09034E  (260 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Zoom_09034e, "ax", @progbits
        .global GO_Zoom_09034e
GO_Zoom_09034e:
        move.w  #0xdf,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xc000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        moveq   #0,d0                           | +020
        move.b  d0,0x21(a6)                     | +022
        move.b  d0,0x20(a6)                     | +026
        move.w  #0xec,0x88(a6)                  | +02a
        move.w  #0x14c,0x8a(a6)                 | +030
        move.w  #0xf0,0x8c(a6)                  | +036
        move.w  d0,0x8e(a6)                     | +03c
        move.w  d0,0x86(a6)                     | +040
        lea     0x2f50b4.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        lea     .L0903a4(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L0903a4:
        move.w  0x88(a6),d0                     | +056
        sub.w   0x8c(a6),d0                     | +05a
        move.w  d0,0x22(a6)                     | +05e
        move.w  0x8a(a6),d0                     | +062
        sub.w   0x8e(a6),d0                     | +066
        move.w  d0,0x24(a6)                     | +06a
        jsr     0x28d70.l                       | +06e
        cmpi.w  #0x20,0x82(a6)                  | +074
        bgt.w   .L09044c                        | +07a
        cmpi.w  #0x24,0x8c(a6)                  | +07e
        bge.w   .L0903f0                        | +084
        tst.b   0x21(a6)                        | +088
        bne.w   .L0903f0                        | +08c
        move.b  #0xff,0x21(a6)                  | +090
        move.w  #0x158,0x8a(a6)                 | +096
        move.w  #0xc,0x8e(a6)                   | +09c
.L0903f0:
        move.w  0x8c(a6),d0                     | +0a2
        lsr.w   #0x1,d0                         | +0a6
        tst.w   d0                              | +0a8
        bne.w   .L0903fe                        | +0aa
        moveq   #1,d0                           | +0ae
.L0903fe:
        sub.w   d0,0x8c(a6)                     | +0b0
        bpl.w   .L090410                        | +0b4
        move.w  #0xec,0x22(a6)                  | +0b8
        clr.w   0x8c(a6)                        | +0be
.L090410:
        move.w  0x8e(a6),d0                     | +0c2
        lsr.w   #0x2,d0                         | +0c6
        tst.w   d0                              | +0c8
        bne.w   .L09041e                        | +0ca
        moveq   #1,d0                           | +0ce
.L09041e:
        sub.w   d0,0x8e(a6)                     | +0d0
        bpl.w   .L090430                        | +0d4
        move.w  #0x158,0x24(a6)                 | +0d8
        clr.w   0x8e(a6)                        | +0de
.L090430:
        tst.b   0x21(a6)                        | +0e2
        beq.w   .L09044c                        | +0e6
        addq.w  #0x1,0x86(a6)                   | +0ea
        cmpi.w  #0x5,0x86(a6)                   | +0ee
        blt.w   .L09044c                        | +0f4
        lea     GO_Zoom_Path_090452(pc),a1      | +0f8
        move.l  a1,(a6)                         | +0fc
.L09044c:
        subq.w  #0x1,0x82(a6)                   | +0fe
        rts                                     | +102

| ----------------------------------------------------------------------------
|  GO_Zoom_Path_090452  @ $090452  (116 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Zoom_Path_090452, "ax", @progbits
        .global GO_Zoom_Path_090452
GO_Zoom_Path_090452:
        move.w  #0xdf,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xec,0x22(a6)                  | +00a
        move.w  #0x158,0x24(a6)                 | +010
        move.w  #0x0,0x86(a6)                   | +016
        lea     .L090474(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L090474:
        cmpi.w  #0x9,0x86(a6)                   | +022
        bge.w   .L09049c                        | +028
        lea     0x2f4b24.l,a0                   | +02c
        move.w  0x86(a6),d0                     | +032
        add.w   d0,d0                           | +036
        add.w   d0,d0                           | +038
        move.w  (a0,d0.w),d1                    | +03a
        add.w   d1,0x22(a6)                     | +03e
        move.w  0x2(a0,d0.w),d1                 | +042
        add.w   d1,0x24(a6)                     | +046
.L09049c:
        jsr     0x28d70.l                       | +04a
        addq.w  #0x1,0x86(a6)                   | +050
        cmpi.w  #0xd,0x86(a6)                   | +054
        blt.w   .L0904c0                        | +05a
        movea.l 0xc(a6),a0                      | +05e
        move.b  #0x1,0x20(a0)                   | +062
        lea     GO_Zoom_Wait_0904c6(pc),a1      | +068
        move.l  a1,(a6)                         | +06c
.L0904c0:
        subq.w  #0x1,0x82(a6)                   | +06e
        rts                                     | +072

| ----------------------------------------------------------------------------
|  GO_Zoom_Wait_0904c6  @ $0904C6  (8 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Zoom_Wait_0904c6, "ax", @progbits
        .global GO_Zoom_Wait_0904c6
GO_Zoom_Wait_0904c6:
        subq.w  #0x1,0x82(a6)                   | +000
        bne.w   JsrAbsThunk_0904dc              | +004

| ----------------------------------------------------------------------------
|  GO_Shake_0904e4  @ $0904E4  (104 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Shake_0904e4, "ax", @progbits
        .global GO_Shake_0904e4
GO_Shake_0904e4:
        move.w  #0xdf,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xc000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        move.w  #0xc8,0x22(a6)                  | +020
        move.w  #0x140,0x24(a6)                 | +026
        move.w  0x22(a6),0x88(a6)               | +02c
        move.w  0x24(a6),0x8a(a6)               | +032
        move.w  #0x0,0x86(a6)                   | +038
        lea     GO_Flash_090592(pc),a1          | +03e
        jsr     0x4ae.l                         | +042
        jsr     0x5dd02.l                       | +048
        lea     0x2f50c4.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
        lea     .L090544(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L090544:
        subq.w  #0x1,0x82(a6)                   | +060
        bne.w   GO_Shake_Step_09055a            | +064

| ----------------------------------------------------------------------------
|  GO_Shake_Step_09055a  @ $09055A  (48 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Shake_Step_09055a, "ax", @progbits
        .global GO_Shake_Step_09055a
GO_Shake_Step_09055a:
        lea     0x2f4b48.l,a0                   | +000
        move.w  0x86(a6),d0                     | +006
        add.w   d0,d0                           | +00a
        add.w   d0,d0                           | +00c
        move.w  (a0,d0.w),d1                    | +00e
        add.w   0x88(a6),d1                     | +012
        move.w  d1,0x22(a6)                     | +016
        move.w  0x2(a0,d0.w),d1                 | +01a
        add.w   0x8a(a6),d1                     | +01e
        move.w  d1,0x24(a6)                     | +022
        addq.w  #0x2,0x86(a6)                   | +026
        andi.w  #0x7,0x86(a6)                   | +02a

| ----------------------------------------------------------------------------
|  GO_Flash_090592  @ $090592  (40 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Flash_090592, "ax", @progbits
        .global GO_Flash_090592
GO_Flash_090592:
        move.w  #0xe0,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xa8,0x22(a6)                  | +00a
        move.w  #0x180,0x24(a6)                 | +010
        lea     0x2f50fc.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     JsrAbsThunk_0905ba(pc),a1       | +022
        move.l  a1,(a6)                         | +026

| ----------------------------------------------------------------------------
|  GO_Banner_0905c2  @ $0905C2  (88 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Banner_0905c2, "ax", @progbits
        .global GO_Banner_0905c2
GO_Banner_0905c2:
        move.w  #0xe2,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xf000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        move.w  #0xa0,0x22(a6)                  | +020
        move.w  #0x180,0x24(a6)                 | +026
        lea     0x2f506e.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L090600(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L090600:
        subq.w  #0x1,0x82(a6)                   | +03e
        bne.w   JsrAbsThunk_090628              | +042
        move.w  #0x10df,d0                      | +046
        jsr     0x2352.l                        | +04a
        movea.l 0xc(a6),a0                      | +050
        clr.b   0x21(a0)                        | +054

| ----------------------------------------------------------------------------
|  GO_Banner_B_090630  @ $090630  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Banner_B_090630, "ax", @progbits
        .global GO_Banner_B_090630
GO_Banner_B_090630:
        move.w  #0xe2,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xf000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        move.w  #0xa0,0x22(a6)                  | +020
        move.w  #0x180,0x24(a6)                 | +026
        lea     0x2f506e.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L09066e(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L09066e:
        jsr     GO_ParentState20Is3_0910d6(pc)  | +03e
        bcc.w   JsrAbsThunk_090684              | +042

| ----------------------------------------------------------------------------
|  GO_Banner_C_09068c  @ $09068C  (62 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Banner_C_09068c, "ax", @progbits
        .global GO_Banner_C_09068c
GO_Banner_C_09068c:
        move.w  #0xe1,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x0,d0                         | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        move.w  #0xa0,0x22(a6)                  | +020
        move.w  #0x180,0x24(a6)                 | +026
        lea     0x2f507a.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     GO_Banner_Tick_0906ca(pc),a1    | +038
        move.l  a1,(a6)                         | +03c

| ----------------------------------------------------------------------------
|  GO_Banner_Tick_0906ca  @ $0906CA  (8 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Banner_Tick_0906ca, "ax", @progbits
        .global GO_Banner_Tick_0906ca
GO_Banner_Tick_0906ca:
        subq.w  #0x1,0x82(a6)                   | +000
        bne.w   JsrAbsThunk_0906e0              | +004

| ----------------------------------------------------------------------------
|  GO_Banner_D_0906e8  @ $0906E8  (64 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Banner_D_0906e8, "ax", @progbits
        .global GO_Banner_D_0906e8
GO_Banner_D_0906e8:
        move.w  #0xe2,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x0,d0                         | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        move.w  #0xa0,0x22(a6)                  | +020
        move.w  #0x180,0x24(a6)                 | +026
        lea     0x2f5086.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     GO_Banner_Tick_0906ca(pc),a1    | +038
        move.l  a1,(a6)                         | +03c
        bra.b   GO_Banner_Tick_0906ca           | +03e

| ----------------------------------------------------------------------------
|  GO_Letter_V0_090728  @ $090728  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V0_090728, "ax", @progbits
        .global GO_Letter_V0_090728
GO_Letter_V0_090728:
        move.w  #0xc5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0x0,0x80(a6)                   | +024
        move.w  #0xb7,0x7c(a6)                  | +02a
        move.w  #0xbf,0x7e(a6)                  | +030
        lea     0x2f4d9e.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V1_09076e  @ $09076E  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V1_09076e, "ax", @progbits
        .global GO_Letter_V1_09076e
GO_Letter_V1_09076e:
        move.w  #0xc7,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x18,0x38(a6)                  | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0x1,0x80(a6)                   | +024
        move.w  #0xb9,0x7c(a6)                  | +02a
        move.w  #0xc1,0x7e(a6)                  | +030
        lea     0x2f4daa.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V2_0907b4  @ $0907B4  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V2_0907b4, "ax", @progbits
        .global GO_Letter_V2_0907b4
GO_Letter_V2_0907b4:
        move.w  #0xc5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x14,0x38(a6)                  | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0x2,0x80(a6)                   | +024
        move.w  #0xb7,0x7c(a6)                  | +02a
        move.w  #0xbf,0x7e(a6)                  | +030
        lea     0x2f4e08.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V3_0907fa  @ $0907FA  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V3_0907fa, "ax", @progbits
        .global GO_Letter_V3_0907fa
GO_Letter_V3_0907fa:
        move.w  #0xc7,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0x3,0x80(a6)                   | +024
        move.w  #0xb9,0x7c(a6)                  | +02a
        move.w  #0xc1,0x7e(a6)                  | +030
        lea     0x2f4e14.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V4_090840  @ $090840  (78 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V4_090840, "ax", @progbits
        .global GO_Letter_V4_090840
GO_Letter_V4_090840:
        move.w  #0xc5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        move.b  #0xff,0x21(a6)                  | +020
        move.w  #0x4,0x80(a6)                   | +026
        move.w  #0xb7,0x7c(a6)                  | +02c
        move.w  #0xbf,0x7e(a6)                  | +032
        move.w  #0x20,0x86(a6)                  | +038
        lea     0x2f4e42.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        bra.w   GO_Letter_Common_090b08   | +04a

| ----------------------------------------------------------------------------
|  GO_Letter_V5_09088e  @ $09088E  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V5_09088e, "ax", @progbits
        .global GO_Letter_V5_09088e
GO_Letter_V5_09088e:
        move.w  #0xc5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x18,0x38(a6)                  | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0x5,0x80(a6)                   | +024
        move.w  #0xb7,0x7c(a6)                  | +02a
        move.w  #0xbf,0x7e(a6)                  | +030
        lea     0x2f4e4e.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V6_0908d4  @ $0908D4  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V6_0908d4, "ax", @progbits
        .global GO_Letter_V6_0908d4
GO_Letter_V6_0908d4:
        move.w  #0xc5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x14,0x38(a6)                  | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0x6,0x80(a6)                   | +024
        move.w  #0xb7,0x7c(a6)                  | +02a
        move.w  #0xbf,0x7e(a6)                  | +030
        lea     0x2f4e5a.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V7_09091a  @ $09091A  (78 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V7_09091a, "ax", @progbits
        .global GO_Letter_V7_09091a
GO_Letter_V7_09091a:
        move.w  #0xc6,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        move.b  #0xff,0x21(a6)                  | +020
        move.w  #0x7,0x80(a6)                   | +026
        move.w  #0xb8,0x7c(a6)                  | +02c
        move.w  #0xc0,0x7e(a6)                  | +032
        move.w  #0x20,0x86(a6)                  | +038
        lea     0x2f4e66.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        bra.w   GO_Letter_Common_090b08   | +04a

| ----------------------------------------------------------------------------
|  GO_Letter_V8_090968  @ $090968  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V8_090968, "ax", @progbits
        .global GO_Letter_V8_090968
GO_Letter_V8_090968:
        move.w  #0xc5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0x8,0x80(a6)                   | +024
        move.w  #0xb7,0x7c(a6)                  | +02a
        move.w  #0xbf,0x7e(a6)                  | +030
        lea     0x2f4e72.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V9_0909ae  @ $0909AE  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V9_0909ae, "ax", @progbits
        .global GO_Letter_V9_0909ae
GO_Letter_V9_0909ae:
        move.w  #0xc9,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0xc,0x38(a6)                   | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0x9,0x80(a6)                   | +024
        move.w  #0xbb,0x7c(a6)                  | +02a
        move.w  #0xc3,0x7e(a6)                  | +030
        lea     0x2f4e7e.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V10_0909f4  @ $0909F4  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V10_0909f4, "ax", @progbits
        .global GO_Letter_V10_0909f4
GO_Letter_V10_0909f4:
        move.w  #0xc8,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0xa,0x80(a6)                   | +024
        move.w  #0xba,0x7c(a6)                  | +02a
        move.w  #0xc2,0x7e(a6)                  | +030
        lea     0x2f4e8a.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V11_090a3a  @ $090A3A  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V11_090a3a, "ax", @progbits
        .global GO_Letter_V11_090a3a
GO_Letter_V11_090a3a:
        move.w  #0xc6,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0xb,0x80(a6)                   | +024
        move.w  #0xb8,0x7c(a6)                  | +02a
        move.w  #0xc0,0x7e(a6)                  | +030
        lea     0x2f4e96.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V12_090a80  @ $090A80  (70 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V12_090a80, "ax", @progbits
        .global GO_Letter_V12_090a80
GO_Letter_V12_090a80:
        move.w  #0xc5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x8,0x38(a6)                   | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0xc,0x80(a6)                   | +024
        move.w  #0xb7,0x7c(a6)                  | +02a
        move.w  #0xbf,0x7e(a6)                  | +030
        lea     0x2f4ea2.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        bra.w   GO_Letter_Common_090b08   | +042

| ----------------------------------------------------------------------------
|  GO_Letter_V13_090ac6  @ $090AC6  (324 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_V13_090ac6, "ax", @progbits
        .global GO_Letter_V13_090ac6
GO_Letter_V13_090ac6:
        move.w  #0xc5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x4,0x38(a6)                   | +01a
        clr.b   0x21(a6)                        | +020
        move.w  #0xd,0x80(a6)                   | +024
        move.w  #0xb7,0x7c(a6)                  | +02a
        move.w  #0xbf,0x7e(a6)                  | +030
        lea     0x2f4eae.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        .global GO_Letter_Common_090b08
GO_Letter_Common_090b08:
        clr.b   0x20(a6)                        | +042
        move.w  #0x1,0x82(a6)                   | +046
        lea     .L090b18(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L090b18:
        jsr     GO_ParentState21Is2_0910a2(pc)  | +052
        bcs.w   .L090bb0                        | +056
        movea.l 0xc(a6),a0                      | +05a
        lea     0x2f4a46.l,a1                   | +05e
        lea     0x2f4a7c.l,a2                   | +064
        move.w  0x80(a6),d2                     | +06a
        add.w   d2,d2                           | +06e
        add.w   d2,d2                           | +070
        movea.l (a2,d2.w),a2                    | +072
        moveq   #0,d2                           | +076
        move.w  0x84(a0),d2                     | +078
        add.w   d2,d2                           | +07c
        add.w   d2,d2                           | +07e
        adda.l  d2,a2                           | +080
        move.w  (a1),d1                         | +082
        add.w   (a2),d1                         | +084
        add.w   0x22(a0),d1                     | +086
        move.w  d1,0x22(a6)                     | +08a
        move.w  0x2(a1),d1                      | +08e
        add.w   0x2(a2),d1                      | +092
        add.w   0x24(a0),d1                     | +096
        move.w  d1,0x24(a6)                     | +09a
        tst.b   0x21(a6)                        | +09e
        beq.w   .L090b7c                        | +0a2
        tst.w   0x86(a6)                        | +0a6
        beq.w   .L090b7c                        | +0aa
        subq.w  #0x1,0x86(a6)                   | +0ae
        bra.w   .L090b82                        | +0b2
.L090b7c:
        jsr     0x28d70.l                       | +0b6
.L090b82:
        movea.l 0xc(a6),a0                      | +0bc
        cmpi.b  #0x1,0x21(a0)                   | +0c0
        bne.w   .L090bb0                        | +0c6
        tst.b   0x20(a6)                        | +0ca
        bne.w   .L090bb0                        | +0ce
        move.b  #0x80,0x20(a6)                  | +0d2
        move.w  0x14(a6),d1                     | +0d8
        move.w  0x7c(a6),d2                     | +0dc
        moveq   #1,d3                           | +0e0
        moveq   #2,d4                           | +0e2
        jsr     0x2c30.l                        | +0e4
.L090bb0:
        jsr     GO_ParentState21Is3_0910f0(pc)  | +0ea
        bcc.w   .L090bd2                        | +0ee
        cmpi.b  #0xff,0x20(a6)                  | +0f2
        beq.w   .L090bd2                        | +0f8
        move.b  #0xff,0x20(a6)                  | +0fc
        move.w  0x7e(a6),d1                     | +102
        jsr     0x236e.l                        | +106
.L090bd2:
        jsr     GO_ParentState21Is3_0910f0(pc)  | +10c
        bcc.w   JsrAbsRts_090c10                | +110
        tst.w   0x82(a6)                        | +114
        beq.w   GO_Letter_FadeOut_090c12        | +118
        subq.w  #0x1,0x82(a6)                   | +11c
        bne.w   JsrAbsRts_090c10                | +120
        move.b  #0xfc,0x32(a6)                  | +124
        move.b  #0xfc,0x33(a6)                  | +12a
        cmpi.w  #0x5,0x80(a6)                   | +130
        bne.w   JsrAbsRts_090c10                | +136
        lea     GO_Glow_090c30(pc),a1           | +13a
        jsr     0x4ae.l                         | +13e

| ----------------------------------------------------------------------------
|  GO_Letter_FadeOut_090c12  @ $090C12  (30 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Letter_FadeOut_090c12, "ax", @progbits
        .global GO_Letter_FadeOut_090c12
GO_Letter_FadeOut_090c12:
        subi.b  #0x12,0x32(a6)                  | +000
        subi.b  #0x12,0x33(a6)                  | +006
        cmpi.b  #0x20,0x32(a6)                  | +00c
        bhi.w   .L090c2e                        | +012
        jmp     0x518.l                         | +016
.L090c2e:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  GO_Glow_090c30  @ $090C30  (138 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Glow_090c30, "ax", @progbits
        .global GO_Glow_090c30
GO_Glow_090c30:
        move.w  #0xb6,d1                        | +000
        jsr     0x236e.l                        | +004
        subi.w  #0x6a,0x22(a6)                  | +00a
        addi.w  #0x30,0x24(a6)                  | +010
        move.b  #0x15,0x32(a6)                  | +016
        move.b  #0x15,0x33(a6)                  | +01c
        move.w  #0x1,0x82(a6)                   | +022
        move.w  #0x1,0x86(a6)                   | +028
        moveq   #2,d0                           | +02e
        jsr     0x523c6.l                       | +030
        move.w  #0xe000,d0                      | +036
        jsr     0x28134.l                       | +03a
        andi.w  #0xffe3,0x38(a6)                | +040
        ori.w   #0x0,0x38(a6)                   | +046
        lea     0x2f5338.l,a0                   | +04c
        jsr     0x28cd4.l                       | +052
        lea     .L090c8e(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L090c8e:
        jsr     0x28d70.l                       | +05e
        subq.w  #0x1,0x86(a6)                   | +064
        bne.w   GO_Glow_Rts_090cb8         | +068
        move.w  0x82(a6),0x86(a6)               | +06c
        cmpi.b  #0xff,0x32(a6)                  | +072
        beq.w   GO_Glow_Rts_090cb8         | +078
        addi.b  #0x12,0x32(a6)                  | +07c
        addi.b  #0x12,0x33(a6)                  | +082
        .global GO_Glow_Rts_090cb8
GO_Glow_Rts_090cb8:
        rts                                     | +088

| ----------------------------------------------------------------------------
|  GO_Glow_Drift_090cba  @ $090CBA  (246 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Glow_Drift_090cba, "ax", @progbits
        .global GO_Glow_Drift_090cba
GO_Glow_Drift_090cba:
        move.w  #0xb6,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  0x22(a6),d0                     | +00a
        addi.w  #0x10,d0                        | +00e
        move.w  d0,0x8c(a6)                     | +012
        move.w  0x24(a6),d0                     | +016
        addi.w  #0x40,d0                        | +01a
        move.w  d0,0x8e(a6)                     | +01e
        subq.w  #0x8,0x22(a6)                   | +022
        addi.w  #0x58,0x24(a6)                  | +026
        move.b  #0xff,0x32(a6)                  | +02c
        move.b  #0xff,0x33(a6)                  | +032
        move.w  #0x1,0x82(a6)                   | +038
        move.w  #0x1,0x86(a6)                   | +03e
        moveq   #1,d0                           | +044
        jsr     0x523da.l                       | +046
        move.w  #0xe000,d0                      | +04c
        jsr     0x28134.l                       | +050
        andi.w  #0xffe3,0x38(a6)                | +056
        ori.w   #0x0,0x38(a6)                   | +05c
        lea     0x2f5338.l,a0                   | +062
        jsr     0x28cd4.l                       | +068
        lea     .L090d2e(pc),a1                 | +06e
        move.l  a1,(a6)                         | +072
.L090d2e:
        jsr     0x28d70.l                       | +074
        move.w  0x8c(a6),d0                     | +07a
        cmp.w   0x22(a6),d0                     | +07e
        beq.w   .L090d56                        | +082
        move.w  0x8c(a6),d0                     | +086
        sub.w   0x22(a6),d0                     | +08a
        asr.w   #0x1,d0                         | +08e
        tst.w   d0                              | +090
        bne.w   .L090d52                        | +092
        moveq   #1,d0                           | +096
.L090d52:
        add.w   d0,0x22(a6)                     | +098
.L090d56:
        move.w  0x8e(a6),d0                     | +09c
        cmp.w   0x24(a6),d0                     | +0a0
        beq.w   .L090d7a                        | +0a4
        move.w  0x8e(a6),d0                     | +0a8
        sub.w   0x24(a6),d0                     | +0ac
        asr.w   #0x1,d0                         | +0b0
        tst.w   d0                              | +0b2
        bne.w   .L090d76                        | +0b4
        move.w  #0xffff,d0                      | +0b8
.L090d76:
        add.w   d0,0x24(a6)                     | +0bc
.L090d7a:
        subq.w  #0x1,0x86(a6)                   | +0c0
        bne.w   GO_Glow_Rts_090cb8         | +0c4
        move.w  0x82(a6),0x86(a6)               | +0c8
        subi.b  #0x10,0x32(a6)                  | +0ce
        subi.b  #0x10,0x33(a6)                  | +0d4
        cmpi.b  #0x1f,0x32(a6)                  | +0da
        bhi.w   .L090dae                        | +0e0
        movea.l 0xc(a6),a0                      | +0e4
        move.b  #0x1,0x20(a0)                   | +0e8
        jmp     0x518.l                         | +0ee
.L090dae:
        rts                                     | +0f4

| ----------------------------------------------------------------------------
|  GO_Figure_A_090db0  @ $090DB0  (50 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Figure_A_090db0, "ax", @progbits
        .global GO_Figure_A_090db0
GO_Figure_A_090db0:
        move.w  #0xc4,d1                        | +000
        jsr     0x236e.l                        | +004
        clr.b   0x21(a6)                        | +00a
        clr.b   0x20(a6)                        | +00e
        move.w  #0x2000,d0                      | +012
        jsr     0x28134.l                       | +016
        move.w  #0xb5,0x7c(a6)                  | +01c
        lea     0x2f4eba.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        bra.w   GO_Figure_Run_090e1c     | +02e

| ----------------------------------------------------------------------------
|  GO_Figure_B_090de2  @ $090DE2  (148 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Figure_B_090de2, "ax", @progbits
        .global GO_Figure_B_090de2
GO_Figure_B_090de2:
        move.w  #0xc4,d1                        | +000
        jsr     0x236e.l                        | +004
        clr.b   0x21(a6)                        | +00a
        clr.b   0x20(a6)                        | +00e
        move.w  #0x0,d0                         | +012
        jsr     0x28134.l                       | +016
        andi.w  #0xffe3,0x38(a6)                | +01c
        ori.w   #0x0,0x38(a6)                   | +022
        move.w  #0xb5,0x7c(a6)                  | +028
        lea     0x2f4eec.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        .global GO_Figure_Run_090e1c
GO_Figure_Run_090e1c:
        lea     .L090e22(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L090e22:
        jsr     GO_ParentState21Is2_0910a2(pc)  | +040
        bcs.w   .L090e6e                        | +044
        movea.l 0xc(a6),a0                      | +048
        move.w  0x22(a0),0x22(a6)               | +04c
        move.w  0x24(a0),0x24(a6)               | +052
        jsr     0x28d70.l                       | +058
        movea.l 0xc(a6),a0                      | +05e
        cmpi.b  #0x1,0x21(a0)                   | +062
        bne.w   .L090e6e                        | +068
        tst.b   0x21(a6)                        | +06c
        bne.w   .L090e6e                        | +070
        move.b  #0xff,0x21(a6)                  | +074
        move.w  0x14(a6),d1                     | +07a
        move.w  0x7c(a6),d2                     | +07e
        moveq   #1,d3                           | +082
        moveq   #2,d4                           | +084
        jsr     0x2c30.l                        | +086
.L090e6e:
        jsr     GO_ParentState21Is4_09110a(pc)  | +08c
        bcc.w   SetHandlerRts_090e7c            | +090

| ----------------------------------------------------------------------------
|  GO_Figure_Part_090e86  @ $090E86  (114 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Figure_Part_090e86, "ax", @progbits
        .global GO_Figure_Part_090e86
GO_Figure_Part_090e86:
        move.w  #0xbc,d1                        | +000
        jsr     0x236e.l                        | +004
        clr.b   0x21(a6)                        | +00a
        clr.b   0x20(a6)                        | +00e
        move.w  #0x0,d0                         | +012
        jsr     0x28134.l                       | +016
        andi.w  #0xffe3,0x38(a6)                | +01c
        ori.w   #0x0,0x38(a6)                   | +022
        move.w  #0xbc,0x7c(a6)                  | +028
        lea     0x2f4ef8.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     .L090ec6(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L090ec6:
        jsr     GO_ParentState21Is2_0910a2(pc)  | +040
        bcs.w   JsrAbsRts_090efe                | +044
        movea.l 0xc(a6),a1                      | +048
        lea     0x2f4a4a.l,a0                   | +04c
        move.w  0x80(a6),d0                     | +052
        add.w   d0,d0                           | +056
        add.w   d0,d0                           | +058
        move.w  (a0,d0.w),d1                    | +05a
        add.w   0x22(a1),d1                     | +05e
        move.w  d1,0x22(a6)                     | +062
        move.w  0x2(a0,d0.w),d1                 | +066
        add.w   0x24(a1),d1                     | +06a
        move.w  d1,0x24(a6)                     | +06e

| ----------------------------------------------------------------------------
|  GO_Scroller_A_090f00  @ $090F00  (104 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Scroller_A_090f00, "ax", @progbits
        .global GO_Scroller_A_090f00
GO_Scroller_A_090f00:
        move.w  #0xc4,d1                        | +000
        jsr     0x236e.l                        | +004
        clr.b   0x21(a6)                        | +00a
        clr.b   0x20(a6)                        | +00e
        move.w  #0x0,d0                         | +012
        jsr     0x28134.l                       | +016
        andi.w  #0xffe3,0x38(a6)                | +01c
        ori.w   #0x10,0x38(a6)                  | +022
        move.w  #0x200,0x28(a6)                 | +028
        lea     0x2f4a72.l,a0                   | +02e
        move.w  0x80(a6),d0                     | +034
        add.w   d0,d0                           | +038
        move.w  (a0,d0.w),d1                    | +03a
        add.w   0x22(a6),d1                     | +03e
        move.w  d1,0x22(a6)                     | +042
        move.w  #0xb5,0x7c(a6)                  | +046
        move.w  #0x10,0x82(a6)                  | +04c
        lea     0x2f4ed4.l,a0                   | +052
        jsr     0x28cd4.l                       | +058
        lea     GO_Scroller_Run_090fce(pc),a1   | +05e
        move.l  a1,(a6)                         | +062
        bra.w   GO_Scroller_Run_090fce          | +064

| ----------------------------------------------------------------------------
|  GO_Scroller_B_090f68  @ $090F68  (102 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Scroller_B_090f68, "ax", @progbits
        .global GO_Scroller_B_090f68
GO_Scroller_B_090f68:
        move.w  #0xc4,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0x1,0x21(a6)                   | +00a
        clr.b   0x20(a6)                        | +010
        move.w  #0x0,d0                         | +014
        jsr     0x28134.l                       | +018
        andi.w  #0xffe3,0x38(a6)                | +01e
        ori.w   #0xc,0x38(a6)                   | +024
        move.w  #0x180,0x28(a6)                 | +02a
        lea     0x2f4a78.l,a0                   | +030
        move.w  0x80(a6),d0                     | +036
        add.w   d0,d0                           | +03a
        move.w  (a0,d0.w),d1                    | +03c
        add.w   0x22(a6),d1                     | +040
        move.w  d1,0x22(a6)                     | +044
        move.w  #0xb5,0x7c(a6)                  | +048
        move.w  #0x10,0x82(a6)                  | +04e
        lea     0x2f4ee0.l,a0                   | +054
        jsr     0x28cd4.l                       | +05a
        lea     GO_Scroller_Run_090fce(pc),a1   | +060
        move.l  a1,(a6)                         | +064

| ----------------------------------------------------------------------------
|  GO_Scroller_Run_090fce  @ $090FCE  (192 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Scroller_Run_090fce, "ax", @progbits
        .global GO_Scroller_Run_090fce
GO_Scroller_Run_090fce:
        jsr     GO_ParentState21Is2_0910a2(pc)  | +000
        bcs.w   .L091086                        | +004
        movea.l 0xc(a6),a0                      | +008
        move.w  0x24(a0),0x24(a6)               | +00c
        addi.w  #0x10,0x24(a6)                  | +012
        cmpi.b  #0xa0,0x32(a6)                  | +018
        ble.w   .L09100c                        | +01e
        move.w  0x22(a6),d0                     | +022
        cmpi.w  #0x20,d0                        | +026
        blt.w   .L09100c                        | +02a
        andi.w  #0x1,d0                         | +02e
        beq.w   .L09100c                        | +032
        subq.b  #0x2,0x32(a6)                   | +036
        subq.b  #0x2,0x33(a6)                   | +03a
.L09100c:
        move.w  0x28(a6),d0                     | +03e
        move.w  d0,d1                           | +042
        asr.w   #0x8,d1                         | +044
        add.b   d0,0x26(a6)                     | +046
        moveq   #0,d0                           | +04a
        addx.w  d0,d1                           | +04c
        add.w   d1,0x22(a6)                     | +04e
        cmpi.w  #0x110,0x22(a6)                 | +052
        ble.w   .L091042                        | +058
        move.b  #0xff,0x32(a6)                  | +05c
        move.b  #0xff,0x33(a6)                  | +062
        move.w  #0x10,0x22(a6)                  | +068
        move.w  #0x0,0x26(a6)                   | +06e
.L091042:
        tst.w   0x82(a6)                        | +074
        beq.w   .L091052                        | +078
        subq.w  #0x1,0x82(a6)                   | +07c
        bra.w   .L091058                        | +080
.L091052:
        jsr     0x28d70.l                       | +084
.L091058:
        movea.l 0xc(a6),a0                      | +08a
        cmpi.b  #0x1,0x21(a0)                   | +08e
        bne.w   .L091086                        | +094
        tst.b   0x21(a6)                        | +098
        bne.w   .L091086                        | +09c
        move.b  #0xff,0x21(a6)                  | +0a0
        move.w  0x14(a6),d1                     | +0a6
        move.w  0x7c(a6),d2                     | +0aa
        moveq   #1,d3                           | +0ae
        moveq   #2,d4                           | +0b0
        jsr     0x2c30.l                        | +0b2
.L091086:
        jsr     GO_ParentState21Is4_09110a(pc)  | +0b8
        bcc.w   SetHandlerRts_091094            | +0bc

| ----------------------------------------------------------------------------
|  GO_CopyParam84_091096  @ $091096  (12 B)
| ----------------------------------------------------------------------------
        .section .text.GO_CopyParam84_091096, "ax", @progbits
        .global GO_CopyParam84_091096
GO_CopyParam84_091096:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x84(a6),0x84(a0)               | +004
        rts                                     | +00a

| ----------------------------------------------------------------------------
|  GO_ParentState21Is2_0910a2  @ $0910A2  (14 B)
| ----------------------------------------------------------------------------
        .section .text.GO_ParentState21Is2_0910a2, "ax", @progbits
        .global GO_ParentState21Is2_0910a2
GO_ParentState21Is2_0910a2:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x2,0x21(a0)                   | +004
        bne.w   ClearXN_0910b6                  | +00a

| ----------------------------------------------------------------------------
|  GO_ParentState20Is2_0910bc  @ $0910BC  (14 B)
| ----------------------------------------------------------------------------
        .section .text.GO_ParentState20Is2_0910bc, "ax", @progbits
        .global GO_ParentState20Is2_0910bc
GO_ParentState20Is2_0910bc:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x2,0x20(a0)                   | +004
        bne.w   ClearXN_0910d0                  | +00a

| ----------------------------------------------------------------------------
|  GO_ParentState20Is3_0910d6  @ $0910D6  (14 B)
| ----------------------------------------------------------------------------
        .section .text.GO_ParentState20Is3_0910d6, "ax", @progbits
        .global GO_ParentState20Is3_0910d6
GO_ParentState20Is3_0910d6:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x3,0x20(a0)                   | +004
        bne.w   ClearXN_0910ea                  | +00a

| ----------------------------------------------------------------------------
|  GO_ParentState21Is3_0910f0  @ $0910F0  (14 B)
| ----------------------------------------------------------------------------
        .section .text.GO_ParentState21Is3_0910f0, "ax", @progbits
        .global GO_ParentState21Is3_0910f0
GO_ParentState21Is3_0910f0:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x3,0x21(a0)                   | +004
        bne.w   ClearXN_091104                  | +00a

| ----------------------------------------------------------------------------
|  GO_ParentState21Is4_09110a  @ $09110A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.GO_ParentState21Is4_09110a, "ax", @progbits
        .global GO_ParentState21Is4_09110a
GO_ParentState21Is4_09110a:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x4,0x21(a0)                   | +004
        bne.w   ClearXN_09111e                  | +00a

| ----------------------------------------------------------------------------
|  GO_Particle_Add_091124  @ $091124  (4 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Particle_Add_091124, "ax", @progbits
        .global GO_Particle_Add_091124
GO_Particle_Add_091124:
        lea     GO_Particle_091130(pc),a1       | +000

| ----------------------------------------------------------------------------
|  GO_Particle_091130  @ $091130  (346 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Particle_091130, "ax", @progbits
        .global GO_Particle_091130
GO_Particle_091130:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x1,d0                         | +006
        beq.w   .L0911e6                        | +00a
        move.w  #0xe0,d1                        | +00e
        jsr     0x236e.l                        | +012
        move.w  #0x20,0x22(a6)                  | +018
        move.w  #0x128,0x24(a6)                 | +01e
        jsr     0x5e9b6.l                       | +024
        andi.w  #0xf,d0                         | +02a
        subq.w  #0x8,d0                         | +02e
        add.w   d0,0x22(a6)                     | +030
        move.w  #0x200,0x28(a6)                 | +034
        move.w  #0xc00,0x2a(a6)                 | +03a
        move.w  #0xa0,0x2e(a6)                  | +040
        move.w  #0x0,0x82(a6)                   | +046
        jsr     0x5e9b6.l                       | +04c
        andi.w  #0xf0,d0                        | +052
        subi.w  #0x80,d0                        | +056
        add.w   d0,0x28(a6)                     | +05a
        jsr     0x5e9b6.l                       | +05e
        andi.w  #0x30,d0                        | +064
        add.w   d0,0x28(a6)                     | +068
        move.w  #0x2000,d0                      | +06c
        jsr     0x28134.l                       | +070
        andi.w  #0xffe3,0x38(a6)                | +076
        ori.w   #0x0,0x38(a6)                   | +07c
        jsr     0x5e9b6.l                       | +082
        andi.w  #0x3,d0                         | +088
        beq.w   .L0911d0                        | +08c
        lea     0x2f512a.l,a0                   | +090
        jsr     0x28cd4.l                       | +096
        bra.w   .L0911dc                        | +09c
.L0911d0:
        lea     0x2f5276.l,a0                   | +0a0
        jsr     0x28cd4.l                       | +0a6
.L0911dc:
        lea     GO_Particle_Run_09128a(pc),a1   | +0ac
        move.l  a1,(a6)                         | +0b0
        bra.w   GO_Particle_Run_09128a          | +0b2
.L0911e6:
        move.w  #0xe0,d1                        | +0b6
        jsr     0x236e.l                        | +0ba
        move.w  #0x20,0x22(a6)                  | +0c0
        move.w  #0x128,0x24(a6)                 | +0c6
        jsr     0x5e9b6.l                       | +0cc
        andi.w  #0xf,d0                         | +0d2
        subq.w  #0x8,d0                         | +0d6
        add.w   d0,0x22(a6)                     | +0d8
        move.w  #0x300,0x28(a6)                 | +0dc
        move.w  #0xc00,0x2a(a6)                 | +0e2
        move.w  #0x90,0x2e(a6)                  | +0e8
        move.w  #0x0,0x82(a6)                   | +0ee
        jsr     0x5e9b6.l                       | +0f4
        andi.w  #0xf0,d0                        | +0fa
        subi.w  #0x80,d0                        | +0fe
        add.w   d0,0x28(a6)                     | +102
        jsr     0x5e9b6.l                       | +106
        andi.w  #0x30,d0                        | +10c
        add.w   d0,0x28(a6)                     | +110
        move.w  #0x2000,d0                      | +114
        jsr     0x28134.l                       | +118
        andi.w  #0xffe3,0x38(a6)                | +11e
        ori.w   #0x0,0x38(a6)                   | +124
        jsr     0x5e9b6.l                       | +12a
        andi.w  #0x3,d0                         | +130
        beq.w   .L091278                        | +134
        lea     0x2f51d0.l,a0                   | +138
        jsr     0x28cd4.l                       | +13e
        bra.w   .L091284                        | +144
.L091278:
        lea     0x2f5276.l,a0                   | +148
        jsr     0x28cd4.l                       | +14e
.L091284:
        lea     GO_Particle_Run_09128a(pc),a1   | +154
        move.l  a1,(a6)                         | +158

| ----------------------------------------------------------------------------
|  GO_Particle_Run_09128a  @ $09128A  (166 B)
| ----------------------------------------------------------------------------
        .section .text.GO_Particle_Run_09128a, "ax", @progbits
        .global GO_Particle_Run_09128a
GO_Particle_Run_09128a:
        movea.l 0xc(a6),a0                      | +000
        movea.l 0xc(a0),a0                      | +004
        cmpi.b  #0x2,0x20(a0)                   | +008
        beq.w   .L091328                        | +00e
        move.w  0x28(a6),d0                     | +012
        move.w  d0,d1                           | +016
        asr.w   #0x8,d1                         | +018
        add.b   d0,0x26(a6)                     | +01a
        moveq   #0,d0                           | +01e
        addx.w  d0,d1                           | +020
        add.w   d1,0x22(a6)                     | +022
        move.w  0x28(a6),d0                     | +026
        move.w  d0,d1                           | +02a
        asr.w   #0x8,d1                         | +02c
        add.b   d0,0x26(a6)                     | +02e
        moveq   #0,d0                           | +032
        addx.w  d0,d1                           | +034
        add.w   d1,0x22(a6)                     | +036
        move.w  0x2e(a6),d0                     | +03a
        sub.w   d0,0x2a(a6)                     | +03e
        move.w  0x2a(a6),d0                     | +042
        move.w  d0,d1                           | +046
        asr.w   #0x8,d1                         | +048
        add.b   d0,0x27(a6)                     | +04a
        moveq   #0,d0                           | +04e
        addx.w  d0,d1                           | +050
        add.w   d1,0x24(a6)                     | +052
        move.w  0x2e(a6),d0                     | +056
        sub.w   d0,0x2a(a6)                     | +05a
        move.w  0x2a(a6),d0                     | +05e
        move.w  d0,d1                           | +062
        asr.w   #0x8,d1                         | +064
        add.b   d0,0x27(a6)                     | +066
        moveq   #0,d0                           | +06a
        addx.w  d0,d1                           | +06c
        add.w   d1,0x24(a6)                     | +06e
        jsr     0x28d70.l                       | +072
        addq.w  #0x1,0x82(a6)                   | +078
        cmpi.w  #0x1,0x82(a6)                   | +07c
        blt.w   .L09131e                        | +082
        move.w  #0x0,0x82(a6)                   | +086
        subq.b  #0x6,0x32(a6)                   | +08c
        subq.b  #0x6,0x33(a6)                   | +090
.L09131e:
        cmpi.w  #0x120,0x24(a6)                 | +094
        bgt.w   .L09132e                        | +09a
.L091328:
        jmp     0x518.l                         | +09e
.L09132e:
        rts                                     | +0a4

| ----------------------------------------------------------------------------
|  Continue_Spawn_091338  @ $091338  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Spawn_091338, "ax", @progbits
        .global Continue_Spawn_091338
Continue_Spawn_091338:
        move.w  #0x90,0x22(a6)                  | +000
        move.w  #0x1c0,0x24(a6)                 | +006
        clr.b   0x20(a6)                        | +00c
        clr.b   0x21(a6)                        | +010
        lea     Continue_Text_Countdown_091578(pc),a1 | +014
        jsr     0x4ae.l                         | +018
        lea     Continue_Text_Init_091514(pc),a1 | +01e
        jsr     0x4ae.l                         | +022
        lea     Continue_Sprite_Left_09148c(pc),a1 | +028
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd02.l                       | +032
        lea     Continue_Sprite_Right_09144a(pc),a1 | +038
        jsr     0x4ae.l                         | +03c
        jsr     0x5dd02.l                       | +042
        lea     Continue_Sprite_Mid_091408(pc),a1 | +048
        jsr     0x4ae.l                         | +04c
        jsr     0x5dd02.l                       | +052
        lea     Continue_Sprite_MidB_0914ce(pc),a1 | +058
        jsr     0x4ae.l                         | +05c
        jsr     0x5dd02.l                       | +062
        jsr     Continue_Text_DrawTitle_091618(pc) | +068

| ----------------------------------------------------------------------------
|  Continue_Tpl_0913ac  @ $0913AC  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Tpl_0913ac, "ax", @progbits
        .global Continue_Tpl_0913ac
Continue_Tpl_0913ac:
        move.w  #0x90,0x22(a6)                  | +000
        move.w  #0x1c0,0x24(a6)                 | +006
        clr.b   0x20(a6)                        | +00c
        clr.b   0x21(a6)                        | +010
        lea     Continue_Sprite_Left_09148c(pc),a1 | +014
        jsr     0x4ae.l                         | +018
        jsr     0x5dd02.l                       | +01e
        lea     Continue_Sprite_Right_09144a(pc),a1 | +024
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        lea     Continue_Sprite_Mid_091408(pc),a1 | +034
        jsr     0x4ae.l                         | +038
        jsr     0x5dd02.l                       | +03e
        lea     Continue_Sprite_MidB_0914ce(pc),a1 | +044
        jsr     0x4ae.l                         | +048
        jsr     0x5dd02.l                       | +04e
        lea     TaskHandler_0913aa(pc),a1       | +054
        move.l  a1,(a6)                         | +058
        bra.b   TaskHandler_0913aa              | +05a

| ----------------------------------------------------------------------------
|  Continue_Sprite_Mid_091408  @ $091408  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Sprite_Mid_091408, "ax", @progbits
        .global Continue_Sprite_Mid_091408
Continue_Sprite_Mid_091408:
        move.w  #0xde,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xa0,0x22(a6)                  | +00a
        move.w  #0x180,0x24(a6)                 | +010
        move.w  #0x2000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x0,0x38(a6)                   | +026
        lea     0x2f5056.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     JsrAbsThunk_09150c(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
        bra.w   JsrAbsThunk_09150c              | +03e

| ----------------------------------------------------------------------------
|  Continue_Sprite_Right_09144a  @ $09144A  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Sprite_Right_09144a, "ax", @progbits
        .global Continue_Sprite_Right_09144a
Continue_Sprite_Right_09144a:
        move.w  #0xd9,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x130,0x22(a6)                 | +00a
        move.w  #0x180,0x24(a6)                 | +010
        move.w  #0xd000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x0,0x38(a6)                   | +026
        lea     0x2f503e.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     JsrAbsThunk_09150c(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
        bra.w   JsrAbsThunk_09150c              | +03e

| ----------------------------------------------------------------------------
|  Continue_Sprite_Left_09148c  @ $09148C  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Sprite_Left_09148c, "ax", @progbits
        .global Continue_Sprite_Left_09148c
Continue_Sprite_Left_09148c:
        move.w  #0xda,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x20,0x22(a6)                  | +00a
        move.w  #0x138,0x24(a6)                 | +010
        move.w  #0xe000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x0,0x38(a6)                   | +026
        lea     0x2f504a.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     JsrAbsThunk_09150c(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
        bra.w   JsrAbsThunk_09150c              | +03e

| ----------------------------------------------------------------------------
|  Continue_Sprite_MidB_0914ce  @ $0914CE  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Sprite_MidB_0914ce, "ax", @progbits
        .global Continue_Sprite_MidB_0914ce
Continue_Sprite_MidB_0914ce:
        move.w  #0xdd,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x90,0x22(a6)                  | +00a
        move.w  #0x160,0x24(a6)                 | +010
        move.w  #0x0,d0                         | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x0,0x38(a6)                   | +026
        lea     0x2f5062.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     JsrAbsThunk_09150c(pc),a1       | +038
        move.l  a1,(a6)                         | +03c

| ----------------------------------------------------------------------------
|  Continue_Text_Init_091514  @ $091514  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Text_Init_091514, "ax", @progbits
        .global Continue_Text_Init_091514
Continue_Text_Init_091514:
        move.w  #0x1,0x82(a6)                   | +000
        move.w  #0x10,0x86(a6)                  | +006
        move.w  #0x0,0x84(a6)                   | +00c
        lea     Continue_Text_Clear_09152c(pc),a1 | +012
        move.l  a1,(a6)                         | +016

| ----------------------------------------------------------------------------
|  Continue_Text_Clear_09152c  @ $09152C  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Text_Clear_09152c, "ax", @progbits
        .global Continue_Text_Clear_09152c
Continue_Text_Clear_09152c:
        movea.l #0x7158,a1                      | +000
        move.w  #0x3480,d0                      | +006
        move.w  #0x14,d1                        | +00a
        moveq   #2,d2                           | +00e
        jsr     0x5da9c.l                       | +010
        subq.w  #0x1,0x82(a6)                   | +016
        bne.w   SetHandlerRts_091556            | +01a
        move.w  0x86(a6),0x82(a6)               | +01e

| ----------------------------------------------------------------------------
|  Continue_Text_Blink_091558  @ $091558  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Text_Blink_091558, "ax", @progbits
        .global Continue_Text_Blink_091558
Continue_Text_Blink_091558:
        jsr     0x52328.l                       | +000
        jsr     Continue_Text_DrawRows_0915dc(pc) | +006
        subq.w  #0x1,0x82(a6)                   | +00a
        bne.w   SetHandlerRts_091576            | +00e
        move.w  0x86(a6),0x82(a6)               | +012

| ----------------------------------------------------------------------------
|  Continue_Text_Countdown_091578  @ $091578  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Text_Countdown_091578, "ax", @progbits
        .global Continue_Text_Countdown_091578
Continue_Text_Countdown_091578:
        movea.l #0x7236,a1                      | +000
        lea     0x2f4cb2.l,a2                   | +006
        move.w  #0x5300,d0                      | +00c
        jsr     0x5dad8.l                       | +010
        lea     .L091594(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L091594:
        movea.l #0x72b6,a1                      | +01c
        move.w  #0x5300,d0                      | +022
        move.b  0x10fdda.l,d3                   | +026
        move.b  d3,d4                           | +02c
        lsr.b   #0x4,d3                         | +02e
        addi.b  #0x30,d3                        | +030
        or.b    d3,d0                           | +034
        moveq   #1,d1                           | +036
        moveq   #1,d2                           | +038
        move.l  d4,-(a7)                        | +03a
        jsr     0x5da56.l                       | +03c
        move.l  (a7)+,d4                        | +042
        movea.l #0x72d6,a1                      | +044
        move.w  #0x5300,d0                      | +04a
        andi.b  #0xf,d4                         | +04e
        addi.b  #0x30,d4                        | +052
        or.b    d4,d0                           | +056
        moveq   #1,d1                           | +058
        moveq   #1,d2                           | +05a

| ----------------------------------------------------------------------------
|  Continue_Text_DrawRows_0915dc  @ $0915DC  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Text_DrawRows_0915dc, "ax", @progbits
        .global Continue_Text_DrawRows_0915dc
Continue_Text_DrawRows_0915dc:
        lea     0x2f4cb8.l,a0                   | +000
        add.w   d0,d0                           | +006
        add.w   d0,d0                           | +008
        movea.l (a0,d0.w),a4                    | +00a
        movea.l #0x7158,a1                      | +00e
        moveq   #0,d4                           | +014
.L0915f2:
        move.w  (a4,d4.w),d0                    | +016
        cmpi.w  #0xffff,d0                      | +01a
        beq.w   .L091616                        | +01e
        move.l  a1,-(a7)                        | +022
        moveq   #1,d1                           | +024
        moveq   #2,d2                           | +026
        jsr     0x5da56.l                       | +028
        movea.l (a7)+,a1                        | +02e
        adda.l  #0x20,a1                        | +030
        addq.w  #0x2,d4                         | +036
        bra.b   .L0915f2                        | +038
.L091616:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Continue_Text_DrawTitle_091618  @ $091618  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Text_DrawTitle_091618, "ax", @progbits
        .global Continue_Text_DrawTitle_091618
Continue_Text_DrawTitle_091618:
        movea.l #0x711b,a1                      | +000
        lea     0x2f4c98.l,a2                   | +006
        move.w  #0x2300,d0                      | +00c

| ----------------------------------------------------------------------------
|  Continue_Text_DrawCredits_091630  @ $091630  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Text_DrawCredits_091630, "ax", @progbits
        .global Continue_Text_DrawCredits_091630
Continue_Text_DrawCredits_091630:
        movea.l #0x71c3,a1                      | +000
        move.w  #0xc8c0,d0                      | +006
        move.w  #0xc,d1                         | +00a
        moveq   #4,d2                           | +00e
        jsr     0x5da56.l                       | +010
        movea.l #0x71c7,a1                      | +016
        move.w  #0xc9c0,d0                      | +01c
        move.w  #0xc,d1                         | +020
        moveq   #4,d2                           | +024
        jsr     0x5da56.l                       | +026
        bclr    #0x1,0x12(a6)                   | +02c

| ----------------------------------------------------------------------------
|  Continue_Tmpl227_09166a  @ $09166A  (78 B)
| ----------------------------------------------------------------------------
        .section .text.Continue_Tmpl227_09166a, "ax", @progbits
        .global Continue_Tmpl227_09166a
Continue_Tmpl227_09166a:
        move.w  #0xdc,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xf000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        jsr     0x267e2.l                       | +020
        move.w  #0xa0,0x22(a6)                  | +026
        move.w  #0x1b0,0x24(a6)                 | +02c
        bclr    #0x1,0x12(a6)                   | +032
        lea     Continue_Text_Init_091514(pc),a1 | +038
        jsr     0x4ae.l                         | +03c
        lea     0x2f53a6.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
