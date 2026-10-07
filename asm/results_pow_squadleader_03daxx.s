| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $03DA98..$040EF2  (12,874 B, 126 entradas, 63 huecos)
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
|  SlugCannon_ArmOffsetCurve_03daa8  @ $03DAA8  (276 B)
| ----------------------------------------------------------------------------
        .section .text.SlugCannon_ArmOffsetCurve_03daa8, "ax", @progbits
        .global SlugCannon_ArmOffsetCurve_03daa8
SlugCannon_ArmOffsetCurve_03daa8:
        .dc.w   0xffea                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +026  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +032  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +036  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +03e  (dato / opcode no decodificado)
        .dc.w   0xffea                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xffee                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffee                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +052  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +056  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xfff3                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +05e  (dato / opcode no decodificado)
        .dc.w   0xfff3                        | +060  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +062  (dato / opcode no decodificado)
        .dc.w   0xfff5                        | +064  (dato / opcode no decodificado)
        .dc.w   0x001b                        | +066  (dato / opcode no decodificado)
        .dc.w   0xfff5                        | +068  (dato / opcode no decodificado)
        .dc.w   0x001b                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +06e  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +070  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +072  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +074  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +076  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +078  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +080  (dato / opcode no decodificado)
        .dc.w   0x001f                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +084  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +088  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +090  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +094  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +098  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x000b                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x000b                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x000d                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x000d                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +102  (dato / opcode no decodificado)
        movea.l 0x8(a6),a1                      | +104
        move.b  0x10(a6),d0                     | +108
        cmp.b   0x10(a1),d0                     | +10c
        bcs.w   SetXN_03dbc2                    | +110

| ----------------------------------------------------------------------------
|  Results_Entry_03dbc8  @ $03DBC8  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Results_Entry_03dbc8, "ax", @progbits
        .global Results_Entry_03dbc8
Results_Entry_03dbc8:
        cmpi.b  #0x1,0x10fdb6.l                 | +000
        beq.w   Results_WaitByLevel_03dbf4      | +008
        cmpi.b  #0x1,0x10fdb7.l                 | +00c
        beq.w   Results_WaitByLevel_03dbf4      | +014
        clr.b   0x106ed2.l                      | +018

| ----------------------------------------------------------------------------
|  Results_WaitByLevel_03dbf4  @ $03DBF4  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Results_WaitByLevel_03dbf4, "ax", @progbits
        .global Results_WaitByLevel_03dbf4
Results_WaitByLevel_03dbf4:
        lea     0x28293a.l,a1                   | +000
        clr.w   d0                              | +006
        move.b  0x106ecf.l,d0                   | +008
        andi.w  #0x7,d0                         | +00e
        lsl.w   #0x1,d0                         | +012
        move.w  (a1,d0.w),0x70(a6)              | +014

| ----------------------------------------------------------------------------
|  Results_Countdown_03dc16  @ $03DC16  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Results_Countdown_03dc16, "ax", @progbits
        .global Results_Countdown_03dc16
Results_Countdown_03dc16:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   SetHandlerRts_03dc2a            | +00a

| ----------------------------------------------------------------------------
|  Results_FadeInit_03dc2c  @ $03DC2C  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Results_FadeInit_03dc2c, "ax", @progbits
        .global Results_FadeInit_03dc2c
