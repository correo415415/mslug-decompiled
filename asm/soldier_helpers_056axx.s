| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $056ACC..$057D04  (4,214 B, 38 entradas, 20 huecos)
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
|  Soldier_PhysicsStep_056acc  @ $056ACC  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_PhysicsStep_056acc, "ax", @progbits
        .global Soldier_PhysicsStep_056acc
Soldier_PhysicsStep_056acc:
        jsr     0x27f08.l                       | +000
        bcc.w   .L056ade                        | +006
        move.b  d3,0x78(a6)                     | +00a
        bra.w   .L056af4                        | +00e
.L056ade:
        cmp.b   0x78(a6),d0                     | +012
        bne.w   .L056aee                        | +016
        move.b  d3,0x78(a6)                     | +01a
        bra.w   .L056af4                        | +01e
.L056aee:
        move.b  #0xff,0x78(a6)                  | +022
.L056af4:
        tst.b   0x78(a6)                        | +028
        beq.w   .L056b06                        | +02c
        jsr     0x28292.l                       | +030
        bra.w   .L056b10                        | +036
.L056b06:
        jsr     0x28364.l                       | +03a
        scs.b   0x78(a6)                        | +040
.L056b10:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Soldier_ApproxDist_056b12  @ $056B12  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ApproxDist_056b12, "ax", @progbits
        .global Soldier_ApproxDist_056b12
Soldier_ApproxDist_056b12:
        ext.l   d0                              | +000
        move.l  d0,d2                           | +002
        swap    d2                              | +004
        eor.w   d2,d0                           | +006
        ext.l   d1                              | +008
        move.l  d1,d2                           | +00a
        swap    d2                              | +00c
        eor.w   d2,d1                           | +00e
        cmp.w   d0,d1                           | +010
        bls.w   .L056b32                        | +012
        lsr.w   #0x1,d0                         | +016
        add.w   d1,d0                           | +018
        rts                                     | +01a
        bra.w   Soldier_FindNearestPlayer_056b38 | +01c
.L056b32:
        lsr.w   #0x1,d1                         | +020
        add.w   d1,d0                           | +022
        rts                                     | +024

| ----------------------------------------------------------------------------
|  Soldier_FindNearestPlayer_056b38  @ $056B38  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_FindNearestPlayer_056b38, "ax", @progbits
        .global Soldier_FindNearestPlayer_056b38
Soldier_FindNearestPlayer_056b38:
        clr.l   -(a7)                           | +000
        move.w  #0xffff,-(a7)                   | +002
        moveq   #0,d0                           | +006
        jsr     0x5e3a2.l                       | +008
        bcc.w   .L056b62                        | +00e
        move.w  0x22(a6),d0                     | +012
        sub.w   0x22(a0),d0                     | +016
        move.w  0x24(a6),d1                     | +01a
        sub.w   0x24(a6),d1                     | +01e
        bsr.b   Soldier_ApproxDist_056b12       | +022
        move.l  a0,0x2(a7)                      | +024
        move.w  d0,(a7)                         | +028
.L056b62:
        moveq   #1,d0                           | +02a
        jsr     0x5e3a2.l                       | +02c
        bcc.w   .L056b8c                        | +032
        move.w  0x22(a6),d0                     | +036
        sub.w   0x22(a0),d0                     | +03a
        move.w  0x24(a6),d1                     | +03e
        sub.w   0x24(a0),d1                     | +042
        bsr.b   Soldier_ApproxDist_056b12       | +046
        cmp.w   (a7),d0                         | +048
        bhi.w   .L056b8c                        | +04a
        move.w  d0,(a7)                         | +04e
        move.l  a0,0x2(a7)                      | +050
.L056b8c:
        move.w  (a7)+,d0                        | +054
        movea.l (a7)+,a0                        | +056
        rts                                     | +058

