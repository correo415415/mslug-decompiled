| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $051AA4..$0527AE  (2,208 B, 38 entradas, 18 huecos)
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
|  CellMap_ReadPacked32_051aa4  @ $051AA4  (26 B)
| ----------------------------------------------------------------------------
        .section .text.CellMap_ReadPacked32_051aa4, "ax", @progbits
        .global CellMap_ReadPacked32_051aa4
CellMap_ReadPacked32_051aa4:
        movem.l a0-a1,-(a7)                     | +000
        lea     0x1081b6.l,a1                   | +004
        bsr.w   Nibbles_Pack8_051862            | +00a
        suba.w  #0x4,a1                         | +00e
        move.l  (a1),d0                         | +012
        movem.l (a7)+,a0-a1                     | +014
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Entity_AllocAndInit_051ABE  @ $051ABE  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_AllocAndInit_051ABE, "ax", @progbits
        .global Entity_AllocAndInit_051ABE
Entity_AllocAndInit_051ABE:
        move.w  d0,0x28(a0)                     | +000
        clr.l   0xe(a0)                         | +004
        move.w  (a1),0x16(a0)                   | +008
        move.w  0x2(a1),0x18(a0)                | +00c
        move.w  0x4(a1),0x1a(a0)                | +012
        move.w  0x6(a1),0x1c(a0)                | +018
        clr.w   0x2a(a0)                        | +01e
        clr.w   0x2c(a0)                        | +022
        clr.w   0x22(a0)                        | +026
        clr.w   0x24(a0)                        | +02a
        clr.w   0xc(a0)                         | +02e
        clr.l   0x4(a0)                         | +032
        clr.l   0x8(a0)                         | +036
        clr.w   (a0)                            | +03a
        clr.w   0x2(a0)                         | +03c
        clr.w   0x1e(a0)                        | +040
        clr.w   0x20(a0)                        | +044
        bsr.w   CellMap_ClearSprites32_051ed6   | +048
        bsr.w   CellMap_ClearVramBlock_051f02   | +04c
        lea     0x32(a0),a1                     | +050
        moveq   #31,d7                          | +054
.L051b14:
        clr.b   (a1)+                           | +056
        dbra    d7,.L051b14                     | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  CellMap_SetCursorAndClear_051b1c  @ $051B1C  (100 B)
| ----------------------------------------------------------------------------
        .section .text.CellMap_SetCursorAndClear_051b1c, "ax", @progbits
        .global CellMap_SetCursorAndClear_051b1c
CellMap_SetCursorAndClear_051b1c:
        andi.b  #0x3f,d0                        | +000
        move.b  d0,0x27(a0)                     | +004
        add.w   0x24(a0),d1                     | +008
        neg.w   d1                              | +00c
        andi.w  #0x1f,d1                        | +00e
        move.b  d1,0x26(a0)                     | +012
        clr.l   0x12(a0)                        | +016
        rts                                     | +01a
        move.l  a1,0x12(a0)                     | +01c
        rts                                     | +020
        move.l  a1,0xe(a0)                      | +022
        move.w  0x4(a0),d2                      | +026
        sub.w   d0,d2                           | +02a
        lsr.w   #0x4,d2                         | +02c
        add.w   0x22(a0),d2                     | +02e
        andi.w  #0x1f,d2                        | +032
        move.w  d2,0x22(a0)                     | +036
        move.w  0x8(a0),d2                      | +03a
        sub.w   d1,d2                           | +03e
        lsr.w   #0x4,d2                         | +040
        add.w   0x24(a0),d2                     | +042
        andi.w  #0x1f,d2                        | +046
        move.w  d2,0x24(a0)                     | +04a
        move.w  d0,0x4(a0)                      | +04e
        move.w  d1,0x8(a0)                      | +052
        clr.w   0x1e(a0)                        | +056
        clr.w   0x20(a0)                        | +05a
        moveq   #32,d0                          | +05e
        moveq   #0,d1                           | +060
        bra.b   CellMap_SetCursorAndClear_051b1c | +062

| ----------------------------------------------------------------------------
|  Nop_Rts_051c80  @ $051C80  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Nop_Rts_051c80, "ax", @progbits
        .global Nop_Rts_051c80
Nop_Rts_051c80:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  ClearC_Rts_051cf0  @ $051CF0  (6 B)
| ----------------------------------------------------------------------------
        .section .text.ClearC_Rts_051cf0, "ax", @progbits
        .global ClearC_Rts_051cf0
ClearC_Rts_051cf0:
        andi.b  #0xfe,ccr                       | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  ClearC_Rts_051d7e  @ $051D7E  (6 B)
| ----------------------------------------------------------------------------
        .section .text.ClearC_Rts_051d7e, "ax", @progbits
        .global ClearC_Rts_051d7e
ClearC_Rts_051d7e:
        andi.b  #0xfe,ccr                       | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  CellMap_ClipRectToWindow_051d84  @ $051D84  (82 B)
| ----------------------------------------------------------------------------
        .section .text.CellMap_ClipRectToWindow_051d84, "ax", @progbits
        .global CellMap_ClipRectToWindow_051d84