Results_FadeInit_03dc2c:
        jsr     0x22c8.l                        | +000
        move.b  #0x60,d0                        | +006
        jsr     0x2308.l                        | +00a
        move.w  #0x18,d0                        | +010
        move.w  #0x18,d1                        | +014
        move.w  #0x18,d2                        | +018
        move.w  #0x5,d3                         | +01c
        jsr     0x52580.l                       | +020
        move.w  #0xf,0x70(a6)                   | +026
        lea     .L03dc5e(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L03dc5e:
        subq.w  #0x1,0x70(a6)                   | +032
        cmpi.w  #0x0,0x70(a6)                   | +036
        bgt.w   SetHandlerRts_03dc72            | +03c

| ----------------------------------------------------------------------------
|  Results_SetupPlayers_03dc74  @ $03DC74  (172 B)
| ----------------------------------------------------------------------------
        .section .text.Results_SetupPlayers_03dc74, "ax", @progbits
        .global Results_SetupPlayers_03dc74
Results_SetupPlayers_03dc74:
        clr.l   0x80(a6)                        | +000
        clr.l   0x84(a6)                        | +004
        clr.b   0x88(a6)                        | +008
        clr.b   0x8a(a6)                        | +00c
        clr.b   0x8e(a6)                        | +010
        clr.b   0x90(a6)                        | +014
        cmpi.b  #0x1,0x10fdb6.l                 | +018
        bne.w   .L03dcb8                        | +020
        lea     Results_PlayerColumn_03df56(pc),a1 | +024
        jsr     0x4ae.l                         | +028
        move.l  a0,0x80(a6)                     | +02e
        addq.b  #0x1,0x88(a6)                   | +032
        addq.b  #0x1,0x8e(a6)                   | +036
        move.w  0x106f4c.l,d1                   | +03a
        bra.w   .L03dcba                        | +040
.L03dcb8:
        clr.w   d1                              | +044
.L03dcba:
        cmpi.b  #0x1,0x10fdb7.l                 | +046
        bne.w   .L03dce6                        | +04e
        lea     Results_PlayerColumn_03df56__L03dfdc(pc),a1 | +052
        jsr     0x4ae.l                         | +056
        move.l  a0,0x84(a6)                     | +05c
        addq.b  #0x1,0x88(a6)                   | +060
        addq.b  #0x1,0x8e(a6)                   | +064
        move.w  0x106f4e.l,d2                   | +068
        bra.w   .L03dce8                        | +06e
.L03dce6:
        clr.w   d2                              | +072
.L03dce8:
        jsr     0x516da.l                       | +074
        cmp.w   d1,d2                           | +07a
        bcc.w   .L03dcfe                        | +07c
        move.w  #0x0,0x8c(a6)                   | +080
        bra.w   .L03dd04                        | +086
.L03dcfe:
        move.w  #0x1,0x8c(a6)                   | +08a
.L03dd04:
        cmp.w   d1,d2                           | +090
        bne.w   .L03dd10                        | +092
        move.w  #0x2,0x8c(a6)                   | +096
.L03dd10:
        move.b  0x88(a6),0x89(a6)               | +09c
        cmpi.b  #0x0,0x88(a6)                   | +0a2
        bne.w   Results_DrawFrame_03dd28        | +0a8

| ----------------------------------------------------------------------------
|  Results_DrawFrame_03dd28  @ $03DD28  (376 B)
| ----------------------------------------------------------------------------
        .section .text.Results_DrawFrame_03dd28, "ax", @progbits
        .global Results_DrawFrame_03dd28
Results_DrawFrame_03dd28:
        cmpi.b  #0x0,0x10fd83.l                 | +000
        bne.w   .L03de8c                        | +008
        move.w  #0x71e6,d0                      | +00c
        move.w  #0xd22,d1                       | +010
        ori.w   #0x9000,d1                      | +014
        movem.w d0-d1,0x3c0000.l                | +018
        addi.w  #0x20,d0                        | +020
        addq.w  #0x1,d1                         | +024
        movem.w d0-d1,0x3c0000.l                | +026
        subi.w  #0x1f,d0                        | +02e
        addi.w  #0xf,d1                         | +032
        movem.w d0-d1,0x3c0000.l                | +036
        addi.w  #0x20,d0                        | +03e
        addq.w  #0x1,d1                         | +042
        movem.w d0-d1,0x3c0000.l                | +044
        subi.w  #0x21,d0                        | +04c
        move.w  #0x7226,d0                      | +050
        move.w  #0xd24,d1                       | +054
        ori.w   #0x9000,d1                      | +058
        movem.w d0-d1,0x3c0000.l                | +05c
        addi.w  #0x20,d0                        | +064
        addq.w  #0x1,d1                         | +068
        movem.w d0-d1,0x3c0000.l                | +06a
        subi.w  #0x1f,d0                        | +072
        addi.w  #0xf,d1                         | +076
        movem.w d0-d1,0x3c0000.l                | +07a
        addi.w  #0x20,d0                        | +082
        addq.w  #0x1,d1                         | +086
        movem.w d0-d1,0x3c0000.l                | +088
        subi.w  #0x21,d0                        | +090
        move.w  #0x7266,d0                      | +094
        move.w  #0xde6,d1                       | +098
        ori.w   #0x9000,d1                      | +09c
        movem.w d0-d1,0x3c0000.l                | +0a0
        addi.w  #0x20,d0                        | +0a8
        addq.w  #0x1,d1                         | +0ac
        movem.w d0-d1,0x3c0000.l                | +0ae
        subi.w  #0x1f,d0                        | +0b6
        addi.w  #0xf,d1                         | +0ba
        movem.w d0-d1,0x3c0000.l                | +0be
        addi.w  #0x20,d0                        | +0c6
        addq.w  #0x1,d1                         | +0ca
        movem.w d0-d1,0x3c0000.l                | +0cc
        subi.w  #0x21,d0                        | +0d4
        move.w  #0x72a6,d0                      | +0d8
        move.w  #0xde8,d1                       | +0dc
        ori.w   #0x9000,d1                      | +0e0
        movem.w d0-d1,0x3c0000.l                | +0e4
        addi.w  #0x20,d0                        | +0ec
        addq.w  #0x1,d1                         | +0f0
        movem.w d0-d1,0x3c0000.l                | +0f2
        subi.w  #0x1f,d0                        | +0fa
        addi.w  #0xf,d1                         | +0fe
        movem.w d0-d1,0x3c0000.l                | +102
        addi.w  #0x20,d0                        | +10a
        addq.w  #0x1,d1                         | +10e
        movem.w d0-d1,0x3c0000.l                | +110
        subi.w  #0x21,d0                        | +118
        move.w  #0x72e6,d0                      | +11c
        move.w  #0xdea,d1                       | +120
        ori.w   #0x9000,d1                      | +124
        movem.w d0-d1,0x3c0000.l                | +128
        addi.w  #0x20,d0                        | +130
        addq.w  #0x1,d1                         | +134
        movem.w d0-d1,0x3c0000.l                | +136
        subi.w  #0x1f,d0                        | +13e
        addi.w  #0xf,d1                         | +142
        movem.w d0-d1,0x3c0000.l                | +146
        addi.w  #0x20,d0                        | +14e
        addq.w  #0x1,d1                         | +152
        movem.w d0-d1,0x3c0000.l                | +154
        subi.w  #0x21,d0                        | +15c
        bra.w   SetTaskHandler_03dea0           | +160
.L03de8c:
        movea.w #0x7146,a1                      | +164
        move.b  #0x3,d1                         | +168
        lea     0x282a68.l,a2                   | +16c
        jsr     0x477fc.l                       | +172

| ----------------------------------------------------------------------------
|  Results_WaitDone_03dea8  @ $03DEA8  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Results_WaitDone_03dea8, "ax", @progbits
        .global Results_WaitDone_03dea8
Results_WaitDone_03dea8:
        jsr     Results_PollStartP1_03edd2(pc)  | +000
        cmpi.b  #0x0,0x90(a6)                   | +004
        beq.w   SetHandlerRts_03debc            | +00a

| ----------------------------------------------------------------------------
|  Results_FadeOut_03debe  @ $03DEBE  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Results_FadeOut_03debe, "ax", @progbits
        .global Results_FadeOut_03debe
Results_FadeOut_03debe:
        move.w  #0x0,d0                         | +000
        move.w  #0x0,d1                         | +004
        move.w  #0x0,d2                         | +008
        move.w  #0x3,d3                         | +00c
        jsr     0x52580.l                       | +010
        move.w  #0xa,0x70(a6)                   | +016

| ----------------------------------------------------------------------------
|  Results_Teardown_03dee2  @ $03DEE2  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Results_Teardown_03dee2, "ax", @progbits
        .global Results_Teardown_03dee2
Results_Teardown_03dee2:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   SetHandlerRts_03df30            | +00a
        adda.w  0x24(a6),a1                     | +00e
        movea.w #0x7046,a1                      | +012
        move.w  #0x2320,d0                      | +016
        move.w  #0x24,d1                        | +01a
        move.w  #0x16,d2                        | +01e
        jsr     0x5da9c.l                       | +022
        jsr     0x5b6.l                         | +028
        lea     0x28294a.l,a1                   | +02e
        clr.w   d0                              | +034
        move.b  0x106ecf.l,d0                   | +036
        andi.w  #0x7,d0                         | +03c
        lsl.w   #0x1,d0                         | +040
        move.w  (a1,d0.w),0x70(a6)              | +042

| ----------------------------------------------------------------------------
|  Results_ToBanner_03df32  @ $03DF32  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ToBanner_03df32, "ax", @progbits
        .global Results_ToBanner_03df32
Results_ToBanner_03df32:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   SetHandlerRts_03df52            | +00a
        lea     0x7a9f0.l,a1                    | +00e
        jsr     0x4ae.l                         | +014

| ----------------------------------------------------------------------------
|  Results_PlayerColumn_03df56  @ $03DF56  (294 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PlayerColumn_03df56, "ax", @progbits
        .global Results_PlayerColumn_03df56
Results_PlayerColumn_03df56:
        clr.b   0x20(a6)                        | +000
        lea     Results_PowRoster_03e514(pc),a1 | +004
        jsr     0x4ae.l                         | +008
        move.b  #0x0,0x68(a0)                   | +00e
        move.b  #0x0,0x6e(a0)                   | +014
        move.w  #0x182,d1                       | +01a
        jsr     0x29f2.l                        | +01e
        move.w  0x14(a6),d1                     | +024
        jsr     0x2c66.l                        | +028
        move.w  #0x0,0x76(a6)                   | +02e
        move.b  #0x0,0x68(a6)                   | +034
        move.b  #0x0,0x6e(a6)                   | +03a
        move.w  #0x30,0x22(a6)                  | +040
        move.w  #0x190,0x24(a6)                 | +046
        clr.w   0x26(a6)                        | +04c
        move.w  #0x0,0x72(a6)                   | +050
        move.w  0x106f4c.l,0x74(a6)             | +056
        lea     Results_DrawLabels_03ecac(pc),a1 | +05e
        jsr     0x4ae.l                         | +062
        move.w  0x72(a6),0x72(a0)               | +068
        movea.w #0x7149,a1                      | +06e
        move.b  #0x4,d1                         | +072
        lea     0x282a3e.l,a2                   | +076
        jsr     0x4784c.l                       | +07c
        bra.w   .L03e05e                        | +082
        .global Results_PlayerColumn_03df56__L03dfdc
Results_PlayerColumn_03df56__L03dfdc:
.L03dfdc:
        clr.b   0x20(a6)                        | +086
        lea     Results_PowRoster_03e514(pc),a1 | +08a
        jsr     0x4ae.l                         | +08e
        move.b  #0x1,0x68(a0)                   | +094
        move.b  #0x1,0x6e(a0)                   | +09a
        move.w  #0x182,d1                       | +0a0
        jsr     0x29f2.l                        | +0a4
        move.w  0x14(a6),d1                     | +0aa
        jsr     0x2c66.l                        | +0ae
        move.w  #0x1,0x76(a6)                   | +0b4
        move.b  #0x1,0x68(a6)                   | +0ba
        move.b  #0x1,0x6e(a6)                   | +0c0
        move.w  #0xb8,0x22(a6)                  | +0c6
        move.w  #0x190,0x24(a6)                 | +0cc
        clr.w   0x26(a6)                        | +0d2
        move.w  #0x220,0x72(a6)                 | +0d6
        move.w  0x106f4e.l,0x74(a6)             | +0dc
        lea     Results_DrawLabels_03ecac(pc),a1 | +0e4
        jsr     0x4ae.l                         | +0e8
        move.w  0x72(a6),0x72(a0)               | +0ee
        movea.w #0x7369,a1                      | +0f4
        move.b  #0x4,d1                         | +0f8
        lea     0x282a42.l,a2                   | +0fc
        jsr     0x4784c.l                       | +102
.L03e05e:
        move.w  0x74(a6),0x5c(a6)               | +108
        move.w  #0xe000,0x38(a6)                | +10e
        lea     0x282966.l,a0                   | +114
        jsr     0x28cd4.l                       | +11a
        move.w  #0x5,0x70(a6)                   | +120

| ----------------------------------------------------------------------------
|  Results_ColPhaseScore_03e084  @ $03E084  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColPhaseScore_03e084, "ax", @progbits
        .global Results_ColPhaseScore_03e084
Results_ColPhaseScore_03e084:
        movea.l 0xc(a6),a1                      | +000
        cmpi.b  #0x1,0x8a(a1)                   | +004
        bne.w   .L03e0a2                        | +00a
        cmpi.w  #0x1,0x70(a6)                   | +00e
        ble.w   .L03e0a2                        | +014
        move.w  #0x1,0x70(a6)                   | +018
.L03e0a2:
        subq.w  #0x1,0x70(a6)                   | +01e
        cmpi.w  #0x0,0x70(a6)                   | +022
        bge.w   .L03e0ce                        | +028
        move.w  #0x0,d0                         | +02c
        move.w  #0x714c,d4                      | +030
        add.w   0x72(a6),d4                     | +034
        jsr     0x4772a.l                       | +038
        lea     Results_ColPhaseWait_03e0d0(pc),a1 | +03e
        move.l  a1,(a6)                         | +042
        move.w  #0x1e,0x70(a6)                  | +044
.L03e0ce:
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  Results_ColPhaseWait_03e0d0  @ $03E0D0  (78 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColPhaseWait_03e0d0, "ax", @progbits
        .global Results_ColPhaseWait_03e0d0
Results_ColPhaseWait_03e0d0:
        movea.l 0xc(a6),a1                      | +000
        cmpi.b  #0x1,0x8a(a1)                   | +004
        bne.w   .L03e0ee                        | +00a
        cmpi.w  #0x1,0x70(a6)                   | +00e
        ble.w   .L03e0ee                        | +014
        move.w  #0x1,0x70(a6)                   | +018
.L03e0ee:
        subq.w  #0x1,0x70(a6)                   | +01e
        cmpi.w  #0x0,0x70(a6)                   | +022
        bge.w   JsrAbsThunk_03e176              | +028
        cmpi.w  #0x0,0x5c(a6)                   | +02c
        bne.w   Results_ColCommitScore_03e126   | +032
        clr.w   0x5c(a6)                        | +036
        movea.l 0xc(a6),a1                      | +03a
        subq.b  #0x1,0x88(a1)                   | +03e
        move.w  #0x1e,0x70(a6)                  | +042
        lea     Results_ColWaitOther_03e17e(pc),a1 | +048
        move.l  a1,(a6)                         | +04c

| ----------------------------------------------------------------------------
|  Results_ColCommitScore_03e126  @ $03E126  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColCommitScore_03e126, "ax", @progbits
        .global Results_ColCommitScore_03e126
Results_ColCommitScore_03e126:
        subq.w  #0x1,0x5c(a6)                   | +000
        cmpi.w  #0x0,0x76(a6)                   | +004
        bne.w   .L03e140                        | +00a
        move.w  0x5c(a6),0x106f4c.l             | +00e
        bra.w   .L03e148                        | +016
.L03e140:
        move.w  0x5c(a6),0x106f4e.l             | +01a
.L03e148:
        lea     Results_PrizeSprite_03e91c(pc),a1 | +022
        jsr     0x4ae.l                         | +026
        move.w  0x76(a6),0x76(a0)               | +02c
        move.w  0x72(a6),0x72(a0)               | +032
        move.w  0x74(a6),d0                     | +038
        sub.w   0x5c(a6),d0                     | +03c
        move.w  d0,0x74(a0)                     | +040
        move.w  0x5c(a6),0x5c(a0)               | +044
        move.w  #0xa,0x70(a6)                   | +04a

| ----------------------------------------------------------------------------
|  Results_ColWaitOther_03e17e  @ $03E17E  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColWaitOther_03e17e, "ax", @progbits
        .global Results_ColWaitOther_03e17e
Results_ColWaitOther_03e17e:
        movea.l 0xc(a6),a1                      | +000
        cmpi.b  #0x0,0x88(a1)                   | +004
        bne.w   JsrAbsThunk_03e198              | +00a
        move.w  #0x14,0x70(a6)                  | +00e
        lea     Results_ColPhaseBonus_03e1a0(pc),a1 | +014
        move.l  a1,(a6)                         | +018

| ----------------------------------------------------------------------------
|  Results_ColPhaseBonus_03e1a0  @ $03E1A0  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColPhaseBonus_03e1a0, "ax", @progbits
        .global Results_ColPhaseBonus_03e1a0
Results_ColPhaseBonus_03e1a0:
        movea.l 0xc(a6),a1                      | +000
        cmpi.b  #0x1,0x8a(a1)                   | +004
        bne.w   .L03e1be                        | +00a
        cmpi.w  #0x1,0x70(a6)                   | +00e
        ble.w   .L03e1be                        | +014
        move.w  #0x1,0x70(a6)                   | +018
.L03e1be:
        subq.w  #0x1,0x70(a6)                   | +01e
        cmpi.w  #0x0,0x70(a6)                   | +022
        bge.w   JsrAbsThunk_03e200              | +028
        movea.w #0x71cd,a1                      | +02c
        adda.w  0x72(a6),a1                     | +030
        move.w  #0x2300,d0                      | +034
        lea     0x282a96.l,a2                   | +038
        jsr     0x5dad8.l                       | +03e
        clr.w   0x5c(a6)                        | +044
        move.w  #0x0,d0                         | +048
        move.w  #0x71cf,d4                      | +04c
        add.w   0x72(a6),d4                     | +050
        jsr     0x4772a.l                       | +054
        lea     Results_ColPhaseTotal_03e208(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e

| ----------------------------------------------------------------------------
|  Results_ColPhaseTotal_03e208  @ $03E208  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColPhaseTotal_03e208, "ax", @progbits
        .global Results_ColPhaseTotal_03e208
Results_ColPhaseTotal_03e208:
        movea.l 0xc(a6),a1                      | +000
        cmpi.b  #0x1,0x8a(a1)                   | +004
        bne.w   .L03e226                        | +00a
        cmpi.w  #0x1,0x70(a6)                   | +00e
        ble.w   .L03e226                        | +014
        move.w  #0x1,0x70(a6)                   | +018
.L03e226:
        subq.w  #0x1,0x70(a6)                   | +01e
        cmpi.w  #0x0,0x70(a6)                   | +022
        bge.w   JsrAbsThunk_03e2ac              | +028
        move.w  #0x3,0x70(a6)                   | +02c
        tst.w   0x5c(a6)                        | +032
        beq.w   .L03e28e                        | +036
        move.w  #0x10d8,d0                      | +03a
        jsr     0x2352.l                        | +03e
        move.l  #0x1000,d0                      | +044
        jsr     0x51a44.l                       | +04a
        move.w  #0x0,d0                         | +050
        move.w  #0x714f,d4                      | +054
        add.w   0x72(a6),d4                     | +058
        jsr     0x4772a.l                       | +05c
        move.w  #0x0,d0                         | +062
        move.w  #0x71cf,d4                      | +066
        add.w   0x72(a6),d4                     | +06a
        jsr     0x4768a.l                       | +06e
        move.w  0x5c(a6),d0                     | +074
        move.w  #0x710f,d4                      | +078
        add.w   0x72(a6),d4                     | +07c
        jsr     0x4772a.l                       | +080
.L03e28e:
        addq.w  #0x1,0x5c(a6)                   | +086
        move.w  0x74(a6),d0                     | +08a
        cmp.w   0x5c(a6),d0                     | +08e
        bcc.w   JsrAbsThunk_03e2ac              | +092
        movea.l 0xc(a6),a1                      | +096
        subq.b  #0x1,0x8e(a1)                   | +09a
        lea     Results_ColPause_03e2b4(pc),a1  | +09e
        move.l  a1,(a6)                         | +0a2

| ----------------------------------------------------------------------------
|  Results_ColPause_03e2b4  @ $03E2B4  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColPause_03e2b4, "ax", @progbits
        .global Results_ColPause_03e2b4
Results_ColPause_03e2b4:
        move.w  #0xa,0x70(a6)                   | +000
        lea     .L03e2c0(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L03e2c0:
        subq.w  #0x1,0x70(a6)                   | +00c
        cmpi.w  #0x0,0x70(a6)                   | +010
        bge.w   JsrAbsThunk_03e2d4              | +016
        lea     Results_ColWaitAll_03e2dc(pc),a1 | +01a
        move.l  a1,(a6)                         | +01e

| ----------------------------------------------------------------------------
|  Results_ColWaitAll_03e2dc  @ $03E2DC  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColWaitAll_03e2dc, "ax", @progbits
        .global Results_ColWaitAll_03e2dc
Results_ColWaitAll_03e2dc:
        movea.l 0xc(a6),a1                      | +000
        cmpi.b  #0x0,0x8e(a1)                   | +004
        bne.w   JsrAbsThunk_03e2f6              | +00a
        move.w  #0x1e,0x70(a6)                  | +00e
        lea     Results_ColWinner_03e2fe(pc),a1 | +014
        move.l  a1,(a6)                         | +018

| ----------------------------------------------------------------------------
|  Results_ColWinner_03e2fe  @ $03E2FE  (260 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColWinner_03e2fe, "ax", @progbits
        .global Results_ColWinner_03e2fe
Results_ColWinner_03e2fe:
        move.w  0x76(a6),d0                     | +000
        jsr     0x2ad36.l                       | +004
        bcc.w   .L03e3dc                        | +00a
        move.l  #0x10000,d0                     | +00e
        jsr     0x51a44.l                       | +014
        move.w  #0x10d8,d0                      | +01a
        jsr     0x2352.l                        | +01e
        jsr     0x2ad70.l                       | +024
        move.w  0x74(a6),d0                     | +02a
        addi.w  #0xa,d0                         | +02e
        move.w  #0x710f,d4                      | +032
        add.w   0x72(a6),d4                     | +036
        jsr     0x4772a.l                       | +03a
        cmpi.w  #0x0,0x74(a6)                   | +040
        bne.w   .L03e3dc                        | +046
        move.w  #0x718f,d0                      | +04a
        add.w   0x72(a6),d0                     | +04e
        move.w  #0x0,d1                         | +052
        lsl.w   #0x1,d1                         | +056
        addi.w  #0x4b60,d1                      | +058
        movem.w d0-d1,0x3c0000.l                | +05c
        addi.w  #0x20,d0                        | +064
        addq.w  #0x1,d1                         | +068
        movem.w d0-d1,0x3c0000.l                | +06a
        subi.w  #0x1f,d0                        | +072
        addi.w  #0xf,d1                         | +076
        movem.w d0-d1,0x3c0000.l                | +07a
        addi.w  #0x20,d0                        | +082
        addq.w  #0x1,d1                         | +086
        movem.w d0-d1,0x3c0000.l                | +088
        subi.w  #0x21,d0                        | +090
        move.w  #0x71cf,d0                      | +094
        add.w   0x72(a6),d0                     | +098
        move.w  #0x0,d1                         | +09c
        lsl.w   #0x1,d1                         | +0a0
        addi.w  #0x4b60,d1                      | +0a2
        movem.w d0-d1,0x3c0000.l                | +0a6
        addi.w  #0x20,d0                        | +0ae
        addq.w  #0x1,d1                         | +0b2
        movem.w d0-d1,0x3c0000.l                | +0b4
        subi.w  #0x1f,d0                        | +0bc
        addi.w  #0xf,d1                         | +0c0
        movem.w d0-d1,0x3c0000.l                | +0c4
        addi.w  #0x20,d0                        | +0cc
        addq.w  #0x1,d1                         | +0d0
        movem.w d0-d1,0x3c0000.l                | +0d2
        subi.w  #0x21,d0                        | +0da
.L03e3dc:
        lea     .L03e3e2(pc),a1                 | +0de
        move.l  a1,(a6)                         | +0e2
.L03e3e2:
        subq.w  #0x1,0x70(a6)                   | +0e4
        cmpi.w  #0x0,0x70(a6)                   | +0e8
        bge.w   JsrAbsThunk_03e402              | +0ee
        lea     Results_ColSpawnPrize_03e40a(pc),a1 | +0f2
        move.l  a1,(a6)                         | +0f6
        move.w  #0x1e,0x70(a6)                  | +0f8
        jsr     0x2a252.l                       | +0fe

| ----------------------------------------------------------------------------
|  Results_ColSpawnPrize_03e40a  @ $03E40A  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColSpawnPrize_03e40a, "ax", @progbits
        .global Results_ColSpawnPrize_03e40a
Results_ColSpawnPrize_03e40a:
        cmpi.w  #0x0,0x74(a6)                   | +000
        bne.w   Results_ColSpawnPrizeTie_03e438 | +006
        lea     Results_BannerB_03eb84(pc),a1   | +00a
        jsr     0x4ae.l                         | +00e
        move.w  0x72(a6),0x72(a0)               | +014
        move.w  #0x3c,0x70(a6)                  | +01a
        jsr     0x28d70.l                       | +020

| ----------------------------------------------------------------------------
|  Results_ColSpawnPrizeTie_03e438  @ $03E438  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColSpawnPrizeTie_03e438, "ax", @progbits
        .global Results_ColSpawnPrizeTie_03e438
Results_ColSpawnPrizeTie_03e438:
        movea.l 0xc(a6),a1                      | +000
        cmpi.b  #0x2,0x89(a1)                   | +004
        bne.w   Results_ColAnimWait_03e494      | +00a
        movea.l 0xc(a6),a1                      | +00e
        move.w  0x8c(a1),d0                     | +012
        cmpi.w  #0x2,d0                         | +016
        bne.w   Results_ColAnim_03e47a          | +01a
        lea     Results_BannerC_03ec18(pc),a1   | +01e
        jsr     0x4ae.l                         | +022
        move.w  0x72(a6),0x72(a0)               | +028
        move.w  #0x3c,0x70(a6)                  | +02e
        jsr     0x28d70.l                       | +034

| ----------------------------------------------------------------------------
|  Results_ColAnim_03e47a  @ $03E47A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColAnim_03e47a, "ax", @progbits
        .global Results_ColAnim_03e47a
Results_ColAnim_03e47a:
        cmp.w   0x76(a6),d0                     | +000
        beq.w   Results_ColRts_03e490           | +004
        jsr     0x28d70.l                       | +008

| ----------------------------------------------------------------------------
|  Results_ColRts_03e490  @ $03E490  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColRts_03e490, "ax", @progbits
        .global Results_ColRts_03e490
Results_ColRts_03e490:
        bra.w   Results_ColSpawnFinal_03e4b2    | +000

| ----------------------------------------------------------------------------
|  Results_ColAnimWait_03e494  @ $03E494  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColAnimWait_03e494, "ax", @progbits
        .global Results_ColAnimWait_03e494
Results_ColAnimWait_03e494:
        cmpi.w  #0xa,0x74(a6)                   | +000
        bcc.w   Results_ColSpawnFinal_03e4b2    | +006
        jsr     0x28d70.l                       | +00a
        move.w  #0x3c,0x70(a6)                  | +010

| ----------------------------------------------------------------------------
|  Results_ColSpawnFinal_03e4b2  @ $03E4B2  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColSpawnFinal_03e4b2, "ax", @progbits
        .global Results_ColSpawnFinal_03e4b2
Results_ColSpawnFinal_03e4b2:
        lea     Results_BannerA_03ea2e(pc),a1   | +000
        jsr     0x4ae.l                         | +004
        move.w  0x72(a6),0x72(a0)               | +00a
        move.w  0x68(a6),0x68(a0)               | +010
        move.w  0x6e(a6),0x6e(a0)               | +016
        move.w  0x74(a6),0x74(a0)               | +01c
        move.w  0x76(a6),0x76(a0)               | +022
        move.w  #0x64,0x70(a6)                  | +028
        lea     Results_ColFinish_03e4e6(pc),a1 | +02e
        move.l  a1,(a6)                         | +032

| ----------------------------------------------------------------------------
|  Results_ColFinish_03e4e6  @ $03E4E6  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ColFinish_03e4e6, "ax", @progbits
        .global Results_ColFinish_03e4e6
Results_ColFinish_03e4e6:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   JsrAbsThunk_03e504              | +00a
        movea.l 0xc(a6),a1                      | +00e
        move.b  #0x1,0x90(a1)                   | +012
        lea     JsrAbsThunk_03e50c(pc),a1       | +018
        move.l  a1,(a6)                         | +01c

| ----------------------------------------------------------------------------
|  Results_PowRoster_03e514  @ $03E514  (448 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PowRoster_03e514, "ax", @progbits
        .global Results_PowRoster_03e514
Results_PowRoster_03e514:
        clr.l   0x70(a6)                        | +000
        clr.l   0x74(a6)                        | +004
        clr.l   0x78(a6)                        | +008
        clr.l   0x7c(a6)                        | +00c
        clr.l   0x80(a6)                        | +010
        clr.l   0x84(a6)                        | +014
        clr.l   0x88(a6)                        | +018
        clr.l   0x8c(a6)                        | +01c
        clr.l   0x90(a6)                        | +020
        clr.l   0x94(a6)                        | +024
        clr.l   0x98(a6)                        | +028
        clr.l   0x9c(a6)                        | +02c
        clr.w   0x5c(a6)                        | +030
        clr.b   0x5e(a6)                        | +034
        lea     .L03e552(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L03e552:
        movea.l 0xc(a6),a1                      | +03e
        cmpi.b  #0x0,0x20(a1)                   | +042
        beq.w   JsrPcRts_03e6d8                 | +048
        clr.b   0x20(a1)                        | +04c
        cmpi.b  #0x0,0x5e(a6)                   | +050
        beq.w   .L03e5ec                        | +056
        move.l  0x74(a6),0x70(a6)               | +05a
        move.l  0x78(a6),0x74(a6)               | +060
        move.l  0x7c(a6),0x78(a6)               | +066
        move.l  0x80(a6),0x7c(a6)               | +06c
        move.l  0x84(a6),0x80(a6)               | +072
        move.l  0x88(a6),0x84(a6)               | +078
        move.l  0x8c(a6),0x88(a6)               | +07e
        move.b  0x91(a6),0x90(a6)               | +084
        move.b  0x92(a6),0x91(a6)               | +08a
        move.b  0x93(a6),0x92(a6)               | +090
        move.b  0x94(a6),0x93(a6)               | +096
        move.b  0x95(a6),0x94(a6)               | +09c
        move.b  0x96(a6),0x95(a6)               | +0a2
        move.b  0x97(a6),0x96(a6)               | +0a8
        move.b  0x99(a6),0x98(a6)               | +0ae
        move.b  0x9a(a6),0x99(a6)               | +0b4
        move.b  0x9b(a6),0x9a(a6)               | +0ba
        move.b  0x9c(a6),0x9b(a6)               | +0c0
        move.b  0x9d(a6),0x9c(a6)               | +0c6
        move.b  0x9e(a6),0x9d(a6)               | +0cc
        move.b  0x9f(a6),0x9e(a6)               | +0d2
.L03e5ec:
        cmpi.b  #0x0,0x10fd83.l                 | +0d8
        bne.w   .L03e600                        | +0e0
        jsr     Results_RosterPickA_03e6da(pc)  | +0e4
        bra.w   .L03e604                        | +0e8
.L03e600:
        jsr     Results_RosterPickB_03e740(pc)  | +0ec
.L03e604:
        addq.w  #0x1,0x5c(a6)                   | +0f0
        cmpi.w  #0x6,0x5c(a6)                   | +0f4
        bls.w   .L03e61e                        | +0fa
        move.b  #0xff,0x5e(a6)                  | +0fe
        move.w  #0x6,0x5c(a6)                   | +104
.L03e61e:
        cmpi.b  #0x0,0x68(a6)                   | +10a
        bne.w   .L03e62c                        | +110
        movea.w #0x7094,a1                      | +114
.L03e62c:
        cmpi.b  #0x1,0x68(a6)                   | +118
        bne.w   .L03e63a                        | +11e
        movea.w #0x72b4,a1                      | +122
.L03e63a:
        move.w  #0x2320,d0                      | +126
        move.w  #0xf,d1                         | +12a
        move.w  #0x7,d2                         | +12e
        jsr     0x5da9c.l                       | +132
        movea.l 0x70(a6),a1                     | +138
        move.b  0x90(a6),d2                     | +13c
        move.b  0x98(a6),d3                     | +140
        move.w  #0x7094,d0                      | +144
        jsr     Results_RosterDrawDispatch_03e7a6(pc) | +148
        movea.l 0x74(a6),a1                     | +14c
        move.b  0x91(a6),d2                     | +150
        move.b  0x99(a6),d3                     | +154
        move.w  #0x7095,d0                      | +158
        jsr     Results_RosterDrawDispatch_03e7a6(pc) | +15c
        movea.l 0x78(a6),a1                     | +160
        move.b  0x92(a6),d2                     | +164
        move.b  0x9a(a6),d3                     | +168
        move.w  #0x7096,d0                      | +16c
        jsr     Results_RosterDrawDispatch_03e7a6(pc) | +170
        movea.l 0x7c(a6),a1                     | +174
        move.b  0x93(a6),d2                     | +178
        move.b  0x9b(a6),d3                     | +17c
        move.w  #0x7097,d0                      | +180
        jsr     Results_RosterDrawDispatch_03e7a6(pc) | +184
        movea.l 0x80(a6),a1                     | +188
        move.b  0x94(a6),d2                     | +18c
        move.b  0x9c(a6),d3                     | +190
        move.w  #0x7098,d0                      | +194
        jsr     Results_RosterDrawDispatch_03e7a6(pc) | +198
        movea.l 0x84(a6),a1                     | +19c
        move.b  0x95(a6),d2                     | +1a0
        move.b  0x9d(a6),d3                     | +1a4
        move.w  #0x7099,d0                      | +1a8
        jsr     Results_RosterDrawDispatch_03e7a6(pc) | +1ac
        movea.l 0x88(a6),a1                     | +1b0
        move.b  0x96(a6),d2                     | +1b4
        move.b  0x9e(a6),d3                     | +1b8
        move.w  #0x709a,d0                      | +1bc

| ----------------------------------------------------------------------------
|  Results_RosterPickA_03e6da  @ $03E6DA  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Results_RosterPickA_03e6da, "ax", @progbits
        .global Results_RosterPickA_03e6da
Results_RosterPickA_03e6da:
        move.w  #0x40,d0                        | +000
        jsr     0x5e9e4.l                       | +004
        move.w  0x5c(a6),d1                     | +00a
        addi.w  #0x98,d1                        | +00e
        move.b  d0,(a6,d1.w)                    | +012
        move.w  #0x3b2,d0                       | +016
        jsr     0x5e9e4.l                       | +01a
        lea     0x282de0.l,a1                   | +020
        clr.l   d2                              | +026
        move.b  (a1,d0.w),d2                    | +028
        move.w  0x5c(a6),d1                     | +02c
        addi.w  #0x90,d1                        | +030
        move.b  d2,(a6,d1.w)                    | +034
        move.l  d2,d3                           | +038
        subq.w  #0x1,d2                         | +03a
        lsl.w   #0x3,d2                         | +03c
        lea     0x2831b2.l,a1                   | +03e
        movea.l (a1,d2.w),a2                    | +044
        move.l  0x4(a1,d2.w),d0                 | +048
        jsr     0x5e9e4.l                       | +04c
        mulu.w  d3,d0                           | +052
        adda.l  d0,a2                           | +054
        move.w  0x5c(a6),d1                     | +056
        lsl.w   #0x2,d1                         | +05a
        addi.w  #0x70,d1                        | +05c
        move.l  a2,(a6,d1.w)                    | +060
        rts                                     | +064

| ----------------------------------------------------------------------------
|  Results_RosterPickB_03e740  @ $03E740  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Results_RosterPickB_03e740, "ax", @progbits
        .global Results_RosterPickB_03e740
Results_RosterPickB_03e740:
        move.w  #0x40,d0                        | +000
        jsr     0x5e9e4.l                       | +004
        move.w  0x5c(a6),d1                     | +00a
        addi.w  #0x98,d1                        | +00e
        move.b  d0,(a6,d1.w)                    | +012
        move.w  #0x14c,d0                       | +016
        jsr     0x5e9e4.l                       | +01a
        lea     0x282c3c.l,a1                   | +020
        clr.l   d2                              | +026
        move.b  (a1,d0.w),d2                    | +028
        move.w  0x5c(a6),d1                     | +02c
        addi.w  #0x90,d1                        | +030
        move.b  d2,(a6,d1.w)                    | +034
        move.l  d2,d3                           | +038
        subq.w  #0x2,d2                         | +03a
        lsl.w   #0x3,d2                         | +03c
        lea     0x282d88.l,a1                   | +03e
        movea.l (a1,d2.w),a2                    | +044
        move.l  0x4(a1,d2.w),d0                 | +048
        jsr     0x5e9e4.l                       | +04c
        mulu.w  d3,d0                           | +052
        adda.l  d0,a2                           | +054
        move.w  0x5c(a6),d1                     | +056
        lsl.w   #0x2,d1                         | +05a
        addi.w  #0x70,d1                        | +05c
        move.l  a2,(a6,d1.w)                    | +060
        rts                                     | +064

| ----------------------------------------------------------------------------
|  Results_RosterDrawDispatch_03e7a6  @ $03E7A6  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Results_RosterDrawDispatch_03e7a6, "ax", @progbits
        .global Results_RosterDrawDispatch_03e7a6
Results_RosterDrawDispatch_03e7a6:
        cmpi.b  #0x0,0x10fd83.l                 | +000
        bne.w   JsrPcThunk_03e7ba               | +008
        jsr     Results_RosterDrawA_03e7c0(pc)  | +00c
        bra.w   JsrPcRts_03e7be                 | +010

| ----------------------------------------------------------------------------
|  Results_RosterDrawA_03e7c0  @ $03E7C0  (132 B)
| ----------------------------------------------------------------------------
        .section .text.Results_RosterDrawA_03e7c0, "ax", @progbits
        .global Results_RosterDrawA_03e7c0
Results_RosterDrawA_03e7c0:
        cmpa.l  #0x0,a1                         | +000
        bne.w   .L03e7cc                        | +006
        rts                                     | +00a
.L03e7cc:
        lea     0x282a9c.l,a2                   | +00c
        clr.l   d1                              | +012
        move.b  d2,d1                           | +014
        lsl.w   #0x1,d1                         | +016
        add.w   (a2,d1.w),d0                    | +018
        cmpi.b  #0x1,0x68(a6)                   | +01c
        bne.w   .L03e7ea                        | +022
        addi.w  #0x220,d0                       | +026
.L03e7ea:
        clr.w   d1                              | +02a
        move.b  (a1)+,d1                        | +02c
        cmpi.b  #0x80,d1                        | +02e
        bcc.w   .L03e7fe                        | +032
        ori.w   #0x2300,d1                      | +036
        bra.w   .L03e806                        | +03a
.L03e7fe:
        subi.w  #0x80,d1                        | +03e
        ori.w   #0x2a00,d1                      | +042
.L03e806:
        movem.w d0-d1,0x3c0000.l                | +046
        addi.w  #0x20,d0                        | +04e
        subq.b  #0x1,d2                         | +052
        cmpi.b  #0x0,d2                         | +054
        bgt.b   .L03e7ea                        | +058
        move.w  #0x2320,d1                      | +05a
        movem.w d0-d1,0x3c0000.l                | +05e
        addi.w  #0x20,d0                        | +066
        movea.w d0,a1                           | +06a
        lea     0x282bbc.l,a2                   | +06c
        clr.w   d4                              | +072
        move.b  d3,d4                           | +074
        lsl.w   #0x1,d4                         | +076
        move.w  (a2,d4.w),d0                    | +078
        move.w  #0x4,d1                         | +07c
        move.w  #0x1,d2                         | +080

| ----------------------------------------------------------------------------
|  Results_RosterDrawB_03e84c  @ $03E84C  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Results_RosterDrawB_03e84c, "ax", @progbits
        .global Results_RosterDrawB_03e84c
Results_RosterDrawB_03e84c:
        cmpa.l  #0x0,a1                         | +000
        bne.w   .L03e858                        | +006
        rts                                     | +00a
.L03e858:
        cmpi.b  #0x1,0x68(a6)                   | +00c
        bne.w   .L03e866                        | +012
        addi.w  #0x220,d0                       | +016
.L03e866:
        lea     0x282abc.l,a2                   | +01a
        clr.l   d4                              | +020
        move.b  d3,d4                           | +022
        lsl.l   #0x2,d4                         | +024
        adda.l  d4,a2                           | +026
        move.w  #0x2300,d1                      | +028
        add.b   (a2)+,d1                        | +02c
        movem.w d0-d1,0x3c0000.l                | +02e
        addi.w  #0x20,d0                        | +036
        move.w  #0x2300,d1                      | +03a
        add.b   (a2)+,d1                        | +03e
        movem.w d0-d1,0x3c0000.l                | +040
        addi.w  #0x20,d0                        | +048
        move.w  #0x2300,d1                      | +04c
        add.b   (a2)+,d1                        | +050
        movem.w d0-d1,0x3c0000.l                | +052
        addi.w  #0x20,d0                        | +05a
        move.w  #0x2300,d1                      | +05e
        add.b   (a2),d1                         | +062
        movem.w d0-d1,0x3c0000.l                | +064
        addi.w  #0x20,d0                        | +06c
.L03e8bc:
        move.w  #0x2300,d1                      | +070
        add.b   (a1)+,d1                        | +074
        movem.w d0-d1,0x3c0000.l                | +076
        addi.w  #0x20,d0                        | +07e
        subq.b  #0x1,d2                         | +082
        cmpi.b  #0x0,d2                         | +084
        bgt.b   .L03e8bc                        | +088
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  Results_ClampScoreP1_03e8d8  @ $03E8D8  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ClampScoreP1_03e8d8, "ax", @progbits
        .global Results_ClampScoreP1_03e8d8
Results_ClampScoreP1_03e8d8:
        move.w  0x106f4c.l,d1                   | +000
        addq.w  #0x1,d1                         | +006
        cmpi.w  #0xe,d1                         | +008
        bls.w   .L03e8ec                        | +00c
        move.w  #0xe,d1                         | +010
.L03e8ec:
        lea     0x2829fe.l,a1                   | +014
        lsl.w   #0x1,d1                         | +01a
        move.w  (a1,d1.w),d0                    | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Results_ClampScoreP2_03e8fa  @ $03E8FA  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Results_ClampScoreP2_03e8fa, "ax", @progbits
        .global Results_ClampScoreP2_03e8fa
Results_ClampScoreP2_03e8fa:
        move.w  0x106f4e.l,d1                   | +000
        addq.w  #0x1,d1                         | +006
        cmpi.w  #0xe,d1                         | +008
        bls.w   .L03e90e                        | +00c
        move.w  #0xe,d1                         | +010
.L03e90e:
        lea     0x282a1e.l,a1                   | +014
        lsl.w   #0x1,d1                         | +01a
        move.w  (a1,d1.w),d0                    | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Results_PrizeSprite_03e91c  @ $03E91C  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PrizeSprite_03e91c, "ax", @progbits
        .global Results_PrizeSprite_03e91c
Results_PrizeSprite_03e91c:
        move.w  #0xa2,d1                        | +000
        jsr     0x29f2.l                        | +004
        move.w  0x14(a6),d1                     | +00a
        jsr     0x2c66.l                        | +00e
        move.w  #0xffff,0x38(a6)                | +014
        bset    #0x6,0x12(a6)                   | +01a
        lea     0x28295a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        jsr     0x267e2.l                       | +02c
        move.w  #0x11c,0x24(a6)                 | +032
        cmpi.w  #0x0,0x76(a6)                   | +038
        bne.w   .L03e96c                        | +03e
        lea     0x2829fe.l,a1                   | +042
        move.w  #0x60,d2                        | +048
        bra.w   .L03e976                        | +04c
.L03e96c:
        lea     0x282a1e.l,a1                   | +050
        move.w  #0xe8,d2                        | +056
.L03e976:
        move.w  0x5c(a6),d0                     | +05a
        cmpi.w  #0xe,0x5c(a6)                   | +05e
        bls.w   .L03e988                        | +064
        move.w  #0xe,d0                         | +068
.L03e988:
        lsl.w   #0x1,d0                         | +06c
        move.w  (a1,d0.w),0x22(a6)              | +06e
        sub.w   0x22(a6),d2                     | +074
        move.w  d2,d0                           | +078
        move.w  #0x7d,d1                        | +07a
        jsr     0x5e018.l                       | +07e
        move.w  d0,0x34(a6)                     | +084
        move.w  #0x1000,d1                      | +088
        jsr     0x13c0e.l                       | +08c
        move.w  d1,0x28(a6)                     | +092
        move.w  d2,0x2a(a6)                     | +096
        lea     .L03e9bc(pc),a1                 | +09a
        move.l  a1,(a6)                         | +09e
.L03e9bc:
        movea.l 0xc(a6),a1                      | +0a0
        movea.l 0xc(a1),a2                      | +0a4
        cmpi.b  #0x1,0x8a(a2)                   | +0a8
        bne.w   Results_PrizeFall_03e9dc        | +0ae
        lea     Results_PrizeLand_03e9fa(pc),a1 | +0b2
        move.l  a1,(a6)                         | +0b6

| ----------------------------------------------------------------------------
|  Results_PrizeFall_03e9dc  @ $03E9DC  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PrizeFall_03e9dc, "ax", @progbits
        .global Results_PrizeFall_03e9dc
Results_PrizeFall_03e9dc:
        jsr     0x8d2f8.l                       | +000
        cmpi.w  #0x198,0x24(a6)                 | +006
        ble.w   JsrAbsThunk_03e9f2              | +00c
        lea     Results_PrizeLand_03e9fa(pc),a1 | +010
        move.l  a1,(a6)                         | +014

| ----------------------------------------------------------------------------
|  Results_PrizeLand_03e9fa  @ $03E9FA  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PrizeLand_03e9fa, "ax", @progbits
        .global Results_PrizeLand_03e9fa
Results_PrizeLand_03e9fa:
        move.w  #0x10d6,d0                      | +000
        jsr     0x2352.l                        | +004
        movea.l 0xc(a6),a1                      | +00a
        move.b  #0xff,0x20(a1)                  | +00e
        move.w  0x74(a6),d0                     | +014
        move.w  #0x714c,d4                      | +018
        add.w   0x72(a6),d4                     | +01c
        jsr     0x4772a.l                       | +020

| ----------------------------------------------------------------------------
|  Results_BannerA_03ea2e  @ $03EA2E  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Results_BannerA_03ea2e, "ax", @progbits
        .global Results_BannerA_03ea2e
Results_BannerA_03ea2e:
        move.w  #0x6,0x30(a6)                   | +000
        move.w  #0x0,0x70(a6)                   | +006
        move.b  #0x0,0x5c(a6)                   | +00c
        lea     .L03ea46(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L03ea46:
        subq.w  #0x1,0x70(a6)                   | +018
        cmpi.w  #0x0,0x70(a6)                   | +01c
        bgt.w   Results_BannerABlink_03ea66__L03eaa0 | +022
        cmpi.w  #0x0,0x30(a6)                   | +026
        bgt.w   Results_BannerABlink_03ea66     | +02c

| ----------------------------------------------------------------------------
|  Results_BannerABlink_03ea66  @ $03EA66  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Results_BannerABlink_03ea66, "ax", @progbits
        .global Results_BannerABlink_03ea66
Results_BannerABlink_03ea66:
        cmpi.b  #0x0,0x5c(a6)                   | +000
        bne.w   .L03ea7a                        | +006
        lea     0x282a4c.l,a2                   | +00a
        bra.w   .L03ea80                        | +010
.L03ea7a:
        lea     0x282a54.l,a2                   | +014
.L03ea80:
        movea.w #0x70d1,a1                      | +01a
        adda.w  0x72(a6),a1                     | +01e
        move.b  #0x4,d1                         | +022
        jsr     0x4784c.l                       | +026
        move.w  #0x8,0x70(a6)                   | +02c
        not.b   0x5c(a6)                        | +032
        subq.w  #0x1,0x30(a6)                   | +036
        .global Results_BannerABlink_03ea66__L03eaa0
Results_BannerABlink_03ea66__L03eaa0:
.L03eaa0:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Results_BannerAFinal_03eaa2  @ $03EAA2  (218 B)
| ----------------------------------------------------------------------------
        .section .text.Results_BannerAFinal_03eaa2, "ax", @progbits
        .global Results_BannerAFinal_03eaa2
Results_BannerAFinal_03eaa2:
        move.w  #0x1083,d0                      | +000
        jsr     0x2352.l                        | +004
        move.l  #0x100000,d0                    | +00a
        jsr     0x51a44.l                       | +010
        move.w  #0x70cf,d0                      | +016
        add.w   0x72(a6),d0                     | +01a
        move.w  #0x1,d1                         | +01e
        lsl.w   #0x1,d1                         | +022
        addi.w  #0x4b60,d1                      | +024
        movem.w d0-d1,0x3c0000.l                | +028
        addi.w  #0x20,d0                        | +030
        addq.w  #0x1,d1                         | +034
        movem.w d0-d1,0x3c0000.l                | +036
        subi.w  #0x1f,d0                        | +03e
        addi.w  #0xf,d1                         | +042
        movem.w d0-d1,0x3c0000.l                | +046
        addi.w  #0x20,d0                        | +04e
        addq.w  #0x1,d1                         | +052
        movem.w d0-d1,0x3c0000.l                | +054
        subi.w  #0x21,d0                        | +05c
        cmpi.w  #0xa,0x74(a6)                   | +060
        bcc.w   .L03eb64                        | +066
        move.w  0x76(a6),d0                     | +06a
        jsr     0x2ad36.l                       | +06e
        bcs.w   .L03eb64                        | +074
        move.w  #0x710f,d0                      | +078
        add.w   0x72(a6),d0                     | +07c
        move.w  #0x0,d1                         | +080
        lsl.w   #0x1,d1                         | +084
        addi.w  #0x4b60,d1                      | +086
        movem.w d0-d1,0x3c0000.l                | +08a
        addi.w  #0x20,d0                        | +092
        addq.w  #0x1,d1                         | +096
        movem.w d0-d1,0x3c0000.l                | +098
        subi.w  #0x1f,d0                        | +0a0
        addi.w  #0xf,d1                         | +0a4
        movem.w d0-d1,0x3c0000.l                | +0a8
        addi.w  #0x20,d0                        | +0b0
        addq.w  #0x1,d1                         | +0b4
        movem.w d0-d1,0x3c0000.l                | +0b6
        subi.w  #0x21,d0                        | +0be
.L03eb64:
        movea.w #0x70d1,a1                      | +0c2
        adda.w  0x72(a6),a1                     | +0c6
        move.b  #0x4,d1                         | +0ca
        lea     0x282a4c.l,a2                   | +0ce
        jsr     0x4784c.l                       | +0d4

| ----------------------------------------------------------------------------
|  Results_BannerB_03eb84  @ $03EB84  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Results_BannerB_03eb84, "ax", @progbits
        .global Results_BannerB_03eb84
Results_BannerB_03eb84:
        move.w  #0x6,0x30(a6)                   | +000
        move.w  #0x0,0x70(a6)                   | +006
        move.b  #0x0,0x5c(a6)                   | +00c
        lea     .L03eb9c(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L03eb9c:
        subq.w  #0x1,0x70(a6)                   | +018
        cmpi.w  #0x0,0x70(a6)                   | +01c
        bgt.w   Results_BannerBBlink_03ebbc__L03ebf6 | +022
        cmpi.w  #0x0,0x30(a6)                   | +026
        bgt.w   Results_BannerBBlink_03ebbc     | +02c

| ----------------------------------------------------------------------------
|  Results_BannerBBlink_03ebbc  @ $03EBBC  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Results_BannerBBlink_03ebbc, "ax", @progbits
        .global Results_BannerBBlink_03ebbc
Results_BannerBBlink_03ebbc:
        cmpi.b  #0x0,0x5c(a6)                   | +000
        bne.w   .L03ebd0                        | +006
        lea     0x282a7c.l,a2                   | +00a
        bra.w   .L03ebd6                        | +010
.L03ebd0:
        lea     0x282a88.l,a2                   | +014
.L03ebd6:
        movea.w #0x70d1,a1                      | +01a
        adda.w  0x72(a6),a1                     | +01e
        move.b  #0x4,d1                         | +022
        jsr     0x477fc.l                       | +026
        move.w  #0x8,0x70(a6)                   | +02c
        not.b   0x5c(a6)                        | +032
        subq.w  #0x1,0x30(a6)                   | +036
        .global Results_BannerBBlink_03ebbc__L03ebf6
Results_BannerBBlink_03ebbc__L03ebf6:
.L03ebf6:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Results_BannerBDraw_03ebf8  @ $03EBF8  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Results_BannerBDraw_03ebf8, "ax", @progbits
        .global Results_BannerBDraw_03ebf8
Results_BannerBDraw_03ebf8:
        movea.w #0x70d1,a1                      | +000
        adda.w  0x72(a6),a1                     | +004
        move.b  #0x4,d1                         | +008
        lea     0x282a7c.l,a2                   | +00c
        jsr     0x477fc.l                       | +012

| ----------------------------------------------------------------------------
|  Results_BannerC_03ec18  @ $03EC18  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Results_BannerC_03ec18, "ax", @progbits
        .global Results_BannerC_03ec18
Results_BannerC_03ec18:
        move.w  #0x6,0x30(a6)                   | +000
        move.w  #0x0,0x70(a6)                   | +006
        move.b  #0x0,0x5c(a6)                   | +00c
        lea     .L03ec30(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L03ec30:
        subq.w  #0x1,0x70(a6)                   | +018
        cmpi.w  #0x0,0x70(a6)                   | +01c
        bgt.w   Results_BannerCBlink_03ec50__L03ec8a | +022
        cmpi.w  #0x0,0x30(a6)                   | +026
        bgt.w   Results_BannerCBlink_03ec50     | +02c

| ----------------------------------------------------------------------------
|  Results_BannerCBlink_03ec50  @ $03EC50  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Results_BannerCBlink_03ec50, "ax", @progbits
        .global Results_BannerCBlink_03ec50
Results_BannerCBlink_03ec50:
        cmpi.b  #0x0,0x5c(a6)                   | +000
        bne.w   .L03ec64                        | +006
        lea     0x282a5c.l,a2                   | +00a
        bra.w   .L03ec6a                        | +010
.L03ec64:
        lea     0x282a62.l,a2                   | +014
.L03ec6a:
        movea.w #0x70f1,a1                      | +01a
        adda.w  0x72(a6),a1                     | +01e
        move.b  #0x4,d1                         | +022
        jsr     0x4784c.l                       | +026
        move.w  #0x8,0x70(a6)                   | +02c
        not.b   0x5c(a6)                        | +032
        subq.w  #0x1,0x30(a6)                   | +036
        .global Results_BannerCBlink_03ec50__L03ec8a
Results_BannerCBlink_03ec50__L03ec8a:
.L03ec8a:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Results_BannerCDraw_03ec8c  @ $03EC8C  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Results_BannerCDraw_03ec8c, "ax", @progbits
        .global Results_BannerCDraw_03ec8c
Results_BannerCDraw_03ec8c:
        movea.w #0x70f1,a1                      | +000
        adda.w  0x72(a6),a1                     | +004
        move.b  #0x4,d1                         | +008
        lea     0x282a5c.l,a2                   | +00c
        jsr     0x4784c.l                       | +012

| ----------------------------------------------------------------------------
|  Results_DrawLabels_03ecac  @ $03ECAC  (292 B)
| ----------------------------------------------------------------------------
        .section .text.Results_DrawLabels_03ecac, "ax", @progbits
        .global Results_DrawLabels_03ecac
Results_DrawLabels_03ecac:
        movea.w #0x7068,a1                      | +000
        adda.w  0x72(a6),a1                     | +004
        move.w  #0x23ba,d0                      | +008
        move.w  #0x1,d1                         | +00c
        move.w  #0x1,d2                         | +010
        jsr     0x5da9c.l                       | +014
        movea.w #0x7088,a1                      | +01a
        adda.w  0x72(a6),a1                     | +01e
        move.w  #0x23bf,d0                      | +022
        move.w  #0xf,d1                         | +026
        move.w  #0x1,d2                         | +02a
        jsr     0x5da9c.l                       | +02e
        movea.w #0x7268,a1                      | +034
        adda.w  0x72(a6),a1                     | +038
        move.w  #0x23bb,d0                      | +03c
        move.w  #0x1,d1                         | +040
        move.w  #0x1,d2                         | +044
        jsr     0x5da9c.l                       | +048
        movea.w #0x7093,a1                      | +04e
        adda.w  0x72(a6),a1                     | +052
        move.w  #0x23cc,d0                      | +056
        move.w  #0x1,d1                         | +05a
        move.w  #0x1,d2                         | +05e
        jsr     0x5da9c.l                       | +062
        movea.w #0x70b3,a1                      | +068
        adda.w  0x72(a6),a1                     | +06c
        move.w  #0x23cd,d0                      | +070
        move.w  #0xd,d1                         | +074
        move.w  #0x1,d2                         | +078
        jsr     0x5da9c.l                       | +07c
        movea.w #0x7253,a1                      | +082
        adda.w  0x72(a6),a1                     | +086
        move.w  #0x23ce,d0                      | +08a
        move.w  #0x1,d1                         | +08e
        move.w  #0x1,d2                         | +092
        jsr     0x5da9c.l                       | +096
        movea.w #0x707b,a1                      | +09c
        adda.w  0x72(a6),a1                     | +0a0
        move.w  #0x23bc,d0                      | +0a4
        move.w  #0x1,d1                         | +0a8
        move.w  #0x1,d2                         | +0ac
        jsr     0x5da9c.l                       | +0b0
        movea.w #0x709b,a1                      | +0b6
        adda.w  0x72(a6),a1                     | +0ba
        move.w  #0x23cb,d0                      | +0be
        move.w  #0xf,d1                         | +0c2
        move.w  #0x1,d2                         | +0c6
        jsr     0x5da9c.l                       | +0ca
        movea.w #0x727b,a1                      | +0d0
        adda.w  0x72(a6),a1                     | +0d4
        move.w  #0x23bd,d0                      | +0d8
        move.w  #0x1,d1                         | +0dc
        move.w  #0x1,d2                         | +0e0
        jsr     0x5da9c.l                       | +0e4
        movea.w #0x7069,a1                      | +0ea
        adda.w  0x72(a6),a1                     | +0ee
        move.w  #0x23be,d0                      | +0f2
        move.w  #0x1,d1                         | +0f6
        move.w  #0x12,d2                        | +0fa
        jsr     0x5da9c.l                       | +0fe
        movea.w #0x7269,a1                      | +104
        adda.w  0x72(a6),a1                     | +108
        move.w  #0x23ca,d0                      | +10c
        move.w  #0x1,d1                         | +110
        move.w  #0x12,d2                        | +114
        jsr     0x5da9c.l                       | +118
        jmp     0x518.l                         | +11e

| ----------------------------------------------------------------------------
|  Rts_03edd0  @ $03EDD0  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_03edd0, "ax", @progbits
        .global Rts_03edd0
Rts_03edd0:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Results_PollStartP1_03edd2  @ $03EDD2  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PollStartP1_03edd2, "ax", @progbits
        .global Results_PollStartP1_03edd2
Results_PollStartP1_03edd2:
        move.b  0x10e214.l,d0                   | +000
        btst    #0x5,d0                         | +006
        bne.w   .L03ede8                        | +00a
        clr.b   0x8a(a6)                        | +00e
        bra.w   Results_PollStartP2_03edf4      | +012
.L03ede8:
        move.b  #0x1,0x8a(a6)                   | +016

| ----------------------------------------------------------------------------
|  Results_PollStartP2_03edf4  @ $03EDF4  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PollStartP2_03edf4, "ax", @progbits
        .global Results_PollStartP2_03edf4
Results_PollStartP2_03edf4:
        move.b  0x10e21a.l,d0                   | +000
        btst    #0x5,d0                         | +006
        bne.w   Results_PollSetDone_03ee10      | +00a
        clr.b   0x8a(a6)                        | +00e

| ----------------------------------------------------------------------------
|  Results_PollRts_03ee0c  @ $03EE0C  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PollRts_03ee0c, "ax", @progbits
        .global Results_PollRts_03ee0c
Results_PollRts_03ee0c:
        bra.w   Stub_0003EE1C                   | +000

| ----------------------------------------------------------------------------
|  Results_PollSetDone_03ee10  @ $03EE10  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PollSetDone_03ee10, "ax", @progbits
        .global Results_PollSetDone_03ee10
Results_PollSetDone_03ee10:
        move.b  #0x1,0x8a(a6)                   | +000

| ----------------------------------------------------------------------------
|  Results_PollTail_03ee1e  @ $03EE1E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Results_PollTail_03ee1e, "ax", @progbits
        .global Results_PollTail_03ee1e
Results_PollTail_03ee1e:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_03ee34                    | +00c

| ----------------------------------------------------------------------------
|  Subsystem_ScoresInit_03EE3A  @ $03EE3A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Subsystem_ScoresInit_03EE3A, "ax", @progbits
        .global Subsystem_ScoresInit_03EE3A
Subsystem_ScoresInit_03EE3A:
        clr.w   0x106f4c.l                      | +000
        clr.w   0x106f4e.l                      | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Pow_CountIfPending_03ee48  @ $03EE48  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_CountIfPending_03ee48, "ax", @progbits
        .global Pow_CountIfPending_03ee48
Pow_CountIfPending_03ee48:
        tst.b   0x106ed2.l                      | +000
        bne.w   Pow_RescueCredit_03ee58         | +006

| ----------------------------------------------------------------------------
|  Pow_RescueCredit_03ee58  @ $03EE58  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueCredit_03ee58, "ax", @progbits
        .global Pow_RescueCredit_03ee58
Pow_RescueCredit_03ee58:
        move.l  #0xffffffff,-(a7)               | +000
        moveq   #0,d0                           | +006
        jsr     0x5e3a2.l                       | +008
        bcc.w   .L03ee74                        | +00e
        bsr.w   Pow_RectOverlap_03ef5c          | +012
        bcc.w   .L03ee74                        | +016
        move.l  a0,(a7)                         | +01a
.L03ee74:
        moveq   #1,d0                           | +01c
        jsr     0x5e3a2.l                       | +01e
        bcc.w   .L03eea0                        | +024
        bsr.w   Pow_RectOverlap_03ef5c          | +028
        bcc.w   .L03eea0                        | +02c
        move.l  (a7),d0                         | +030
        bmi.w   .L03ee9e                        | +032
        movea.l d0,a2                           | +036
        move.w  0x22(a0),d0                     | +038
        cmp.w   0x22(a2),d0                     | +03c
        bcc.w   .L03ee9e                        | +040
        movea.l a2,a0                           | +044
.L03ee9e:
        move.l  a0,(a7)                         | +046
.L03eea0:
        moveq   #0,d0                           | +048
        move.b  #0x1,d2                         | +04a
        move.l  (a7)+,d1                        | +04e
        bpl.w   Pow_RescueCreditP2_03eeb6       | +050

| ----------------------------------------------------------------------------
|  Pow_RescueRts_03eeb2  @ $03EEB2  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueRts_03eeb2, "ax", @progbits
        .global Pow_RescueRts_03eeb2
Pow_RescueRts_03eeb2:
        bra.w   SetC_03eec6                     | +000

| ----------------------------------------------------------------------------
|  Pow_RescueCreditP2_03eeb6  @ $03EEB6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueCreditP2_03eeb6, "ax", @progbits
        .global Pow_RescueCreditP2_03eeb6
Pow_RescueCreditP2_03eeb6:
        cmpi.l  #0x100440,d1                    | +000
        beq.w   SetC_03eec6                     | +006
        moveq   #1,d0                           | +00a
        move.b  #0x2,d2                         | +00c

| ----------------------------------------------------------------------------
|  Pow_CountIfPendingB_03eecc  @ $03EECC  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_CountIfPendingB_03eecc, "ax", @progbits
        .global Pow_CountIfPendingB_03eecc
Pow_CountIfPendingB_03eecc:
        tst.b   0x106ed2.l                      | +000
        bne.w   Pow_RescueCreditB_03eedc        | +006

| ----------------------------------------------------------------------------
|  Pow_RescueCreditB_03eedc  @ $03EEDC  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueCreditB_03eedc, "ax", @progbits
        .global Pow_RescueCreditB_03eedc
Pow_RescueCreditB_03eedc:
        move.l  #0xffffffff,-(a7)               | +000
        moveq   #0,d0                           | +006
        jsr     0x5e3a2.l                       | +008
        bcc.w   .L03eefc                        | +00e
        lea     Pow_AddRescued_03ef9a(pc),a1    | +012
        bsr.w   Pow_RectOverlap_03ef5c          | +016
        bcc.w   .L03eefc                        | +01a
        move.l  a0,(a7)                         | +01e
.L03eefc:
        moveq   #0,d0                           | +020
        move.b  #0x1,d2                         | +022
        move.l  (a7)+,d1                        | +026
        bpl.w   SetC_03ef0e                     | +028

| ----------------------------------------------------------------------------
|  Pow_CountIfPendingC_03ef14  @ $03EF14  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_CountIfPendingC_03ef14, "ax", @progbits
        .global Pow_CountIfPendingC_03ef14
Pow_CountIfPendingC_03ef14:
        tst.b   0x106ed2.l                      | +000
        bne.w   Pow_RescueCreditC_03ef24        | +006

| ----------------------------------------------------------------------------
|  Pow_RescueCreditC_03ef24  @ $03EF24  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueCreditC_03ef24, "ax", @progbits
        .global Pow_RescueCreditC_03ef24
Pow_RescueCreditC_03ef24:
        move.l  #0xffffffff,-(a7)               | +000
        moveq   #1,d0                           | +006
        jsr     0x5e3a2.l                       | +008
        bcc.w   .L03ef44                        | +00e
        lea     Pow_AddRescued_03ef9a(pc),a1    | +012
        bsr.w   Pow_RectOverlap_03ef5c          | +016
        bcc.w   .L03ef44                        | +01a
        move.l  a0,(a7)                         | +01e
.L03ef44:
        moveq   #1,d0                           | +020
        move.b  #0x2,d2                         | +022
        move.l  (a7)+,d1                        | +026
        bpl.w   SetC_03ef56                     | +028

| ----------------------------------------------------------------------------
|  Pow_RectOverlap_03ef5c  @ $03EF5C  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RectOverlap_03ef5c, "ax", @progbits
        .global Pow_RectOverlap_03ef5c
Pow_RectOverlap_03ef5c:
        btst    #0x3,0x13(a0)                   | +000
        bne.w   ClearC_03ef94                   | +006
        btst    #0x0,0x13(a0)                   | +00a
        bne.w   ClearC_03ef94                   | +010
        move.w  0x22(a0),d0                     | +014
        sub.w   0x22(a6),d0                     | +018
        add.w   (a1),d0                         | +01c
        cmp.w   0x2(a1),d0                      | +01e
        bcc.w   .L03ef92                        | +022
        move.w  0x24(a0),d0                     | +026
        sub.w   0x24(a6),d0                     | +02a
        add.w   0x4(a1),d0                      | +02e
        cmp.w   0x6(a1),d0                      | +032
.L03ef92:
        rts                                     | +036

| ----------------------------------------------------------------------------
|  Pow_AddRescued_03ef9a  @ $03EF9A  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_AddRescued_03ef9a, "ax", @progbits
        .global Pow_AddRescued_03ef9a
Pow_AddRescued_03ef9a:
        ori.b   #0x20,(a0)                      | +000
        ori.b   #0x38,(a0)                      | +004
        .global Pow_AddRescued_03ef9a__L03efa2
Pow_AddRescued_03ef9a__L03efa2:
.L03efa2:
        movem.w d0,-(a7)                        | +008
        jsr     0x5e3a2.l                       | +00c
        movem.w (a7)+,d0                        | +012
        bcc.w   .L03efcc                        | +016
        tst.b   d0                              | +01a
        bne.w   .L03efc4                        | +01c
        lea     0x106f4c.l,a1                   | +020
        bra.w   .L03efca                        | +026
.L03efc4:
        lea     0x106f4e.l,a1                   | +02a
.L03efca:
        addq.w  #0x1,(a1)                       | +030
.L03efcc:
        rts                                     | +032

| ----------------------------------------------------------------------------
|  Pow_OffworldFree_03efce  @ $03EFCE  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_OffworldFree_03efce, "ax", @progbits
        .global Pow_OffworldFree_03efce
Pow_OffworldFree_03efce:
        lea     Pow_HitboxHdr_03efe4(pc),a0     | +000
        jsr     0x5dd56.l                       | +004
        bcc.w   .L03efe2                        | +00a
        jmp     0x518.l                         | +00e
.L03efe2:
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Pow_HitboxHdr_03efe4  @ $03EFE4  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_HitboxHdr_03efe4, "ax", @progbits
        .global Pow_HitboxHdr_03efe4
Pow_HitboxHdr_03efe4:
        .dc.w   0xffd8                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +002  (dato / opcode no decodificado)
        .dc.w   0xff60                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +006  (dato / opcode no decodificado)
        move.b  0x9b(a6),d0                     | +008
        move.w  #0x9c,d1                        | +00c
        jmp     0x9a7cc.l                       | +010

| ----------------------------------------------------------------------------
|  Pow_AnimTblA_03effa  @ $03EFFA  (1588 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_AnimTblA_03effa, "ax", @progbits
        .global Pow_AnimTblA_03effa
Pow_AnimTblA_03effa:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +020  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f01c
Pow_AnimTblA_03effa__L03f01c:
.L03f01c:
        .dc.w   0x0002                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +024  (dato / opcode no decodificado)
        .dc.w   0xeffa                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +036  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +042  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +044  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +046  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +048  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +060  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +066  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +068  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +070  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f070
Pow_AnimTblA_03effa__L03f070:
.L03f070:
        .dc.w   0x0900                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +078  (dato / opcode no decodificado)
        .dc.w   0xfa84                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +080  (dato / opcode no decodificado)
        .dc.w   0xcc78                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xcc88                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +094  (dato / opcode no decodificado)
        .dc.w   0xcc98                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xccb0                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0xccc8                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0xccb0                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0xcc98                        | +0be  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0xcc88                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0xf076                        | +0d0  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f0cc
Pow_AnimTblA_03effa__L03f0cc:
.L03f0cc:
        .dc.w   0x0400                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x1048                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0da  (dato / opcode no decodificado)
        .dc.w   0xcce0                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0xccf8                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0xcd10                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0xcd30                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +0fe  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f0fa
Pow_AnimTblA_03effa__L03f0fa:
.L03f0fa:
        .dc.w   0x0700                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +102  (dato / opcode no decodificado)
        .dc.w   0xceac                        | +104  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +10c  (dato / opcode no decodificado)
        .dc.w   0xcd50                        | +10e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +114  (dato / opcode no decodificado)
        .dc.w   0xceba                        | +116  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +11e  (dato / opcode no decodificado)
        .dc.w   0xcd68                        | +120  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +126  (dato / opcode no decodificado)
        .dc.w   0xcec8                        | +128  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +130  (dato / opcode no decodificado)
        .dc.w   0xcd90                        | +132  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +138  (dato / opcode no decodificado)
        .dc.w   0xceba                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +140  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +142  (dato / opcode no decodificado)
        .dc.w   0xcda8                        | +144  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +14a  (dato / opcode no decodificado)
        .dc.w   0xceac                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +154  (dato / opcode no decodificado)
        .dc.w   0xcdc0                        | +156  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +158  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +15c  (dato / opcode no decodificado)
        .dc.w   0xceba                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +160  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +164  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +166  (dato / opcode no decodificado)
        .dc.w   0xcda8                        | +168  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +16e  (dato / opcode no decodificado)
        .dc.w   0xcec8                        | +170  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +174  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +176  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +178  (dato / opcode no decodificado)
        .dc.w   0xcd90                        | +17a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +180  (dato / opcode no decodificado)
        .dc.w   0xceba                        | +182  (dato / opcode no decodificado)
        .dc.w   0x005c                        | +184  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +186  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +188  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +18a  (dato / opcode no decodificado)
        .dc.w   0xcd68                        | +18c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +190  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +192  (dato / opcode no decodificado)
        .dc.w   0xf0fa                        | +194  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f190
Pow_AnimTblA_03effa__L03f190:
.L03f190:
        .dc.w   0x0400                        | +196  (dato / opcode no decodificado)
        .dc.w   0x1048                        | +198  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f194
Pow_AnimTblA_03effa__L03f194:
.L03f194:
        .dc.w   0x0001                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +19c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +19e  (dato / opcode no decodificado)
        .dc.w   0xce32                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0xce4e                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0xce32                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0xce62                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0xce7c                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0xce94                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0xce7c                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +200  (dato / opcode no decodificado)
        .dc.w   0xce62                        | +202  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +204  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +206  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +208  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +20c  (dato / opcode no decodificado)
        .dc.w   0xf194                        | +20e  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f20a
Pow_AnimTblA_03effa__L03f20a:
.L03f20a:
        .dc.w   0x0001                        | +210  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +212  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +214  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +216  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +218  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +21a  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f216
Pow_AnimTblA_03effa__L03f216:
.L03f216:
        .dc.w   0x0002                        | +21c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +21e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +220  (dato / opcode no decodificado)
        .dc.w   0xcdd8                        | +222  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +224  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +226  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +228  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +22a  (dato / opcode no decodificado)
        .dc.w   0xcde8                        | +22c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +22e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +230  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +232  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +234  (dato / opcode no decodificado)
        .dc.w   0xce00                        | +236  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +238  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +23a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +23c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +23e  (dato / opcode no decodificado)
        .dc.w   0xce14                        | +240  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +242  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +244  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +246  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +248  (dato / opcode no decodificado)
        .dc.w   0xce28                        | +24a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +24c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +24e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +250  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +252  (dato / opcode no decodificado)
        .dc.w   0xced6                        | +254  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +256  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +258  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +25a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +25c  (dato / opcode no decodificado)
        .dc.w   0xcee0                        | +25e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +260  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +262  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f25e
Pow_AnimTblA_03effa__L03f25e:
.L03f25e:
        .dc.w   0x0008                        | +264  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +266  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +268  (dato / opcode no decodificado)
        .dc.w   0xcec8                        | +26a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +26c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +26e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +270  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +272  (dato / opcode no decodificado)
        .dc.w   0xceea                        | +274  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +276  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +278  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +27a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +27c  (dato / opcode no decodificado)
        .dc.w   0xcef8                        | +27e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +280  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +282  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +284  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +286  (dato / opcode no decodificado)
        .dc.w   0xcf06                        | +288  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +28a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +28c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +28e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +290  (dato / opcode no decodificado)
        .dc.w   0xcf14                        | +292  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +294  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +296  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f292
Pow_AnimTblA_03effa__L03f292:
.L03f292:
        .dc.w   0x0900                        | +298  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +29a  (dato / opcode no decodificado)
        .dc.w   0xfa30                        | +29c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +29e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2a0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2a2  (dato / opcode no decodificado)
        .dc.w   0xcf22                        | +2a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2a6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2a8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2aa  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2ac  (dato / opcode no decodificado)
        .dc.w   0xcf3e                        | +2ae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2b0  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +2b2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2b4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2b6  (dato / opcode no decodificado)
        .dc.w   0xcf5a                        | +2b8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2ba  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +2bc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2be  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2c0  (dato / opcode no decodificado)
        .dc.w   0xcf3e                        | +2c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2c4  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +2c6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2c8  (dato / opcode no decodificado)
        .dc.w   0xf298                        | +2ca  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +2cc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2ce  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2d0  (dato / opcode no decodificado)
        .dc.w   0xcf22                        | +2d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2d4  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +2d6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2d8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2da  (dato / opcode no decodificado)
        .dc.w   0xcf3e                        | +2dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2de  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2e0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2e2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2e4  (dato / opcode no decodificado)
        .dc.w   0xcf5a                        | +2e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2e8  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2ea  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2ec  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2ee  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +2f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2f2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2f4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2f6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2f8  (dato / opcode no decodificado)
        .dc.w   0xcf96                        | +2fa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2fc  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2fe  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +300  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +302  (dato / opcode no decodificado)
        .dc.w   0xcfb4                        | +304  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +306  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +308  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +30a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +30c  (dato / opcode no decodificado)
        .dc.w   0xcfd2                        | +30e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +310  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +312  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +314  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +316  (dato / opcode no decodificado)
        .dc.w   0xcfee                        | +318  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +31a  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +31c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +31e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +320  (dato / opcode no decodificado)
        .dc.w   0xcfd2                        | +322  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +324  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +326  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +328  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +32a  (dato / opcode no decodificado)
        .dc.w   0xcfee                        | +32c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +32e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +330  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +332  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +334  (dato / opcode no decodificado)
        .dc.w   0xcfd2                        | +336  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +338  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +33a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +33c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +33e  (dato / opcode no decodificado)
        .dc.w   0xcfee                        | +340  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +342  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +344  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +346  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +348  (dato / opcode no decodificado)
        .dc.w   0xcfd2                        | +34a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +34c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +34e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +350  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +352  (dato / opcode no decodificado)
        .dc.w   0xcfee                        | +354  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +356  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +358  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +35a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +35c  (dato / opcode no decodificado)
        .dc.w   0xcfd2                        | +35e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +360  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +362  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +364  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +366  (dato / opcode no decodificado)
        .dc.w   0xcfee                        | +368  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +36a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +36c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +36e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +370  (dato / opcode no decodificado)
        .dc.w   0xcfd2                        | +372  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +374  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +376  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +378  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +37a  (dato / opcode no decodificado)
        .dc.w   0xcfee                        | +37c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +37e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +380  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +382  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +384  (dato / opcode no decodificado)
        .dc.w   0xcfb4                        | +386  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +388  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +38a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +38c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +38e  (dato / opcode no decodificado)
        .dc.w   0xcf96                        | +390  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +392  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +394  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +396  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +398  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +39a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +39c  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +39e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +3a0  (dato / opcode no decodificado)
        .dc.w   0xf298                        | +3a2  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f39e
Pow_AnimTblA_03effa__L03f39e:
.L03f39e:
        .dc.w   0x0400                        | +3a4  (dato / opcode no decodificado)
        .dc.w   0x1048                        | +3a6  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3a8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3aa  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3ac  (dato / opcode no decodificado)
        .dc.w   0xd00c                        | +3ae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3b0  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3b2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3b4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3b6  (dato / opcode no decodificado)
        .dc.w   0xd02a                        | +3b8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3ba  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3bc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3be  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3c0  (dato / opcode no decodificado)
        .dc.w   0xd044                        | +3c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3c4  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +3c6  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f3c2
Pow_AnimTblA_03effa__L03f3c2:
.L03f3c2:
        .dc.w   0x0005                        | +3c8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3ca  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3cc  (dato / opcode no decodificado)
        .dc.w   0xd05e                        | +3ce  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3d0  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3d2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3d4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3d6  (dato / opcode no decodificado)
        .dc.w   0xd074                        | +3d8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3da  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3dc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3de  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3e0  (dato / opcode no decodificado)
        .dc.w   0xd088                        | +3e2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3e4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3e6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3e8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3ea  (dato / opcode no decodificado)
        .dc.w   0xd0a4                        | +3ec  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3ee  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3f0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3f2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3f4  (dato / opcode no decodificado)
        .dc.w   0xd0be                        | +3f6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3f8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3fa  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3fc  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +3fe  (dato / opcode no decodificado)
        .dc.w   0xd0d8                        | +400  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +402  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +404  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +406  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +408  (dato / opcode no decodificado)
        .dc.w   0xd0f2                        | +40a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +40c  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +40e  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f40a
Pow_AnimTblA_03effa__L03f40a:
.L03f40a:
        .dc.w   0x0003                        | +410  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +412  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +414  (dato / opcode no decodificado)
        .dc.w   0xca32                        | +416  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +418  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +41a  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +41c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +41e  (dato / opcode no decodificado)
        .dc.w   0xca54                        | +420  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +422  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +424  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +426  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +428  (dato / opcode no decodificado)
        .dc.w   0xca72                        | +42a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +42c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +42e  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +430  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +432  (dato / opcode no decodificado)
        .dc.w   0xca8a                        | +434  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +436  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +438  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +43a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +43c  (dato / opcode no decodificado)
        .dc.w   0xca9e                        | +43e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +440  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +442  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +444  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +446  (dato / opcode no decodificado)
        .dc.w   0xcab6                        | +448  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +44a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +44c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +44e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +450  (dato / opcode no decodificado)
        .dc.w   0xcace                        | +452  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +454  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +456  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +458  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +45a  (dato / opcode no decodificado)
        .dc.w   0xcae2                        | +45c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +45e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +460  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +462  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +464  (dato / opcode no decodificado)
        .dc.w   0xcaf6                        | +466  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +468  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +46a  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +46c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +46e  (dato / opcode no decodificado)
        .dc.w   0xcb0e                        | +470  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +472  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +474  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +476  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +478  (dato / opcode no decodificado)
        .dc.w   0xcb26                        | +47a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +47c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +47e  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +480  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +482  (dato / opcode no decodificado)
        .dc.w   0xcb3a                        | +484  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +486  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +488  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +48a  (dato / opcode no decodificado)
        .dc.w   0xf40a                        | +48c  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f488
Pow_AnimTblA_03effa__L03f488:
.L03f488:
        .dc.w   0x0001                        | +48e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +490  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +492  (dato / opcode no decodificado)
        .dc.w   0xcb5a                        | +494  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +496  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +498  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +49a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +49c  (dato / opcode no decodificado)
        .dc.w   0xcb5a                        | +49e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4a0  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +4a2  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f49e
Pow_AnimTblA_03effa__L03f49e:
.L03f49e:
        .dc.w   0x0002                        | +4a4  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +4a6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4a8  (dato / opcode no decodificado)
        .dc.w   0xcb72                        | +4aa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4ac  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +4ae  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +4b0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4b2  (dato / opcode no decodificado)
        .dc.w   0xcb8a                        | +4b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4b6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +4b8  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +4ba  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4bc  (dato / opcode no decodificado)
        .dc.w   0xcb9e                        | +4be  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4c0  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +4c2  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +4c4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4c6  (dato / opcode no decodificado)
        .dc.w   0xcbb2                        | +4c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4ca  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +4cc  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +4ce  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4d0  (dato / opcode no decodificado)
        .dc.w   0xcbd0                        | +4d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4d4  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +4d6  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +4d8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4da  (dato / opcode no decodificado)
        .dc.w   0xcbe8                        | +4dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4de  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4e0  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +4e2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4e4  (dato / opcode no decodificado)
        .dc.w   0xcc04                        | +4e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4e8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4ea  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +4ec  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4ee  (dato / opcode no decodificado)
        .dc.w   0xcc22                        | +4f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4f2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4f4  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +4f6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +4f8  (dato / opcode no decodificado)
        .dc.w   0xcc3c                        | +4fa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4fc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4fe  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +500  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +502  (dato / opcode no decodificado)
        .dc.w   0xcc22                        | +504  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +506  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +508  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +50a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +50c  (dato / opcode no decodificado)
        .dc.w   0xcc5a                        | +50e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +510  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +512  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +514  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +516  (dato / opcode no decodificado)
        .dc.w   0xcc22                        | +518  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +51a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +51c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +51e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +520  (dato / opcode no decodificado)
        .dc.w   0xcc3c                        | +522  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +524  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +526  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +528  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +52a  (dato / opcode no decodificado)
        .dc.w   0xcc22                        | +52c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +52e  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +530  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +532  (dato / opcode no decodificado)
        .dc.w   0xefec                        | +534  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +536  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +538  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +53a  (dato / opcode no decodificado)
        .dc.w   0xcc5a                        | +53c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +53e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +540  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +542  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +544  (dato / opcode no decodificado)
        .dc.w   0xcc22                        | +546  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +548  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +54a  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +54c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +54e  (dato / opcode no decodificado)
        .dc.w   0xcc04                        | +550  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +552  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +554  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +556  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +558  (dato / opcode no decodificado)
        .dc.w   0xcbe8                        | +55a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +55c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +55e  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +560  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +562  (dato / opcode no decodificado)
        .dc.w   0xcbb2                        | +564  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +566  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +568  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +56a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +56c  (dato / opcode no decodificado)
        .dc.w   0xcbd0                        | +56e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +570  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +572  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +574  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +576  (dato / opcode no decodificado)
        .dc.w   0xcbb2                        | +578  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +57a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +57c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +57e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +580  (dato / opcode no decodificado)
        .dc.w   0xcb9e                        | +582  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +584  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +586  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +588  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +58a  (dato / opcode no decodificado)
        .dc.w   0xcb8a                        | +58c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +58e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +590  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +592  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +594  (dato / opcode no decodificado)
        .dc.w   0xcb72                        | +596  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +598  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +59a  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f596
Pow_AnimTblA_03effa__L03f596:
.L03f596:
        .dc.w   0x0002                        | +59c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +59e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5a0  (dato / opcode no decodificado)
        .dc.w   0xd26c                        | +5a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5a4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +5a6  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +5a8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5aa  (dato / opcode no decodificado)
        .dc.w   0xd284                        | +5ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5ae  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +5b0  (dato / opcode no decodificado)
        .global Pow_AnimTblA_03effa__L03f5ac
Pow_AnimTblA_03effa__L03f5ac:
.L03f5ac:
        .dc.w   0x0001                        | +5b2  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +5b4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5b6  (dato / opcode no decodificado)
        .dc.w   0xd29c                        | +5b8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5ba  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5bc  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +5be  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5c0  (dato / opcode no decodificado)
        .dc.w   0xd2b4                        | +5c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5c4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5c6  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +5c8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5ca  (dato / opcode no decodificado)
        .dc.w   0xd2cc                        | +5cc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5ce  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5d0  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +5d2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5d4  (dato / opcode no decodificado)
        .dc.w   0xd2e0                        | +5d6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5d8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5da  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +5dc  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5de  (dato / opcode no decodificado)
        .dc.w   0xd2f4                        | +5e0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5e2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5e4  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +5e6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5e8  (dato / opcode no decodificado)
        .dc.w   0xd308                        | +5ea  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5ec  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5ee  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +5f0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5f2  (dato / opcode no decodificado)
        .dc.w   0xd31c                        | +5f4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5f6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5f8  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +5fa  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +5fc  (dato / opcode no decodificado)
        .dc.w   0xd330                        | +5fe  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +600  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +602  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +604  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +606  (dato / opcode no decodificado)
        .dc.w   0xd344                        | +608  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +60a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +60c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +60e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +610  (dato / opcode no decodificado)
        .dc.w   0xd358                        | +612  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +614  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +616  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +618  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +61a  (dato / opcode no decodificado)
        .dc.w   0xd36c                        | +61c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +61e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +620  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +622  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +624  (dato / opcode no decodificado)
        .dc.w   0xd380                        | +626  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +628  (dato / opcode no decodificado)
        move.b  d0,d3                           | +62a
        addi.w  #0x24,0x24(a6)                  | +62c
        rts                                     | +632

| ----------------------------------------------------------------------------
|  Pow_SubX36_03f62e  @ $03F62E  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_SubX36_03f62e, "ax", @progbits
        .global Pow_SubX36_03f62e
Pow_SubX36_03f62e:
        subi.w  #0x24,0x24(a6)                  | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Pow_AnimTblB_03f636  @ $03F636  (976 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_AnimTblB_03f636, "ax", @progbits
        .global Pow_AnimTblB_03f636
Pow_AnimTblB_03f636:
        .dc.w   0x0800                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0xf626                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xd398                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffdc                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0xd3b0                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xffdd                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +026  (dato / opcode no decodificado)
        .dc.w   0xd3c8                        | +028  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xffde                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +034  (dato / opcode no decodificado)
        .dc.w   0xd3e0                        | +036  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffdf                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +042  (dato / opcode no decodificado)
        .dc.w   0xd3f8                        | +044  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +050  (dato / opcode no decodificado)
        .dc.w   0xd410                        | +052  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffe1                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +05e  (dato / opcode no decodificado)
        .dc.w   0xd428                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +064  (dato / opcode no decodificado)
        .dc.w   0xffe6                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +06c  (dato / opcode no decodificado)
        .dc.w   0xd440                        | +06e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xd458                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +080  (dato / opcode no decodificado)
        .dc.w   0xffee                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +088  (dato / opcode no decodificado)
        .dc.w   0xd476                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +096  (dato / opcode no decodificado)
        .dc.w   0xd48e                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09c  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0xd4a6                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0xd4ba                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0xd4d2                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0xd4ea                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0xd502                        | +0de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +0e6  (dato / opcode no decodificado)
        .global Pow_AnimTblB_03f636__L03f71e
Pow_AnimTblB_03f636__L03f71e:
.L03f71e:
        .dc.w   0x0001                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0xcb72                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +0f2  (dato / opcode no decodificado)
        .global Pow_AnimTblB_03f636__L03f72a
Pow_AnimTblB_03f636__L03f72a:
.L03f72a:
        .dc.w   0x0001                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0xd502                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +106  (dato / opcode no decodificado)
        .dc.w   0xd4ea                        | +108  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10c  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +114  (dato / opcode no decodificado)
        .dc.w   0xd4d2                        | +116  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +11a  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +120  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +122  (dato / opcode no decodificado)
        .dc.w   0xd4ba                        | +124  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +128  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +130  (dato / opcode no decodificado)
        .dc.w   0xd4a6                        | +132  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +136  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +13e  (dato / opcode no decodificado)
        .dc.w   0xd48e                        | +140  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +142  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +144  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +14c  (dato / opcode no decodificado)
        .dc.w   0xd476                        | +14e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +152  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +154  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +156  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +158  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +15a  (dato / opcode no decodificado)
        .dc.w   0xd458                        | +15c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +160  (dato / opcode no decodificado)
        .dc.w   0xffee                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +164  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +166  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +168  (dato / opcode no decodificado)
        .dc.w   0xd440                        | +16a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +16e  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +170  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +174  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +176  (dato / opcode no decodificado)
        .dc.w   0xd428                        | +178  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +17c  (dato / opcode no decodificado)
        .dc.w   0xffe6                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +180  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +184  (dato / opcode no decodificado)
        .dc.w   0xd410                        | +186  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +188  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +18a  (dato / opcode no decodificado)
        .dc.w   0xffe1                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +190  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +192  (dato / opcode no decodificado)
        .dc.w   0xd3f8                        | +194  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +196  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +198  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +19c  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +19e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0xd3e0                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0xffdf                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0xd3c8                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0xffde                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0x0201                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0xd3b0                        | +1be  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0xffdd                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0xf62e                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0xd398                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +1d6  (dato / opcode no decodificado)
        .global Pow_AnimTblB_03f636__L03f80e
Pow_AnimTblB_03f636__L03f80e:
.L03f80e:
        .dc.w   0x0001                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0xd380                        | +1de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0xd36c                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0xd358                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0xd344                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +200  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +202  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +204  (dato / opcode no decodificado)
        .dc.w   0xd330                        | +206  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +208  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +20c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +20e  (dato / opcode no decodificado)
        .dc.w   0xd31c                        | +210  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +212  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +214  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +216  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +218  (dato / opcode no decodificado)
        .dc.w   0xd308                        | +21a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +21c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +21e  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +220  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +222  (dato / opcode no decodificado)
        .dc.w   0xd2f4                        | +224  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +226  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +228  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +22a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +22c  (dato / opcode no decodificado)
        .dc.w   0xd2e0                        | +22e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +230  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +232  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +234  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +236  (dato / opcode no decodificado)
        .dc.w   0xd2cc                        | +238  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +23a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +23c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +23e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +240  (dato / opcode no decodificado)
        .dc.w   0xd2b4                        | +242  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +244  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +246  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +248  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +24a  (dato / opcode no decodificado)
        .dc.w   0xd29c                        | +24c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +24e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +250  (dato / opcode no decodificado)
        .global Pow_AnimTblB_03f636__L03f888
Pow_AnimTblB_03f636__L03f888:
.L03f888:
        .dc.w   0x0002                        | +252  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +254  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +256  (dato / opcode no decodificado)
        .dc.w   0xd284                        | +258  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +25a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +25c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +25e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +260  (dato / opcode no decodificado)
        .dc.w   0xd26c                        | +262  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +264  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +266  (dato / opcode no decodificado)
        .global Pow_AnimTblB_03f636__L03f89e
Pow_AnimTblB_03f636__L03f89e:
.L03f89e:
        .dc.w   0x0002                        | +268  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +26a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +26c  (dato / opcode no decodificado)
        .dc.w   0xd512                        | +26e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +270  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +272  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +274  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +276  (dato / opcode no decodificado)
        .dc.w   0xd53a                        | +278  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +27a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +27c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +27e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +280  (dato / opcode no decodificado)
        .dc.w   0xd556                        | +282  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +284  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +286  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +288  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +28a  (dato / opcode no decodificado)
        .dc.w   0xd56a                        | +28c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +28e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +290  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +292  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +294  (dato / opcode no decodificado)
        .dc.w   0xd58e                        | +296  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +298  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +29a  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +29c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +29e  (dato / opcode no decodificado)
        .dc.w   0xd5a8                        | +2a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2a2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2a4  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +2a6  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2a8  (dato / opcode no decodificado)
        .dc.w   0xd5c4                        | +2aa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2ac  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2ae  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +2b0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2b2  (dato / opcode no decodificado)
        .dc.w   0xd5e0                        | +2b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2b6  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +2b8  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2ba  (dato / opcode no decodificado)
        .dc.w   0xf89e                        | +2bc  (dato / opcode no decodificado)
        .global Pow_AnimTblB_03f636__L03f8f4
Pow_AnimTblB_03f636__L03f8f4:
.L03f8f4:
        .dc.w   0x0002                        | +2be  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +2c0  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2c2  (dato / opcode no decodificado)
        .dc.w   0xda26                        | +2c4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2c6  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2c8  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +2ca  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2cc  (dato / opcode no decodificado)
        .dc.w   0xda3a                        | +2ce  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2d0  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2d2  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +2d4  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2d6  (dato / opcode no decodificado)
        .dc.w   0xda4e                        | +2d8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2da  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2dc  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +2de  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2e0  (dato / opcode no decodificado)
        .dc.w   0xda62                        | +2e2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2e4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +2e6  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +2e8  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2ea  (dato / opcode no decodificado)
        .dc.w   0xda76                        | +2ec  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2ee  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2f0  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +2f2  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2f4  (dato / opcode no decodificado)
        .dc.w   0xda8e                        | +2f6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2f8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2fa  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +2fc  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +2fe  (dato / opcode no decodificado)
        .dc.w   0xdaa6                        | +300  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +302  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +304  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +306  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +308  (dato / opcode no decodificado)
        .dc.w   0xdabe                        | +30a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +30c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +30e  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +310  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +312  (dato / opcode no decodificado)
        .dc.w   0xdad6                        | +314  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +316  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +318  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +31a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +31c  (dato / opcode no decodificado)
        .dc.w   0xdaee                        | +31e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +320  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +322  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +324  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +326  (dato / opcode no decodificado)
        .dc.w   0xdb0c                        | +328  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +32a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +32c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +32e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +330  (dato / opcode no decodificado)
        .dc.w   0xdb20                        | +332  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +334  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +336  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +338  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +33a  (dato / opcode no decodificado)
        .dc.w   0xdb0c                        | +33c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +33e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +340  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +342  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +344  (dato / opcode no decodificado)
        .dc.w   0xdb20                        | +346  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +348  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +34a  (dato / opcode no decodificado)
        .dc.w   0x1056                        | +34c  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +34e  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +350  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +352  (dato / opcode no decodificado)
        .dc.w   0xdb0c                        | +354  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +356  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +358  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +35a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +35c  (dato / opcode no decodificado)
        .dc.w   0xdb34                        | +35e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +360  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +362  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +364  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +366  (dato / opcode no decodificado)
        .dc.w   0xdb48                        | +368  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +36a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +36c  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +36e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +370  (dato / opcode no decodificado)
        .dc.w   0xdb60                        | +372  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +374  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +376  (dato / opcode no decodificado)
        .dc.w   0x0209                        | +378  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +37a  (dato / opcode no decodificado)
        .dc.w   0xcb72                        | +37c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +37e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +380  (dato / opcode no decodificado)
        .global Pow_AnimTblB_03f636__L03f9b8
Pow_AnimTblB_03f636__L03f9b8:
.L03f9b8:
        tst.b   0x9a(a6)                        | +382
        beq.w   .L03f9c6                        | +386
        bset    #0x0,0x3a(a6)                   | +38a
.L03f9c6:
        move.w  #0x182,d1                       | +390
        jsr     0x236e.l                        | +394
        move.w  #0x8000,d0                      | +39a
        jsr     0x28134.l                       | +39e
        andi.w  #0xffe3,0x38(a6)                | +3a4
        ori.w   #0x18,0x38(a6)                  | +3aa
        clr.w   0x28(a6)                        | +3b0
        clr.w   0x2a(a6)                        | +3b4
        clr.w   0x2c(a6)                        | +3b8
        clr.w   0x2e(a6)                        | +3bc
        jsr     0x27f60.l                       | +3c0
        scc.b   0x70(a6)                        | +3c6
        lea     0x776e2.l,a1                    | +3ca

| ----------------------------------------------------------------------------
|  Pow_AnimTblC_03fa0e  @ $03FA0E  (604 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_AnimTblC_03fa0e, "ax", @progbits
        .global Pow_AnimTblC_03fa0e
Pow_AnimTblC_03fa0e:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
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
        .dc.w   0x0001                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfa0e                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +036  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +042  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +044  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +046  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +066  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +068  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +070  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +078  (dato / opcode no decodificado)
        .dc.w   0xfa0e                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +080  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +084  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +096  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0xfa0e                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +102  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +104  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +110  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +114  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +116  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +118  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +11a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +120  (dato / opcode no decodificado)
        .dc.w   0xfa0e                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +126  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +132  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +134  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +140  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +142  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +14c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +14e  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +154  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +156  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +158  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +160  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +162  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +164  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +166  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +168  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +16c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +16e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +170  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +174  (dato / opcode no decodificado)
        .dc.w   0xfa0e                        | +176  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +178  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +17a  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +180  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +184  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +186  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +188  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +18a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +190  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +192  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +194  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +196  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +198  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +19c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +19e  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1be  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0xfa0e                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0x00c0                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0x00c0                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +200  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +202  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +204  (dato / opcode no decodificado)
        .dc.w   0x00c0                        | +206  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +208  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +20c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +20e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +210  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +212  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +214  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +216  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +218  (dato / opcode no decodificado)
        .global Pow_AnimTblC_03fa0e__L03fc28
Pow_AnimTblC_03fa0e__L03fc28:
.L03fc28:
        .dc.w   0x0003                        | +21a  (dato / opcode no decodificado)
        .dc.w   0xfad8                        | +21c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +21e  (dato / opcode no decodificado)
        .dc.w   0xfb2c                        | +220  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +222  (dato / opcode no decodificado)
        .dc.w   0xfb80                        | +224  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +226  (dato / opcode no decodificado)
        .dc.w   0xfbd4                        | +228  (dato / opcode no decodificado)
        bsr.w   Pow_AnimTblB_03f636__L03f9b8    | +22a
        lea     Pow_AnimTblA_03effa__L03f292(pc),a0 | +22e
        jsr     0x28cd4.l                       | +232
        lea     .L03fc4c(pc),a1                 | +238
        move.l  a1,(a6)                         | +23c
.L03fc4c:
        jsr     Pow_Physics_03fece(pc)          | +23e
        jsr     0x28d70.l                       | +242
        jsr     0x2870a.l                       | +248
        bcc.w   .L03fc66                        | +24e
        lea     Pow_Freed_03fc6a(pc),a1         | +252
        move.l  a1,(a6)                         | +256
.L03fc66:
        jmp     Pow_OffworldFree_03efce(pc)     | +258

| ----------------------------------------------------------------------------
|  Pow_Freed_03fc6a  @ $03FC6A  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Freed_03fc6a, "ax", @progbits
        .global Pow_Freed_03fc6a
Pow_Freed_03fc6a:
        lea     Pow_AnimTblA_03effa__L03f39e(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        lea     .L03fc7a(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L03fc7a:
        jsr     0x2783a.l                       | +010
        jsr     0x28d70.l                       | +016
        bcc.w   .L03fca0                        | +01c
        lea     Pow_Despawn_03ff14(pc),a1       | +020
        move.l  a1,(a6)                         | +024
        lea     Pow_FreedCheer_03fca4(pc),a1    | +026
        jsr     0x6fe.l                         | +02a
        jsr     0x5dd02.l                       | +030
.L03fca0:
        jmp     Pow_OffworldFree_03efce(pc)     | +036

| ----------------------------------------------------------------------------
|  Pow_FreedCheer_03fca4  @ $03FCA4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreedCheer_03fca4, "ax", @progbits
        .global Pow_FreedCheer_03fca4
Pow_FreedCheer_03fca4:
        move.w  #0x182,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     Pow_AnimTblA_03effa__L03f3c2(pc),a0 | +00a
        jsr     0x28cd4.l                       | +00e

| ----------------------------------------------------------------------------
|  Pow_FreedWait_03fcc0  @ $03FCC0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreedWait_03fcc0, "ax", @progbits
        .global Pow_FreedWait_03fcc0
Pow_FreedWait_03fcc0:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L03fcd6                        | +00c
        jmp     0x518.l                         | +010
.L03fcd6:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Pow_Tied_03fcd8  @ $03FCD8  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Tied_03fcd8, "ax", @progbits
        .global Pow_Tied_03fcd8
Pow_Tied_03fcd8:
        bsr.w   Pow_AnimTblB_03f636__L03f9b8    | +000
        lea     Pow_AnimTblA_03effa__L03f0fa(pc),a0 | +004
        jsr     0x28cd4.l                       | +008
        moveq   #0,d7                           | +00e
        move.b  0x99(a6),d7                     | +010
        cmpi.b  #0x4,d7                         | +014
        bcs.w   .L03fcf8                        | +018
        move.w  #0x3,d7                         | +01c
.L03fcf8:
        add.w   d7,d7                           | +020
        add.w   d7,d7                           | +022
        lea     Pow_AnimTblC_03fa0e__L03fc28(pc),a0 | +024
        adda.w  d7,a0                           | +028
        move.l  (a0),0x48(a6)                   | +02a
.L03fd06:
        lea     Pow_TiedCheer_03fd98(pc),a1     | +02e
        jsr     0x4ae.l                         | +032
        move.b  0x99(a6),0x5c(a0)               | +038
        subq.b  #0x1,0x99(a6)                   | +03e
        bcc.b   .L03fd06                        | +042
        lea     .L03fd22(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L03fd22:
        jsr     0x27cee.l                       | +04a
        jsr     0x28d70.l                       | +050
        jsr     0x2870a.l                       | +056
        bcc.w   .L03fd3e                        | +05c
        lea     Pow_TiedStruggle_03fd42(pc),a1  | +060
        move.l  a1,(a6)                         | +064
.L03fd3e:
        jmp     Pow_OffworldFree_03efce(pc)     | +066

| ----------------------------------------------------------------------------
|  Pow_TiedStruggle_03fd42  @ $03FD42  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_TiedStruggle_03fd42, "ax", @progbits
        .global Pow_TiedStruggle_03fd42
Pow_TiedStruggle_03fd42:
        lea     Pow_AnimTblA_03effa__L03f190(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        lea     .L03fd52(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L03fd52:
        jsr     Pow_Physics_03fece(pc)          | +010
        tst.b   0x70(a6)                        | +014
        beq.w   .L03fd64                        | +018
        lea     Pow_TiedBreak_03fd6e(pc),a1     | +01c
        move.l  a1,(a6)                         | +020
.L03fd64:
        jsr     0x28d70.l                       | +022
        jmp     Pow_OffworldFree_03efce(pc)     | +028

| ----------------------------------------------------------------------------
|  Pow_TiedBreak_03fd6e  @ $03FD6E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_TiedBreak_03fd6e, "ax", @progbits
        .global Pow_TiedBreak_03fd6e
Pow_TiedBreak_03fd6e:
        lea     Pow_AnimTblA_03effa__L03f20a(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        lea     .L03fd7e(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L03fd7e:
        jsr     0x2783a.l                       | +010
        jsr     0x28d70.l                       | +016
        bcc.w   .L03fd94                        | +01c
        lea     Pow_Despawn_03ff14(pc),a1       | +020
        move.l  a1,(a6)                         | +024
.L03fd94:
        jmp     Pow_OffworldFree_03efce(pc)     | +026

| ----------------------------------------------------------------------------
|  Pow_TiedCheer_03fd98  @ $03FD98  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_TiedCheer_03fd98, "ax", @progbits
        .global Pow_TiedCheer_03fd98
Pow_TiedCheer_03fd98:
        move.w  #0x182,d1                       | +000
        jsr     0x236e.l                        | +004
        moveq   #0,d0                           | +00a
        move.b  0x5c(a6),d0                     | +00c
        lsl.w   #0x4,d0                         | +010
        move.w  d0,d1                           | +012
        add.w   d1,d0                           | +014
        add.w   d1,d0                           | +016
        movea.l 0xc(a6),a0                      | +018
        move.w  0x32(a0),0x32(a6)               | +01c
        move.w  0x22(a0),0x22(a6)               | +022
        add.w   0x24(a0),d0                     | +028
        move.w  d0,0x24(a6)                     | +02c

| ----------------------------------------------------------------------------
|  Pow_WaitRider_03fdd0  @ $03FDD0  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_WaitRider_03fdd0, "ax", @progbits
        .global Pow_WaitRider_03fdd0
Pow_WaitRider_03fdd0:
        jsr     0x2783a.l                       | +000
        movea.l 0xc(a6),a0                      | +006
        cmpi.l  #0x52a,(a0)                     | +00a
        bne.w   .L03fdea                        | +010
        jmp     0x518.l                         | +014
.L03fdea:
        btst    #0x3,0x13(a0)                   | +01a
        beq.w   .L03fdfa                        | +020
        lea     Pow_RiderGreet_03fe1a(pc),a1    | +024
        move.l  a1,(a6)                         | +028
.L03fdfa:
        move.l  0x5c(a0),0x3c(a6)               | +02a
        jsr     0x5ca2a.l                       | +030
        move.w  0x22(a6),d0                     | +036
        addi.w  #0x10,d0                        | +03a
        bpl.w   .L03fe18                        | +03e
        jmp     0x518.l                         | +042
.L03fe18:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  Pow_RiderGreet_03fe1a  @ $03FE1A  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RiderGreet_03fe1a, "ax", @progbits
        .global Pow_RiderGreet_03fe1a
Pow_RiderGreet_03fe1a:
        tst.b   0x5c(a6)                        | +000
        bne.w   .L03fe30                        | +004
        lea     Pow_AnimTblA_03effa__L03f216(pc),a0 | +008
        jsr     0x28cd4.l                       | +00c
        bra.w   .L03fe3a                        | +012
.L03fe30:
        lea     Pow_AnimTblA_03effa__L03f25e(pc),a0 | +016
        jsr     0x28cd4.l                       | +01a
.L03fe3a:
        lea     .L03fe40(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L03fe40:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        bcc.w   .L03fe56                        | +032
        jmp     0x518.l                         | +036
.L03fe56:
        jmp     Pow_OffworldFree_03efce(pc)     | +03c

| ----------------------------------------------------------------------------
|  Pow_Entry_03fe5a  @ $03FE5A  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Entry_03fe5a, "ax", @progbits
        .global Pow_Entry_03fe5a
Pow_Entry_03fe5a:
        bsr.w   Pow_AnimTblB_03f636__L03f9b8    | +000

| ----------------------------------------------------------------------------
|  Pow_Idle_03fe66  @ $03FE66  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Idle_03fe66, "ax", @progbits
        .global Pow_Idle_03fe66
Pow_Idle_03fe66:
        lea     Pow_AnimTblA_03effa__L03f070(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        clr.w   0x28(a6)                        | +00a
        clr.w   0x2a(a6)                        | +00e
        lea     .L03fe7e(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L03fe7e:
        jsr     Pow_Physics_03fece(pc)          | +018
        jsr     0x28d70.l                       | +01c
        jsr     0x2870a.l                       | +022
        bcc.w   .L03fe98                        | +028
        lea     Pow_IdleWave_03fe9c(pc),a1      | +02c
        move.l  a1,(a6)                         | +030
.L03fe98:
        jmp     Pow_OffworldFree_03efce(pc)     | +032

| ----------------------------------------------------------------------------
|  Pow_IdleWave_03fe9c  @ $03FE9C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_IdleWave_03fe9c, "ax", @progbits
        .global Pow_IdleWave_03fe9c
Pow_IdleWave_03fe9c:
        lea     Pow_AnimTblA_03effa__L03f0cc(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        lea     .L03feac(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L03feac:
        jsr     0x28364.l                       | +010
        jsr     0x28d70.l                       | +016
        bcc.w   .L03fec2                        | +01c
        lea     Pow_Despawn_03ff14(pc),a1       | +020
        move.l  a1,(a6)                         | +024
.L03fec2:
        jmp     Pow_OffworldFree_03efce(pc)     | +026

| ----------------------------------------------------------------------------
|  Pow_EntryB_03fec6  @ $03FEC6  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_EntryB_03fec6, "ax", @progbits
        .global Pow_EntryB_03fec6
Pow_EntryB_03fec6:
        bsr.w   Pow_AnimTblB_03f636__L03f9b8    | +000
        bra.w   Pow_Despawn_03ff14              | +004

| ----------------------------------------------------------------------------
|  Pow_Physics_03fece  @ $03FECE  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Physics_03fece, "ax", @progbits
        .global Pow_Physics_03fece
Pow_Physics_03fece:
        jsr     0x27f08.l                       | +000
        bcc.w   .L03fee0                        | +006
        move.b  d3,0x70(a6)                     | +00a
        bra.w   .L03fef6                        | +00e
.L03fee0:
        cmp.b   0x70(a6),d0                     | +012
        bne.w   .L03fef0                        | +016
        move.b  d3,0x70(a6)                     | +01a
        bra.w   .L03fef6                        | +01e
.L03fef0:
        move.b  #0xff,0x70(a6)                  | +022
.L03fef6:
        tst.b   0x70(a6)                        | +028
        bne.w   JsrAbsThunk_03ff0c              | +02c
        jsr     0x28364.l                       | +030
        scs.b   0x70(a6)                        | +036
        bra.w   JsrAbsRts_03ff12                | +03a

| ----------------------------------------------------------------------------
|  Pow_Despawn_03ff14  @ $03FF14  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Despawn_03ff14, "ax", @progbits
        .global Pow_Despawn_03ff14
Pow_Despawn_03ff14:
        jsr     0x13600.l                       | +000
        move.w  #0x181,d1                       | +006
        jsr     0x236e.l                        | +00a

| ----------------------------------------------------------------------------
|  Pow_PickWalk_03ff24  @ $03FF24  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_PickWalk_03ff24, "ax", @progbits
        .global Pow_PickWalk_03ff24
Pow_PickWalk_03ff24:
        clr.b   0x70(a6)                        | +000
        lea     Pow_AnimTblA_03effa__L03f01c(pc),a0 | +004
        move.l  a0,0x48(a6)                     | +008
        bclr    #0x3,0x13(a6)                   | +00c
        bclr    #0x0,0x13(a6)                   | +012
        move.b  0x98(a6),d0                     | +018
        beq.w   .L03ff5c                        | +01c
        cmpi.b  #0x1,d0                         | +020
        bne.w   .L03ff56                        | +024
        bclr    #0x0,0x3a(a6)                   | +028
        bra.w   .L03ff5c                        | +02e
.L03ff56:
        bset    #0x0,0x3a(a6)                   | +032
.L03ff5c:
        move.w  #0xf,0x5c(a6)                   | +038

| ----------------------------------------------------------------------------
|  Pow_Walk_03ff62  @ $03FF62  (178 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Walk_03ff62, "ax", @progbits
        .global Pow_Walk_03ff62
Pow_Walk_03ff62:
        lea     Pow_AnimTblA_03effa__L03f40a(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        move.w  #0x180,d0                       | +00a
        btst    #0x0,0x3a(a6)                   | +00e
        bne.w   .L03ff7c                        | +014
        neg.w   d0                              | +018
.L03ff7c:
        move.w  d0,0x28(a6)                     | +01a
        lea     .L03ff86(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L03ff86:
        bsr.w   Pow_Physics_03fece              | +024
        tst.b   0x70(a6)                        | +028
        bne.w   .L03ff9c                        | +02c
        lea     Pow_WalkTurn_040014(pc),a1      | +030
        move.l  a1,(a6)                         | +034
        bra.w   .L03ffbc                        | +036
.L03ff9c:
        btst    #0x5,0x5a(a6)                   | +03a
        bne.w   .L03ffb6                        | +040
        tst.b   0x98(a6)                        | +044
        bne.w   .L03ffbc                        | +048
        subq.w  #0x1,0x5c(a6)                   | +04c
        bcc.w   .L03ffbc                        | +050
.L03ffb6:
        lea     Pow_WalkStop_040054(pc),a1      | +054
        move.l  a1,(a6)                         | +058
.L03ffbc:
        jsr     0x770cc.l                       | +05a
        tst.w   d0                              | +060
        bmi.w   .L03ffe4                        | +062
        move.b  d0,0x72(a6)                     | +066
        andi.b  #0x1,d1                         | +06a
        beq.w   .L03ffde                        | +06e
        lea     Pow_Jump_04008c(pc),a1          | +072
        move.l  a1,(a6)                         | +076
        bra.w   .L03ffe4                        | +078
.L03ffde:
        lea     Pow_Fall_0401a8(pc),a1          | +07c
        move.l  a1,(a6)                         | +080
.L03ffe4:
        jsr     0x28d70.l                       | +082
        .global Pow_Walk_03ff62__L03ffea
Pow_Walk_03ff62__L03ffea:
.L03ffea:
        lea     Pow_AddRescued_03ef9a(pc),a1    | +088
        bsr.w   Pow_CountIfPending_03ee48       | +08c
        bcc.w   .L040000                        | +090
        move.b  d0,0x71(a6)                     | +094
        lea     Pow_Rescued_040268(pc),a1       | +098
        move.l  a1,(a6)                         | +09c
.L040000:
        jsr     0x2870a.l                       | +09e
        bcc.w   .L040010                        | +0a4
        jmp     0x518.l                         | +0a8
.L040010:
        jmp     Pow_OffworldFree_03efce(pc)     | +0ae

| ----------------------------------------------------------------------------
|  Pow_WalkTurn_040014  @ $040014  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_WalkTurn_040014, "ax", @progbits
        .global Pow_WalkTurn_040014
Pow_WalkTurn_040014:
        lea     Pow_AnimTblA_03effa__L03f01c(pc),a0 | +000
        move.l  a0,0x48(a6)                     | +004
        lea     Pow_AnimTblA_03effa__L03f194(pc),a0 | +008
        jsr     0x28cd4.l                       | +00c
        clr.w   0x28(a6)                        | +012
        clr.w   0x2a(a6)                        | +016
        lea     .L040034(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L040034:
        jsr     0x28364.l                       | +020
        scs.b   0x70(a6)                        | +026
        tst.b   0x70(a6)                        | +02a
        beq.w   .L04004c                        | +02e
        lea     Pow_Walk_03ff62(pc),a1          | +032
        move.l  a1,(a6)                         | +036
.L04004c:
        jsr     0x28d70.l                       | +038
        bra.b   Pow_Walk_03ff62__L03ffea        | +03e

| ----------------------------------------------------------------------------
|  Pow_WalkStop_040054  @ $040054  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_WalkStop_040054, "ax", @progbits
        .global Pow_WalkStop_040054
Pow_WalkStop_040054:
        clr.w   0x28(a6)                        | +000
        lea     Pow_AnimTblA_03effa__L03f488(pc),a0 | +004
        jsr     0x28cd4.l                       | +008
        lea     .L040068(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L040068:
        bsr.w   Pow_Physics_03fece              | +014
        jsr     0x28d70.l                       | +018
        bcc.w   .L040088                        | +01e
        bchg    #0x0,0x3a(a6)                   | +022
        move.w  #0x1e,0x5c(a6)                  | +028
        lea     Pow_Walk_03ff62(pc),a1          | +02e
        move.l  a1,(a6)                         | +032
.L040088:
        bra.w   Pow_Walk_03ff62__L03ffea        | +034

| ----------------------------------------------------------------------------
|  Pow_Jump_04008c  @ $04008C  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Jump_04008c, "ax", @progbits
        .global Pow_Jump_04008c
Pow_Jump_04008c:
        lea     Pow_AnimTblA_03effa__L03f596(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        lea     .L04009c(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L04009c:
        clr.w   0x2e(a6)                        | +010
        clr.w   0x2a(a6)                        | +014
        move.b  0x72(a6),d0                     | +018
        jsr     0x77190.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L0400be                        | +028
        lea     Pow_JumpAir_0400c2(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L0400be:
        bra.w   Pow_Walk_03ff62__L03ffea        | +032

| ----------------------------------------------------------------------------
|  Pow_JumpAir_0400c2  @ $0400C2  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_JumpAir_0400c2, "ax", @progbits
        .global Pow_JumpAir_0400c2
Pow_JumpAir_0400c2:
        lea     Pow_AnimTblA_03effa__L03f5ac(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        clr.w   0x2e(a6)                        | +00a
        move.w  #0x154,0x2a(a6)                 | +00e
        lea     .L0400dc(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L0400dc:
        move.b  0x72(a6),d0                     | +01a
        jsr     0x77190.l                       | +01e
        tst.w   d0                              | +024
        bpl.w   .L0400f6                        | +026
        lea     Pow_PickWalk_03ff24(pc),a1      | +02a
        move.l  a1,(a6)                         | +02e
        bra.w   .L040108                        | +030
.L0400f6:
        subi.w  #0x24,d0                        | +034
        sub.w   0x24(a6),d0                     | +038
        bcc.w   .L040108                        | +03c
        lea     Pow_Crouch_040120(pc),a1        | +040
        move.l  a1,(a6)                         | +044
.L040108:
        jsr     0x28d70.l                       | +046
        bcc.w   .L04011c                        | +04c
        lea     Pow_AnimTblA_03effa__L03f5ac(pc),a0 | +050
        jsr     0x28cd4.l                       | +054
.L04011c:
        bra.w   Pow_Walk_03ff62__L03ffea        | +05a

| ----------------------------------------------------------------------------
|  Pow_Crouch_040120  @ $040120  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Crouch_040120, "ax", @progbits
        .global Pow_Crouch_040120
Pow_Crouch_040120:
        lea     Pow_AnimTblB_03f636(pc),a0      | +000
        jsr     0x28cd4.l                       | +004
        lea     .L040130(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L040130:
        jsr     0x2783a.l                       | +010
        jsr     0x28d70.l                       | +016
        bcc.w   .L040146                        | +01c
        lea     Pow_CrouchHop_04014a(pc),a1     | +020
        move.l  a1,(a6)                         | +024
.L040146:
        bra.w   Pow_Walk_03ff62__L03ffea        | +026

| ----------------------------------------------------------------------------
|  Pow_CrouchHop_04014a  @ $04014A  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_CrouchHop_04014a, "ax", @progbits
        .global Pow_CrouchHop_04014a
Pow_CrouchHop_04014a:
        lea     Pow_AnimTblB_03f636__L03f71e(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        move.w  #0xfccd,d0                      | +00a
        jsr     0x5dca4.l                       | +00e
        move.w  d0,0x28(a6)                     | +014
        move.w  #0x4c9,0x2a(a6)                 | +018
        move.w  #0xff0b,0x2e(a6)                | +01e
        move.w  #0x0,0x2c(a6)                   | +024
        lea     .L04017a(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L04017a:
        move.w  0x28(a6),d0                     | +030
        beq.w   .L040190                        | +034
        asr.w   #0x4,d0                         | +038
        seq.b   d1                              | +03a
        ext.w   d1                              | +03c
        eor.w   d1,d0                           | +03e
        sub.w   d1,d0                           | +040
        sub.w   d0,0x28(a6)                     | +042
.L040190:
        bsr.w   Pow_Physics_03fece              | +046
        jsr     0x28d70.l                       | +04a
        bcc.w   .L0401a4                        | +050
        lea     Pow_PickWalk_03ff24(pc),a1      | +054
        move.l  a1,(a6)                         | +058
.L0401a4:
        bra.w   Pow_Walk_03ff62__L03ffea        | +05a

| ----------------------------------------------------------------------------
|  Pow_Fall_0401a8  @ $0401A8  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Fall_0401a8, "ax", @progbits
        .global Pow_Fall_0401a8
Pow_Fall_0401a8:
        lea     Pow_AnimTblB_03f636__L03f72a(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        clr.w   0x2a(a6)                        | +00a
        clr.w   0x2e(a6)                        | +00e
        lea     .L0401c0(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L0401c0:
        move.b  0x72(a6),d0                     | +018
        jsr     0x77190.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L0401da                        | +028
        lea     Pow_FallAir_0401de(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L0401da:
        bra.w   Pow_Walk_03ff62__L03ffea        | +032

| ----------------------------------------------------------------------------
|  Pow_FallAir_0401de  @ $0401DE  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FallAir_0401de, "ax", @progbits
        .global Pow_FallAir_0401de
Pow_FallAir_0401de:
        lea     Pow_AnimTblB_03f636__L03f80e(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        clr.w   0x2e(a6)                        | +00a
        move.w  #0xfeac,0x2a(a6)                | +00e
        lea     .L0401f8(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L0401f8:
        move.b  0x72(a6),d0                     | +01a
        jsr     0x77190.l                       | +01e
        move.w  d0,-(a7)                        | +024
        jsr     0x28d70.l                       | +026
        bcc.w   .L040214                        | +02c
        lea     Pow_FallAir_0401de(pc),a1       | +030
        move.l  a1,(a6)                         | +034
.L040214:
        move.w  (a7)+,d0                        | +036
        bpl.w   .L040220                        | +038
        lea     Pow_WalkTurn_040014(pc),a1      | +03c
        move.l  a1,(a6)                         | +040
.L040220:
        jsr     0x27eba.l                       | +042
        bcs.w   .L040230                        | +048
        lea     Pow_Land_040234(pc),a1          | +04c
        move.l  a1,(a6)                         | +050
.L040230:
        bra.w   Pow_Walk_03ff62__L03ffea        | +052

| ----------------------------------------------------------------------------
|  Pow_Land_040234  @ $040234  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Land_040234, "ax", @progbits
        .global Pow_Land_040234
Pow_Land_040234:
        lea     Pow_AnimTblB_03f636__L03f888(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        clr.w   0x28(a6)                        | +00a
        clr.w   0x2a(a6)                        | +00e
        clr.b   0x70(a6)                        | +012
        lea     .L040250(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L040250:
        bsr.w   Pow_Physics_03fece              | +01c
        jsr     0x28d70.l                       | +020
        bcc.w   .L040264                        | +026
        lea     Pow_PickWalk_03ff24(pc),a1      | +02a
        move.l  a1,(a6)                         | +02e
.L040264:
        bra.w   Pow_Walk_03ff62__L03ffea        | +030

| ----------------------------------------------------------------------------
|  Pow_Rescued_040268  @ $040268  (156 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Rescued_040268, "ax", @progbits
        .global Pow_Rescued_040268
Pow_Rescued_040268:
        move.w  #0x113a,d0                      | +000
        jsr     0x2352.l                        | +004
        move.b  0x71(a6),d0                     | +00a
        bsr.w   Pow_AddRescued_03ef9a__L03efa2  | +00e
        lea     0xffff.w,a0                     | +012
        move.l  a0,0x48(a6)                     | +016
        lea     Pow_AnimTblA_03effa__L03f40a(pc),a0 | +01a
        jsr     0x28cd4.l                       | +01e
        move.w  #0xc0,d0                        | +024
        sub.w   0x22(a6),d0                     | +028
        bmi.w   .L0402a8                        | +02c
        bset    #0x0,0x3a(a6)                   | +030
        move.w  #0x700,0x28(a6)                 | +036
        bra.w   .L0402b4                        | +03c
.L0402a8:
        bclr    #0x0,0x3a(a6)                   | +040
        move.w  #0xf900,0x28(a6)                | +046
.L0402b4:
        move.w  #0x4,0x30(a6)                   | +04c
        lea     .L0402c0(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L0402c0:
        bsr.w   Pow_Physics_03fece              | +058
        jsr     0x28d70.l                       | +05c
        subq.w  #0x1,0x30(a6)                   | +062
        bne.w   .L0402f0                        | +066
        bchg    #0x0,0x3a(a6)                   | +06a
        tst.b   0x9b(a6)                        | +070
        beq.w   .L0402ea                        | +074
        lea     Pow_RescuedBow_040304(pc),a1    | +078
        move.l  a1,(a6)                         | +07c
        bra.w   .L0402f0                        | +07e
.L0402ea:
        lea     Pow_RescuedRun_04032e(pc),a1    | +082
        move.l  a1,(a6)                         | +086
        .global Pow_Rescued_040268__L0402f0
Pow_Rescued_040268__L0402f0:
.L0402f0:
        jsr     0x2870a.l                       | +088
        bcc.w   .L040300                        | +08e
        jmp     0x518.l                         | +092
.L040300:
        bra.w   Pow_OffworldFree_03efce         | +098

| ----------------------------------------------------------------------------
|  Pow_RescuedBow_040304  @ $040304  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescuedBow_040304, "ax", @progbits
        .global Pow_RescuedBow_040304
Pow_RescuedBow_040304:
        lea     Pow_AnimTblA_03effa__L03f49e(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        clr.w   0x28(a6)                        | +00a
        lea     .L040318(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L040318:
        bsr.w   Pow_Physics_03fece              | +014
        jsr     0x28d70.l                       | +018
        bcc.w   .L04032c                        | +01e
        lea     Pow_RescuedRun_04032e(pc),a1    | +022
        move.l  a1,(a6)                         | +026
.L04032c:
        bra.b   Pow_Rescued_040268__L0402f0     | +028

| ----------------------------------------------------------------------------
|  Pow_RescuedRun_04032e  @ $04032E  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescuedRun_04032e, "ax", @progbits
        .global Pow_RescuedRun_04032e
Pow_RescuedRun_04032e:
        lea     Pow_AnimTblB_03f636__L03f8f4(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        clr.w   0x28(a6)                        | +00a
        lea     .L040342(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L040342:
        jsr     Pow_Physics_03fece(pc)          | +014
        jsr     0x28d70.l                       | +018
        bcc.w   .L040356                        | +01e
        lea     .L040358(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L040356:
        bra.b   Pow_Rescued_040268__L0402f0     | +028
.L040358:
        lea     Pow_AnimTblB_03f636__L03f89e(pc),a0 | +02a
        jsr     0x28cd4.l                       | +02e
        bclr    #0x0,0x3a(a6)                   | +034
        move.w  #0xfc00,0x28(a6)                | +03a
        lea     .L040374(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L040374:
        jsr     Pow_Physics_03fece(pc)          | +046
        jsr     0x28d70.l                       | +04a
        btst    #0x5,0x5a(a6)                   | +050
        beq.w   .L04038e                        | +056
        lea     Pow_RescuedExit_040392(pc),a1   | +05a
        move.l  a1,(a6)                         | +05e
.L04038e:
        bra.w   Pow_Rescued_040268__L0402f0     | +060

| ----------------------------------------------------------------------------
|  Pow_RescuedExit_040392  @ $040392  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescuedExit_040392, "ax", @progbits
        .global Pow_RescuedExit_040392
Pow_RescuedExit_040392:
        move.b  #0x1e,0x78(a6)                  | +000
        lea     .L04039e(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L04039e:
        jsr     Pow_Physics_03fece(pc)          | +00c
        subq.b  #0x1,0x78(a6)                   | +010
        btst    #0x0,0x78(a6)                   | +014
        beq.w   .L0403b6                        | +01a
        jsr     0x28d70.l                       | +01e
.L0403b6:
        tst.b   0x78(a6)                        | +024
        bne.w   .L0403c4                        | +028
        jmp     0x518.l                         | +02c
.L0403c4:
        bra.w   Pow_Rescued_040268__L0402f0     | +032

| ----------------------------------------------------------------------------
|  Pow_RescuedRts_0403c8  @ $0403C8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescuedRts_0403c8, "ax", @progbits
        .global Pow_RescuedRts_0403c8
Pow_RescuedRts_0403c8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0403de                    | +00c

| ----------------------------------------------------------------------------
|  SquadLeader_Spawn_0403e4  @ $0403E4  (176 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Spawn_0403e4, "ax", @progbits
        .global SquadLeader_Spawn_0403e4
SquadLeader_Spawn_0403e4:
        move.w  #0x9a,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x13,0x1c(a6)                  | +00a
        jsr     0x138fe.l                       | +010
        clr.b   0x7c(a6)                        | +016
        move.w  #0x4000,d0                      | +01a
        jsr     0x28134.l                       | +01e
        andi.w  #0xffe3,0x38(a6)                | +024
        ori.w   #0x14,0x38(a6)                  | +02a
        lea     0x2bb7b6.l,a0                   | +030
        jsr     0x799de.l                       | +036
        move.w  d0,0x66(a6)                     | +03c
        lea     0x723d2.l,a1                    | +040
        jsr     0x4ae.l                         | +046
        jsr     0x5dd02.l                       | +04c
        clr.w   0x98(a0)                        | +052
        jsr     Squad_SpawnEight_041FB4(pc)     | +056
        move.b  #0x80,0x80(a6)                  | +05a
        move.b  #0x80,0x81(a6)                  | +060
        move.b  #0x80,0x82(a6)                  | +066
        move.b  #0x80,0x83(a6)                  | +06c
        move.b  #0x80,0x84(a6)                  | +072
        move.b  #0x80,0x85(a6)                  | +078
        move.b  #0x80,0x86(a6)                  | +07e
        move.b  #0x80,0x87(a6)                  | +084
        lea     0x2863e8.l,a0                   | +08a
        jsr     0x28cd4.l                       | +090
        move.w  #0x3c,0x70(a6)                  | +096
        lea     .L040486(pc),a1                 | +09c
        move.l  a1,(a6)                         | +0a0
.L040486:
        subq.w  #0x1,0x70(a6)                   | +0a2
        cmpi.w  #0x0,0x70(a6)                   | +0a6
        bgt.w   SetHandlerRts_04049a            | +0ac

| ----------------------------------------------------------------------------
|  SquadLeader_Enter_04049c  @ $04049C  (76 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Enter_04049c, "ax", @progbits
        .global SquadLeader_Enter_04049c
SquadLeader_Enter_04049c:
        move.w  #0x10a2,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  #0xc000,0x34(a6)                | +00a
        move.w  #0x200,0x36(a6)                 | +010
        move.w  #0x160,0x22(a6)                 | +016
        move.w  #0x160,0x24(a6)                 | +01c
        move.w  #0xf0,0x8a(a6)                  | +022
        move.w  #0x180,0x8c(a6)                 | +028
        lea     .L0404d0(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L0404d0:
        move.w  #0x14,d5                        | +034
        jsr     Squad_SteerTowardTarget_041CE0(pc) | +038
        bcc.w   .L0404e2                        | +03c
        lea     SquadLeader_Turn_0404ee(pc),a1  | +040
        move.l  a1,(a6)                         | +044
.L0404e2:
        jsr     0x27cee.l                       | +046

| ----------------------------------------------------------------------------
|  SquadLeader_Turn_0404ee  @ $0404EE  (56 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Turn_0404ee, "ax", @progbits
        .global SquadLeader_Turn_0404ee
SquadLeader_Turn_0404ee:
        move.w  #0x200,0x36(a6)                 | +000
        lea     .L0404fa(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0404fa:
        move.w  #0xffe0,d1                      | +00c
        clr.w   d0                              | +010
        jsr     Squad_TurnRateStepClamp_041D6C(pc) | +012
        bcc.w   .L04050e                        | +016
        lea     SquadLeader_Hover_04052c(pc),a1 | +01a
        move.l  a1,(a6)                         | +01e
.L04050e:
        move.w  #0x8,d5                         | +020
        jsr     Squad_SteerTowardTarget_041CE0(pc) | +024
        bcc.w   .L040520                        | +028
        lea     SquadLeader_Hover_04052c(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L040520:
        jsr     0x27cee.l                       | +032

| ----------------------------------------------------------------------------
|  SquadLeader_Hover_04052c  @ $04052C  (134 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Hover_04052c, "ax", @progbits
        .global SquadLeader_Hover_04052c
SquadLeader_Hover_04052c:
        move.w  #0x100,0x36(a6)                 | +000
        clr.b   0x21(a6)                        | +006
        clr.w   0x76(a6)                        | +00a
        move.w  #0x2d,0x70(a6)                  | +00e
        lea     .L040546(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L040546:
        cmpi.w  #0x110,0x22(a6)                 | +01a
        ble.w   .L04056c                        | +020
        move.w  #0x1,d0                         | +024
        jsr     Squad_SteerTowardTarget_041CE0(pc) | +028
        bcc.w   .L040562                        | +02c
        eori.w  #0x10,0x8c(a6)                  | +030
.L040562:
        jsr     0x27cee.l                       | +036
        bra.w   .L040572                        | +03c
.L04056c:
        jsr     0x2783a.l                       | +040
.L040572:
        jsr     Squad_BobYWide_041DDC(pc)       | +046
        subq.w  #0x1,0x70(a6)                   | +04a
        cmpi.w  #0x0,0x70(a6)                   | +04e
        bne.w   .L040590                        | +054
        move.b  #0x2,0x81(a6)                   | +058
        move.b  #0x8,0x84(a6)                   | +05e
.L040590:
        btst    #0x4,0x21(a6)                   | +064
        beq.w   .L0405ae                        | +06a
        move.b  #0x82,0x84(a6)                  | +06e
        move.l  #0x285d16,0x48(a6)              | +074
        lea     SquadLeader_Formation_0405b2(pc),a1 | +07c
        move.l  a1,(a6)                         | +080
.L0405ae:
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +082

| ----------------------------------------------------------------------------
|  SquadLeader_Formation_0405b2  @ $0405B2  (258 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Formation_0405b2, "ax", @progbits
        .global SquadLeader_Formation_0405b2
SquadLeader_Formation_0405b2:
        lea     0x2bb8ba.l,a0                   | +000
        cmpi.b  #0xff,0x7c(a6)                  | +006
        bne.w   .L0405c8                        | +00c
        lea     0x2bb93c.l,a0                   | +010
.L0405c8:
        jsr     0x799de.l                       | +016
        move.w  d0,0x70(a6)                     | +01c
        jsr     Squad_InitFormationSlot_041C52(pc) | +020
        clr.b   0x21(a6)                        | +024
        lea     .L0405e0(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L0405e0:
        move.w  #0x8,d5                         | +02e
        jsr     Squad_SteerTowardTarget_041CE0(pc) | +032
        bcc.w   .L0405f0                        | +036
        jsr     Squad_InitFormationSlot_041C52(pc) | +03a
.L0405f0:
        jsr     0x27cee.l                       | +03e
        jsr     Squad_BobYWide_041DDC(pc)       | +044
        subq.w  #0x1,0x70(a6)                   | +048
        cmpi.w  #0x0,0x70(a6)                   | +04c
        bgt.w   .L040622                        | +052
        cmpi.w  #0x110,0x22(a6)                 | +056
        bgt.w   .L040622                        | +05c
        tst.b   0x92(a6)                        | +060
        bne.w   .L040622                        | +064
        jsr     SquadAnim_State4Select_041E70(pc) | +068
        jsr     Squad_StateDispatch_041E4E(pc)  | +06c
.L040622:
        lea     0x2bb838.l,a0                   | +070
        jsr     0x799de.l                       | +076
        cmp.w   0x66(a6),d0                     | +07c
        blt.w   .L04063c                        | +080
        move.b  #0xff,0x7c(a6)                  | +084
.L04063c:
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +08a
        lea     0x2bb9be.l,a0                   | +08e
        cmpi.b  #0xff,0x7c(a6)                  | +094
        bne.w   .L040656                        | +09a
        lea     0x2bba40.l,a0                   | +09e
.L040656:
        jsr     0x799de.l                       | +0a4
        move.w  d0,0x74(a6)                     | +0aa
        move.w  #0xc000,0x76(a6)                | +0ae
        jsr     Squad_PickSwoopState_041C9C(pc) | +0b4
        clr.w   0x36(a6)                        | +0b8
        clr.b   0x21(a6)                        | +0bc
        move.b  #0x85,0x84(a6)                  | +0c0
        lea     .L04067e(pc),a1                 | +0c6
        move.l  a1,(a6)                         | +0ca
.L04067e:
        move.w  0x74(a6),d0                     | +0cc
        move.w  d0,d1                           | +0d0
        lsr.w   #0x5,d1                         | +0d2
        jsr     Squad_TurnRateStepClamp_041D6C(pc) | +0d4
        jsr     Squad_ComputeTargetPos_041C1A(pc) | +0d8
        move.w  #0x20,d5                        | +0dc
        jsr     Squad_SteerTowardTarget_041CE0(pc) | +0e0
        bcc.w   .L0406a0                        | +0e4
        lea     SquadLeader_Swoop_0406b4(pc),a1 | +0e8
        move.l  a1,(a6)                         | +0ec
.L0406a0:
        jsr     0x27cee.l                       | +0ee
        jsr     0x28d70.l                       | +0f4
        clr.w   0x8e(a6)                        | +0fa
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +0fe

| ----------------------------------------------------------------------------
|  SquadLeader_Swoop_0406b4  @ $0406B4  (74 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Swoop_0406b4, "ax", @progbits
        .global SquadLeader_Swoop_0406b4
SquadLeader_Swoop_0406b4:
        lea     .L0406ba(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L0406ba:
        move.w  0x74(a6),d0                     | +006
        move.w  d0,d1                           | +00a
        lsr.w   #0x5,d1                         | +00c
        neg.w   d1                              | +00e
        clr.w   d0                              | +010
        jsr     Squad_TurnRateStepClamp_041D6C(pc) | +012
        jsr     Squad_ComputeTargetPos_041C1A(pc) | +016
        move.w  #0x4,d5                         | +01a
        jsr     Squad_SteerTowardTarget_041CE0(pc) | +01e
        bcc.w   .L0406ea                        | +022
        btst    #0x4,0x21(a6)                   | +026
        beq.w   .L0406ea                        | +02c
        lea     SquadLeader_PickAttack_0406fe(pc),a1 | +030
        move.l  a1,(a6)                         | +034
.L0406ea:
        jsr     0x27cee.l                       | +036
        jsr     0x28d70.l                       | +03c
        clr.w   0x8e(a6)                        | +042
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +046

| ----------------------------------------------------------------------------
|  SquadLeader_PickAttack_0406fe  @ $0406FE  (184 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_PickAttack_0406fe, "ax", @progbits
        .global SquadLeader_PickAttack_0406fe
SquadLeader_PickAttack_0406fe:
        lea     0x2bbac2.l,a0                   | +000
        cmpi.b  #0xff,0x7c(a6)                  | +006
        bne.w   .L040714                        | +00c
        lea     0x2bbb44.l,a0                   | +010
.L040714:
        jsr     0x799de.l                       | +016
        move.w  d0,0x70(a6)                     | +01c
        lea     0x2bbdce.l,a0                   | +020
        cmpi.b  #0xff,0x7c(a6)                  | +026
        bne.w   .L040734                        | +02c
        lea     0x2bbe50.l,a0                   | +030
.L040734:
        jsr     0x799de.l                       | +036
        move.w  d0,0x72(a6)                     | +03c
        lea     0x2bbcca.l,a0                   | +040
        cmpi.b  #0xff,0x7c(a6)                  | +046
        bne.w   .L040754                        | +04c
        lea     0x2bbd4c.l,a0                   | +050
.L040754:
        jsr     0x799de.l                       | +056
        move.b  d0,0x7b(a6)                     | +05c
        bset    #0x4,0x21(a6)                   | +060
        bra.w   .L04077c                        | +066
.L040768:
        clr.b   0x21(a6)                        | +06a
        subq.b  #0x1,0x7b(a6)                   | +06e
        move.b  #0x86,0x84(a6)                  | +072
        move.w  0x72(a6),0x70(a6)               | +078
.L04077c:
        lea     .L040782(pc),a1                 | +07e
        move.l  a1,(a6)                         | +082
.L040782:
        subq.w  #0x1,0x70(a6)                   | +084
        cmpi.w  #0x0,0x70(a6)                   | +088
        bgt.w   .L0407a8                        | +08e
        btst    #0x4,0x21(a6)                   | +092
        beq.w   .L0407a8                        | +098
        cmpi.b  #0x0,0x7b(a6)                   | +09c
        bgt.b   .L040768                        | +0a2
        lea     SquadLeader_Dive_0407b6(pc),a1  | +0a4
        move.l  a1,(a6)                         | +0a8
.L0407a8:
        jsr     0x2783a.l                       | +0aa
        jsr     Squad_BobYNarrow_041E02(pc)     | +0b0
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +0b4

| ----------------------------------------------------------------------------
|  SquadLeader_Dive_0407b6  @ $0407B6  (136 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Dive_0407b6, "ax", @progbits
        .global SquadLeader_Dive_0407b6
SquadLeader_Dive_0407b6:
        move.w  #0xc000,0x76(a6)                | +000
        move.w  #0x100,0x8a(a6)                 | +006
        move.w  #0x190,0x8c(a6)                 | +00c
        move.b  #0x84,0x84(a6)                  | +012
        lea     .L0407d4(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L0407d4:
        move.w  #0x200,d0                       | +01e
        move.w  #0x20,d1                        | +022
        jsr     Squad_TurnRateStepClamp_041D6C(pc) | +026
        jsr     Squad_ComputeTargetPos_041C1A(pc) | +02a
        move.w  #0x8,d5                         | +02e
        jsr     Squad_SteerTowardTarget_041CE0(pc) | +032
        bcc.w   .L0407f6                        | +036
        lea     SquadLeader_Formation_0405b2(pc),a1 | +03a
        move.l  a1,(a6)                         | +03e
.L0407f6:
        jsr     0x27cee.l                       | +040
        jsr     0x28d70.l                       | +046
        clr.w   0x8e(a6)                        | +04c
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +050
        move.b  #0x4,0x82(a6)                   | +054
        move.b  #0x9,0x85(a6)                   | +05a
        clr.b   0x21(a6)                        | +060
        lea     .L040820(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L040820:
        btst    #0x2,0x21(a6)                   | +06a
        beq.w   .L040830                        | +070
        lea     SquadLeader_DiveTurn_04083e(pc),a1 | +074
        move.l  a1,(a6)                         | +078
.L040830:
        jsr     0x2783a.l                       | +07a
        jsr     Squad_BobYNarrow_041E02(pc)     | +080
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +084

| ----------------------------------------------------------------------------
|  SquadLeader_DiveTurn_04083e  @ $04083E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_DiveTurn_04083e, "ax", @progbits
        .global SquadLeader_DiveTurn_04083e
SquadLeader_DiveTurn_04083e:
        move.w  #0xc000,0x76(a6)                | +000
        move.w  #0x120,0x8a(a6)                 | +006
        move.w  #0x200,0x8c(a6)                 | +00c
        lea     .L040856(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L040856:
        move.w  #0x200,d0                       | +018
        move.w  #0x20,d1                        | +01c
        jsr     Squad_TurnRateStepClamp_041D6C(pc) | +020
        move.w  #0x10,d5                        | +024
        jsr     Squad_SteerTowardTarget_041CE0(pc) | +028
        bcc.w   .L040874                        | +02c
        lea     SquadLeader_Regroup_040886(pc),a1 | +030
        move.l  a1,(a6)                         | +034
.L040874:
        jsr     0x27cee.l                       | +036
        clr.w   0x8e(a6)                        | +03c
        jsr     Squad_BobYNarrow_041E02(pc)     | +040
        bra.w   SquadLeader_HitCheck_040b40__L040bc4 | +044

| ----------------------------------------------------------------------------
|  SquadLeader_Regroup_040886  @ $040886  (160 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Regroup_040886, "ax", @progbits
        .global SquadLeader_Regroup_040886
SquadLeader_Regroup_040886:
        lea     0x2bbed2.l,a0                   | +000
        cmpi.b  #0xff,0x7c(a6)                  | +006
        bne.w   .L04089c                        | +00c
        lea     0x2bbf54.l,a0                   | +010
.L04089c:
        jsr     0x799de.l                       | +016
        lea     0x2bbfd6.l,a0                   | +01c
        lsl.w   #0x2,d0                         | +022
        move.w  (a0,d0.w),0x90(a6)              | +024
        addq.w  #0x2,d0                         | +02a
        move.w  (a0,d0.w),d1                    | +02c
        move.w  d1,0x36(a6)                     | +030
        neg.w   d1                              | +034
        move.w  d1,0x28(a6)                     | +036
        clr.w   0x70(a6)                        | +03a
        bset    #0x5,0x21(a6)                   | +03e
        lea     .L0408d0(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L0408d0:
        subq.w  #0x1,0x70(a6)                   | +04a
        cmpi.w  #0x0,0x70(a6)                   | +04e
        bgt.w   .L040908                        | +054
        btst    #0x5,0x21(a6)                   | +058
        beq.w   .L040908                        | +05e
        move.w  0x90(a6),0x70(a6)               | +062
        cmpi.w  #0x12,0x70(a6)                  | +068
        ble.w   .L040902                        | +06e
        move.b  #0xd,0x85(a6)                   | +072
        beq.w   .L040908                        | +078
.L040902:
        move.b  #0xe,0x85(a6)                   | +07c
.L040908:
        jsr     0x27cee.l                       | +082
        jsr     Squad_BobYNarrow_041E02(pc)     | +088
        cmpi.w  #0xffe2,0x22(a6)                | +08c
        bgt.w   .L040922                        | +092
        lea     SquadLeader_Reform_040926(pc),a1 | +096
        move.l  a1,(a6)                         | +09a
.L040922:
        bra.w   SquadLeader_HitCheck_040b40__L040bc4 | +09c

| ----------------------------------------------------------------------------
|  SquadLeader_Reform_040926  @ $040926  (214 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Reform_040926, "ax", @progbits
        .global SquadLeader_Reform_040926
SquadLeader_Reform_040926:
        move.b  #0x80,0x82(a6)                  | +000
        move.w  #0x160,0x22(a6)                 | +006
        move.w  #0x160,0x8a(a6)                 | +00c
        move.w  0x24(a6),0x8c(a6)               | +012
        move.w  #0x3c,0x70(a6)                  | +018
        lea     .L04094a(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L04094a:
        cmpi.w  #0x1e,0x70(a6)                  | +024
        bgt.w   .L04095e                        | +02a
        jsr     Squad_InitFormationSlot_041C52(pc) | +02e
        lea     .L04095e(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L04095e:
        move.w  #0x8,d5                         | +038
        jsr     Squad_SteerTowardTarget_041CE0(pc) | +03c
        move.b  #0x80,0x85(a6)                  | +040
        jsr     0x27cee.l                       | +046
        jsr     Squad_BobYWide_041DDC(pc)       | +04c
        subq.w  #0x1,0x70(a6)                   | +050
        cmpi.w  #0x0,0x70(a6)                   | +054
        bgt.w   .L0409a4                        | +05a
        lea     0x2bca3e.l,a0                   | +05e
        cmpi.b  #0xff,0x7c(a6)                  | +064
        bne.w   .L04099a                        | +06a
        lea     0x2bcac0.l,a0                   | +06e
.L04099a:
        jsr     0x799de.l                       | +074
        jsr     Squad_StateDispatch_041E4E(pc)  | +07a
.L0409a4:
        bra.w   SquadLeader_HitCheck_040b40__L040bc4 | +07e
        jsr     0x5e1ea.l                       | +082
        move.l  a0,0x9c(a6)                     | +088
        move.b  #0x84,0x84(a6)                  | +08c
        lea     .L0409be(pc),a1                 | +092
        move.l  a1,(a6)                         | +096
.L0409be:
        jsr     0x2783a.l                       | +098
        movea.l 0x9c(a6),a0                     | +09e
        move.w  0x22(a6),d0                     | +0a2
        subi.w  #0x20,d0                        | +0a6
        cmp.w   0x22(a0),d0                     | +0aa
        bgt.w   .L0409e4                        | +0ae
        addq.w  #0x1,0x22(a6)                   | +0b2
        jsr     Squad_BobYWide_041DDC(pc)       | +0b6
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +0ba
.L0409e4:
        jsr     Squad_BobYWide_041DDC(pc)       | +0be
        cmpi.b  #0x1,0x7d(a6)                   | +0c2
        bne.w   .L0409f8                        | +0c8
        lea     SquadLeader_Order86_0409fc(pc),a1 | +0cc
        move.l  a1,(a6)                         | +0d0
.L0409f8:
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +0d2

| ----------------------------------------------------------------------------
|  SquadLeader_Order86_0409fc  @ $0409FC  (46 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Order86_0409fc, "ax", @progbits
        .global SquadLeader_Order86_0409fc
SquadLeader_Order86_0409fc:
        clr.b   0x21(a6)                        | +000
        move.b  #0x86,0x84(a6)                  | +004
        lea     .L040a0c(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L040a0c:
        btst    #0x4,0x21(a6)                   | +010
        beq.w   .L040a1c                        | +016
        lea     SquadLeader_Order84_040a2a(pc),a1 | +01a
        move.l  a1,(a6)                         | +01e
.L040a1c:
        jsr     0x2783a.l                       | +020
        jsr     Squad_BobYWide_041DDC(pc)       | +026
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +02a

| ----------------------------------------------------------------------------
|  SquadLeader_Order84_040a2a  @ $040A2A  (60 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Order84_040a2a, "ax", @progbits
        .global SquadLeader_Order84_040a2a
SquadLeader_Order84_040a2a:
        clr.b   0x21(a6)                        | +000
        move.b  #0x84,0x84(a6)                  | +004
        bra.w   SquadLeader_Formation_0405b2    | +00a
        move.b  #0x0,0x80(a6)                   | +00e
        clr.b   0x21(a6)                        | +014
        lea     .L040a48(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L040a48:
        btst    #0x0,0x21(a6)                   | +01e
        beq.w   .L040a58                        | +024
        lea     SquadLeader_PickOrder_040a66(pc),a1 | +028
        move.l  a1,(a6)                         | +02c
.L040a58:
        jsr     0x2783a.l                       | +02e
        jsr     Squad_BobYNarrow_041E02(pc)     | +034
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +038

| ----------------------------------------------------------------------------
|  SquadLeader_PickOrder_040a66  @ $040A66  (64 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_PickOrder_040a66, "ax", @progbits
        .global SquadLeader_PickOrder_040a66
SquadLeader_PickOrder_040a66:
        lea     0x2bc0fa.l,a0                   | +000
        cmpi.b  #0xff,0x7c(a6)                  | +006
        bne.w   .L040a7c                        | +00c
        lea     0x2bc17c.l,a0                   | +010
.L040a7c:
        jsr     0x799de.l                       | +016
        move.b  d0,0x7b(a6)                     | +01c
        lea     0x2bc322.l,a0                   | +020
        cmpi.b  #0xff,0x7c(a6)                  | +026
        bne.w   .L040a9c                        | +02c
        lea     0x2bc3a4.l,a0                   | +030
.L040a9c:
        jsr     0x799de.l                       | +036
        move.w  d0,0x72(a6)                     | +03c

| ----------------------------------------------------------------------------
|  SquadLeader_Circle_040aa6  @ $040AA6  (154 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Circle_040aa6, "ax", @progbits
        .global SquadLeader_Circle_040aa6
SquadLeader_Circle_040aa6:
        cmpi.b  #0x0,0x7b(a6)                   | +000
        ble.w   .L040ae8                        | +006
        subq.b  #0x1,0x7b(a6)                   | +00a
        move.w  0x72(a6),0x70(a6)               | +00e
        move.b  #0xb,0x80(a6)                   | +014
        lea     .L040ac6(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L040ac6:
        jsr     0x2783a.l                       | +020
        jsr     Squad_BobYNarrow_041E02(pc)     | +026
        subq.w  #0x1,0x70(a6)                   | +02a
        cmpi.w  #0x0,0x70(a6)                   | +02e
        bgt.w   .L040ae4                        | +034
        lea     SquadLeader_Circle_040aa6(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L040ae4:
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +03e
.L040ae8:
        move.b  #0x1,0x80(a6)                   | +042
        lea     .L040af4(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L040af4:
        jsr     0x2783a.l                       | +04e
        jsr     Squad_BobYNarrow_041E02(pc)     | +054
        btst    #0x0,0x21(a6)                   | +058
        beq.w   .L040b0e                        | +05e
        lea     SquadLeader_Formation_0405b2(pc),a1 | +062
        move.l  a1,(a6)                         | +066
.L040b0e:
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +068
        clr.b   0x21(a6)                        | +06c
        move.b  #0x6,0x83(a6)                   | +070
        lea     .L040b22(pc),a1                 | +076
        move.l  a1,(a6)                         | +07a
.L040b22:
        btst    #0x3,0x21(a6)                   | +07c
        beq.w   .L040b32                        | +082
        lea     SquadLeader_HitCheck_040b40(pc),a1 | +086
        move.l  a1,(a6)                         | +08a
        .global SquadLeader_Circle_040aa6__L040b32
SquadLeader_Circle_040aa6__L040b32:
.L040b32:
        jsr     0x2783a.l                       | +08c
        jsr     Squad_BobYWide_041DDC(pc)       | +092
        bra.w   SquadLeader_HitCheck_040b40__L040b68 | +096

| ----------------------------------------------------------------------------
|  SquadLeader_HitCheck_040b40  @ $040B40  (166 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_HitCheck_040b40, "ax", @progbits
        .global SquadLeader_HitCheck_040b40
SquadLeader_HitCheck_040b40:
        clr.b   0x21(a6)                        | +000
        move.b  #0x81,0x83(a6)                  | +004
        lea     .L040b50(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L040b50:
        btst    #0x3,0x21(a6)                   | +010
        beq.w   .L040b66                        | +016
        move.b  #0x80,0x83(a6)                  | +01a
        lea     SquadLeader_Formation_0405b2(pc),a1 | +020
        move.l  a1,(a6)                         | +024
.L040b66:
        bra.b   SquadLeader_Circle_040aa6__L040b32 | +026
        .global SquadLeader_HitCheck_040b40__L040b68
SquadLeader_HitCheck_040b40__L040b68:
.L040b68:
        clr.b   0x92(a6)                        | +028
        jsr     0x2870a.l                       | +02c
        bclr    #0x3,0x13(a6)                   | +032
        lea     0x5e766.l,a0                    | +038
        jsr     0x5e770.l                       | +03e
        jsr     0x28758.l                       | +044
        bcc.w   .L040bc2                        | +04a
        cmpi.w  #0x90,0x22(a6)                  | +04e
        blt.w   .L040bb6                        | +054
        cmpi.w  #0x120,0x22(a6)                 | +058
        bgt.w   .L040bb6                        | +05e
        jsr     0x5e366.l                       | +062
        bcs.w   .L040bb6                        | +068
        lea     SquadLeader_Death_040be6(pc),a1 | +06c
        move.l  a1,(a6)                         | +070
        bra.w   .L040bc2                        | +072
.L040bb6:
        move.w  #0x1,0x66(a6)                   | +076
        bclr    #0x0,0x13(a6)                   | +07c
.L040bc2:
        rts                                     | +082
        .global SquadLeader_HitCheck_040b40__L040bc4
SquadLeader_HitCheck_040b40__L040bc4:
.L040bc4:
        jsr     0x2870a.l                       | +084
        bclr    #0x3,0x13(a6)                   | +08a
        lea     0x5e766.l,a0                    | +090
        jsr     0x5e770.l                       | +096
        jsr     0x28758.l                       | +09c
        bcc.b   .L040bc2                        | +0a2
        bra.b   .L040bb6                        | +0a4

| ----------------------------------------------------------------------------
|  SquadLeader_Death_040be6  @ $040BE6  (298 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Death_040be6, "ax", @progbits
        .global SquadLeader_Death_040be6
SquadLeader_Death_040be6:
        clr.b   0x106ed3.l                      | +000
        move.b  #0x0,0x10a2d0.l                 | +006
        move.b  #0x0,0x10a2d1.l                 | +00e
        move.l  #0xffffffff,0x48(a6)            | +016
        move.b  #0x80,0x84(a6)                  | +01e
        lea     SquadChild_SwoopPhysics_041408(pc),a1 | +024
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        lea     SquadChild_DieToScheduler_041586(pc),a1 | +034
        jsr     0x4ae.l                         | +038
        jsr     0x5dd02.l                       | +03e
        lea     0x8121c.l,a1                    | +044
        jsr     0x4ae.l                         | +04a
        jsr     0x5dd02.l                       | +050
        move.w  #0xc000,0x38(a0)                | +056
        lea     0x8123c.l,a1                    | +05c
        jsr     0x4ae.l                         | +062
        jsr     0x5dd02.l                       | +068
        move.w  #0xc000,0x38(a0)                | +06e
        lea     0x81260.l,a1                    | +074
        jsr     0x4ae.l                         | +07a
        jsr     0x5dd02.l                       | +080
        move.w  #0xc000,0x38(a0)                | +086
        lea     0x81284.l,a1                    | +08c
        jsr     0x4ae.l                         | +092
        jsr     0x5dd02.l                       | +098
        move.w  #0xc000,0x38(a0)                | +09e
        move.w  #0x1e,0x70(a6)                  | +0a4
        jsr     0x267e2.l                       | +0aa
        lea     .L040c9c(pc),a1                 | +0b0
        move.l  a1,(a6)                         | +0b4
.L040c9c:
        movea.l #0xffffffff,a0                  | +0b6
        lea     0x286102.l,a0                   | +0bc
        jsr     0x5dd5c.l                       | +0c2
        bcc.w   .L040cc2                        | +0c8
        move.w  #0x10a2,d0                      | +0cc
        jsr     0x2222.l                        | +0d0
        lea     .L040cc2(pc),a1                 | +0d6
        move.l  a1,(a6)                         | +0da
.L040cc2:
        jsr     0x27d50.l                       | +0dc
        bcc.w   .L040cd2                        | +0e2
        lea     SquadLeader_DeathDone_040d18(pc),a1 | +0e6
        move.l  a1,(a6)                         | +0ea
.L040cd2:
        jsr     Squad_BobYFast_041DB6(pc)       | +0ec
        subq.w  #0x1,0x70(a6)                   | +0f0
        cmpi.w  #0x0,0x70(a6)                   | +0f4
        bne.w   .L040d06                        | +0fa
        move.w  #0xc0,0x28(a6)                  | +0fe
        move.w  #0xffc0,0x2a(a6)                | +104
        move.w  #0x1032,d0                      | +10a
        jsr     0x2352.l                        | +10e
        lea     0x286112.l,a1                   | +114
        jsr     0x77c7e.l                       | +11a
.L040d06:
        cmpi.w  #0x120,0x22(a6)                 | +120
        blt.w   SetHandlerRts_040d16            | +126

| ----------------------------------------------------------------------------
|  SquadLeader_DeathDone_040d18  @ $040D18  (84 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_DeathDone_040d18, "ax", @progbits
        .global SquadLeader_DeathDone_040d18
SquadLeader_DeathDone_040d18:
        clr.b   0x106ed2.l                      | +000
        jsr     0x434ce.l                       | +006
        move.w  0x22(a6),0x5c(a6)               | +00c
        move.w  #0x120,0x22(a6)                 | +012
        move.w  #0x1033,d0                      | +018
        jsr     0x2352.l                        | +01c
        move.b  #0x4,0x10a2d0.l                 | +022
        move.b  #0x0,0x10a2d1.l                 | +02a
        lea     0x77fd6.l,a1                    | +032
        jsr     0x4ae.l                         | +038
        jsr     0x5dd02.l                       | +03e
        jsr     0x628ea.l                       | +044
        move.w  0x5c(a6),0x22(a6)               | +04a
        bra.w   SquadLeader_Order8A_040eba      | +050

| ----------------------------------------------------------------------------
|  SquadLeader_Respawn_040d6c  @ $040D6C  (156 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Respawn_040d6c, "ax", @progbits
        .global SquadLeader_Respawn_040d6c
SquadLeader_Respawn_040d6c:
        move.w  #0x9a,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x4000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x14,0x38(a6)                  | +01a
        jsr     Squad_SpawnEight_041FB4(pc)     | +020
        move.b  #0x80,0x80(a6)                  | +024
        move.b  #0x80,0x81(a6)                  | +02a
        move.b  #0x80,0x82(a6)                  | +030
        move.b  #0x80,0x83(a6)                  | +036
        move.b  #0x80,0x84(a6)                  | +03c
        move.b  #0x80,0x85(a6)                  | +042
        move.b  #0x80,0x86(a6)                  | +048
        move.b  #0x80,0x87(a6)                  | +04e
        move.w  #0x400,0x36(a6)                 | +054
        lea     0x2863e8.l,a0                   | +05a
        jsr     0x28cd4.l                       | +060
        clr.w   0x8e(a6)                        | +066
        move.w  #0x180,0x22(a6)                 | +06a
        move.w  #0x1e0,0x24(a6)                 | +070
        move.l  #0x288382,0x90(a6)              | +076
        clr.w   0x94(a6)                        | +07e
        clr.b   0x96(a6)                        | +082
        lea     .L040df8(pc),a1                 | +086
        move.l  a1,(a6)                         | +08a
.L040df8:
        jsr     0x78f8a.l                       | +08c
        bcc.w   JsrPcThunk_040e08               | +092
        lea     SquadLeader_RespawnWait_040e0e(pc),a1 | +096
        move.l  a1,(a6)                         | +09a

| ----------------------------------------------------------------------------
|  SquadLeader_RespawnWait_040e0e  @ $040E0E  (62 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_RespawnWait_040e0e, "ax", @progbits
        .global SquadLeader_RespawnWait_040e0e
SquadLeader_RespawnWait_040e0e:
        clr.b   0x21(a6)                        | +000
        move.w  #0x2d,0x70(a6)                  | +004
        move.b  #0x2,0x81(a6)                   | +00a
        move.b  #0x8,0x84(a6)                   | +010
        lea     .L040e2a(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L040e2a:
        jsr     0x2783a.l                       | +01c
        jsr     Squad_BobYWide_041DDC(pc)       | +022
        subq.w  #0x1,0x70(a6)                   | +026
        cmpi.w  #0x0,0x70(a6)                   | +02a
        bgt.w   SetHandlerRts_040e52            | +030
        btst    #0x4,0x21(a6)                   | +034
        beq.w   SetHandlerRts_040e52            | +03a

| ----------------------------------------------------------------------------
|  SquadLeader_Order88_040e54  @ $040E54  (94 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Order88_040e54, "ax", @progbits
        .global SquadLeader_Order88_040e54
SquadLeader_Order88_040e54:
        move.w  #0xf,0x70(a6)                   | +000
        clr.b   0x21(a6)                        | +006
        move.b  #0x88,0x84(a6)                  | +00a
        lea     .L040e6a(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L040e6a:
        btst    #0x4,0x21(a6)                   | +016
        beq.w   .L040e92                        | +01c
        subq.w  #0x1,0x70(a6)                   | +020
        cmpi.w  #0x0,0x70(a6)                   | +024
        bgt.w   .L040e92                        | +02a
        clr.b   0x21(a6)                        | +02e
        move.b  #0x89,0x84(a6)                  | +032
        lea     .L040e92(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L040e92:
        jsr     0x2783a.l                       | +03e
        jsr     Squad_BobYWide_041DDC(pc)       | +044
        movea.l #0xffffffff,a0                  | +048
        lea     0x286102.l,a0                   | +04e
        jsr     0x5dd56.l                       | +054
        bcc.w   SetHandlerRts_040eb8            | +05a

| ----------------------------------------------------------------------------
|  SquadLeader_Order8A_040eba  @ $040EBA  (54 B)
| ----------------------------------------------------------------------------
        .section .text.SquadLeader_Order8A_040eba, "ax", @progbits
        .global SquadLeader_Order8A_040eba
SquadLeader_Order8A_040eba:
        move.b  #0x8a,0x80(a6)                  | +000
        move.b  #0x8a,0x81(a6)                  | +006
        move.b  #0x8a,0x82(a6)                  | +00c
        move.b  #0x8a,0x83(a6)                  | +012
        move.b  #0x8a,0x84(a6)                  | +018
        move.b  #0x8a,0x85(a6)                  | +01e
        move.b  #0x8a,0x86(a6)                  | +024
        move.b  #0x8a,0x87(a6)                  | +02a
        jmp     0x518.l                         | +030

| ----------------------------------------------------------------------------
|  Rts_040ef0  @ $040EF0  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_040ef0, "ax", @progbits
        .global Rts_040ef0
Rts_040ef0:
        rts                                     | +000