| ----------------------------------------------------------------------------
|  Soldier_Think_056b92  @ $056B92  (646 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Think_056b92, "ax", @progbits
        .global Soldier_Think_056b92
Soldier_Think_056b92:
        move.w  0x8c(a6),d0                     | +000
        beq.w   .L056ba0                        | +004
        subq.w  #0x1,d0                         | +008
        move.w  d0,0x8c(a6)                     | +00a
.L056ba0:
        move.w  0x8e(a6),d0                     | +00e
        beq.w   .L056bae                        | +012
        subq.w  #0x1,d0                         | +016
        move.w  d0,0x8e(a6)                     | +018
.L056bae:
        bclr    #0x5,0x72(a6)                   | +01c
        bclr    #0x6,0x72(a6)                   | +022
        btst    #0x5,0x5a(a6)                   | +028
        beq.w   .L056bfc                        | +02e
        move.w  0x22(a6),d1                     | +032
        move.w  0x24(a6),d2                     | +036
        btst    #0x0,0x3a(a6)                   | +03a
        bne.w   .L056bdc                        | +040
        subq.w  #0x3,d1                         | +044
        bra.w   .L056bde                        | +046
.L056bdc:
        addq.w  #0x3,d1                         | +04a
.L056bde:
        addi.w  #0x18,d2                        | +04c
        jsr     0x280c6.l                       | +050
        bcc.w   .L056bf6                        | +056
        bset    #0x5,0x72(a6)                   | +05a
        bra.w   .L056bfc                        | +060
.L056bf6:
        bset    #0x6,0x72(a6)                   | +064
.L056bfc:
        bsr.w   Soldier_FindNearestPlayer_056b38 | +06a
        move.w  d0,0x7e(a6)                     | +06e
        move.l  a0,d0                           | +072
        beq.w   .L056df0                        | +074
        bclr    #0x0,0x73(a6)                   | +078
        move.l  a0,0x7a(a6)                     | +07e
        move.w  0x22(a6),d0                     | +082
        sub.w   0x22(a0),d0                     | +086
        bmi.w   .L056c44                        | +08a
        bclr    #0x1,0x72(a6)                   | +08e
        btst    #0x0,0x3a(a6)                   | +094
        bne.w   .L056c3a                        | +09a
        bset    #0x0,0x72(a6)                   | +09e
        bra.w   .L056c40                        | +0a4
.L056c3a:
        bclr    #0x0,0x72(a6)                   | +0a8
.L056c40:
        bra.w   .L056c64                        | +0ae
.L056c44:
        bset    #0x1,0x72(a6)                   | +0b2
        btst    #0x0,0x3a(a6)                   | +0b8
        bne.w   .L056c5e                        | +0be
        bclr    #0x0,0x72(a6)                   | +0c2
        bra.w   .L056c64                        | +0c8
.L056c5e:
        bset    #0x0,0x72(a6)                   | +0cc
.L056c64:
        clr.l   0x88(a6)                        | +0d2
        clr.l   0x84(a6)                        | +0d6
        jsr     0x8f344.l                       | +0da
        bcc.w   .L056ce0                        | +0e0
        tst.b   d0                              | +0e4
        bpl.w   .L056c92                        | +0e6
        move.w  #0x400,0x84(a6)                 | +0ea
        bset    #0x2,0x73(a6)                   | +0f0
        bclr    #0x3,0x73(a6)                   | +0f6
        bra.w   .L056cd6                        | +0fc
.L056c92:
        jsr     0x5e9b6.l                       | +100
        tst.b   d0                              | +106
        bmi.w   .L056cb4                        | +108
        move.w  #0x400,0x86(a6)                 | +10c
        bclr    #0x2,0x73(a6)                   | +112
        bset    #0x3,0x73(a6)                   | +118
        bra.w   .L056cd6                        | +11e
.L056cb4:
        andi.w  #0x3ff,d0                       | +122
        move.w  d0,0x84(a6)                     | +126
        jsr     0x5e9b6.l                       | +12a
        andi.w  #0x3ff,d0                       | +130
        move.w  d0,0x86(a6)                     | +134
        move.w  #0x80,0x88(a6)                  | +138
        move.w  #0x80,0x8a(a6)                  | +13e
.L056cd6:
        bset    #0x1,0x73(a6)                   | +144
        bra.w   .L056ce6                        | +14a
.L056ce0:
        andi.b  #0xf1,0x73(a6)                  | +14e
.L056ce6:
        move.b  0x98(a6),d0                     | +154
        beq.w   .L056d16                        | +158
        cmpi.b  #0x1,d0                         | +15c
        beq.w   .L056d0c                        | +160
        cmpi.b  #0x2,d0                         | +164
        beq.w   .L056d20                        | +168
        jsr     0x5e9b6.l                       | +16c
        btst    #0x0,d0                         | +172
        beq.w   .L056d16                        | +176
.L056d0c:
        ori.b   #0x30,0x13(a6)                  | +17a
        bra.w   .L056d44                        | +180
.L056d16:
        andi.b  #0xcf,0x13(a6)                  | +184
        bra.w   .L056d44                        | +18a
.L056d20:
        move.b  0x70(a6),d0                     | +18e
        bmi.w   .L056d38                        | +192
        bset    #0x4,0x13(a6)                   | +196
        bclr    #0x5,0x13(a6)                   | +19c
        bra.w   .L056d44                        | +1a2
.L056d38:
        bset    #0x5,0x13(a6)                   | +1a6
        bclr    #0x4,0x13(a6)                   | +1ac
.L056d44:
        move.w  0x7e(a6),d0                     | +1b2
        move.w  d0,d1                           | +1b6
        move.w  0x90(a6),d2                     | +1b8
        move.b  0x90(a6),d3                     | +1bc
        andi.w  #0x1f,d3                        | +1c0
        andi.w  #0x1f,d2                        | +1c4
        add.w   d3,d2                           | +1c8
        addi.w  #0x10,d2                        | +1ca
        sub.w   d2,d0                           | +1ce
        bpl.w   .L056d68                        | +1d0
        clr.w   d0                              | +1d4
.L056d68:
        lsl.w   #0x2,d0                         | +1d6
        cmpi.w  #0x100,d0                       | +1d8
        bcs.w   .L056d76                        | +1dc
        move.w  #0x100,d0                       | +1e0
.L056d76:
        neg.w   d0                              | +1e4
        addi.w  #0x100,d0                       | +1e6
        move.w  d0,0x80(a6)                     | +1ea
        moveq   #0,d0                           | +1ee
        move.w  d1,d0                           | +1f0
        move.w  0x90(a6),d2                     | +1f2
        move.b  0x90(a6),d3                     | +1f6
        andi.w  #0x1f,d3                        | +1fa
        andi.w  #0x1f,d2                        | +1fe
        add.w   d3,d2                           | +202
        addi.w  #0x10,d2                        | +204
        sub.w   d2,d0                           | +208
        bpl.w   .L056da2                        | +20a
        clr.l   d0                              | +20e
.L056da2:
        move.b  0x9c(a6),d7                     | +210
        andi.l  #0x7,d7                         | +214
        beq.w   .L056db2                        | +21a
        lsl.l   d7,d0                           | +21e
.L056db2:
        lsl.l   #0x1,d0                         | +220
        cmpi.l  #0x100,d0                       | +222
        bcs.w   .L056dc2                        | +228
        move.w  #0x100,d0                       | +22c
.L056dc2:
        move.w  d0,0x82(a6)                     | +230
        move.w  0x22(a6),d0                     | +234
        cmpi.w  #0x30,d0                        | +238
        bmi.w   .L056ddc                        | +23c
        cmpi.w  #0x120,d0                       | +240
        bpl.w   .L056ddc                        | +244
        rts                                     | +248
.L056ddc:
        move.w  #0x100,0x82(a6)                 | +24a
        clr.w   0x80(a6)                        | +250
        clr.w   0x84(a6)                        | +254
        clr.w   0x86(a6)                        | +258
        rts                                     | +25c
.L056df0:
        bset    #0x0,0x73(a6)                   | +25e
        clr.l   0x7a(a6)                        | +264
        bset    #0x0,0x72(a6)                   | +268
        move.w  #0x20,d0                        | +26e
        move.w  d0,0x80(a6)                     | +272
        move.w  d0,0x82(a6)                     | +276
        move.w  d0,0x86(a6)                     | +27a
        move.w  d0,0x84(a6)                     | +27e
        move.w  d0,0x88(a6)                     | +282

| ----------------------------------------------------------------------------
|  Soldier_DespawnIfOffscreen_056e1e  @ $056E1E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_DespawnIfOffscreen_056e1e, "ax", @progbits
        .global Soldier_DespawnIfOffscreen_056e1e
Soldier_DespawnIfOffscreen_056e1e:
        lea     0x2b71d4.l,a0                   | +000
        jsr     0x5dd56.l                       | +006
        bcc.w   .L056e34                        | +00c
        jmp     0x518.l                         | +010
.L056e34:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Soldier_LeaveTimerExpired_056e36  @ $056E36  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_LeaveTimerExpired_056e36, "ax", @progbits
        .global Soldier_LeaveTimerExpired_056e36
Soldier_LeaveTimerExpired_056e36:
        tst.w   0x8e(a6)                        | +000
        beq.w   SetC_056e44                     | +004

| ----------------------------------------------------------------------------
|  Soldier_PickGrabAnchor_056e4a  @ $056E4A  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_PickGrabAnchor_056e4a, "ax", @progbits
        .global Soldier_PickGrabAnchor_056e4a
Soldier_PickGrabAnchor_056e4a:
        move.b  0x74(a6),d0                     | +000
        andi.b  #0x7,d0                         | +004
        beq.w   ClearC_056ecc                   | +008
        btst    #0x0,0x72(a6)                   | +00c
        beq.w   ClearC_056ecc                   | +012
        move.l  0x7a(a6),d0                     | +016
        beq.w   ClearC_056ecc                   | +01a
        movea.l d0,a0                           | +01e
        jsr     0x8f7ce.l                       | +020
        not.b   d0                              | +026
        move.b  0x74(a6),d1                     | +028
        andi.b  #0x7,d1                         | +02c
        and.b   d1,d0                           | +030
        beq.w   ClearC_056ecc                   | +032
        clr.w   d1                              | +036
        btst    #0x0,d0                         | +038
        beq.w   .L056e90                        | +03c
        move.w  #0x0,-(a7)                      | +040
        addq.w  #0x1,d1                         | +044
.L056e90:
        btst    #0x1,d0                         | +046
        beq.w   .L056e9e                        | +04a
        move.w  #0x1,-(a7)                      | +04e
        addq.w  #0x1,d1                         | +052
.L056e9e:
        btst    #0x2,d0                         | +054
        beq.w   .L056eac                        | +058
        move.w  #0x2,-(a7)                      | +05c
        addq.w  #0x1,d1                         | +060
.L056eac:
        move.w  d1,d0                           | +062
        movem.w d1,-(a7)                        | +064
        jsr     0x5e9e4.l                       | +068
        movem.w (a7)+,d1                        | +06e
        add.w   d0,d0                           | +072
        move.w  (a7,d0.w),d0                    | +074
        add.w   d1,d1                           | +078
        adda.w  d1,a7                           | +07a

| ----------------------------------------------------------------------------
|  Soldier_AnchorPickTbl_056ed2  @ $056ED2  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_AnchorPickTbl_056ed2, "ax", @progbits
        .global Soldier_AnchorPickTbl_056ed2
Soldier_AnchorPickTbl_056ed2:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0102                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0102                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0203                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Soldier_TestPlayerInSlug_056eda  @ $056EDA  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_TestPlayerInSlug_056eda, "ax", @progbits
        .global Soldier_TestPlayerInSlug_056eda
Soldier_TestPlayerInSlug_056eda:
        move.b  0x74(a6),d0                     | +000
        andi.b  #0x7,d0                         | +004
        beq.w   ClearC_056f0a                   | +008
        btst    #0x0,0x72(a6)                   | +00c
        beq.w   ClearC_056f0a                   | +012
        move.l  0x7a(a6),d0                     | +016
        beq.w   ClearC_056f0a                   | +01a
        movea.l d0,a0                           | +01e
        jsr     0x2ac0e.l                       | +020
        bcs.w   ClearC_056f0a                   | +026

| ----------------------------------------------------------------------------
|  Soldier_GrabStruggleProgress_056f10  @ $056F10  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabStruggleProgress_056f10, "ax", @progbits
        .global Soldier_GrabStruggleProgress_056f10
Soldier_GrabStruggleProgress_056f10:
        move.w  0x80(a6),d0                     | +000
        move.w  d0,0x82(a6)                     | +004
        beq.w   .L056f28                        | +008
        asr.w   #0x6,d0                         | +00c
        bne.w   .L056f24                        | +00e
        moveq   #1,d0                           | +012
.L056f24:
        sub.w   d0,0x80(a6)                     | +014
.L056f28:
        move.l  0x7a(a6),d7                     | +018
        beq.w   ClearC_056f5e                   | +01c
        movea.l d7,a0                           | +020
        movea.l 0xc(a0),a0                      | +022
        movea.l 0x72(a0),a0                     | +026
        move.b  0x3(a0),d0                      | +02a
        beq.w   ClearC_056f5e                   | +02e
        move.w  0x80(a6),d0                     | +032
        move.w  d0,d1                           | +036
        addi.w  #0x100,d0                       | +038
        move.w  d0,0x80(a6)                     | +03c
        cmpi.w  #0xb00,d0                       | +040
        bcs.w   ClearC_056f5e                   | +044

| ----------------------------------------------------------------------------
|  Soldier_TestGrabBreak_056f64  @ $056F64  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_TestGrabBreak_056f64, "ax", @progbits
        .global Soldier_TestGrabBreak_056f64
Soldier_TestGrabBreak_056f64:
        move.w  0x80(a6),d0                     | +000
        cmpi.w  #0x300,d0                       | +004
        bcc.w   SetC_056f84                     | +008
        tst.w   d0                              | +00c
        beq.w   ClearC_056f7e                   | +00e
        tst.w   0x82(a6)                        | +012
        beq.w   SetC_056f84                     | +016

| ----------------------------------------------------------------------------
|  Soldier_SetVelXByFacing_056f8a  @ $056F8A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SetVelXByFacing_056f8a, "ax", @progbits
        .global Soldier_SetVelXByFacing_056f8a
Soldier_SetVelXByFacing_056f8a:
        move.w  0x36(a6),d0                     | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   SetTaskW_056f9a                 | +00a
        neg.w   d0                              | +00e

| ----------------------------------------------------------------------------
|  Soldier_TestSurrender_056fa0  @ $056FA0  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_TestSurrender_056fa0, "ax", @progbits
        .global Soldier_TestSurrender_056fa0
Soldier_TestSurrender_056fa0:
        btst    #0x3,0x74(a6)                   | +000
        beq.w   ClearC_056fe6                   | +006
        move.w  0x22(a6),d0                     | +00a
        cmpi.w  #0x30,d0                        | +00e
        bmi.w   ClearC_056fe6                   | +012
        cmpi.w  #0x120,d0                       | +016
        bpl.w   ClearC_056fe6                   | +01a
        tst.w   0x8c(a6)                        | +01e
        bne.w   ClearC_056fe6                   | +022
        btst    #0x0,0x72(a6)                   | +026
        beq.w   ClearC_056fe6                   | +02c
        jsr     0x8f2a0.l                       | +030
        bcs.w   ClearC_056fe6                   | +036
        move.b  0x9a(a6),0x8d(a6)               | +03a

| ----------------------------------------------------------------------------
|  Soldier_ProbeWalkEdge_056fec  @ $056FEC  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ProbeWalkEdge_056fec, "ax", @progbits
        .global Soldier_ProbeWalkEdge_056fec
Soldier_ProbeWalkEdge_056fec:
        btst    #0x0,0x3a(a6)                   | +000
        seq.b   d1                              | +006
        move.b  0x28(a6),d2                     | +008
        smi.b   d2                              | +00c
        eor.b   d2,d1                           | +00e
        bne.w   SetHandlerRts_057042            | +010
        jsr     0x770cc.l                       | +014
        tst.w   d0                              | +01a
        bmi.w   SetHandlerRts_057042            | +01c
        move.b  d0,0x77(a6)                     | +020
        btst    #0x4,0x13(a6)                   | +024
        sne.b   d2                              | +02a
        and.b   d1,d2                           | +02c
        andi.b  #0x1,d2                         | +02e
        beq.w   Soldier_ProbeWalkEdge_Bit5_05702a | +032

| ----------------------------------------------------------------------------
|  Soldier_ProbeWalkEdge_Bit5_05702a  @ $05702A  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ProbeWalkEdge_Bit5_05702a, "ax", @progbits
        .global Soldier_ProbeWalkEdge_Bit5_05702a
Soldier_ProbeWalkEdge_Bit5_05702a:
        btst    #0x5,0x13(a6)                   | +000
        sne.b   d2                              | +006
        and.b   d1,d2                           | +008
        andi.b  #0x2,d2                         | +00a
        beq.w   SetHandlerRts_057042            | +00e

| ----------------------------------------------------------------------------
|  Soldier_InitCommon_0570a8  @ $0570A8  (204 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_InitCommon_0570a8, "ax", @progbits
        .global Soldier_InitCommon_0570a8
Soldier_InitCommon_0570a8:
        jsr     0x27f60.l                       | +000
        scc.b   0x78(a6)                        | +006
        bset    #0x3,0x6b(a6)                   | +00a
        lea     0x2b74ec.l,a0                   | +010
        jsr     0x799de.l                       | +016
        clr.w   d1                              | +01c
        move.b  0x99(a6),d1                     | +01e
        bne.w   .L0570d0                        | +022
        moveq   #-1,d0                          | +026
.L0570d0:
        mulu.w  d1,d0                           | +028
        lsr.l   #0x8,d0                         | +02a
        move.w  d0,0x8c(a6)                     | +02c
        lea     0x2b746a.l,a0                   | +030
        jsr     0x799de.l                       | +036
        clr.w   d1                              | +03c
        move.b  0x9a(a6),d1                     | +03e
        bne.w   .L0570f2                        | +042
        move.b  #0x1e,d1                        | +046
.L0570f2:
        mulu.w  d1,d0                           | +04a
        lsr.l   #0x8,d0                         | +04c
        cmpi.w  #0x100,d0                       | +04e
        bcs.w   .L057102                        | +052
        move.b  #0xff,d0                        | +056
.L057102:
        move.b  d0,0x9a(a6)                     | +05a
        moveq   #0,d0                           | +05e
        move.b  0x9b(a6),d0                     | +060
        bne.w   .L057114                        | +064
        move.w  #0xa,d0                         | +068
.L057114:
        lsl.w   #0x6,d0                         | +06c
        move.w  d0,0x8e(a6)                     | +06e
        jsr     0x13600.l                       | +072
        move.w  #0xe,d1                         | +078
        jsr     0x236e.l                        | +07c
        clr.w   0x2c(a6)                        | +082
        clr.w   0x2e(a6)                        | +086
        move.w  #0x8000,d0                      | +08a
        jsr     0x28134.l                       | +08e
        andi.w  #0xffe3,0x38(a6)                | +094
        ori.w   #0x18,0x38(a6)                  | +09a
        lea     0x776e2.l,a1                    | +0a0
        jsr     0x4ae.l                         | +0a6
        clr.w   0x84(a6)                        | +0ac
        clr.w   0x86(a6)                        | +0b0
        bsr.w   Soldier_Think_056b92            | +0b4
        clr.b   0x74(a6)                        | +0b8
        jsr     0x5e9b6.l                       | +0bc
        move.w  d0,0x70(a6)                     | +0c2
        jsr     0x5e9b6.l                       | +0c6

| ----------------------------------------------------------------------------
|  Soldier_SpawnVariants_057226  @ $057226  (392 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnVariants_057226, "ax", @progbits
        .global Soldier_SpawnVariants_057226
Soldier_SpawnVariants_057226:
        jsr     0x13600.l                       | +000
        bset    #0x1,0x12(a6)                   | +006
        move.b  #0x2,0x98(a6)                   | +00c
        move.b  #0xa,0x99(a6)                   | +012
        move.b  #0x1e,0x9a(a6)                  | +018
        move.b  #0xa,0x9b(a6)                   | +01e
        bra.w   .L0573a2                        | +024
        jsr     0x13600.l                       | +028
        bset    #0x1,0x12(a6)                   | +02e
        move.b  #0x2,0x98(a6)                   | +034
        move.b  #0xa,0x99(a6)                   | +03a
        move.b  #0x1e,0x9a(a6)                  | +040
        move.b  #0xa,0x9b(a6)                   | +046
        bra.w   .L0572a6                        | +04c
        bsr.w   Soldier_InitCommon_0570a8       | +050
        bsr.w   EntityState_SetSubstate1_0571D8 | +054
        bra.w   Soldier_SpawnDispatch_05752c    | +058
        bsr.w   Soldier_InitCommon_0570a8       | +05c
        bsr.w   EntityState_SetSubstate2_0571C4 | +060
        bra.w   Soldier_SpawnDispatch_05752c    | +064
        bsr.w   Soldier_InitCommon_0570a8       | +068
        bsr.w   EntityState_SetSubstate3_0571EC | +06c
        bra.w   Soldier_SpawnDispatch_05752c    | +070
        bsr.w   Soldier_InitCommon_0570a8       | +074
        bsr.w   EntityState_SetState74Bit0_05720E | +078
        bra.w   Soldier_SpawnDispatch_05752c    | +07c
.L0572a6:
        bsr.w   Soldier_InitCommon_0570a8       | +080
        bsr.w   EntityState_SetState74Bit1_05721E | +084
        bra.w   Soldier_SpawnDispatch_05752c    | +088
        bsr.w   Soldier_InitCommon_0570a8       | +08c
        bsr.w   EntityState_SetState74Bit2_057216 | +090
        bra.w   Soldier_SpawnDispatch_05752c    | +094
        bsr.w   Soldier_InitCommon_0570a8       | +098
        bsr.w   EntityState_SetState74Bit0_05720E | +09c
        bsr.w   EntityState_SetState74Bit1_05721E | +0a0
        bsr.w   EntityState_SetState74Bit2_057216 | +0a4
        bra.w   Soldier_SpawnDispatch_05752c    | +0a8
        bsr.w   Soldier_InitCommon_0570a8       | +0ac
        bsr.w   EntityState_SetState74Bit0_05720E | +0b0
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +0b4
        bra.w   Soldier_SpawnDispatch_05752c    | +0b8
        bsr.w   Soldier_InitCommon_0570a8       | +0bc
        bsr.w   EntityState_SetState74Bit1_05721E | +0c0
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +0c4
        bra.w   Soldier_SpawnDispatch_05752c    | +0c8
        bsr.w   Soldier_InitCommon_0570a8       | +0cc
        bsr.w   EntityState_SetState74Bit2_057216 | +0d0
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +0d4
        bra.w   Soldier_SpawnDispatch_05752c    | +0d8
        bsr.w   Soldier_InitCommon_0570a8       | +0dc
        bsr.w   EntityState_SetState74Bit0_05720E | +0e0
        bsr.w   EntityState_SetState74Bit1_05721E | +0e4
        bsr.w   EntityState_SetState74Bit2_057216 | +0e8
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +0ec
        bra.w   Soldier_SpawnDispatch_05752c    | +0f0
        bsr.w   Soldier_InitCommon_0570a8       | +0f4
        bsr.w   EntityState_SetState74Bit0_05720E | +0f8
        bra.w   Soldier_SpawnDispatch_05752c    | +0fc
        bsr.w   Soldier_InitCommon_0570a8       | +100
        bsr.w   EntityState_SetState74Bit1_05721E | +104
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +108
        bsr.w   EntityState_SetState74Bit4_057200 | +10c
        bra.w   Soldier_SpawnDispatch_05752c    | +110
        bsr.w   Soldier_InitCommon_0570a8       | +114
        bsr.w   EntityState_SetState74Bit2_057216 | +118
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +11c
        bsr.w   EntityState_SetState74Bit4_057200 | +120
        bra.w   Soldier_SpawnDispatch_05752c    | +124
        bsr.w   Soldier_InitCommon_0570a8       | +128
        bsr.w   EntityState_SetState74Bit0_05720E | +12c
        bsr.w   EntityState_SetState74Bit1_05721E | +130
        bsr.w   EntityState_SetState74Bit2_057216 | +134
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +138
        bsr.w   EntityState_SetState74Bit4_057200 | +13c
        bra.w   Soldier_SpawnDispatch_05752c    | +140
        bclr    #0x0,0x3a(a6)                   | +144
        bra.w   .L05737a                        | +14a
        bset    #0x0,0x3a(a6)                   | +14e
.L05737a:
        bsr.w   Soldier_InitCommon_0570a8       | +154
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +158
        bra.w   Soldier_TauntInit_058e08        | +15c
        bsr.w   Soldier_InitCommon_0570a8       | +160
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +164
        bsr.w   EntityState_SetState74Bit4_057200 | +168
        bra.w   Soldier_SpawnDispatch_05752c    | +16c
        bsr.w   Soldier_InitCommon_0570a8       | +170
        bsr.w   EntityState_SetState74Bit4_057200 | +174
        bra.w   Soldier_SpawnDispatch_05752c    | +178
.L0573a2:
        bsr.w   Soldier_InitCommon_0570a8       | +17c
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +180
        bra.w   Soldier_SpawnDispatch_05752c    | +184

| ----------------------------------------------------------------------------
|  Soldier_SpawnAtGroundA_0573ae  @ $0573AE  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnAtGroundA_0573ae, "ax", @progbits
        .global Soldier_SpawnAtGroundA_0573ae
Soldier_SpawnAtGroundA_0573ae:
        bsr.w   Soldier_InitCommon_0570a8       | +000
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +004
        jsr     0x77148.l                       | +008
        tst.w   d0                              | +00e
        bpl.w   .L0573ce                        | +010
        jsr     0x5b6.l                         | +014
        jmp     0x518.l                         | +01a
.L0573ce:
        move.w  d6,0x22(a6)                     | +020
        move.b  d0,0x77(a6)                     | +024

| ----------------------------------------------------------------------------
|  Soldier_SpawnAtGroundB_0573de  @ $0573DE  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnAtGroundB_0573de, "ax", @progbits
        .global Soldier_SpawnAtGroundB_0573de
Soldier_SpawnAtGroundB_0573de:
        bsr.w   Soldier_InitCommon_0570a8       | +000
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +004
        jsr     0x77148.l                       | +008
        tst.w   d0                              | +00e
        bpl.w   .L0573fe                        | +010
        jsr     0x5b6.l                         | +014
        jmp     0x518.l                         | +01a
.L0573fe:
        move.w  d6,0x22(a6)                     | +020
        move.b  d0,0x77(a6)                     | +024

| ----------------------------------------------------------------------------
|  Soldier_PickFallAnim_05740e  @ $05740E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_PickFallAnim_05740e, "ax", @progbits
        .global Soldier_PickFallAnim_05740e
Soldier_PickFallAnim_05740e:
        move.w  0x28(a6),d0                     | +000
        bpl.w   .L057418                        | +004
        neg.w   d0                              | +008
.L057418:
        move.w  0x2a(a6),d1                     | +00a
        bpl.w   .L057422                        | +00e
        neg.w   d1                              | +012
.L057422:
        cmp.w   d1,d0                           | +014
        bcc.w   .L057432                        | +016
        lea     0x2b5c22.l,a0                   | +01a
        bra.w   JsrAbsThunk_057438              | +020
.L057432:
        lea     0x29b7c8.l,a0                   | +024

| ----------------------------------------------------------------------------
|  Soldier_AttackTblMeleeProbe_057440  @ $057440  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_AttackTblMeleeProbe_057440, "ax", @progbits
        .global Soldier_AttackTblMeleeProbe_057440
Soldier_AttackTblMeleeProbe_057440:
        .dc.w   0x000a                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Soldier_AttackTblMelee_057494  @ $057494  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_AttackTblMelee_057494, "ax", @progbits
        .global Soldier_AttackTblMelee_057494
Soldier_AttackTblMelee_057494:
        .dc.w   0x000a                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Soldier_TestMeleeRange_0574e8  @ $0574E8  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_TestMeleeRange_0574e8, "ax", @progbits
        .global Soldier_TestMeleeRange_0574e8
Soldier_TestMeleeRange_0574e8:
        move.w  0x22(a6),d0                     | +000
        cmpi.w  #0x20,d0                        | +004
        bmi.w   ClearC_057526                   | +008
        cmpi.w  #0x300,d0                       | +00c
        bpl.w   ClearC_057526                   | +010
        lea     Soldier_AttackTblMeleeProbe_057440(pc),a0 | +014
        move.l  a0,0x4c(a6)                     | +018
        jsr     0x283ca.l                       | +01c
        jsr     0x283ca.l                       | +022
        jsr     0x283d8.l                       | +028
        btst    #0x1,0x13(a6)                   | +02e
        beq.w   ClearC_057526                   | +034

| ----------------------------------------------------------------------------
|  Soldier_SpawnDispatch_05752c  @ $05752C  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnDispatch_05752c, "ax", @progbits
        .global Soldier_SpawnDispatch_05752c
Soldier_SpawnDispatch_05752c:
        move.w  0x22(a6),d0                     | +000
        cmpi.w  #0xa0,d0                        | +004
        bpl.w   .L05753e                        | +008
        bset    #0x0,0x3a(a6)                   | +00c
.L05753e:
        tst.b   0x78(a6)                        | +012
        beq.w   SetTaskHandler_057550           | +016
        lea     Soldier_WalkStart_057558(pc),a1 | +01a
        move.l  a1,(a6)                         | +01e
        bra.w   SetHandlerRts_057556            | +020

| ----------------------------------------------------------------------------
|  Soldier_WalkStart_057558  @ $057558  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_WalkStart_057558, "ax", @progbits
        .global Soldier_WalkStart_057558
Soldier_WalkStart_057558:
        lea     0x29b744.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2b756e.l,a0                   | +00c
        jsr     0x799de.l                       | +012
        move.w  d0,0x36(a6)                     | +018
        bsr.w   Soldier_SetVelXByFacing_056f8a  | +01c
        clr.w   0x2a(a6)                        | +020
        lea     Soldier_Walk_Loop_057582(pc),a1 | +024
        move.l  a1,(a6)                         | +028

| ----------------------------------------------------------------------------
|  Soldier_Walk_Loop_057582  @ $057582  (388 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Walk_Loop_057582, "ax", @progbits
        .global Soldier_Walk_Loop_057582
Soldier_Walk_Loop_057582:
        bsr.w   Soldier_PhysicsStep_056acc      | +000
        bsr.w   Soldier_Think_056b92            | +004
        bsr.w   Soldier_PickGrabAnchor_056e4a   | +008
        bcc.w   .L05759c                        | +00c
        lea     Soldier_GrabApproach_057728(pc),a1 | +010
        move.l  a1,(a6)                         | +014
        bra.w   Soldier_GrabApproach_057728     | +016
.L05759c:
        bsr.w   Soldier_TestPlayerInSlug_056eda | +01a
        bcc.w   .L0575ae                        | +01e
        lea     Soldier_SlugApproach_0577bc(pc),a1 | +022
        move.l  a1,(a6)                         | +026
        bra.w   Soldier_SlugApproach_0577bc     | +028
.L0575ae:
        jsr     0x28d70.l                       | +02c
        bsr.w   Soldier_ProbeWalkEdge_056fec    | +032
        btst    #0x0,0x72(a6)                   | +036
        beq.w   .L05767c                        | +03c
        move.w  0x82(a6),d1                     | +040
        cmp.w   0x84(a6),d1                     | +044
        bcc.w   .L0575d2                        | +048
        move.w  0x84(a6),d1                     | +04c
.L0575d2:
        neg.w   d1                              | +050
        addi.w  #0x100,d1                       | +052
        jsr     0x5e9b6.l                       | +056
        move.w  d0,-(a7)                        | +05c
        andi.w  #0xff,d0                        | +05e
        cmp.w   d1,d0                           | +062
        bcc.w   .L0575ee                        | +064
        move.b  (a7),d0                         | +068
        cmp.w   d1,d0                           | +06a
.L0575ee:
        addq.w  #0x2,a7                         | +06c
        bcc.w   .L0575fa                        | +06e
        lea     Soldier_RunToward_05804c(pc),a1 | +072
        move.l  a1,(a6)                         | +076
.L0575fa:
        jsr     0x5e9b6.l                       | +078
        move.w  d0,-(a7)                        | +07e
        andi.w  #0xff,d0                        | +080
        cmp.w   0x8a(a6),d0                     | +084
        bcc.w   .L057614                        | +088
        move.b  (a7),d0                         | +08c
        cmp.w   0x8a(a6),d0                     | +08e
.L057614:
        addq.w  #0x2,a7                         | +092
        bcc.w   .L057620                        | +094
        lea     Soldier_Flee_058658(pc),a1      | +098
        move.l  a1,(a6)                         | +09c
.L057620:
        move.w  0x86(a6),d1                     | +09e
        cmp.w   0x80(a6),d1                     | +0a2
        bcc.w   .L057630                        | +0a6
        move.w  0x80(a6),d1                     | +0aa
.L057630:
        jsr     0x5e9b6.l                       | +0ae
        move.w  d0,-(a7)                        | +0b4
        andi.w  #0xff,d0                        | +0b6
        cmp.w   d1,d0                           | +0ba
        bcc.w   .L057646                        | +0bc
        move.b  (a7),d0                         | +0c0
        cmp.w   d1,d0                           | +0c2
.L057646:
        addq.w  #0x2,a7                         | +0c4
        bcc.w   .L057652                        | +0c6
        lea     Soldier_Brake_0585f6(pc),a1     | +0ca
        move.l  a1,(a6)                         | +0ce
.L057652:
        jsr     0x5e9b6.l                       | +0d0
        move.w  d0,-(a7)                        | +0d6
        andi.w  #0xff,d0                        | +0d8
        cmp.w   0x88(a6),d0                     | +0dc
        bcc.w   .L05766c                        | +0e0
        move.b  (a7),d0                         | +0e4
        cmp.w   0x88(a6),d0                     | +0e6
.L05766c:
        addq.w  #0x2,a7                         | +0ea
        bcc.w   .L057678                        | +0ec
        lea     Soldier_Jump_0588f6(pc),a1      | +0f0
        move.l  a1,(a6)                         | +0f4
.L057678:
        bra.w   .L0576a2                        | +0f6
.L05767c:
        move.w  0x82(a6),d1                     | +0fa
        jsr     0x5e9b6.l                       | +0fe
        move.w  d0,-(a7)                        | +104
        andi.w  #0xff,d0                        | +106
        cmp.w   d1,d0                           | +10a
        bcc.w   .L057696                        | +10c
        move.b  (a7),d0                         | +110
        cmp.w   d1,d0                           | +112
.L057696:
        addq.w  #0x2,a7                         | +114
        bcc.w   .L0576a2                        | +116
        lea     Soldier_RunToward_05804c(pc),a1 | +11a
        move.l  a1,(a6)                         | +11e
.L0576a2:
        btst    #0x5,0x72(a6)                   | +120
        beq.w   .L0576b2                        | +126
        lea     Soldier_Brake_0585f6(pc),a1     | +12a
        move.l  a1,(a6)                         | +12e
.L0576b2:
        btst    #0x6,0x72(a6)                   | +130
        beq.w   .L0576c2                        | +136
        lea     Soldier_SurrenderFlee_0589f8__L058a50(pc),a1 | +13a
        move.l  a1,(a6)                         | +13e
.L0576c2:
        jsr     Soldier_LeaveTimerExpired_056e36(pc) | +140
        bcc.w   .L0576d0                        | +144
        lea     Soldier_SpawnFaceTarget_059062(pc),a1 | +148
        move.l  a1,(a6)                         | +14c
.L0576d0:
        jsr     Soldier_TestSurrender_056fa0(pc) | +14e
        bcc.w   .L0576de                        | +152
        lea     Soldier_Surrender_058968(pc),a1 | +156
        move.l  a1,(a6)                         | +15a
.L0576de:
        tst.b   0x78(a6)                        | +15c
        bne.w   .L0576ec                        | +160
        lea     Soldier_AirborneDispatch_057706(pc),a1 | +164
        move.l  a1,(a6)                         | +168
.L0576ec:
        jsr     Soldier_TestMeleeRange_0574e8(pc) | +16a
        bcc.w   .L0576fa                        | +16e
        lea     Soldier_MeleeAttack_0585ae(pc),a1 | +172
        move.l  a1,(a6)                         | +176
.L0576fa:
        jsr     0x49fd0.l                       | +178
        bsr.w   Soldier_DespawnIfOffscreen_056e1e | +17e
        rts                                     | +182

| ----------------------------------------------------------------------------
|  Soldier_AirborneDispatch_057706  @ $057706  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_AirborneDispatch_057706, "ax", @progbits
        .global Soldier_AirborneDispatch_057706
Soldier_AirborneDispatch_057706:
        jsr     0x5e9b6.l                       | +000
        lea     Soldier_AirborneJT_057718(pc),a0 | +006
        andi.w  #0xc,d0                         | +00a
        jmp     (a0,d0.w)                       | +00e

| ----------------------------------------------------------------------------
|  Soldier_AirborneJT_057718  @ $057718  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_AirborneJT_057718, "ax", @progbits
        .global Soldier_AirborneJT_057718
Soldier_AirborneJT_057718:
        bra.w   Soldier_SurrenderFlee_0589f8__L058a50 | +000
        bra.w   Soldier_Flee_058658             | +004
        bra.w   Soldier_Hurt_058412             | +008
        bra.w   Soldier_SurrenderFlee_0589f8__L058a50 | +00c

| ----------------------------------------------------------------------------
|  Soldier_GrabApproach_057728  @ $057728  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabApproach_057728, "ax", @progbits
        .global Soldier_GrabApproach_057728
Soldier_GrabApproach_057728:
        bsr.w   Soldier_PhysicsStep_056acc      | +000
        bsr.w   Soldier_Think_056b92            | +004
        jsr     0x28d70.l                       | +008
        bsr.w   Soldier_PickGrabAnchor_056e4a   | +00e
        bcc.w   .L0577aa                        | +012
        move.w  0x80(a6),d1                     | +016
        mulu.w  d1,d1                           | +01a
        lsr.l   #0x8,d1                         | +01c
        jsr     0x5e9b6.l                       | +01e
        move.w  d0,-(a7)                        | +024
        andi.w  #0xff,d0                        | +026
        cmp.w   d1,d0                           | +02a
        bcc.w   .L05775c                        | +02c
        move.b  (a7),d0                         | +030
        cmp.w   d1,d0                           | +032
.L05775c:
        addq.w  #0x2,a7                         | +034
        bcc.w   .L057768                        | +036
        lea     Soldier_Leap_057880__L0578dc(pc),a1 | +03a
        move.l  a1,(a6)                         | +03e
.L057768:
        btst    #0x0,0x72(a6)                   | +040
        bne.w   .L057778                        | +046
        lea     Soldier_Brake_0585f6(pc),a1     | +04a
        move.l  a1,(a6)                         | +04e
.L057778:
        tst.b   0x78(a6)                        | +050
        bne.w   .L057786                        | +054
        lea     Soldier_Leap_057880__L0578dc(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L057786:
        btst    #0x6,0x72(a6)                   | +05e
        beq.w   .L057796                        | +064
        lea     Soldier_SurrenderFlee_0589f8__L058a50(pc),a1 | +068
        move.l  a1,(a6)                         | +06c
.L057796:
        btst    #0x5,0x72(a6)                   | +06e
        beq.w   .L0577a6                        | +074
        lea     Soldier_RunToward_05804c(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L0577a6:
        bra.w   .L0577b0                        | +07e
.L0577aa:
        lea     Soldier_Walk_Loop_057582(pc),a1 | +082
        move.l  a1,(a6)                         | +086
.L0577b0:
        jsr     0x49fd0.l                       | +088

| ----------------------------------------------------------------------------
|  Soldier_SlugApproach_0577bc  @ $0577BC  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SlugApproach_0577bc, "ax", @progbits
        .global Soldier_SlugApproach_0577bc
Soldier_SlugApproach_0577bc:
        bsr.w   Soldier_PhysicsStep_056acc      | +000
        bsr.w   Soldier_Think_056b92            | +004
        jsr     0x28d70.l                       | +008
        move.w  0x80(a6),d1                     | +00e
        mulu.w  d1,d1                           | +012
        lsr.l   #0x8,d1                         | +014
        jsr     0x5e9b6.l                       | +016
        move.w  d0,-(a7)                        | +01c
        andi.w  #0xff,d0                        | +01e
        cmp.w   d1,d0                           | +022
        bcc.w   .L0577e8                        | +024
        move.b  (a7),d0                         | +028
        cmp.w   d1,d0                           | +02a
.L0577e8:
        addq.w  #0x2,a7                         | +02c
        bcc.w   .L0577f4                        | +02e
        lea     Soldier_Leap_057880__L05799e(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L0577f4:
        btst    #0x0,0x72(a6)                   | +038
        bne.w   .L057804                        | +03e
        lea     Soldier_Brake_0585f6(pc),a1     | +042
        move.l  a1,(a6)                         | +046
.L057804:
        tst.b   0x78(a6)                        | +048
        bne.w   .L057812                        | +04c
        lea     Soldier_Leap_057880__L05799e(pc),a1 | +050
        move.l  a1,(a6)                         | +054
.L057812:
        btst    #0x6,0x72(a6)                   | +056
        beq.w   .L057822                        | +05c
        lea     Soldier_SurrenderFlee_0589f8__L058a50(pc),a1 | +060
        move.l  a1,(a6)                         | +064
.L057822:
        btst    #0x5,0x72(a6)                   | +066
        beq.w   .L057832                        | +06c
        lea     Soldier_RunToward_05804c(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L057832:
        jsr     0x49fd0.l                       | +076

| ----------------------------------------------------------------------------
|  Soldier_SpawnJumpIn_05783e  @ $05783E  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnJumpIn_05783e, "ax", @progbits
        .global Soldier_SpawnJumpIn_05783e
Soldier_SpawnJumpIn_05783e:
        bsr.w   Soldier_InitCommon_0570a8       | +000
        bsr.w   EntityState_PublishByProbeN_ClearSub75_05719C | +004
        lea     0x2b69d8.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        move.w  #0xfe67,d0                      | +014
        jsr     0x5dca4.l                       | +018
        move.w  d0,0x28(a6)                     | +01e
        move.w  #0x190,0x2a(a6)                 | +022
        move.w  #0xffd8,0x2e(a6)                | +028
        move.w  #0x0,0x2c(a6)                   | +02e
        clr.w   0x2a(a6)                        | +034
        lea     Soldier_SurrenderFlee_0589f8__L058a82(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
        bra.w   Soldier_SurrenderFlee_0589f8__L058a82 | +03e

| ----------------------------------------------------------------------------
|  Soldier_Leap_057880  @ $057880  (542 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Leap_057880, "ax", @progbits
        .global Soldier_Leap_057880
Soldier_Leap_057880:
        bsr.w   Soldier_InitCommon_0570a8       | +000
        bsr.w   EntityState_SetState74Bit0_05720E | +004
        bsr.w   EntityState_SetState74Bit1_05721E | +008
        bsr.w   EntityState_SetState74Bit2_057216 | +00c
        lea     0x2b69aa.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        move.w  #0xfe67,d0                      | +01c
        jsr     0x5dca4.l                       | +020
        move.w  d0,0x28(a6)                     | +026
        move.w  #0x190,0x2a(a6)                 | +02a
        move.w  #0xffd8,0x2e(a6)                | +030
        move.w  #0x0,0x2c(a6)                   | +036
        clr.w   0x2a(a6)                        | +03c
        bsr.w   Soldier_Think_056b92            | +040
        bsr.w   Soldier_TestPlayerInSlug_056eda | +044
        bcc.w   .L057908                        | +048
        lea     0x2b6fee.l,a0                   | +04c
        jsr     0x28cd4.l                       | +052
        bra.w   .L057a5a                        | +058
        .global Soldier_Leap_057880__L0578dc
Soldier_Leap_057880__L0578dc:
        lea     0x2b6922.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
        move.w  #0xfdde,d0                      | +068
        jsr     0x5dca4.l                       | +06c
        move.w  d0,0x28(a6)                     | +072
        move.w  #0x663,0x2a(a6)                 | +076
        move.w  #0xff93,0x2e(a6)                | +07c
        move.w  #0x0,0x2c(a6)                   | +082
.L057908:
        move.l  0x7a(a6),d0                     | +088
        beq.w   .L05793e                        | +08c
        cmpi.l  #0x100440,d0                    | +090
        bne.w   .L05792e                        | +096
        lea     0x2b71dc.l,a0                   | +09a
        move.l  a0,0x4c(a6)                     | +0a0
        jsr     0x283ca.l                       | +0a4
        bra.w   .L05793e                        | +0aa
.L05792e:
        lea     0x2b7230.l,a0                   | +0ae
        move.l  a0,0x4c(a6)                     | +0b4
        jsr     0x283ca.l                       | +0b8
.L05793e:
        lea     .L057944(pc),a1                 | +0be
        move.l  a1,(a6)                         | +0c2
.L057944:
        jsr     0x27d50.l                       | +0c4
        bcc.w   .L057954                        | +0ca
        lea     Soldier_Land_058464(pc),a1      | +0ce
        move.l  a1,(a6)                         | +0d2
.L057954:
        jsr     0x28d70.l                       | +0d4
        bsr.w   Soldier_PickGrabAnchor_056e4a   | +0da
        bcc.w   .L057994                        | +0de
        tst.w   0x2a(a6)                        | +0e2
        bpl.w   .L057994                        | +0e6
        jsr     0x283d8.l                       | +0ea
        btst    #0x1,0x13(a6)                   | +0f0
        beq.w   .L057994                        | +0f6
        movea.l 0x7a(a6),a0                     | +0fa
        move.w  0x24(a0),d0                     | +0fe
        addi.w  #0x10,d0                        | +102
        cmp.w   0x24(a6),d0                     | +106
        bpl.w   .L057994                        | +10a
        lea     Soldier_GrabLatch_057b06(pc),a1 | +10e
        move.l  a1,(a6)                         | +112
.L057994:
        jsr     0x49fd0.l                       | +114
        bra.w   Soldier_DespawnIfOffscreen_056e1e | +11a
        .global Soldier_Leap_057880__L05799e
Soldier_Leap_057880__L05799e:
        lea     0x2b6f40.l,a0                   | +11e
        jsr     0x28cd4.l                       | +124
        cmpi.b  #0xff,0x9d(a6)                  | +12a
        bne.w   .L0579d8                        | +130
        move.w  #0xfdde,d0                      | +134
        jsr     0x5dca4.l                       | +138
        move.w  d0,0x28(a6)                     | +13e
        move.w  #0x438,0x2a(a6)                 | +142
        move.w  #0xffb8,0x2e(a6)                | +148
        move.w  #0x0,0x2c(a6)                   | +14e
        bra.w   .L057a54                        | +154
.L0579d8:
        cmpi.b  #0x1,0x9d(a6)                   | +158
        bne.w   .L057a06                        | +15e
        move.w  #0xfe67,d0                      | +162
        jsr     0x5dca4.l                       | +166
        move.w  d0,0x28(a6)                     | +16c
        move.w  #0x7f8,0x2a(a6)                 | +170
        move.w  #0xff9a,0x2e(a6)                | +176
        move.w  #0x0,0x2c(a6)                   | +17c
        bra.w   .L057a54                        | +182
.L057a06:
        cmpi.b  #0x2,0x9d(a6)                   | +186
        bne.w   .L057a34                        | +18c
        move.w  #0xfedc,d0                      | +190
        jsr     0x5dca4.l                       | +194
        move.w  d0,0x28(a6)                     | +19a
        move.w  #0x7fc,0x2a(a6)                 | +19e
        move.w  #0xffb7,0x2e(a6)                | +1a4
        move.w  #0x0,0x2c(a6)                   | +1aa
        bra.w   .L057a54                        | +1b0
.L057a34:
        move.w  #0xfd56,d0                      | +1b4
        jsr     0x5dca4.l                       | +1b8
        move.w  d0,0x28(a6)                     | +1be
        move.w  #0x7f8,0x2a(a6)                 | +1c2
        move.w  #0xff56,0x2e(a6)                | +1c8
        move.w  #0x0,0x2c(a6)                   | +1ce
.L057a54:
        jsr     0x283ca.l                       | +1d4
.L057a5a:
        lea     .L057a60(pc),a1                 | +1da
        move.l  a1,(a6)                         | +1de
.L057a60:
        jsr     0x27d50.l                       | +1e0
        bcc.w   .L057a70                        | +1e6
        lea     Soldier_Hurt_Loop_058424__L058454(pc),a1 | +1ea
        move.l  a1,(a6)                         | +1ee
.L057a70:
        jsr     0x28d70.l                       | +1f0
        tst.w   0x2a(a6)                        | +1f6
        bpl.w   .L057a84                        | +1fa
        jsr     0x283d8.l                       | +1fe
.L057a84:
        btst    #0x1,0x13(a6)                   | +204
        beq.w   .L057a94                        | +20a
        lea     Soldier_LeapLand_057a9e(pc),a1  | +20e
        move.l  a1,(a6)                         | +212
.L057a94:
        jsr     0x49fd0.l                       | +214
        bra.w   Soldier_DespawnIfOffscreen_056e1e | +21a

| ----------------------------------------------------------------------------
|  Soldier_LeapLand_057a9e  @ $057A9E  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_LeapLand_057a9e, "ax", @progbits
        .global Soldier_LeapLand_057a9e
Soldier_LeapLand_057a9e:
        lea     0x2b6fba.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        jsr     0x283ca.l                       | +00c
        lea     .L057ab6(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L057ab6:
        jsr     0x27d50.l                       | +018
        scs.b   0x78(a6)                        | +01e
        jsr     0x28d70.l                       | +022
        jsr     0x5e8da.l                       | +028
        tst.b   0x78(a6)                        | +02e
        beq.w   .L057ada                        | +032
        lea     Soldier_Hurt_Loop_058424__L058454(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
.L057ada:
        jsr     0x49fd0.l                       | +03c
        bra.w   Soldier_DespawnIfOffscreen_056e1e | +042

| ----------------------------------------------------------------------------
|  Soldier_HitCheckTail_057ae4  @ $057AE4  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_HitCheckTail_057ae4, "ax", @progbits
        .global Soldier_HitCheckTail_057ae4
Soldier_HitCheckTail_057ae4:
        jsr     0x2870a.l                       | +000
        bcc.w   .L057b04                        | +006
        cmpi.b  #0x1c,0x58(a6)                  | +00a
        bne.w   .L057afe                        | +010
        jsr     0x49fd0.l                       | +014
.L057afe:
        bclr    #0x3,0x13(a6)                   | +01a
.L057b04:
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Soldier_GrabLatch_057b06  @ $057B06  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabLatch_057b06, "ax", @progbits
        .global Soldier_GrabLatch_057b06
Soldier_GrabLatch_057b06:
        lea     0x2b6a02.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2b67de.l,a0                   | +00c
        move.l  a0,0x48(a6)                     | +012
        bclr    #0x3,0x13(a6)                   | +016
        lea     0xffff.w,a0                     | +01c
        move.l  a0,0x4c(a6)                     | +020
        jsr     0x283ca.l                       | +024
        clr.w   0x80(a6)                        | +02a
        bsr.w   Soldier_PickGrabAnchor_056e4a   | +02e
        bcs.w   .L057b44                        | +032
        bra.w   Soldier_GrabThrownB_057fc6      | +036
        bra.w   .L057b66                        | +03a
.L057b44:
        movea.l 0x7a(a6),a0                     | +03e
        move.w  0x22(a6),d0                     | +042
        move.w  0x24(a6),d1                     | +046
        sub.w   0x22(a0),d0                     | +04a
        sub.w   0x24(a0),d1                     | +04e
        move.w  d0,0x88(a6)                     | +052
        move.w  d1,0x8a(a6)                     | +056
        lea     .L057b66(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L057b66:
        bsr.w   Soldier_PickGrabAnchor_056e4a   | +060
        bcs.w   .L057b74                        | +064
        lea     Soldier_GrabThrownB_057fc6(pc),a1 | +068
        move.l  a1,(a6)                         | +06c
.L057b74:
        movea.l 0x7a(a6),a0                     | +06e
        move.w  0x38(a0),d0                     | +072
        andi.w  #0xffe3,d0                      | +076
        ori.w   #0x18,d0                        | +07a
        move.w  d0,0x38(a6)                     | +07e
        move.w  0x22(a0),d0                     | +082
        move.w  0x24(a0),d1                     | +086
        add.w   0x88(a6),d0                     | +08a
        add.w   0x8a(a6),d1                     | +08e
        move.w  d0,0x22(a6)                     | +092
        move.w  d1,0x24(a6)                     | +096
        jsr     0x28d70.l                       | +09a
        bcc.w   .L057bb0                        | +0a0
        lea     Soldier_GrabSlideToAnchor_057bb4(pc),a1 | +0a4
        move.l  a1,(a6)                         | +0a8
.L057bb0:
        jmp     Soldier_HitCheckTail_057ae4(pc) | +0aa

| ----------------------------------------------------------------------------
|  Soldier_GrabSlideToAnchor_057bb4  @ $057BB4  (244 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabSlideToAnchor_057bb4, "ax", @progbits
        .global Soldier_GrabSlideToAnchor_057bb4
Soldier_GrabSlideToAnchor_057bb4:
        lea     0x2b6a60.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bclr    #0x0,0x3a(a6)                   | +00c
        bsr.w   Soldier_PickGrabAnchor_056e4a   | +012
        bcc.w   Soldier_GrabThrownB_057fc6      | +016
        move.b  d0,0x76(a6)                     | +01a
        movea.l 0x7a(a6),a0                     | +01e
        jsr     0x8f82a.l                       | +022
        bcc.w   Soldier_GrabThrownB_057fc6      | +028
        sub.w   0x22(a6),d0                     | +02c
        sub.w   0x24(a6),d1                     | +030
        neg.w   d0                              | +034
        neg.w   d1                              | +036
        move.w  d0,0x88(a6)                     | +038
        move.w  d1,0x8a(a6)                     | +03c
        lea     .L057bfa(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L057bfa:
        move.w  0x88(a6),d0                     | +046
        beq.w   .L057c0e                        | +04a
        asr.w   #0x5,d0                         | +04e
        bne.w   .L057c0a                        | +050
        moveq   #1,d0                           | +054
.L057c0a:
        sub.w   d0,0x88(a6)                     | +056
.L057c0e:
        move.w  0x8a(a6),d0                     | +05a
        beq.w   .L057c22                        | +05e
        asr.w   #0x5,d0                         | +062
        bne.w   .L057c1e                        | +064
        moveq   #1,d0                           | +068
.L057c1e:
        sub.w   d0,0x8a(a6)                     | +06a
.L057c22:
        move.b  0x76(a6),d0                     | +06e
        movea.l 0x7a(a6),a0                     | +072
        jsr     0x8f82a.l                       | +076
        bcs.w   .L057c3e                        | +07c
        lea     Soldier_GrabThrownB_057fc6(pc),a1 | +080
        move.l  a1,(a6)                         | +084
        bra.w   .L057c5e                        | +086
.L057c3e:
        add.w   0x88(a6),d0                     | +08a
        add.w   0x8a(a6),d1                     | +08e
        move.w  d0,0x22(a6)                     | +092
        move.w  d1,0x24(a6)                     | +096
        move.w  0x38(a0),d0                     | +09a
        andi.w  #0xffe3,d0                      | +09e
        ori.w   #0x18,d0                        | +0a2
        move.w  d0,0x38(a6)                     | +0a6
.L057c5e:
        jsr     0x28d70.l                       | +0aa
        move.w  0x88(a6),d0                     | +0b0
        or.w    0x8a(a6),d0                     | +0b4
        bne.w   .L057ca4                        | +0b8
        lea     Soldier_GrabThrownB_057fc6(pc),a1 | +0bc
        move.l  a1,(a6)                         | +0c0
        move.b  0x76(a6),d0                     | +0c2
        cmpi.b  #0x0,d0                         | +0c6
        bne.w   .L057c88                        | +0ca
        lea     Soldier_GrabPlayer_057d04(pc),a1 | +0ce
        move.l  a1,(a6)                         | +0d2
.L057c88:
        cmpi.b  #0x1,d0                         | +0d4
        bne.w   .L057c96                        | +0d8
        lea     Soldier_GrabPlayer_057d04__L057d6e(pc),a1 | +0dc
        move.l  a1,(a6)                         | +0e0
.L057c96:
        cmpi.b  #0x2,d0                         | +0e2
        bne.w   .L057ca4                        | +0e6
        lea     Soldier_GrabPlayer_057d04__L057e0a(pc),a1 | +0ea
        move.l  a1,(a6)                         | +0ee
.L057ca4:
        jmp     Soldier_HitCheckTail_057ae4(pc) | +0f0

| ----------------------------------------------------------------------------
|  Soldier_GrabHoldFlag_057ca8  @ $057CA8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabHoldFlag_057ca8, "ax", @progbits
        .global Soldier_GrabHoldFlag_057ca8
Soldier_GrabHoldFlag_057ca8:
        tst.b   0x106ed3.l                      | +000
        bne.w   Soldier_GrabFollowPlayer_057cc0 | +006
        bset    #0x3,0x13(a6)                   | +00a

| ----------------------------------------------------------------------------
|  Soldier_GrabFollowPlayer_057cc0  @ $057CC0  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabFollowPlayer_057cc0, "ax", @progbits
        .global Soldier_GrabFollowPlayer_057cc0
Soldier_GrabFollowPlayer_057cc0:
        move.b  0x76(a6),d0                     | +000
        movea.l 0x7a(a6),a0                     | +004
        jsr     0x8f884.l                       | +008
        bcc.w   .L057cee                        | +00e
        move.w  d0,0x22(a6)                     | +012
        move.w  d1,0x24(a6)                     | +016
        move.w  0x38(a0),d0                     | +01a
        andi.w  #0xffe3,d0                      | +01e
        ori.w   #0x18,d0                        | +022
        move.w  d0,0x38(a6)                     | +026
        bra.w   .L057cf4                        | +02a
.L057cee:
        lea     Soldier_GrabThrownB_057fc6(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L057cf4:
        bsr.w   Soldier_GrabStruggleProgress_056f10 | +034
        bcc.w   SetHandlerRts_057d02            | +038