CellMap_ClipRectToWindow_051d84:
        add.w   d2,d4                           | +000
        move.w  (a1),d6                         | +002
        cmp.w   d2,d6                           | +004
        bcc.w   .L051d98                        | +006
        cmp.w   d2,d4                           | +00a
        bhi.w   ClearC_051ddc                   | +00c
        sub.w   d2,d0                           | +010
        clr.w   d2                              | +012
.L051d98:
        cmp.w   d4,d6                           | +014
        bcc.w   .L051da0                        | +016
        move.w  d6,d4                           | +01a
.L051da0:
        sub.w   d2,d4                           | +01c
        beq.w   ClearC_051ddc                   | +01e
        add.w   d3,d5                           | +022
        move.w  0x2(a1),d6                      | +024
        cmp.w   d3,d6                           | +028
        bcc.w   .L051dbc                        | +02a
        cmp.w   d3,d5                           | +02e
        bhi.w   ClearC_051ddc                   | +030
        sub.w   d3,d1                           | +034
        clr.w   d3                              | +036
.L051dbc:
        cmp.w   d5,d6                           | +038
        bcc.w   .L051dc4                        | +03a
        move.w  d6,d5                           | +03e
.L051dc4:
        sub.w   d3,d5                           | +040
        beq.w   ClearC_051ddc                   | +042
        andi.l  #0xffff,d4                      | +046
        andi.l  #0xffff,d5                      | +04c

| ----------------------------------------------------------------------------
|  CellMap_BlitRectToLSPC_051e74  @ $051E74  (90 B)
| ----------------------------------------------------------------------------
        .section .text.CellMap_BlitRectToLSPC_051e74, "ax", @progbits
        .global CellMap_BlitRectToLSPC_051e74
CellMap_BlitRectToLSPC_051e74:
        andi.l  #0xffff,d4                      | +000
        andi.l  #0xffff,d5                      | +006
        subq.w  #0x1,d4                         | +00c
        subq.w  #0x1,d5                         | +00e
        add.w   0x22(a0),d0                     | +010
        add.w   0x24(a0),d1                     | +014
        clr.w   d7                              | +018
.L051e8e:
        movem.w d0-d1/d4-d5,-(a7)               | +01a
        andi.w  #0x1f,d0                        | +01e
        add.b   0x52(a0,d0.w),d1                | +022
        andi.w  #0x1f,d1                        | +026
        add.w   0x28(a0),d0                     | +02a
        lsl.w   #0x6,d0                         | +02e
        addi.w  #0x40,d0                        | +030
        add.w   d1,d1                           | +034
.L051eaa:
        move.w  d0,d6                           | +036
        add.w   d1,d6                           | +038
        movem.w d6-d7,0x3c0000.l                | +03a
        addq.w  #0x2,d1                         | +042
        andi.w  #0x3f,d1                        | +044
        dbra    d5,.L051eaa                     | +048
        movem.w (a7)+,d0-d1/d4-d5               | +04c
        addq.w  #0x1,d0                         | +050
        adda.w  d6,a3                           | +052
        dbra    d4,.L051e8e                     | +054
        rts                                     | +058

| ----------------------------------------------------------------------------
|  CellMap_SetDirty_051ece  @ $051ECE  (8 B)
| ----------------------------------------------------------------------------
        .section .text.CellMap_SetDirty_051ece, "ax", @progbits
        .global CellMap_SetDirty_051ece
CellMap_SetDirty_051ece:
        bset    #0x0,0xc(a0)                    | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  CellMap_ClearSprites32_051ed6  @ $051ED6  (44 B)
| ----------------------------------------------------------------------------
        .section .text.CellMap_ClearSprites32_051ed6, "ax", @progbits
        .global CellMap_ClearSprites32_051ed6
CellMap_ClearSprites32_051ed6:
        bclr    #0x0,0xc(a0)                    | +000
        moveq   #31,d1                          | +006
        move.w  0x28(a0),d0                     | +008
        addi.w  #0x8201,d0                      | +00c
.L051ee6:
        move.w  d0,0x106ee4.l                   | +010
        move.w  d0,0x3c0000.l                   | +016
        move.w  #0x0,0x3c0002.l                 | +01c
        addq.w  #0x1,d0                         | +024
        dbra    d1,.L051ee6                     | +026
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  CellMap_ClearVramBlock_051f02  @ $051F02  (46 B)
| ----------------------------------------------------------------------------
        .section .text.CellMap_ClearVramBlock_051f02, "ax", @progbits
        .global CellMap_ClearVramBlock_051f02
CellMap_ClearVramBlock_051f02:
        move.l  #0x3ff,d7                       | +000
        move.w  0x28(a0),d0                     | +006
        lsl.w   #0x6,d0                         | +00a
        addi.w  #0x40,d0                        | +00c
        swap    d0                              | +010
        clr.w   d0                              | +012
        moveq   #1,d1                           | +014
        swap    d1                              | +016
.L051f1a:
        move.l  d0,0x3c0000.l                   | +018
        add.l   d1,d0                           | +01e
        move.l  d0,0x3c0000.l                   | +020
        add.l   d1,d0                           | +026
        dbra    d7,.L051f1a                     | +028
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  Scheduler_CompareField10_052032  @ $052032  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Scheduler_CompareField10_052032, "ax", @progbits
        .global Scheduler_CompareField10_052032
Scheduler_CompareField10_052032:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_052048                    | +00c

| ----------------------------------------------------------------------------
|  FixOverlay_DrawCreditsOrFree_052050  @ $052050  (146 B)
| ----------------------------------------------------------------------------
        .section .text.FixOverlay_DrawCreditsOrFree_052050, "ax", @progbits
        .global FixOverlay_DrawCreditsOrFree_052050
FixOverlay_DrawCreditsOrFree_052050:
        tst.b   0x10fd82.l                      | +000
        beq.w   .L052068                        | +006
        move.b  0x10fd91.l,d0                   | +00a
        andi.b  #0x2,d0                         | +010
        bne.w   FixOverlay_PutDigit_052132__L052142 | +014
.L052068:
        btst    #0x1,0x1081be.l                 | +018
        bne.w   FixOverlay_PutDigit_052132__L052144 | +020
        btst    #0x0,0x1081be.l                 | +024
        beq.w   FixOverlay_PutDigit_052132__L052142 | +02c
        bsr.w   FixOverlay_ClearCreditsArea_0521b2 | +030
        tst.b   0x10fd82.l                      | +034
        beq.w   .L0520a6                        | +03a
        lea     0x10fe00.l,a1                   | +03e
        tst.w   0x10fe80.l                      | +044
        bne.b   .L0520ac                        | +04a
        lea     0xd00034.l,a1                   | +04c
        bra.w   .L0520ac                        | +052
.L0520a6:
        lea     0x1081bf.l,a1                   | +056
.L0520ac:
        move.w  #0x2300,d1                      | +05c
        btst    #0x4,0x1081be.l                 | +060
        beq.w   .L0520c0                        | +068
        move.w  #0x2300,d1                      | +06c
.L0520c0:
        tst.b   0x10fd82.l                      | +070
        beq.w   .L0520d4                        | +076
        cmpi.b  #0x1,0x10fd83.l                 | +07a
        bne.b   .L0520de                        | +082
.L0520d4:
        move.w  #0x707d,d0                      | +084
        bsr.w   FixOverlay_PutCreditsLine_0520e2 | +088
        addq.l  #0x1,a1                         | +08c
.L0520de:
        move.w  #0x737d,d0                      | +08e

| ----------------------------------------------------------------------------
|  FixOverlay_PutCreditsLine_0520e2  @ $0520E2  (80 B)
| ----------------------------------------------------------------------------
        .section .text.FixOverlay_PutCreditsLine_0520e2, "ax", @progbits
        .global FixOverlay_PutCreditsLine_0520e2
FixOverlay_PutCreditsLine_0520e2:
        lea     Str_CREDITS_052230(pc),a2       | +000
        moveq   #5,d7                           | +004
.L0520e8:
        move.b  (a2)+,d1                        | +006
        movem.w d0-d1,0x3c0000.l                | +008
        addi.w  #0x20,d0                        | +010
        dbra    d7,.L0520e8                     | +014
        move.b  (a2)+,d1                        | +018
        move.b  (a1),d2                         | +01a
        cmpi.b  #0x2,d2                         | +01c
        bcc.w   .L05210a                        | +020
        move.b  #0x20,d1                        | +024
.L05210a:
        movem.w d0-d1,0x3c0000.l                | +028
        addi.w  #0x20,d0                        | +030
        move.b  (a2)+,d1                        | +034
        movem.w d0-d1,0x3c0000.l                | +036
        addi.w  #0x20,d0                        | +03e
        move.b  d2,d1                           | +042
        lsr.b   #0x4,d1                         | +044
        bsr.w   FixOverlay_PutDigit_052132      | +046
        andi.b  #0xf,d2                         | +04a
        move.b  d2,d1                           | +04e

| ----------------------------------------------------------------------------
|  FixOverlay_PutDigit_052132  @ $052132  (128 B)
| ----------------------------------------------------------------------------
        .section .text.FixOverlay_PutDigit_052132, "ax", @progbits
        .global FixOverlay_PutDigit_052132
FixOverlay_PutDigit_052132:
        addi.b  #0x30,d1                        | +000
        movem.w d0-d1,0x3c0000.l                | +004
        addi.w  #0x20,d0                        | +00c
        .global FixOverlay_PutDigit_052132__L052142
FixOverlay_PutDigit_052132__L052142:
.L052142:
        rts                                     | +010
        .global FixOverlay_PutDigit_052132__L052144
FixOverlay_PutDigit_052132__L052144:
.L052144:
        move.w  #0x701d,d0                      | +012
        moveq   #15,d7                          | +016
        move.w  #0x20,d1                        | +018
        ori.w   #0x2300,d1                      | +01c
        btst    #0x4,0x1081be.l                 | +020
        beq.w   .L052166                        | +028
        move.w  #0x20,d1                        | +02c
        ori.w   #0x2300,d1                      | +030
.L052166:
        movem.w d0-d1,0x3c0000.l                | +034
        addi.w  #0x20,d0                        | +03c
        dbra    d7,.L052166                     | +040
        move.w  #0x72fd,d0                      | +044
        moveq   #16,d7                          | +048
        move.w  #0x20,d1                        | +04a
        ori.w   #0x2300,d1                      | +04e
        btst    #0x4,0x1081be.l                 | +052
        beq.w   .L052198                        | +05a
        move.w  #0x20,d1                        | +05e
        ori.w   #0x2300,d1                      | +062
.L052198:
        movem.w d0-d1,0x3c0000.l                | +066
        addi.w  #0x20,d0                        | +06e
        dbra    d7,.L052198                     | +072
        andi.b  #0xfc,0x1081be.l                | +076
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  FixOverlay_ClearCreditsArea_0521b2  @ $0521B2  (126 B)
| ----------------------------------------------------------------------------
        .section .text.FixOverlay_ClearCreditsArea_0521b2, "ax", @progbits
        .global FixOverlay_ClearCreditsArea_0521b2
FixOverlay_ClearCreditsArea_0521b2:
        move.w  #0x701d,d0                      | +000
        move.w  #0x2,d7                         | +004
        move.w  #0x20,d1                        | +008
        ori.w   #0x2300,d1                      | +00c
        btst    #0x4,0x1081be.l                 | +010
        beq.w   .L0521d6                        | +018
        move.w  #0x20,d1                        | +01c
        ori.w   #0x2300,d1                      | +020
.L0521d6:
        movem.w d0-d1,0x3c0000.l                | +024
        addi.w  #0x20,d0                        | +02c
        dbra    d7,.L0521d6                     | +030
        move.w  #0x719d,d0                      | +034
        move.w  #0x3,d7                         | +038
.L0521ee:
        movem.w d0-d1,0x3c0000.l                | +03c
        addi.w  #0x20,d0                        | +044
        dbra    d7,.L0521ee                     | +048
        move.w  #0x72fd,d0                      | +04c
        move.w  #0x3,d7                         | +050
.L052206:
        movem.w d0-d1,0x3c0000.l                | +054
        addi.w  #0x20,d0                        | +05c
        dbra    d7,.L052206                     | +060
        move.w  #0x749d,d0                      | +064
        move.w  #0x3,d7                         | +068
.L05221e:
        movem.w d0-d1,0x3c0000.l                | +06c
        addi.w  #0x20,d0                        | +074
        dbra    d7,.L05221e                     | +078
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  Str_CREDITS_052230  @ $052230  (186 B)
| ----------------------------------------------------------------------------
        .section .text.Str_CREDITS_052230, "ax", @progbits
        .global Str_CREDITS_052230
Str_CREDITS_052230:
        .dc.b   0x43                          | +000  'C'  (dato, rango --data)
        .dc.b   0x52                          | +001  'R'  (dato, rango --data)
        .dc.b   0x45                          | +002  'E'  (dato, rango --data)
        .dc.b   0x44                          | +003  'D'  (dato, rango --data)
        .dc.b   0x49                          | +004  'I'  (dato, rango --data)
        .dc.b   0x54                          | +005  'T'  (dato, rango --data)
        .dc.b   0x53                          | +006  'S'  (dato, rango --data)
        .dc.b   0x20                          | +007  ' '  (dato, rango --data)
        move.l  -(a0),d0                        | +008
        tst.b   0x10fd82.l                      | +00a
        beq.w   .L0522ac                        | +010
        move.b  0x10fd91.l,d0                   | +014
        andi.b  #0x1,d0                         | +01a
        bne.w   .L0522ac                        | +01e
        btst    #0x3,0x1081be.l                 | +022
        bne.w   .L0522ae                        | +02a
        btst    #0x2,0x1081be.l                 | +02e
        beq.w   .L0522ac                        | +036
        move.w  #0x721d,d0                      | +03a
        lea     FixOverlay_PutPauseTail_0522ea(pc),a2 | +03e
        moveq   #5,d7                           | +042
        move.w  #0x2300,d1                      | +044
        btst    #0x4,0x1081be.l                 | +048
        beq.w   .L052288                        | +050
        move.w  #0x2300,d1                      | +054
.L052288:
        move.b  (a2)+,d1                        | +058
        movem.w d0-d1,0x3c0000.l                | +05a
        addi.w  #0x20,d0                        | +062
        dbra    d7,.L052288                     | +066
        move.b  0x10fd8b.l,d1                   | +06a
        addi.b  #0x31,d1                        | +070
        movem.w d0-d1,0x3c0000.l                | +074
.L0522ac:
        rts                                     | +07c
.L0522ae:
        move.w  #0x721d,d0                      | +07e
        moveq   #7,d7                           | +082
        move.w  #0x20,d1                        | +084
        ori.w   #0x2300,d1                      | +088
        btst    #0x4,0x1081be.l                 | +08c
        beq.w   .L0522d0                        | +094
        move.w  #0x20,d1                        | +098
        ori.w   #0x2300,d1                      | +09c
.L0522d0:
        movem.w d0-d1,0x3c0000.l                | +0a0
        addi.w  #0x20,d0                        | +0a8
        dbra    d7,.L0522d0                     | +0ac
        andi.b  #0xf3,0x1081be.l                | +0b0
        rts                                     | +0b8

| ----------------------------------------------------------------------------
|  FixOverlay_PutPauseTail_0522ea  @ $0522EA  (2 B)
| ----------------------------------------------------------------------------
        .section .text.FixOverlay_PutPauseTail_0522ea, "ax", @progbits
        .global FixOverlay_PutPauseTail_0522ea
FixOverlay_PutPauseTail_0522ea:
        move.l  a3,(a0)                         | +000

| ----------------------------------------------------------------------------
|  Str_PauseTiles_0522ec  @ $0522EC  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Str_PauseTiles_0522ec, "ax", @progbits
        .global Str_PauseTiles_0522ec
Str_PauseTiles_0522ec:
        .dc.b   0x8c                          | +000  '.'  (dato, rango --data)
        .dc.b   0x8d                          | +001  '.'  (dato, rango --data)
        .dc.b   0x8e                          | +002  '.'  (dato, rango --data)
        .dc.b   0x8f                          | +003  '.'  (dato, rango --data)
        movea.l #0x7226,a1                      | +004
        lea     Str_PAUSE_05231c(pc),a2         | +00a
        move.w  #0x300,d0                       | +00e

| ----------------------------------------------------------------------------
|  FixOverlay_ClearPause_052306  @ $052306  (14 B)
| ----------------------------------------------------------------------------
        .section .text.FixOverlay_ClearPause_052306, "ax", @progbits
        .global FixOverlay_ClearPause_052306
FixOverlay_ClearPause_052306:
        movea.l #0x7226,a1                      | +000
        lea     Str_PAUSE_05231c__L052322(pc),a2 | +006
        move.w  #0x0,d0                         | +00a

| ----------------------------------------------------------------------------
|  Str_PAUSE_05231c  @ $05231C  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Str_PAUSE_05231c, "ax", @progbits
        .global Str_PAUSE_05231c
Str_PAUSE_05231c:
        .dc.b   0x50                          | +000  'P'  (dato, rango --data)
        .dc.b   0x41                          | +001  'A'  (dato, rango --data)
        .dc.b   0x55                          | +002  'U'  (dato, rango --data)
        .dc.b   0x53                          | +003  'S'  (dato, rango --data)
        .dc.b   0x45                          | +004  'E'  (dato, rango --data)
        .dc.b   0xfe                          | +005  '.'  (dato, rango --data)
        .global Str_PAUSE_05231c__L052322
Str_PAUSE_05231c__L052322:
.L052322:
        .dc.b   0xff                          | +006  '.'  (dato, rango --data)
        .dc.b   0xff                          | +007  '.'  (dato, rango --data)
        .dc.b   0xff                          | +008  '.'  (dato, rango --data)
        .dc.b   0xff                          | +009  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00a  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +00b  '.'  (dato, rango --data)
        moveq   #0,d0                           | +00c
        tst.b   0x10fd82.l                      | +00e
        bne.w   .L05233a                        | +014
        bset    #0x2,d0                         | +018
        rts                                     | +01c
.L05233a:
        move.b  #0x1,0x10fdb0.l                 | +01e
        move.b  #0x1,0x10fdb1.l                 | +026
        move.b  #0x0,0x10fdb2.l                 | +02e
        move.b  #0x0,0x10fdb3.l                 | +036
        move.l  d0,-(a7)                        | +03e
        jsr     0xc00450.l                      | +040
        move.l  (a7)+,d0                        | +046
        tst.b   0x10fdb0.l                      | +048
        beq.w   .L052372                        | +04e
        bset    #0x0,d0                         | +052
.L052372:
        tst.b   0x10fdb1.l                      | +056
        beq.w   .L052380                        | +05c
        bset    #0x1,d0                         | +060
.L052380:
        rts                                     | +064

| ----------------------------------------------------------------------------
|  Scheduler_CompareField10_052382  @ $052382  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Scheduler_CompareField10_052382, "ax", @progbits
        .global Scheduler_CompareField10_052382
Scheduler_CompareField10_052382:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_052398                    | +00c

| ----------------------------------------------------------------------------
|  PalFade_SpawnIn_0523c6  @ $0523C6  (20 B)
| ----------------------------------------------------------------------------
        .section .text.PalFade_SpawnIn_0523c6, "ax", @progbits
        .global PalFade_SpawnIn_0523c6
PalFade_SpawnIn_0523c6:
        move.l  d0,-(a7)                        | +000
        lea     PalFade_In_Task_0523fa(pc),a1   | +002
        jsr     0x4ae.l                         | +006
        move.l  (a7)+,d0                        | +00c
        move.w  d0,0x70(a0)                     | +00e
        rts                                     | +012

| ----------------------------------------------------------------------------
|  PalFade_SpawnOut_0523da  @ $0523DA  (20 B)
| ----------------------------------------------------------------------------
        .section .text.PalFade_SpawnOut_0523da, "ax", @progbits
        .global PalFade_SpawnOut_0523da
PalFade_SpawnOut_0523da:
        move.l  d0,-(a7)                        | +000
        lea     PalFade_Out_Task_0524b6(pc),a1  | +002
        jsr     0x4ae.l                         | +006
        move.l  (a7)+,d0                        | +00c
        move.w  d0,0x70(a0)                     | +00e
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Template_0523EE  @ $0523EE  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Template_0523EE, "ax", @progbits
        .global Template_0523EE
Template_0523EE:
        move.b  #0x1,0x10a2c8.l                 | +000
        bra.w   PalFade_In_Task_0523fa__L052402 | +008

| ----------------------------------------------------------------------------
|  PalFade_In_Task_0523fa  @ $0523FA  (176 B)
| ----------------------------------------------------------------------------
        .section .text.PalFade_In_Task_0523fa, "ax", @progbits
        .global PalFade_In_Task_0523fa
PalFade_In_Task_0523fa:
        move.b  #0xff,0x10a2c8.l                | +000
        .global PalFade_In_Task_0523fa__L052402
PalFade_In_Task_0523fa__L052402:
.L052402:
        moveq   #0,d0                           | +008
        move.b  d0,0x10a2ca.l                   | +00a
        move.b  d0,0x10a2cb.l                   | +010
        move.b  d0,0x10a2cc.l                   | +016
        move.b  d0,0x10a2cf.l                   | +01c
        moveq   #1,d1                           | +022
        cmpi.w  #0x3,0x70(a6)                   | +024
        bhi.w   .L05243e                        | +02a
        move.w  0x70(a6),d0                     | +02e
        add.w   d0,d0                           | +032
        add.w   d0,d0                           | +034
        lea     PalFade_SpeedTable_052570(pc),a0 | +036
        move.w  (a0,d0.w),d1                    | +03a
        move.w  0x2(a0,d0.w),0x70(a6)           | +03e
.L05243e:
        move.w  0x70(a6),0x72(a6)               | +044
        move.w  #0x0,0x74(a6)                   | +04a
        move.w  d1,0x8c(a6)                     | +050
        lea     .L052454(pc),a1                 | +054
        move.l  a1,(a6)                         | +058
.L052454:
        subq.w  #0x1,0x72(a6)                   | +05a
        bne.w   .L0524a8                        | +05e
        move.w  0x70(a6),0x72(a6)               | +062
        move.w  0x8c(a6),d0                     | +068
        add.b   d0,0x10a2ca.l                   | +06c
        add.b   d0,0x10a2cb.l                   | +072
        add.b   d0,0x10a2cc.l                   | +078
        add.w   d0,0x74(a6)                     | +07e
        cmpi.w  #0x1f,0x74(a6)                  | +082
        blt.w   .L0524a8                        | +088
        moveq   #31,d0                          | +08c
        move.b  d0,0x10a2ca.l                   | +08e
        move.b  d0,0x10a2cb.l                   | +094
        move.b  d0,0x10a2cc.l                   | +09a
        move.b  #0xff,0x10a2cf.l                | +0a0
        jmp     0x518.l                         | +0a8
.L0524a8:
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  Template_0524AA  @ $0524AA  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Template_0524AA, "ax", @progbits
        .global Template_0524AA
Template_0524AA:
        move.b  #0x1,0x10a2c8.l                 | +000
        bra.w   PalFade_Out_Task_0524b6__L0524be | +008

| ----------------------------------------------------------------------------
|  PalFade_Out_Task_0524b6  @ $0524B6  (86 B)
| ----------------------------------------------------------------------------
        .section .text.PalFade_Out_Task_0524b6, "ax", @progbits
        .global PalFade_Out_Task_0524b6
PalFade_Out_Task_0524b6:
        move.b  #0xff,0x10a2c8.l                | +000
        .global PalFade_Out_Task_0524b6__L0524be
PalFade_Out_Task_0524b6__L0524be:
.L0524be:
        moveq   #31,d0                          | +008
        move.b  d0,0x10a2ca.l                   | +00a
        move.b  d0,0x10a2cb.l                   | +010
        move.b  d0,0x10a2cc.l                   | +016
        move.b  #0x0,0x10a2cf.l                 | +01c
        moveq   #1,d1                           | +024
        cmpi.w  #0x3,0x70(a6)                   | +026
        bhi.w   .L0524fc                        | +02c
        move.w  0x70(a6),d0                     | +030
        add.w   d0,d0                           | +034
        add.w   d0,d0                           | +036
        lea     PalFade_SpeedTable_052570(pc),a0 | +038
        move.w  (a0,d0.w),d1                    | +03c
        move.w  0x2(a0,d0.w),0x70(a6)           | +040
.L0524fc:
        move.w  0x70(a6),0x72(a6)               | +046
        move.w  #0x0,0x74(a6)                   | +04c
        move.w  d1,0x8c(a6)                     | +052

| ----------------------------------------------------------------------------
|  PalFade_Out_Step_052514  @ $052514  (92 B)
| ----------------------------------------------------------------------------
        .section .text.PalFade_Out_Step_052514, "ax", @progbits
        .global PalFade_Out_Step_052514
PalFade_Out_Step_052514:
        subq.w  #0x1,0x72(a6)                   | +000
        bne.w   .L05256e                        | +004
        move.w  0x70(a6),0x72(a6)               | +008
        move.w  0x8c(a6),d0                     | +00e
        sub.b   d0,0x10a2ca.l                   | +012
        sub.b   d0,0x10a2cb.l                   | +018
        sub.b   d0,0x10a2cc.l                   | +01e
        add.w   d0,0x74(a6)                     | +024
        cmpi.w  #0x1f,0x74(a6)                  | +028
        blt.w   .L05256e                        | +02e
        moveq   #0,d0                           | +032
        move.b  d0,0x10a2c8.l                   | +034
        move.b  d0,0x10a2ca.l                   | +03a
        move.b  d0,0x10a2cb.l                   | +040
        move.b  d0,0x10a2cc.l                   | +046
        move.b  #0xff,0x10a2cf.l                | +04c
        jmp     0x518.l                         | +054
.L05256e:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  PalFade_SpeedTable_052570  @ $052570  (52 B)
| ----------------------------------------------------------------------------
        .section .text.PalFade_SpeedTable_052570, "ax", @progbits
        .global PalFade_SpeedTable_052570
PalFade_SpeedTable_052570:
        ori.b   #0x1,(a7)+                      | +000
        ori.b   #0x1,d4                         | +004
        ori.b   #0x1,d2                         | +008
        ori.b   #0x1,d1                         | +00c
        movem.l d0-d3,-(a7)                     | +010
        lea     PalFade_ToColor_Task_0525a4(pc),a1 | +014
        jsr     0x4ae.l                         | +018
        movem.l (a7)+,d0-d3                     | +01e
        move.w  d0,0x7c(a0)                     | +022
        move.w  d1,0x7e(a0)                     | +026
        move.w  d2,0x80(a0)                     | +02a
        move.w  d3,0x70(a0)                     | +02e
        rts                                     | +032

| ----------------------------------------------------------------------------
|  PalFade_ToColor_Task_0525a4  @ $0525A4  (254 B)
| ----------------------------------------------------------------------------
        .section .text.PalFade_ToColor_Task_0525a4, "ax", @progbits
        .global PalFade_ToColor_Task_0525a4
PalFade_ToColor_Task_0525a4:
        move.b  #0x80,0x10a2c8.l                | +000
        move.b  0x10a2ca.l,0x88(a6)             | +008
        move.b  0x10a2cb.l,0x89(a6)             | +010
        move.b  0x10a2cc.l,0x8a(a6)             | +018
        moveq   #0,d0                           | +020
        move.w  d0,0x82(a6)                     | +022
        move.w  d0,0x84(a6)                     | +026
        move.w  d0,0x86(a6)                     | +02a
        moveq   #0,d0                           | +02e
        move.b  0x10a2ca.l,d0                   | +030
        asl.w   #0x8,d0                         | +036
        move.w  0x7c(a6),d1                     | +038
        asl.w   #0x8,d1                         | +03c
        sub.w   d1,d0                           | +03e
        ext.l   d0                              | +040
        divs.w  0x70(a6),d0                     | +042
        move.w  d0,0x76(a6)                     | +046
        moveq   #0,d0                           | +04a
        move.b  0x10a2cb.l,d0                   | +04c
        asl.w   #0x8,d0                         | +052
        move.w  0x7e(a6),d1                     | +054
        asl.w   #0x8,d1                         | +058
        sub.w   d1,d0                           | +05a
        ext.l   d0                              | +05c
        divs.w  0x70(a6),d0                     | +05e
        move.w  d0,0x78(a6)                     | +062
        moveq   #0,d0                           | +066
        move.b  0x10a2cc.l,d0                   | +068
        asl.w   #0x8,d0                         | +06e
        move.w  0x80(a6),d1                     | +070
        asl.w   #0x8,d1                         | +074
        sub.w   d1,d0                           | +076
        ext.l   d0                              | +078
        divs.w  0x70(a6),d0                     | +07a
        move.w  d0,0x7a(a6)                     | +07e
        lea     .L05262c(pc),a1                 | +082
        move.l  a1,(a6)                         | +086
.L05262c:
        subq.w  #0x1,0x70(a6)                   | +088
        beq.w   .L052684                        | +08c
        move.w  0x76(a6),d0                     | +090
        add.w   d0,0x82(a6)                     | +094
        move.w  0x78(a6),d0                     | +098
        add.w   d0,0x84(a6)                     | +09c
        move.w  0x7a(a6),d0                     | +0a0
        add.w   d0,0x86(a6)                     | +0a4
        move.w  0x82(a6),d0                     | +0a8
        asr.w   #0x8,d0                         | +0ac
        move.b  0x88(a6),d1                     | +0ae
        sub.b   d0,d1                           | +0b2
        move.b  d1,0x10a2ca.l                   | +0b4
        move.w  0x84(a6),d0                     | +0ba
        asr.w   #0x8,d0                         | +0be
        move.b  0x89(a6),d1                     | +0c0
        sub.b   d0,d1                           | +0c4
        move.b  d1,0x10a2cb.l                   | +0c6
        move.w  0x86(a6),d0                     | +0cc
        asr.w   #0x8,d0                         | +0d0
        move.b  0x8a(a6),d1                     | +0d2
        sub.b   d0,d1                           | +0d6
        move.b  d1,0x10a2cc.l                   | +0d8
        rts                                     | +0de
.L052684:
        move.w  0x7c(a6),d0                     | +0e0
        move.b  d0,0x10a2ca.l                   | +0e4
        move.w  0x7e(a6),d0                     | +0ea
        move.b  d0,0x10a2cb.l                   | +0ee
        move.w  0x80(a6),d0                     | +0f4
        move.b  d0,0x10a2cc.l                   | +0f8

| ----------------------------------------------------------------------------
|  SpriteTable_Init256_0526b8  @ $0526B8  (90 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteTable_Init256_0526b8, "ax", @progbits
        .global SpriteTable_Init256_0526b8
SpriteTable_Init256_0526b8:
        lea     0x1082c8.l,a1                   | +000
        move.w  #0xff,d2                        | +006
        move.w  #0x8800,d0                      | +00a
.L0526c6:
        move.w  d0,(a1)+                        | +00e
        clr.w   (a1)+                           | +010
        clr.l   (a1)+                           | +012
        clr.l   (a1)+                           | +014
        clr.l   (a1)+                           | +016
        clr.l   (a1)+                           | +018
        clr.l   (a1)+                           | +01a
        clr.l   (a1)+                           | +01c
        clr.l   (a1)+                           | +01e
        dbra    d2,.L0526c6                     | +020
        lea     0x1081c2.l,a1                   | +024
        move.w  #0x30,d1                        | +02a
        move.w  d1,0x1082c2.l                   | +02e
        subq.w  #0x1,d1                         | +034
        clr.b   d2                              | +036
.L0526f0:
        move.b  d2,(a1)+                        | +038
        addq.b  #0x1,d2                         | +03a
        dbra    d1,.L0526f0                     | +03c
        moveq   #0,d0                           | +040
        move.w  d0,0x1082c4.l                   | +042
        move.w  d0,0x1082c6.l                   | +048
        move.b  d0,0x10a2c9.l                   | +04e
        move.b  d0,0x10a2d1.l                   | +054

| ----------------------------------------------------------------------------
|  Scheduler_CompareField10_05273a  @ $05273A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Scheduler_CompareField10_05273a, "ax", @progbits
        .global Scheduler_CompareField10_05273a
Scheduler_CompareField10_05273a:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_052750                    | +00c

| ----------------------------------------------------------------------------
|  Sprite_SetupSlotFromTableA_Thunk_052756  @ $052756  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Sprite_SetupSlotFromTableA_Thunk_052756, "ax", @progbits
        .global Sprite_SetupSlotFromTableA_Thunk_052756
Sprite_SetupSlotFromTableA_Thunk_052756:
        move.w  (a0),d1                         | +000
        move.w  0x2(a0),d2                      | +002
        move.w  0x4(a0),d3                      | +006
        move.w  #0x1,d4                         | +00a

| ----------------------------------------------------------------------------
|  Entity_SpawnAndPublishD0At70_Thunk_05276c  @ $05276C  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SpawnAndPublishD0At70_Thunk_05276c, "ax", @progbits
        .global Entity_SpawnAndPublishD0At70_Thunk_05276c
Entity_SpawnAndPublishD0At70_Thunk_05276c:
        move.w  (a0),d0                         | +000

| ----------------------------------------------------------------------------
|  Entity_SpawnAndPublishD0At70_Thunk_052776  @ $052776  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SpawnAndPublishD0At70_Thunk_052776, "ax", @progbits
        .global Entity_SpawnAndPublishD0At70_Thunk_052776
Entity_SpawnAndPublishD0At70_Thunk_052776:
        move.w  (a0),d0                         | +000

| ----------------------------------------------------------------------------
|  Task_AllocProp86586_052788  @ $052788  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Task_AllocProp86586_052788, "ax", @progbits
        .global Task_AllocProp86586_052788
Task_AllocProp86586_052788:
        lea     0x86586.l,a1                    | +000

| ----------------------------------------------------------------------------
|  Scheduler_CompareField10_05279e  @ $05279E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Scheduler_CompareField10_05279e, "ax", @progbits
        .global Scheduler_CompareField10_05279e
Scheduler_CompareField10_05279e:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0527b4                    | +00c
