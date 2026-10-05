| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $036632..$0388F0  (8,624 B, 39 entradas, 2 huecos)
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
|  Player_RideSlug_Tail_036632  @ $036632  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Player_RideSlug_Tail_036632, "ax", @progbits
        .global Player_RideSlug_Tail_036632
Player_RideSlug_Tail_036632:
        jsr     PlayerRoute_PublishState_033522(pc) | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  Player_HangRing_036638  @ $036638  (198 B)
| ----------------------------------------------------------------------------
        .section .text.Player_HangRing_036638, "ax", @progbits
        .global Player_HangRing_036638
Player_HangRing_036638:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x1,0x8c(a6)                   | +006
        bclr    #0x3,0x8c(a6)                   | +00c
        bclr    #0x0,0x3a(a6)                   | +012
        lea     Sub_0003292C(pc),a0             | +018
        move.l  a0,0x48(a6)                     | +01c
        move.w  #0x0,0x7c(a6)                   | +020
        move.w  #0x10,0x7e(a6)                  | +026
        move.b  #0x3,0x70(a6)                   | +02c
        lea     0x279f08.l,a0                   | +032
        move.l  -0x4(a0),0x74(a6)               | +038
        move.b  #0xff,0x21(a6)                  | +03e
        lea     0x279f08.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        clr.w   0x28(a6)                        | +050
        clr.w   0x2c(a6)                        | +054
        clr.w   0x2a(a6)                        | +058
        clr.w   0x2e(a6)                        | +05c
        lea     .L03669e(pc),a1                 | +060
        move.l  a1,(a6)                         | +064
.L03669e:
        jsr     Player_FrameCommon_NoPrio_033016(pc) | +066
        jsr     0x2783a.l                       | +06a
        move.b  0x86(a6),d0                     | +070
        jsr     0x8f5dc.l                       | +074
        bcc.w   .L0366c2                        | +07a
        move.w  d0,0x22(a6)                     | +07e
        move.w  d1,0x24(a6)                     | +082
        bra.w   .L0366d4                        | +086
.L0366c2:
        lea     Player_JumpStart_036914(pc),a1  | +08a
        move.l  a1,(a6)                         | +08e
        addi.w  #0x20,0x24(a6)                  | +090
        move.b  #0x14,0x45(a6)                  | +096
.L0366d4:
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +09c
        jsr     0x5cdb4.l                       | +0a0
        bcc.w   .L0366f8                        | +0a6
        lea     Player_SlugJumpOff_036796(pc),a1 | +0aa
        move.l  a1,(a6)                         | +0ae
        addi.w  #0x20,0x24(a6)                  | +0b0
        move.b  #0x14,0x45(a6)                  | +0b6
        bra.w   .L0366f8                        | +0bc
.L0366f8:
        jsr     PlayerRoute_PublishState_033522(pc) | +0c0
        rts                                     | +0c4

| ----------------------------------------------------------------------------
|  Player_RideSlug_Pose2_0366fe  @ $0366FE  (152 B)
| ----------------------------------------------------------------------------
        .section .text.Player_RideSlug_Pose2_0366fe, "ax", @progbits
        .global Player_RideSlug_Pose2_0366fe
Player_RideSlug_Pose2_0366fe:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        jsr     0x5d5b6.l                       | +01e
        asl.w   #0x1,d0                         | +024
        lea     Sub_000324D0(pc),a0             | +026
        move.w  (a0,d0.w),0x2c(a6)              | +02a
        lea     Sub_000328D8(pc),a0             | +030
        move.l  a0,0x48(a6)                     | +034
        move.b  #0x2,0x70(a6)                   | +038
        lea     0x279f1c.l,a0                   | +03e
        move.l  -0x4(a0),0x74(a6)               | +044
        move.b  #0xff,0x21(a6)                  | +04a
        lea     0x279f1c.l,a0                   | +050
        jsr     0x28cd4.l                       | +056
        clr.b   0x3b(a6)                        | +05c
        lea     .L036764(pc),a1                 | +060
        move.l  a1,(a6)                         | +064
.L036764:
        jsr     Player_FrameCommon_NoPrio_033016(pc) | +066
        jsr     0x2783a.l                       | +06a
        lea     0x100580.l,a0                   | +070
        move.w  0x22(a0),d0                     | +076
        move.w  0x24(a0),d1                     | +07a
        move.w  d0,0x22(a6)                     | +07e
        move.w  d1,0x24(a6)                     | +082
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +086
        bcc.w   .L036792                        | +08a
        lea     Player_RideSlug_0364a2(pc),a1   | +08e
        move.l  a1,(a6)                         | +092
.L036792:
        jmp     Player_RideSlug_Frame_03652e(pc) | +094

| ----------------------------------------------------------------------------
|  Player_SlugJumpOff_036796  @ $036796  (330 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SlugJumpOff_036796, "ax", @progbits
        .global Player_SlugJumpOff_036796
Player_SlugJumpOff_036796:
        bclr    #0x0,0x8c(a6)                   | +000
        move.w  #0x0,0x7c(a6)                   | +006
        move.w  #0x0,0x7e(a6)                   | +00c
        bclr    #0x2,0x8c(a6)                   | +012
        bclr    #0x1,0x8c(a6)                   | +018
        bclr    #0x3,0x8c(a6)                   | +01e
        move.l  #0x32500,0x60(a6)               | +024
        move.b  #0x26,0x70(a6)                  | +02c
        lea     0x279690.l,a0                   | +032
        move.l  -0x4(a0),0x74(a6)               | +038
        move.b  #0xff,0x21(a6)                  | +03e
        lea     0x279690.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        move.w  #0x87f,0x2a(a6)                 | +050
        move.w  #0xff6f,0x2e(a6)                | +056
        lea     Sub_00032884(pc),a0             | +05c
        move.l  a0,0x48(a6)                     | +060
        jsr     0x5d5b6.l                       | +064
        lea     Sub_000324E8(pc),a0             | +06a
        asl.w   #0x1,d0                         | +06e
        move.w  (a0,d0.w),d1                    | +070
        move.w  d1,0x28(a6)                     | +074
        lea     .L036814(pc),a1                 | +078
        move.l  a1,(a6)                         | +07c
.L036814:
        jsr     0x5d5b6.l                       | +07e
        asl.w   #0x1,d0                         | +084
        lea     Sub_000324D8(pc),a0             | +086
        move.w  (a0,d0.w),0x2c(a6)              | +08a
        move.w  0x28(a6),d0                     | +090
        move.w  #0x200,d1                       | +094
        jsr     0x267f4.l                       | +098
        move.w  d0,0x28(a6)                     | +09e
        move.w  0x2a(a6),d0                     | +0a2
        bgt.w   .L03684e                        | +0a6
        move.w  #0x780,d1                       | +0aa
        jsr     0x267f4.l                       | +0ae
        move.w  d0,0x2a(a6)                     | +0b4
.L03684e:
        jsr     Player_FrameCommon_NoPrio_033016(pc) | +0b8
        bset    #0x1,0x8d(a6)                   | +0bc
        jsr     0x27b66.l                       | +0c2
        bcc.w   .L03686e                        | +0c8
        lea     Player_SpawnLand_SetInvuln1E_033ec2(pc),a1 | +0cc
        move.l  a1,(a6)                         | +0d0
        bclr    #0x1,0x8d(a6)                   | +0d2
.L03686e:
        move.w  0x2a(a6),d0                     | +0d8
        bgt.w   .L036892                        | +0dc
        move.b  #0x3,d0                         | +0e0
        and.b   0x69(a6),d0                     | +0e4
        beq.w   .L036892                        | +0e8
        btst    #0x2,0x69(a6)                   | +0ec
        beq.w   .L036892                        | +0f2
        lea     Player_JumpDropEmptyWeapon_0368e0(pc),a1 | +0f6
        move.l  a1,(a6)                         | +0fa
.L036892:
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0fc
        btst    #0x1,0x8d(a6)                   | +100
        beq.w   .L0368da                        | +106
        cmpi.b  #0x3,0x106ece.l                 | +10a
        beq.w   .L0368c0                        | +112
        movea.l #0xffffffff,a0                  | +116
        lea     Sub_000324C6(pc),a0             | +11c
        jsr     0x5dd56.l                       | +120
        bra.w   .L0368d0                        | +126
.L0368c0:
        movea.l #0xffffffff,a0                  | +12a
        lea     Sub_000324BC(pc),a0             | +130
        jsr     0x5dd56.l                       | +134
.L0368d0:
        bcc.w   .L0368da                        | +13a
        lea     Player_DeathPit_037b8e(pc),a1   | +13e
        move.l  a1,(a6)                         | +142
.L0368da:
        jsr     PlayerRoute_PublishState_033522(pc) | +144
        rts                                     | +148

| ----------------------------------------------------------------------------
|  Player_JumpDropEmptyWeapon_0368e0  @ $0368E0  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Player_JumpDropEmptyWeapon_0368e0, "ax", @progbits
        .global Player_JumpDropEmptyWeapon_0368e0
Player_JumpDropEmptyWeapon_0368e0:
        cmpi.w  #0x0,0x82(a6)                   | +000
        bne.w   .L036910                        | +006
        cmpi.b  #0x0,0x71(a6)                   | +00a
        beq.w   .L036910                        | +010
        lea     Sub_00038BE4(pc),a1             | +014
        jsr     0x4ae.l                         | +018
        jsr     0x5dd02.l                       | +01e
        move.b  #0x0,0x71(a6)                   | +024
        move.w  #0xa,0x82(a6)                   | +02a
.L036910:
        bra.w   Player_JumpStart_036914         | +030

| ----------------------------------------------------------------------------
|  Player_JumpStart_036914  @ $036914  (348 B)
| ----------------------------------------------------------------------------
        .section .text.Player_JumpStart_036914, "ax", @progbits
        .global Player_JumpStart_036914
Player_JumpStart_036914:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        move.l  #0x32500,0x60(a6)               | +01e
        tst.w   0x28(a6)                        | +026
        beq.w   .L03696a                        | +02a
        move.b  #0x25,0x70(a6)                  | +02e
        lea     0x279c78.l,a0                   | +034
        move.l  -0x4(a0),0x74(a6)               | +03a
        move.b  #0xff,0x21(a6)                  | +040
        lea     0x279c78.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        bra.w   .L03698e                        | +052
.L03696a:
        move.b  #0x26,0x70(a6)                  | +056
        lea     0x279b2c.l,a0                   | +05c
        move.l  -0x4(a0),0x74(a6)               | +062
        move.b  #0xff,0x21(a6)                  | +068
        lea     0x279b2c.l,a0                   | +06e
        jsr     0x28cd4.l                       | +074
.L03698e:
        jsr     Player_ReadDirNibble_032f3c(pc) | +07a
        cmpi.b  #0x8,0x78(a6)                   | +07e
        bne.w   .L036a16                        | +084
        andi.w  #0xfffe,0x38(a6)                | +088
        move.w  #0x3,0x72(a6)                   | +08e
        cmpi.b  #0x26,0x70(a6)                  | +094
        beq.w   .L0369e6                        | +09a
        move.w  #0x2,0x7c(a6)                   | +09e
        move.w  #0x2,0x7e(a6)                   | +0a4
        move.b  #0x25,0x70(a6)                  | +0aa
        lea     0x279b0e.l,a0                   | +0b0
        move.l  -0x4(a0),0x74(a6)               | +0b6
        move.b  #0xff,0x21(a6)                  | +0bc
        lea     0x279b0e.l,a0                   | +0c2
        jsr     0x28cd4.l                       | +0c8
        bra.w   .L036a16                        | +0ce
.L0369e6:
        move.w  #0x0,0x7c(a6)                   | +0d2
        move.w  #0x7,0x7e(a6)                   | +0d8
        move.b  #0x26,0x70(a6)                  | +0de
        lea     0x279b04.l,a0                   | +0e4
        move.l  -0x4(a0),0x74(a6)               | +0ea
        move.b  #0xff,0x21(a6)                  | +0f0
        lea     0x279b04.l,a0                   | +0f6
        jsr     0x28cd4.l                       | +0fc
.L036a16:
        move.w  #0x9cd,0x2a(a6)                 | +102
        move.w  #0xff3f,0x2e(a6)                | +108
        move.b  #0x5,0x90(a6)                   | +10e
        lea     Sub_00032830(pc),a0             | +114
        move.l  a0,0x48(a6)                     | +118
        lea     .L036a36(pc),a1                 | +11c
        move.l  a1,(a6)                         | +120
.L036a36:
        subi.b  #0x1,0x90(a6)                   | +122
        cmpi.b  #0x4,0x90(a6)                   | +128
        bne.w   .L036a56                        | +12e
        jsr     0x5cd6c.l                       | +132
        bcc.w   .L036a56                        | +138
        lea     Player_JumpAir_036a70(pc),a1    | +13c
        move.l  a1,(a6)                         | +140
.L036a56:
        cmpi.b  #0x0,0x90(a6)                   | +142
        bne.w   Player_JumpAir_036a70           | +148
        move.w  0x2a(a6),d0                     | +14c
        asr.w   #0x2,d0                         | +150
        move.w  d0,0x2a(a6)                     | +152
        lea     Player_JumpAir_036a70(pc),a1    | +156
        move.l  a1,(a6)                         | +15a

| ----------------------------------------------------------------------------
|  Player_JumpAir_036a70  @ $036A70  (540 B)
| ----------------------------------------------------------------------------
        .section .text.Player_JumpAir_036a70, "ax", @progbits
        .global Player_JumpAir_036a70
Player_JumpAir_036a70:
        jsr     0x5cde4.l                       | +000
        bcc.w   .L036aa8                        | +006
        jsr     0x5d5b6.l                       | +00a
        asl.w   #0x1,d0                         | +010
        lea     Sub_000324D0(pc),a0             | +012
        move.w  (a0,d0.w),0x2c(a6)              | +016
        beq.w   .L036aa4                        | +01c
        ble.w   .L036a9e                        | +020
        bclr    #0x0,0x3a(a6)                   | +024
        bra.w   .L036aa4                        | +02a
.L036a9e:
        bset    #0x0,0x3a(a6)                   | +02e
.L036aa4:
        bra.w   .L036aba                        | +034
.L036aa8:
        jsr     0x5d5b6.l                       | +038
        asl.w   #0x1,d0                         | +03e
        lea     Sub_000324D0(pc),a0             | +040
        move.w  (a0,d0.w),0x2c(a6)              | +044
.L036aba:
        cmpi.w  #0x0,0x2c(a6)                   | +04a
        bne.w   .L036ae8                        | +050
        cmpi.w  #0x0,0x28(a6)                   | +054
        beq.w   .L036ae8                        | +05a
        cmpi.w  #0x0,0x28(a6)                   | +05e
        bmi.w   .L036ae2                        | +064
        move.w  #0xffe8,0x2c(a6)                | +068
        bra.w   .L036ae8                        | +06e
.L036ae2:
        move.w  #0x18,0x2c(a6)                  | +072
.L036ae8:
        move.w  0x28(a6),d0                     | +078
        move.w  #0x400,d1                       | +07c
        move.w  #0x300,d1                       | +080
        jsr     0x267f4.l                       | +084
        move.w  d0,0x28(a6)                     | +08a
        move.w  0x2a(a6),d0                     | +08e
        bgt.w   .L036b14                        | +092
        move.w  #0x780,d1                       | +096
        jsr     0x267f4.l                       | +09a
        move.w  d0,0x2a(a6)                     | +0a0
.L036b14:
        jsr     Player_FrameCommon_NoPrio_033016(pc) | +0a4
        bset    #0x1,0x8d(a6)                   | +0a8
        bclr    #0x5,0x8d(a6)                   | +0ae
        btst    #0x5,0x100001.l                 | +0b4
        beq.w   .L036b54                        | +0bc
        move.w  0x2e(a6),d0                     | +0c0
        movem.w d0,-(a7)                        | +0c4
        move.w  0x28(a6),d0                     | +0c8
        movem.w d0,-(a7)                        | +0cc
        move.w  0x2a(a6),d0                     | +0d0
        movem.w d0,-(a7)                        | +0d4
        clr.w   0x28(a6)                        | +0d8
        clr.w   0x2a(a6)                        | +0dc
        clr.w   0x2e(a6)                        | +0e0
.L036b54:
        jsr     0x27b66.l                       | +0e4
        bcc.w   .L036b70                        | +0ea
        lea     Player_SpawnLand_Reset_033ede(pc),a1 | +0ee
        move.l  a1,(a6)                         | +0f2
        bclr    #0x1,0x8d(a6)                   | +0f4
        bclr    #0x5,0x8d(a6)                   | +0fa
.L036b70:
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +100
        bcc.w   .L036b7e                        | +104
        bclr    #0x1,0x8c(a6)                   | +108
.L036b7e:
        jsr     PlayerAnimState_03705A(pc)      | +10e
        btst    #0x5,0x100001.l                 | +112
        beq.w   .L036ba6                        | +11a
        movem.w (a7)+,d0                        | +11e
        move.w  d0,0x2a(a6)                     | +122
        movem.w (a7)+,d0                        | +126
        move.w  d0,0x28(a6)                     | +12a
        movem.w (a7)+,d0                        | +12e
        move.w  d0,0x2e(a6)                     | +132
.L036ba6:
        move.w  0x2a(a6),d0                     | +136
        bgt.w   .L036c36                        | +13a
        bset    #0x5,0x8d(a6)                   | +13e
        btst    #0x0,0x8c(a6)                   | +144
        beq.w   .L036bc8                        | +14a
        lea     Player_RideSlug_0364a2(pc),a1   | +14e
        move.l  a1,(a6)                         | +152
        jmp     Player_Air_Tail_036c3a(pc) | +154
.L036bc8:
        btst    #0x1,0x8d(a6)                   | +158
        beq.w   .L036bfe                        | +15e
        jsr     0x8f470.l                       | +162
        bcc.w   .L036bfe                        | +168
        cmpi.w  #0x0,d2                         | +16c
        bmi.w   .L036bfe                        | +170
        move.b  d2,0x86(a6)                     | +174
        lea     Player_HangRing_036638(pc),a1   | +178
        move.l  a1,(a6)                         | +17c
        jsr     0x8f520.l                       | +17e
        bclr    #0x5,0x8d(a6)                   | +184
        jmp     Player_Air_Tail_036c3a(pc) | +18a
.L036bfe:
        move.b  #0x3,d0                         | +18e
        and.b   0x69(a6),d0                     | +192
        beq.w   .L036c36                        | +196
        btst    #0x2,0x69(a6)                   | +19a
        beq.w   .L036c36                        | +1a0
        lea     Player_JumpDropEmptyWeapon_0368e0(pc),a1 | +1a4
        move.l  a1,(a6)                         | +1a8
        move.w  #0x400,0x28(a6)                 | +1aa
        clr.w   0x2c(a6)                        | +1b0
        btst    #0x1,0x69(a6)                   | +1b4
        bne.w   .L036c32                        | +1ba
        neg.w   0x28(a6)                        | +1be
.L036c32:
        jmp     Player_Air_Tail_036c3a(pc) | +1c2
.L036c36:
        jsr     Player_AirActionSelect_037168(pc) | +1c6
        .global Player_Air_Tail_036c3a
Player_Air_Tail_036c3a:
        btst    #0x1,0x8d(a6)                   | +1ca
        beq.w   .L036c86                        | +1d0
        cmpi.b  #0x3,0x106ece.l                 | +1d4
        beq.w   .L036c64                        | +1dc
        movea.l #0xffffffff,a0                  | +1e0
        lea     Sub_000324C6(pc),a0             | +1e6
        jsr     0x5dd56.l                       | +1ea
        bra.w   .L036c74                        | +1f0
.L036c64:
        movea.l #0xffffffff,a0                  | +1f4
        lea     Sub_000324BC(pc),a0             | +1fa
        jsr     0x5dd56.l                       | +1fe
.L036c74:
        bcc.w   .L036c7e                        | +204
        lea     Player_DeathPit_037b8e(pc),a1   | +208
        move.l  a1,(a6)                         | +20c
.L036c7e:
        jsr     PlayerRoute_PublishState_033522(pc) | +20e
        bra.w   .L036c8a                        | +212
.L036c86:
        jsr     PlayerRoute_PublishState_033522(pc) | +216
.L036c8a:
        rts                                     | +21a

| ----------------------------------------------------------------------------
|  Player_SpawnFreeFall_036c8c  @ $036C8C  (216 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnFreeFall_036c8c, "ax", @progbits
        .global Player_SpawnFreeFall_036c8c
Player_SpawnFreeFall_036c8c:
        cmpa.l  #0x100440,a6                    | +000
        bne.w   .L036ca6                        | +006
        lea     0x32112.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        bra.w   .L036cb2                        | +016
.L036ca6:
        lea     0x32142.l,a1                    | +01a
        jsr     0x4ae.l                         | +020
.L036cb2:
        clr.w   0x28(a6)                        | +026
        clr.w   0x2c(a6)                        | +02a
        move.w  #0x0,0x7c(a6)                   | +02e
        move.w  #0x0,0x7e(a6)                   | +034
        bclr    #0x2,0x8c(a6)                   | +03a
        bclr    #0x1,0x8c(a6)                   | +040
        bclr    #0x3,0x8c(a6)                   | +046
        move.l  #0x32500,0x60(a6)               | +04c
        move.b  #0x26,0x70(a6)                  | +054
        lea     0x279b2c.l,a0                   | +05a
        move.l  -0x4(a0),0x74(a6)               | +060
        move.b  #0xff,0x21(a6)                  | +066
        lea     0x279b2c.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
        move.w  #0xff80,0x2e(a6)                | +078
        lea     0xffff.w,a0                     | +07e
        move.l  a0,0x48(a6)                     | +082
        clr.b   0x59(a6)                        | +086
        clr.b   0x45(a6)                        | +08a
        lea     .L036d20(pc),a1                 | +08e
        move.l  a1,(a6)                         | +092
.L036d20:
        ori.b   #0x3c,0x59(a6)                  | +094
        ori.b   #0x3c,0x45(a6)                  | +09a
        bset    #0x6,0x13(a6)                   | +0a0
        jsr     Player_Fall_Physics_036e42(pc)  | +0a6
        lea     Player_SpawnLand_Reset_033ede(pc),a0 | +0aa
        cmpa.l  (a6),a0                         | +0ae
        bne.w   .L036d4c                        | +0b0
        move.b  #0x50,0x45(a6)                  | +0b4
        move.b  #0x50,0x59(a6)                  | +0ba
.L036d4c:
        lea     Player_JumpDropEmptyWeapon_0368e0(pc),a0 | +0c0
        cmpa.l  (a6),a0                         | +0c4
        bne.w   .L036d62                        | +0c6
        move.b  #0x50,0x45(a6)                  | +0ca
        move.b  #0x50,0x59(a6)                  | +0d0
.L036d62:
        rts                                     | +0d6

| ----------------------------------------------------------------------------
|  Player_Knockback_036d64  @ $036D64  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Knockback_036d64, "ax", @progbits
        .global Player_Knockback_036d64
Player_Knockback_036d64:
        clr.w   0x28(a6)                        | +000
        clr.w   0x2c(a6)                        | +004
        move.w  #0xff80,0x2e(a6)                | +008
        .global Player_Knockback_Setup_036d72
Player_Knockback_Setup_036d72:
        move.w  #0x0,0x7c(a6)                   | +00e
        move.w  #0x0,0x7e(a6)                   | +014
        bclr    #0x2,0x8c(a6)                   | +01a
        bclr    #0x1,0x8c(a6)                   | +020
        bclr    #0x3,0x8c(a6)                   | +026
        move.l  #0x32500,0x60(a6)               | +02c
        move.b  #0x26,0x70(a6)                  | +034
        lea     0x279b2c.l,a0                   | +03a
        move.l  -0x4(a0),0x74(a6)               | +040
        move.b  #0xff,0x21(a6)                  | +046
        lea     0x279b2c.l,a0                   | +04c
        jsr     0x28cd4.l                       | +052
        lea     Sub_00032830(pc),a0             | +058
        move.l  a0,0x48(a6)                     | +05c
        lea     Player_Knockback_AirCtrl_036dca(pc),a1 | +060
        move.l  a1,(a6)                         | +064

| ----------------------------------------------------------------------------
|  Player_Knockback_AirCtrl_036dca  @ $036DCA  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Knockback_AirCtrl_036dca, "ax", @progbits
        .global Player_Knockback_AirCtrl_036dca
Player_Knockback_AirCtrl_036dca:
        jsr     0x5cde4.l                       | +000
        bcc.w   .L036e02                        | +006
        jsr     0x5d5b6.l                       | +00a
        asl.w   #0x1,d0                         | +010
        lea     Sub_000324D0(pc),a0             | +012
        move.w  (a0,d0.w),0x2c(a6)              | +016
        beq.w   .L036dfe                        | +01c
        ble.w   .L036df8                        | +020
        bclr    #0x0,0x3a(a6)                   | +024
        bra.w   .L036dfe                        | +02a
.L036df8:
        bset    #0x0,0x3a(a6)                   | +02e
.L036dfe:
        bra.w   .L036e14                        | +034
.L036e02:
        jsr     0x5d5b6.l                       | +038
        asl.w   #0x1,d0                         | +03e
        lea     Sub_000324D0(pc),a0             | +040
        move.w  (a0,d0.w),0x2c(a6)              | +044
.L036e14:
        cmpi.w  #0x0,0x2c(a6)                   | +04a
        bne.w   Player_Fall_Physics_036e42      | +050
        cmpi.w  #0x0,0x28(a6)                   | +054
        beq.w   Player_Fall_Physics_036e42      | +05a
        cmpi.w  #0x0,0x28(a6)                   | +05e
        bmi.w   .L036e3c                        | +064
        move.w  #0xffe8,0x2c(a6)                | +068
        bra.w   Player_Fall_Physics_036e42      | +06e
.L036e3c:
        move.w  #0x18,0x2c(a6)                  | +072

| ----------------------------------------------------------------------------
|  Player_Fall_Physics_036e42  @ $036E42  (384 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Fall_Physics_036e42, "ax", @progbits
        .global Player_Fall_Physics_036e42
Player_Fall_Physics_036e42:
        move.w  0x28(a6),d0                     | +000
        move.w  #0x400,d1                       | +004
        move.w  #0x300,d1                       | +008
        jsr     0x267f4.l                       | +00c
        move.w  d0,0x28(a6)                     | +012
        move.w  0x2a(a6),d0                     | +016
        bgt.w   .L036e72                        | +01a
        move.w  0x2a(a6),d0                     | +01e
        move.w  #0x400,d1                       | +022
        jsr     0x267f4.l                       | +026
        move.w  d0,0x2a(a6)                     | +02c
.L036e72:
        bset    #0x1,0x8d(a6)                   | +030
        bclr    #0x5,0x8d(a6)                   | +036
        jsr     Player_FrameCommon_NoPrio_033016(pc) | +03c
        jsr     0x27bc8.l                       | +040
        bcc.w   .L036e9e                        | +046
        lea     Player_SpawnLand_Reset_033ede(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
        bclr    #0x1,0x8d(a6)                   | +050
        bclr    #0x5,0x8d(a6)                   | +056
.L036e9e:
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +05c
        bcc.w   .L036eac                        | +060
        bclr    #0x1,0x8c(a6)                   | +064
.L036eac:
        cmpi.l  #0x36ff6,(a6)                   | +06a
        beq.b   .L036ebe                        | +070
        cmpi.l  #0x37046,(a6)                   | +072
        bne.w   .L036ec2                        | +078
.L036ebe:
        bra.w   .L036ec6                        | +07c
.L036ec2:
        jsr     PlayerAnimState_03705A(pc)      | +080
.L036ec6:
        move.w  0x2a(a6),d0                     | +084
        bgt.w   .L036f56                        | +088
        bset    #0x5,0x8d(a6)                   | +08c
        btst    #0x0,0x8c(a6)                   | +092
        beq.w   .L036ee8                        | +098
        lea     Player_RideSlug_0364a2(pc),a1   | +09c
        move.l  a1,(a6)                         | +0a0
        jmp     Player_Air_Tail_036c3a(pc) | +0a2
.L036ee8:
        btst    #0x1,0x8d(a6)                   | +0a6
        beq.w   .L036f1e                        | +0ac
        jsr     0x8f470.l                       | +0b0
        bcc.w   .L036f1e                        | +0b6
        cmpi.w  #0x0,d2                         | +0ba
        bmi.w   .L036f1e                        | +0be
        move.b  d2,0x86(a6)                     | +0c2
        lea     Player_HangRing_036638(pc),a1   | +0c6
        move.l  a1,(a6)                         | +0ca
        jsr     0x8f520.l                       | +0cc
        bclr    #0x5,0x8d(a6)                   | +0d2
        jmp     .L036f70(pc)                    | +0d8
.L036f1e:
        move.b  #0x3,d0                         | +0dc
        and.b   0x69(a6),d0                     | +0e0
        beq.w   .L036f56                        | +0e4
        btst    #0x2,0x69(a6)                   | +0e8
        beq.w   .L036f56                        | +0ee
        lea     Player_JumpDropEmptyWeapon_0368e0(pc),a1 | +0f2
        move.l  a1,(a6)                         | +0f6
        move.w  #0x400,0x28(a6)                 | +0f8
        clr.w   0x2c(a6)                        | +0fe
        btst    #0x1,0x69(a6)                   | +102
        bne.w   .L036f52                        | +108
        neg.w   0x28(a6)                        | +10c
.L036f52:
        jmp     .L036f70(pc)                    | +110
.L036f56:
        cmpi.l  #0x36ff6,(a6)                   | +114
        beq.b   .L036f68                        | +11a
        cmpi.l  #0x37046,(a6)                   | +11c
        bne.w   .L036f6c                        | +122
.L036f68:
        bra.w   .L036f70                        | +126
.L036f6c:
        jsr     Player_AirActionSelect_037168(pc) | +12a
.L036f70:
        btst    #0x1,0x8d(a6)                   | +12e
        beq.w   .L036fbc                        | +134
        cmpi.b  #0x3,0x106ece.l                 | +138
        beq.w   .L036f9a                        | +140
        movea.l #0xffffffff,a0                  | +144
        lea     Sub_000324C6(pc),a0             | +14a
        jsr     0x5dd56.l                       | +14e
        bra.w   .L036faa                        | +154
.L036f9a:
        movea.l #0xffffffff,a0                  | +158
        lea     Sub_000324BC(pc),a0             | +15e
        jsr     0x5dd56.l                       | +162
.L036faa:
        bcc.w   .L036fb4                        | +168
        lea     Player_DeathPit_037b8e(pc),a1   | +16c
        move.l  a1,(a6)                         | +170
.L036fb4:
        jsr     PlayerRoute_PublishState_033522(pc) | +172
        bra.w   .L036fc0                        | +176
.L036fbc:
        jsr     PlayerRoute_PublishState_033522(pc) | +17a
.L036fc0:
        rts                                     | +17e

| ----------------------------------------------------------------------------
|  Player_KnockbackDelay_036fc2  @ $036FC2  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Player_KnockbackDelay_036fc2, "ax", @progbits
        .global Player_KnockbackDelay_036fc2
Player_KnockbackDelay_036fc2:
        clr.w   0x28(a6)                        | +000
        clr.w   0x2c(a6)                        | +004
        bclr    #0x2,0x8c(a6)                   | +008
        bclr    #0x1,0x8c(a6)                   | +00e
        bclr    #0x3,0x8c(a6)                   | +014
        move.l  #0x32500,0x60(a6)               | +01a
        move.w  #0xff80,0x2e(a6)                | +022
        move.b  #0xc,0x46(a6)                   | +028
        lea     .L036ff6(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L036ff6:
        cmpi.b  #0x0,0x46(a6)                   | +034
        bgt.w   .L037004                        | +03a
        jmp     Player_Knockback_Setup_036d72(pc) | +03e
.L037004:
        jsr     0x5cde4.l                       | +042
        bcc.w   .L037012                        | +048
        jmp     Player_Knockback_Setup_036d72(pc) | +04c
.L037012:
        jsr     Player_Knockback_AirCtrl_036dca(pc) | +050
        rts                                     | +054

| ----------------------------------------------------------------------------
|  Player_KnockbackHold_037018  @ $037018  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Player_KnockbackHold_037018, "ax", @progbits
        .global Player_KnockbackHold_037018
Player_KnockbackHold_037018:
        clr.w   0x28(a6)                        | +000
        clr.w   0x2c(a6)                        | +004
        bclr    #0x2,0x8c(a6)                   | +008
        bclr    #0x1,0x8c(a6)                   | +00e
        bclr    #0x3,0x8c(a6)                   | +014
        move.l  #0x32500,0x60(a6)               | +01a
        move.w  #0xff80,0x2e(a6)                | +022
        lea     .L037046(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L037046:
        jsr     0x5cde4.l                       | +02e
        bcc.w   .L037054                        | +034
        jmp     Player_Knockback_Setup_036d72(pc) | +038
.L037054:
        jsr     Player_Knockback_AirCtrl_036dca(pc) | +03c
        rts                                     | +040

| ----------------------------------------------------------------------------
|  Player_AirActionSelect_037168  @ $037168  (1130 B)
| ----------------------------------------------------------------------------
        .section .text.Player_AirActionSelect_037168, "ax", @progbits
        .global Player_AirActionSelect_037168
Player_AirActionSelect_037168:
        jsr     Player_ActionSelect_0330d0(pc)  | +000
        bcc.w   .L0375d0                        | +004
        bset    #0x2,0x8c(a6)                   | +008
        cmpi.b  #0xff,d1                        | +00e
        bne.w   .L0371d8                        | +012
        bset    #0x5,0x69(a6)                   | +016
        cmpi.b  #0x26,0x70(a6)                  | +01c
        beq.w   .L0371b0                        | +022
        move.w  #0x0,0x7c(a6)                   | +026
        move.w  #0x2,0x7e(a6)                   | +02c
        move.b  #0x25,0x70(a6)                  | +032
        lea     0x279d56.l,a0                   | +038
        move.l  -0x4(a0),0x74(a6)               | +03e
        bra.w   .L0371ce                        | +044
.L0371b0:
        move.w  #0x0,0x7c(a6)                   | +048
        move.w  #0x7,0x7e(a6)                   | +04e
        move.b  #0x26,0x70(a6)                  | +054
        lea     0x279d56.l,a0                   | +05a
        move.l  -0x4(a0),0x74(a6)               | +060
.L0371ce:
        move.w  #0x0,0x72(a6)                   | +066
        bra.w   .L0375d0                        | +06c
.L0371d8:
        bset    #0x5,0x69(a6)                   | +070
        cmpi.b  #0x3,d1                         | +076
        bne.w   .L037328                        | +07a
        cmpi.b  #0x26,0x70(a6)                  | +07e
        beq.w   .L03729e                        | +084
        move.w  #0x0,0x7c(a6)                   | +088
        move.w  #0x0,0x7e(a6)                   | +08e
        jsr     0x5d5b6.l                       | +094
        btst    #0x0,0x3a(a6)                   | +09a
        bne.w   .L037214                        | +0a0
        move.b  #0x1,d2                         | +0a4
        bra.w   .L037218                        | +0a8
.L037214:
        move.b  #0x2,d2                         | +0ac
.L037218:
        cmpi.b  #0x0,d0                         | +0b0
        bne.w   .L03723c                        | +0b4
        cmpi.b  #0x1,0x78(a6)                   | +0b8
        bne.w   .L03722e                        | +0be
        move.b  #0xff,d2                        | +0c2
.L03722e:
        cmpi.b  #0x2,0x78(a6)                   | +0c6
        bne.w   .L03723c                        | +0cc
        move.b  #0x0,d2                         | +0d0
.L03723c:
        btst    #0x2,0x100001.l                 | +0d4
        bne.w   .L037248                        | +0dc
.L037248:
        cmp.b   d0,d2                           | +0e0
        bne.w   .L037276                        | +0e2
        move.w  #0x3,0x7c(a6)                   | +0e6
        move.w  #0xfffa,0x7e(a6)                | +0ec
        move.b  #0x25,0x70(a6)                  | +0f2
        lea     0x279d4c.l,a0                   | +0f8
        move.l  -0x4(a0),0x74(a6)               | +0fe
        move.w  #0x2,0x72(a6)                   | +104
        bra.w   .L03729a                        | +10a
.L037276:
        move.w  #0x0,0x7c(a6)                   | +10e
        move.w  #0x2,0x7e(a6)                   | +114
        move.b  #0x25,0x70(a6)                  | +11a
        lea     0x279d42.l,a0                   | +120
        move.l  -0x4(a0),0x74(a6)               | +126
        move.w  #0x0,0x72(a6)                   | +12c
.L03729a:
        bra.w   .L037324                        | +132
.L03729e:
        move.w  #0x6,0x7c(a6)                   | +136
        move.w  #0x3,0x7e(a6)                   | +13c
        jsr     0x5d5b6.l                       | +142
        btst    #0x0,0x3a(a6)                   | +148
        bne.w   .L0372c2                        | +14e
        move.b  #0x1,d2                         | +152
        bra.w   .L0372c6                        | +156
.L0372c2:
        move.b  #0x2,d2                         | +15a
.L0372c6:
        btst    #0x2,0x100001.l                 | +15e
        bne.w   .L0372d2                        | +166
.L0372d2:
        cmp.b   d0,d2                           | +16a
        bne.w   .L037300                        | +16c
        move.w  #0x0,0x7c(a6)                   | +170
        move.w  #0x0,0x7e(a6)                   | +176
        move.b  #0x26,0x70(a6)                  | +17c
        lea     0x279d4c.l,a0                   | +182
        move.l  -0x4(a0),0x74(a6)               | +188
        move.w  #0x2,0x72(a6)                   | +18e
        bra.w   .L037324                        | +194
.L037300:
        move.w  #0x0,0x7c(a6)                   | +198
        move.w  #0x7,0x7e(a6)                   | +19e
        move.b  #0x26,0x70(a6)                  | +1a4
        lea     0x279d42.l,a0                   | +1aa
        move.l  -0x4(a0),0x74(a6)               | +1b0
        move.w  #0x0,0x72(a6)                   | +1b6
.L037324:
        bra.w   .L0375d0                        | +1bc
.L037328:
        cmpi.b  #0x4,d1                         | +1c0
        bne.w   .L037390                        | +1c4
        andi.w  #0xfffe,0x38(a6)                | +1c8
        cmpi.b  #0x26,0x70(a6)                  | +1ce
        beq.w   .L037362                        | +1d4
        move.w  #0x2,0x7c(a6)                   | +1d8
        move.w  #0x2,0x7e(a6)                   | +1de
        move.b  #0x25,0x70(a6)                  | +1e4
        lea     0x279c6e.l,a0                   | +1ea
        move.l  -0x4(a0),0x74(a6)               | +1f0
        bra.w   .L037380                        | +1f6
.L037362:
        move.w  #0x0,0x7c(a6)                   | +1fa
        move.w  #0x7,0x7e(a6)                   | +200
        move.b  #0x26,0x70(a6)                  | +206
        lea     0x279afa.l,a0                   | +20c
        move.l  -0x4(a0),0x74(a6)               | +212
.L037380:
        move.w  #0x3,0x72(a6)                   | +218
        bset    #0x1,0x8c(a6)                   | +21e
        bra.w   .L0375d0                        | +224
.L037390:
        cmpi.b  #0x1,d1                         | +228
        bne.w   .L0373f2                        | +22c
        cmpi.b  #0x26,0x70(a6)                  | +230
        beq.w   .L0373c4                        | +236
        move.w  #0x0,0x7c(a6)                   | +23a
        move.w  #0x2,0x7e(a6)                   | +240
        move.b  #0x25,0x70(a6)                  | +246
        lea     0x279c3c.l,a0                   | +24c
        move.l  -0x4(a0),0x74(a6)               | +252
        bra.w   .L0373e2                        | +258
.L0373c4:
        move.w  #0x0,0x7c(a6)                   | +25c
        move.w  #0x6,0x7e(a6)                   | +262
        move.b  #0x26,0x70(a6)                  | +268
        lea     0x279ad2.l,a0                   | +26e
        move.l  -0x4(a0),0x74(a6)               | +274
.L0373e2:
        move.w  #0x1,0x72(a6)                   | +27a
        bset    #0x1,0x8c(a6)                   | +280
        bra.w   .L0375d0                        | +286
.L0373f2:
        btst    #0x2,0x100001.l                 | +28a
        bne.w   .L0373fe                        | +292
.L0373fe:
        cmpi.b  #0x2,d1                         | +296
        bne.w   .L03746c                        | +29a
        cmpi.b  #0x26,0x70(a6)                  | +29e
        beq.w   .L037438                        | +2a4
        move.w  #0x6,0x7c(a6)                   | +2a8
        move.w  #0xfffd,0x7e(a6)                | +2ae
        move.b  #0x25,0x70(a6)                  | +2b4
        lea     0x279c32.l,a0                   | +2ba
        move.l  -0x4(a0),0x74(a6)               | +2c0
        move.w  #0x2,0x72(a6)                   | +2c6
        bra.w   .L03745c                        | +2cc
.L037438:
        move.w  #0x0,0x7c(a6)                   | +2d0
        move.w  #0x0,0x7e(a6)                   | +2d6
        move.b  #0x26,0x70(a6)                  | +2dc
        lea     0x279ac8.l,a0                   | +2e2
        move.l  -0x4(a0),0x74(a6)               | +2e8
        move.w  #0x2,0x72(a6)                   | +2ee
.L03745c:
        move.w  #0x2,0x72(a6)                   | +2f4
        bset    #0x1,0x8c(a6)                   | +2fa
        bra.w   .L0375d0                        | +300
.L03746c:
        cmpi.b  #0x26,0x70(a6)                  | +304
        beq.w   .L037528                        | +30a
        jsr     0x5d5b6.l                       | +30e
        btst    #0x0,0x3a(a6)                   | +314
        bne.w   .L03748e                        | +31a
        move.b  #0x1,d2                         | +31e
        bra.w   .L037492                        | +322
.L03748e:
        move.b  #0x2,d2                         | +326
.L037492:
        cmpi.b  #0x0,d0                         | +32a
        bne.w   .L0374b6                        | +32e
        cmpi.b  #0x1,0x78(a6)                   | +332
        bne.w   .L0374a8                        | +338
        move.b  #0xff,d2                        | +33c
.L0374a8:
        cmpi.b  #0x2,0x78(a6)                   | +340
        bne.w   .L0374b6                        | +346
        move.b  #0x0,d2                         | +34a
.L0374b6:
        btst    #0x2,0x100001.l                 | +34e
        bne.w   .L0374c2                        | +356
.L0374c2:
        cmp.b   d0,d2                           | +35a
        bne.w   .L0374f0                        | +35c
        move.w  #0x6,0x7c(a6)                   | +360
        move.w  #0xfffd,0x7e(a6)                | +366
        move.b  #0x25,0x70(a6)                  | +36c
        lea     0x279c32.l,a0                   | +372
        move.l  -0x4(a0),0x74(a6)               | +378
        move.w  #0x2,0x72(a6)                   | +37e
        bra.w   .L037524                        | +384
.L0374f0:
        move.w  #0x0,0x7c(a6)                   | +388
        move.w  #0x2,0x7e(a6)                   | +38e
        move.b  #0x25,0x70(a6)                  | +394
        lea     0x279c28.l,a0                   | +39a
        move.l  -0x4(a0),0x74(a6)               | +3a0
        cmpi.b  #0x0,0x71(a6)                   | +3a6
        bne.w   .L03751e                        | +3ac
        ori.w   #0x1,0x38(a6)                   | +3b0
.L03751e:
        move.w  #0x0,0x72(a6)                   | +3b6
.L037524:
        bra.w   .L0375c6                        | +3bc
.L037528:
        jsr     0x5d5b6.l                       | +3c0
        btst    #0x0,0x3a(a6)                   | +3c6
        bne.w   .L037540                        | +3cc
        move.b  #0x1,d2                         | +3d0
        bra.w   .L037544                        | +3d4
.L037540:
        move.b  #0x2,d2                         | +3d8
.L037544:
        cmpi.b  #0x0,d0                         | +3dc
        bne.w   .L037568                        | +3e0
        cmpi.b  #0x1,0x78(a6)                   | +3e4
        bne.w   .L03755a                        | +3ea
        move.b  #0xff,d2                        | +3ee
.L03755a:
        cmpi.b  #0x2,0x78(a6)                   | +3f2
        bne.w   .L037568                        | +3f8
        move.b  #0x0,d2                         | +3fc
.L037568:
        btst    #0x2,0x100001.l                 | +400
        bne.w   .L037574                        | +408
.L037574:
        cmp.b   d0,d2                           | +40c
        bne.w   .L0375a2                        | +40e
        move.w  #0x0,0x7c(a6)                   | +412
        move.w  #0x0,0x7e(a6)                   | +418
        move.b  #0x26,0x70(a6)                  | +41e
        lea     0x279ac8.l,a0                   | +424
        move.l  -0x4(a0),0x74(a6)               | +42a
        move.w  #0x2,0x72(a6)                   | +430
        bra.w   .L0375c6                        | +436
.L0375a2:
        move.w  #0x0,0x7c(a6)                   | +43a
        move.w  #0x7,0x7e(a6)                   | +440
        move.b  #0x26,0x70(a6)                  | +446
        lea     0x279abe.l,a0                   | +44c
        move.l  -0x4(a0),0x74(a6)               | +452
        move.w  #0x0,0x72(a6)                   | +458
.L0375c6:
        bset    #0x1,0x8c(a6)                   | +45e
        bra.w   .L0375d0                        | +464
.L0375d0:
        rts                                     | +468

| ----------------------------------------------------------------------------
|  Player_Death_0375d2  @ $0375D2  (136 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_0375d2, "ax", @progbits
        .global Player_Death_0375d2
Player_Death_0375d2:
        move.b  #0xff,0x6c(a6)                  | +000
        lea     0x78840.l,a1                    | +006
        jsr     0x4ae.l                         | +00c
        jsr     0x5dd02.l                       | +012
        move.w  #0x1053,d0                      | +018
        jsr     0x2352.l                        | +01c
        move.b  #0x0,0x70(a6)                   | +022
        lea     0x279f76.l,a0                   | +028
        move.l  -0x4(a0),0x74(a6)               | +02e
        move.b  #0xff,0x21(a6)                  | +034
        lea     0x279f76.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        lea     Player_Death_PlayMusic_03765a(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
        .global Player_Death_GroundTest_03761e
Player_Death_GroundTest_03761e:
        cmpi.b  #0x3,0x106ece.l                 | +04c
        beq.w   .L03763e                        | +054
        movea.l #0xffffffff,a0                  | +058
        lea     Sub_000324C6(pc),a0             | +05e
        jsr     0x5dd56.l                       | +062
        bra.w   .L03764e                        | +068
.L03763e:
        movea.l #0xffffffff,a0                  | +06c
        lea     Sub_000324BC(pc),a0             | +072
        jsr     0x5dd56.l                       | +076
.L03764e:
        bcc.w   .L037658                        | +07c
        lea     Player_DeathPit_037b8e(pc),a1   | +080
        move.l  a1,(a6)                         | +084
.L037658:
        rts                                     | +086

| ----------------------------------------------------------------------------
|  Player_Death_PlayMusic_03765a  @ $03765A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_PlayMusic_03765a, "ax", @progbits
        .global Player_Death_PlayMusic_03765a
Player_Death_PlayMusic_03765a:
        lea     Player_Death_Despawn_037c1a(pc),a1 | +000
        move.l  a1,(a6)                         | +004
        cmpa.l  #0x100440,a6                    | +006
        bne.w   .L037678                        | +00c
        move.w  #0x1123,d0                      | +010
        jsr     0x2352.l                        | +014
        bra.w   .L037682                        | +01a
.L037678:
        move.w  #0x1087,d0                      | +01e
        jsr     0x2352.l                        | +022
.L037682:
        bra.b   Player_Death_GroundTest_03761e    | +028

| ----------------------------------------------------------------------------
|  Player_Death_Alt_037684  @ $037684  (244 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_Alt_037684, "ax", @progbits
        .global Player_Death_Alt_037684
Player_Death_Alt_037684:
        jsr     0x5e9b6.l                       | +000
        andi.b  #0xb,d0                         | +006
        bne.w   Player_Death_0375d2             | +00a
        move.b  #0xff,0x6c(a6)                  | +00e
        lea     Sub_00039214(pc),a1             | +014
        jsr     0x4ae.l                         | +018
        jsr     0x5dd02.l                       | +01e
        lea     0x78840.l,a1                    | +024
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd02.l                       | +030
        move.w  #0xd000,0x38(a0)                | +036
        addi.w  #0x10,0x24(a0)                  | +03c
        move.w  #0x1053,d0                      | +042
        jsr     0x2352.l                        | +046
        move.w  #0x0,0x7c(a6)                   | +04c
        move.w  #0x0,0x7e(a6)                   | +052
        bclr    #0x2,0x8c(a6)                   | +058
        bclr    #0x1,0x8c(a6)                   | +05e
        bclr    #0x3,0x8c(a6)                   | +064
        bset    #0x0,0x13(a6)                   | +06a
        move.b  #0x0,0x70(a6)                   | +070
        lea     0x279f80.l,a0                   | +076
        move.l  -0x4(a0),0x74(a6)               | +07c
        move.b  #0xff,0x21(a6)                  | +082
        lea     0x279f80.l,a0                   | +088
        jsr     0x28cd4.l                       | +08e
        jsr     0x267e2.l                       | +094
        lea     .L037724(pc),a1                 | +09a
        move.l  a1,(a6)                         | +09e
.L037724:
        jsr     Player_FrameCommon_032ff2(pc)   | +0a0
        jsr     0x2783a.l                       | +0a4
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0aa
        bcc.w   .L03773c                        | +0ae
        lea     Player_Death_Despawn_037c1a(pc),a1 | +0b2
        move.l  a1,(a6)                         | +0b6
.L03773c:
        cmpi.b  #0x3,0x106ece.l                 | +0b8
        beq.w   .L03775c                        | +0c0
        movea.l #0xffffffff,a0                  | +0c4
        lea     Sub_000324C6(pc),a0             | +0ca
        jsr     0x5dd56.l                       | +0ce
        bra.w   .L03776c                        | +0d4
.L03775c:
        movea.l #0xffffffff,a0                  | +0d8
        lea     Sub_000324BC(pc),a0             | +0de
        jsr     0x5dd56.l                       | +0e2
.L03776c:
        bcc.w   .L037776                        | +0e8
        lea     Player_DeathPit_037b8e(pc),a1   | +0ec
        move.l  a1,(a6)                         | +0f0
.L037776:
        rts                                     | +0f2

| ----------------------------------------------------------------------------
|  Player_Death_Fall_037778  @ $037778  (252 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_Fall_037778, "ax", @progbits
        .global Player_Death_Fall_037778
Player_Death_Fall_037778:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        bset    #0x0,0x13(a6)                   | +01e
        move.b  #0x0,0x70(a6)                   | +024
        lea     0x279f44.l,a0                   | +02a
        move.l  -0x4(a0),0x74(a6)               | +030
        move.b  #0xff,0x21(a6)                  | +036
        lea     0x279f44.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        jsr     0x267e2.l                       | +048
        move.w  #0xfee9,d0                      | +04e
        jsr     0x5dca4.l                       | +052
        move.w  d0,0x28(a6)                     | +058
        move.w  #0x457,0x2a(a6)                 | +05c
        move.w  #0xff9b,0x2e(a6)                | +062
        move.w  #0x0,0x2c(a6)                   | +068
        lea     .L0377ec(pc),a1                 | +06e
        move.l  a1,(a6)                         | +072
.L0377ec:
        bset    #0x1,0x8d(a6)                   | +074
        jsr     Player_FrameCommon_032ff2(pc)   | +07a
        jsr     0x27bc8.l                       | +07e
        bcc.w   .L037806                        | +084
        lea     Player_Death_FallLanded_037874(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
.L037806:
        cmpi.b  #0x1,0x106ece.l                 | +08e
        bne.w   .L037834                        | +096
        move.w  0x22(a6),d1                     | +09a
        move.w  0x24(a6),d2                     | +09e
        cmpi.w  #0x10c,d2                       | +0a2
        bge.w   .L037834                        | +0a6
        jsr     0x27db2.l                       | +0aa
        cmpi.b  #0x40,d7                        | +0b0
        bne.w   .L037834                        | +0b4
        jsr     PlayerRoute_PublishState_033522(pc) | +0b8
.L037834:
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0bc
        cmpi.b  #0x3,0x106ece.l                 | +0c0
        beq.w   .L037858                        | +0c8
        movea.l #0xffffffff,a0                  | +0cc
        lea     Sub_000324C6(pc),a0             | +0d2
        jsr     0x5dd56.l                       | +0d6
        bra.w   .L037868                        | +0dc
.L037858:
        movea.l #0xffffffff,a0                  | +0e0
        lea     Sub_000324BC(pc),a0             | +0e6
        jsr     0x5dd56.l                       | +0ea
.L037868:
        bcc.w   .L037872                        | +0f0
        lea     Player_DeathPit_037b8e(pc),a1   | +0f4
        move.l  a1,(a6)                         | +0f8
.L037872:
        rts                                     | +0fa

| ----------------------------------------------------------------------------
|  Player_Death_FallLanded_037874  @ $037874  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_FallLanded_037874, "ax", @progbits
        .global Player_Death_FallLanded_037874
Player_Death_FallLanded_037874:
        clr.w   0x28(a6)                        | +000
        clr.w   0x2a(a6)                        | +004
        move.b  #0x0,0x70(a6)                   | +008
        lea     0x279f4e.l,a0                   | +00e
        move.l  -0x4(a0),0x74(a6)               | +014
        move.b  #0xff,0x21(a6)                  | +01a
        lea     0x279f4e.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        jsr     0x267e2.l                       | +02c
        lea     .L0378ac(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L0378ac:
        jsr     Player_FrameCommon_032ff2(pc)   | +038
        jsr     0x27a92.l                       | +03c
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +042
        bcc.w   .L0378c4                        | +046
        lea     Player_Death_Despawn_037c1a(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L0378c4:
        rts                                     | +050

| ----------------------------------------------------------------------------
|  Player_Death_FallSpawnFx_0378c6  @ $0378C6  (284 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_FallSpawnFx_0378c6, "ax", @progbits
        .global Player_Death_FallSpawnFx_0378c6
Player_Death_FallSpawnFx_0378c6:
        lea     Sub_000391EE(pc),a1             | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        lea     Sub_00039148(pc),a1             | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        move.w  #0x0,0x7c(a6)                   | +020
        move.w  #0x0,0x7e(a6)                   | +026
        bclr    #0x2,0x8c(a6)                   | +02c
        bclr    #0x1,0x8c(a6)                   | +032
        bclr    #0x3,0x8c(a6)                   | +038
        bset    #0x0,0x13(a6)                   | +03e
        move.b  #0x0,0x70(a6)                   | +044
        lea     0x279f58.l,a0                   | +04a
        move.l  -0x4(a0),0x74(a6)               | +050
        move.b  #0xff,0x21(a6)                  | +056
        lea     0x279f58.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
        jsr     0x267e2.l                       | +068
        move.w  #0xfee9,d0                      | +06e
        jsr     0x5dca4.l                       | +072
        move.w  d0,0x28(a6)                     | +078
        move.w  #0x457,0x2a(a6)                 | +07c
        move.w  #0xff9b,0x2e(a6)                | +082
        move.w  #0x0,0x2c(a6)                   | +088
        lea     .L03795a(pc),a1                 | +08e
        move.l  a1,(a6)                         | +092
.L03795a:
        bset    #0x1,0x8d(a6)                   | +094
        jsr     Player_FrameCommon_032ff2(pc)   | +09a
        jsr     0x27bc8.l                       | +09e
        bcc.w   .L037974                        | +0a4
        lea     Player_Death_FallLandedB_0379e2(pc),a1 | +0a8
        move.l  a1,(a6)                         | +0ac
.L037974:
        cmpi.b  #0x1,0x106ece.l                 | +0ae
        bne.w   .L0379a2                        | +0b6
        move.w  0x22(a6),d1                     | +0ba
        move.w  0x24(a6),d2                     | +0be
        cmpi.w  #0x10c,d2                       | +0c2
        bge.w   .L0379a2                        | +0c6
        jsr     0x27db2.l                       | +0ca
        cmpi.b  #0x40,d7                        | +0d0
        bne.w   .L0379a2                        | +0d4
        jsr     PlayerRoute_PublishState_033522(pc) | +0d8
.L0379a2:
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0dc
        cmpi.b  #0x3,0x106ece.l                 | +0e0
        beq.w   .L0379c6                        | +0e8
        movea.l #0xffffffff,a0                  | +0ec
        lea     Sub_000324C6(pc),a0             | +0f2
        jsr     0x5dd56.l                       | +0f6
        bra.w   .L0379d6                        | +0fc
.L0379c6:
        movea.l #0xffffffff,a0                  | +100
        lea     Sub_000324BC(pc),a0             | +106
        jsr     0x5dd56.l                       | +10a
.L0379d6:
        bcc.w   .L0379e0                        | +110
        lea     Player_DeathPit_037b8e(pc),a1   | +114
        move.l  a1,(a6)                         | +118
.L0379e0:
        rts                                     | +11a

| ----------------------------------------------------------------------------
|  Player_Death_FallLandedB_0379e2  @ $0379E2  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_FallLandedB_0379e2, "ax", @progbits
        .global Player_Death_FallLandedB_0379e2
Player_Death_FallLandedB_0379e2:
        clr.w   0x28(a6)                        | +000
        clr.w   0x2a(a6)                        | +004
        move.b  #0x0,0x70(a6)                   | +008
        lea     0x279f62.l,a0                   | +00e
        move.l  -0x4(a0),0x74(a6)               | +014
        move.b  #0xff,0x21(a6)                  | +01a
        lea     0x279f62.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        jsr     0x267e2.l                       | +02c
        lea     .L037a1a(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L037a1a:
        jsr     Player_FrameCommon_032ff2(pc)   | +038
        jsr     0x27a92.l                       | +03c
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +042
        bcc.w   .L037a32                        | +046
        lea     Player_Death_Despawn_037c1a(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L037a32:
        rts                                     | +050

| ----------------------------------------------------------------------------
|  Player_Death_PrioE000_037a34  @ $037A34  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_PrioE000_037a34, "ax", @progbits
        .global Player_Death_PrioE000_037a34
Player_Death_PrioE000_037a34:
        move.w  #0xe000,0x38(a6)                | +000
        bra.w   Player_Death_Generic_037a3e     | +006

| ----------------------------------------------------------------------------
|  Player_Death_Generic_037a3e  @ $037A3E  (180 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_Generic_037a3e, "ax", @progbits
        .global Player_Death_Generic_037a3e
Player_Death_Generic_037a3e:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        bset    #0x0,0x13(a6)                   | +01e
        move.b  #0x0,0x70(a6)                   | +024
        lea     0x279f30.l,a0                   | +02a
        move.l  -0x4(a0),0x74(a6)               | +030
        move.b  #0xff,0x21(a6)                  | +036
        lea     0x279f30.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        jsr     0x267e2.l                       | +048
        move.b  #0x28,0x90(a6)                  | +04e
        move.w  #0xfee9,d0                      | +054
        jsr     0x5dca4.l                       | +058
        move.w  d0,0x28(a6)                     | +05e
        move.w  #0x457,0x2a(a6)                 | +062
        move.w  #0xff9b,0x2e(a6)                | +068
        move.w  #0x0,0x2c(a6)                   | +06e
        lea     .L037ab8(pc),a1                 | +074
        move.l  a1,(a6)                         | +078
.L037ab8:
        bset    #0x1,0x8d(a6)                   | +07a
        jsr     Player_FrameCommon_032ff2(pc)   | +080
        jsr     0x27bc8.l                       | +084
        bcc.w   .L037ad4                        | +08a
        clr.w   0x28(a6)                        | +08e
        clr.w   0x2a(a6)                        | +092
.L037ad4:
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +096
        bcs.w   .L037ae6                        | +09a
        subi.b  #0x1,0x90(a6)                   | +09e
        bne.w   .L037af0                        | +0a4
.L037ae6:
        lea     Player_Death_Despawn_037c1a(pc),a1 | +0a8
        move.l  a1,(a6)                         | +0ac
        bra.w   .L037af0                        | +0ae
.L037af0:
        rts                                     | +0b2

| ----------------------------------------------------------------------------
|  Player_Death_Debug_037af2  @ $037AF2  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_Debug_037af2, "ax", @progbits
        .global Player_Death_Debug_037af2
Player_Death_Debug_037af2:
        jsr     Player_SpawnDebugTask_033358(pc) | +000
        move.w  #0x10fb,d0                      | +004
        jsr     0x2352.l                        | +008

| ----------------------------------------------------------------------------
|  Player_Death_Timed_037b00  @ $037B00  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_Timed_037b00, "ax", @progbits
        .global Player_Death_Timed_037b00
Player_Death_Timed_037b00:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        bset    #0x0,0x13(a6)                   | +01e
        move.b  #0x0,0x70(a6)                   | +024
        lea     0x279f6c.l,a0                   | +02a
        move.l  -0x4(a0),0x74(a6)               | +030
        move.b  #0xff,0x21(a6)                  | +036
        lea     0x279f6c.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        jsr     0x267e2.l                       | +048
        move.b  #0x28,0x90(a6)                  | +04e
        lea     .L037b5a(pc),a1                 | +054
        move.l  a1,(a6)                         | +058
.L037b5a:
        jsr     Player_FrameCommon_032ff2(pc)   | +05a
        jsr     0x27a92.l                       | +05e
        bcc.w   .L037b70                        | +064
        clr.w   0x28(a6)                        | +068
        clr.w   0x2a(a6)                        | +06c
.L037b70:
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +070
        bcs.w   .L037b82                        | +074
        subi.b  #0x1,0x90(a6)                   | +078
        bne.w   .L037b8c                        | +07e
.L037b82:
        lea     Player_Death_Despawn_037c1a(pc),a1 | +082
        move.l  a1,(a6)                         | +086
        bra.w   .L037b8c                        | +088
.L037b8c:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  Player_DeathPit_037b8e  @ $037B8E  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Player_DeathPit_037b8e, "ax", @progbits
        .global Player_DeathPit_037b8e
Player_DeathPit_037b8e:
        btst    #0x0,0x13(a6)                   | +000
        bne.w   .L037bba                        | +006
        cmpa.l  #0x100440,a6                    | +00a
        bne.w   .L037bb0                        | +010
        move.w  #0x1123,d0                      | +014
        jsr     0x2352.l                        | +018
        bra.w   .L037bba                        | +01e
.L037bb0:
        move.w  #0x1087,d0                      | +022
        jsr     0x2352.l                        | +026
.L037bba:
        bset    #0x0,0x13(a6)                   | +02c
        move.w  #0x0,0x7c(a6)                   | +032
        move.w  #0x0,0x7e(a6)                   | +038
        move.b  #0x6,0x90(a6)                   | +03e
        lea     0xffff.w,a0                     | +044
        move.l  a0,0x48(a6)                     | +048
        lea     .L037be0(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L037be0:
        jsr     0x27cee.l                       | +052
        subi.b  #0x1,0x90(a6)                   | +058
        bne.w   .L037c18                        | +05e
        jsr     0x267e2.l                       | +062
        jsr     0x5b6.l                         | +068
        bclr    #0x7,0x5b(a6)                   | +06e
        jsr     0x13600.l                       | +074
        lea     0x400.l,a1                      | +07a
        move.l  a1,(a6)                         | +080
        movea.l a6,a0                           | +082
        jsr     0x5fe.l                         | +084
.L037c18:
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  Player_Death_Despawn_037c1a  @ $037C1A  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Death_Despawn_037c1a, "ax", @progbits
        .global Player_Death_Despawn_037c1a
Player_Death_Despawn_037c1a:
        jsr     0x267e2.l                       | +000
        move.w  #0x0,0x7c(a6)                   | +006
        move.w  #0x0,0x7e(a6)                   | +00c
        move.b  #0x28,0x90(a6)                  | +012
        lea     0xffff.w,a0                     | +018
        move.l  a0,0x48(a6)                     | +01c
        lea     .L037c40(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L037c40:
        jsr     0x2783a.l                       | +026
        subi.b  #0x1,0x90(a6)                   | +02c
        bne.w   .L037c72                        | +032
        jsr     0x5b6.l                         | +036
        bclr    #0x7,0x5b(a6)                   | +03c
        jsr     0x13600.l                       | +042
        lea     0x400.l,a1                      | +048
        move.l  a1,(a6)                         | +04e
        movea.l a6,a0                           | +050
        jsr     0x5fe.l                         | +052
.L037c72:
        rts                                     | +058

| ----------------------------------------------------------------------------
|  Player_CrouchEnter_037c74  @ $037C74  (326 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrouchEnter_037c74, "ax", @progbits
        .global Player_CrouchEnter_037c74
Player_CrouchEnter_037c74:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        move.b  #0x30,0x70(a6)                  | +01e
        lea     0x279864.l,a0                   | +024
        move.l  -0x4(a0),0x74(a6)               | +02a
        move.b  #0xff,0x21(a6)                  | +030
        lea     0x279864.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        move.l  #0x32598,0x60(a6)               | +042
        clr.w   0x2c(a6)                        | +04a
        lea     Sub_00032734(pc),a0             | +04e
        move.l  a0,0x48(a6)                     | +052
        lea     .L037cd0(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L037cd0:
        jsr     Player_FrameCommon_032ff2(pc)   | +05c
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +060
        clr.w   0x28(a6)                        | +064
        jsr     JmpAbsThunk_032e3c(pc)          | +068
        bcc.w   .L037cea                        | +06c
        move.w  #0x300,0x28(a6)                 | +070
.L037cea:
        jsr     Input_RightThunk_032e42(pc)     | +076
        bcc.w   .L037cf8                        | +07a
        move.w  #0xfd00,0x28(a6)                | +07e
.L037cf8:
        jsr     0x27a92.l                       | +084
        clr.w   0x28(a6)                        | +08a
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +08e
        bcc.w   .L037d2c                        | +092
        lea     Player_CrouchIdle_03827a(pc),a1 | +096
        move.l  a1,(a6)                         | +09a
        jsr     JmpAbsThunk_032e3c(pc)          | +09c
        bcc.w   .L037d1e                        | +0a0
        lea     Player_CrawlRight_03842c(pc),a1 | +0a4
        move.l  a1,(a6)                         | +0a8
.L037d1e:
        jsr     Input_RightThunk_032e42(pc)     | +0aa
        bcc.w   .L037d2c                        | +0ae
        lea     Player_CrawlLeft_03855a(pc),a1  | +0b2
        move.l  a1,(a6)                         | +0b6
.L037d2c:
        jsr     Player_ActionSelect_0330d0(pc)  | +0b8
        bcc.w   .L037d62                        | +0bc
        cmpi.b  #0xff,d1                        | +0c0
        bne.w   .L037d46                        | +0c4
        lea     Sub_00038A28(pc),a1             | +0c8
        move.l  a1,(a6)                         | +0cc
        bra.w   .L037d62                        | +0ce
.L037d46:
        cmpi.b  #0x3,d1                         | +0d2
        bne.w   .L037d58                        | +0d6
        lea     Sub_000388F0(pc),a1             | +0da
        move.l  a1,(a6)                         | +0de
        bra.w   .L037d62                        | +0e0
.L037d58:
        lea     Player_CrouchShoot_03873c(pc),a1 | +0e4
        move.l  a1,(a6)                         | +0e8
        bra.w   .L037d62                        | +0ea
.L037d62:
        cmpi.b  #0x0,0x85(a6)                   | +0ee
        bne.w   .L037d7c                        | +0f4
        cmpi.b  #0x1,0x71(a6)                   | +0f8
        bne.w   .L037d7c                        | +0fe
        lea     Player_Reload_033b9a(pc),a1     | +102
        move.l  a1,(a6)                         | +106
.L037d7c:
        cmpi.w  #0x0,0x82(a6)                   | +108
        bne.w   .L037d96                        | +10e
        cmpi.b  #0x0,0x71(a6)                   | +112
        beq.w   .L037d96                        | +118
        lea     Player_CrouchWeaponEmpty_038086(pc),a1 | +11c
        move.l  a1,(a6)                         | +120
.L037d96:
        jsr     0x27eba.l                       | +122
        bcc.w   .L037da6                        | +128
        lea     Player_Knockback_036d64(pc),a1  | +12c
        move.l  a1,(a6)                         | +130
.L037da6:
        jsr     Input_FireByMode_033034(pc)     | +132
        bcc.w   .L037db4                        | +136
        lea     Player_JumpStart_036914(pc),a1  | +13a
        move.l  a1,(a6)                         | +13e
.L037db4:
        jsr     PlayerRoute_PublishState_033522(pc) | +140
        rts                                     | +144

| ----------------------------------------------------------------------------
|  Player_CrouchEnterB_037dba  @ $037DBA  (264 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrouchEnterB_037dba, "ax", @progbits
        .global Player_CrouchEnterB_037dba
Player_CrouchEnterB_037dba:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        bset    #0x7,0x5b(a6)                   | +01e
        move.b  #0x30,0x70(a6)                  | +024
        lea     0x279864.l,a0                   | +02a
        move.l  -0x4(a0),0x74(a6)               | +030
        move.b  #0xff,0x21(a6)                  | +036
        lea     0x279864.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        lea     Sub_00032734(pc),a0             | +048
        move.l  a0,0x48(a6)                     | +04c
        clr.w   0x2c(a6)                        | +050
        lea     .L037e14(pc),a1                 | +054
        move.l  a1,(a6)                         | +058
.L037e14:
        jsr     Player_FrameCommon_032ff2(pc)   | +05a
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +05e
        clr.w   0x28(a6)                        | +062
        move.w  #0x300,0x28(a6)                 | +066
        jsr     Input_RightThunk_032e42(pc)     | +06c
        bcc.w   .L037e34                        | +070
        move.w  #0xfd00,0x28(a6)                | +074
.L037e34:
        jsr     0x27a92.l                       | +07a
        clr.w   0x28(a6)                        | +080
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +084
        bcc.w   .L037e68                        | +088
        lea     Player_CrouchIdle_03827a(pc),a1 | +08c
        move.l  a1,(a6)                         | +090
        jsr     JmpAbsThunk_032e3c(pc)          | +092
        bcc.w   .L037e5a                        | +096
        lea     Player_CrawlRight_03842c(pc),a1 | +09a
        move.l  a1,(a6)                         | +09e
.L037e5a:
        jsr     Input_RightThunk_032e42(pc)     | +0a0
        bcc.w   .L037e68                        | +0a4
        lea     Player_CrawlLeft_03855a(pc),a1  | +0a8
        move.l  a1,(a6)                         | +0ac
.L037e68:
        jsr     Player_ActionSelect_0330d0(pc)  | +0ae
        bcc.w   .L037e9e                        | +0b2
        cmpi.b  #0xff,d1                        | +0b6
        bne.w   .L037e82                        | +0ba
        lea     Sub_00038A28(pc),a1             | +0be
        move.l  a1,(a6)                         | +0c2
        bra.w   .L037e9e                        | +0c4
.L037e82:
        cmpi.b  #0x3,d1                         | +0c8
        bne.w   .L037e94                        | +0cc
        lea     Sub_000388F0(pc),a1             | +0d0
        move.l  a1,(a6)                         | +0d4
        bra.w   .L037e9e                        | +0d6
.L037e94:
        lea     Player_CrouchShoot_03873c(pc),a1 | +0da
        move.l  a1,(a6)                         | +0de
        bra.w   .L037e9e                        | +0e0
.L037e9e:
        jsr     0x27eba.l                       | +0e4
        bcc.w   .L037eae                        | +0ea
        lea     Player_Knockback_036d64(pc),a1  | +0ee
        move.l  a1,(a6)                         | +0f2
.L037eae:
        jsr     Input_JumpByMode_033080(pc)     | +0f4
        bcc.w   .L037ebc                        | +0f8
        lea     Player_JumpStart_036914(pc),a1  | +0fc
        move.l  a1,(a6)                         | +100
.L037ebc:
        jsr     PlayerRoute_PublishState_033522(pc) | +102
        rts                                     | +106

| ----------------------------------------------------------------------------
|  Player_CrouchExit_037ec2  @ $037EC2  (386 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrouchExit_037ec2, "ax", @progbits
        .global Player_CrouchExit_037ec2
Player_CrouchExit_037ec2:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        move.b  #0x30,0x70(a6)                  | +01e
        lea     0x27986e.l,a0                   | +024
        move.l  -0x4(a0),0x74(a6)               | +02a
        move.b  #0xff,0x21(a6)                  | +030
        lea     0x27986e.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        clr.w   0x2c(a6)                        | +042
        lea     Sub_00032734(pc),a0             | +046
        move.l  a0,0x48(a6)                     | +04a
        lea     .L037f16(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L037f16:
        jsr     Player_FrameCommon_032ff2(pc)   | +054
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +058
        clr.w   0x28(a6)                        | +05c
        jsr     JmpAbsThunk_032e3c(pc)          | +060
        bcc.w   .L037f30                        | +064
        move.w  #0x300,0x28(a6)                 | +068
.L037f30:
        jsr     Input_RightThunk_032e42(pc)     | +06e
        bcc.w   .L037f3e                        | +072
        move.w  #0xfd00,0x28(a6)                | +076
.L037f3e:
        jsr     0x27a92.l                       | +07c
        clr.w   0x28(a6)                        | +082
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +086
        bcc.w   .L037fc6                        | +08a
        lea     Player_Stand_034704(pc),a1      | +08e
        move.l  a1,(a6)                         | +092
        btst    #0x0,0x88(a6)                   | +094
        beq.w   .L037f66                        | +09a
        lea     Player_Crouch_033a5e(pc),a1     | +09e
        move.l  a1,(a6)                         | +0a2
.L037f66:
        cmpi.b  #0x0,0x85(a6)                   | +0a4
        bne.w   .L037f80                        | +0aa
        cmpi.b  #0x1,0x71(a6)                   | +0ae
        bne.w   .L037f80                        | +0b4
        lea     Player_Reload_033b9a(pc),a1     | +0b8
        move.l  a1,(a6)                         | +0bc
.L037f80:
        jsr     0x5e9b6.l                       | +0be
        andi.w  #0x7,d0                         | +0c4
        moveq   #4,d0                           | +0c8
        cmp.w   0x82(a6),d0                     | +0ca
        bne.w   .L037f9a                        | +0ce
        lea     Player_CrouchB_033afc(pc),a1    | +0d2
        move.l  a1,(a6)                         | +0d6
.L037f9a:
        cmpi.w  #0x0,0x82(a6)                   | +0d8
        bne.w   .L037faa                        | +0de
        lea     Player_Idle_033d64(pc),a1       | +0e2
        move.l  a1,(a6)                         | +0e6
.L037faa:
        jsr     JmpAbsThunk_032e3c(pc)          | +0e8
        bcc.w   .L037fb8                        | +0ec
        lea     Player_WalkLoopRight_0351d8(pc),a1 | +0f0
        move.l  a1,(a6)                         | +0f4
.L037fb8:
        jsr     Input_RightThunk_032e42(pc)     | +0f6
        bcc.w   .L037fc6                        | +0fa
        lea     Player_WalkLoopLeft_0354f2(pc),a1 | +0fe
        move.l  a1,(a6)                         | +102
.L037fc6:
        jsr     Player_ActionSelect_0330d0(pc)  | +104
        bcc.w   .L038020                        | +108
        cmpi.b  #0xff,d1                        | +10c
        bne.w   .L037fe0                        | +110
        lea     Player_Melee_035d34(pc),a1      | +114
        move.l  a1,(a6)                         | +118
        bra.w   .L038020                        | +11a
.L037fe0:
        cmpi.b  #0x3,d1                         | +11e
        bne.w   .L037ff2                        | +122
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +126
        move.l  a1,(a6)                         | +12a
        bra.w   .L038020                        | +12c
.L037ff2:
        cmpi.b  #0x4,d1                         | +130
        bne.w   .L038004                        | +134
        lea     Player_CrouchShoot_03873c(pc),a1 | +138
        move.l  a1,(a6)                         | +13c
        bra.w   .L038020                        | +13e
.L038004:
        cmpi.b  #0x1,d1                         | +142
        bne.w   .L038016                        | +146
        lea     Player_ShootStandUp_03437e(pc),a1 | +14a
        move.l  a1,(a6)                         | +14e
        bra.w   .L038020                        | +150
.L038016:
        lea     Player_ShootStand_0342c4(pc),a1 | +154
        move.l  a1,(a6)                         | +158
        bra.w   .L038020                        | +15a
.L038020:
        jsr     0x27eba.l                       | +15e
        bcc.w   .L038030                        | +164
        lea     Player_Knockback_036d64(pc),a1  | +168
        move.l  a1,(a6)                         | +16c
.L038030:
        jsr     Input_FireByMode_033034(pc)     | +16e
        bcc.w   .L03803e                        | +172
        lea     Player_JumpStart_036914(pc),a1  | +176
        move.l  a1,(a6)                         | +17a
.L03803e:
        jsr     PlayerRoute_PublishState_033522(pc) | +17c
        rts                                     | +180

| ----------------------------------------------------------------------------
|  Player_CrouchReenterByInput_038044  @ $038044  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrouchReenterByInput_038044, "ax", @progbits
        .global Player_CrouchReenterByInput_038044
Player_CrouchReenterByInput_038044:
        btst    #0x0,0x3a(a6)                   | +000
        bne.w   .L03806a                        | +006
        jsr     JmpAbsThunk_032e3c(pc)          | +00a
        bcc.w   .L03805a                        | +00e
        jmp     Player_CrawlRight_03842c(pc)    | +012
.L03805a:
        jsr     Input_RightThunk_032e42(pc)     | +016
        bcc.w   .L038066                        | +01a
        jmp     Player_CrawlLeft_Alt_0386e2(pc) | +01e
.L038066:
        bra.w   .L038082                        | +022
.L03806a:
        jsr     Input_RightThunk_032e42(pc)     | +026
        bcc.w   .L038076                        | +02a
        jmp     Player_CrawlLeft_03855a(pc)     | +02e
.L038076:
        jsr     JmpAbsThunk_032e3c(pc)          | +032
        bcc.w   .L038082                        | +036
        jmp     Player_CrawlRight_Alt_038688(pc) | +03a
.L038082:
        jmp     Player_CrouchIdle_03827a(pc)    | +03e

| ----------------------------------------------------------------------------
|  Player_CrouchWeaponEmpty_038086  @ $038086  (284 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrouchWeaponEmpty_038086, "ax", @progbits
        .global Player_CrouchWeaponEmpty_038086
Player_CrouchWeaponEmpty_038086:
        cmpi.b  #0x0,0x71(a6)                   | +000
        bne.w   .L038094                        | +006
        jmp     Player_CrouchIdleB_0381a2(pc)   | +00a
.L038094:
        bset    #0x2,0x8c(a6)                   | +00e
        bclr    #0x1,0x8c(a6)                   | +014
        bclr    #0x3,0x8c(a6)                   | +01a
        move.b  #0x20,0x70(a6)                  | +020
        lea     0x279832.l,a0                   | +026
        move.l  -0x4(a0),0x74(a6)               | +02c
        move.b  #0xff,0x21(a6)                  | +032
        lea     0x279832.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        lea     0xffff.w,a0                     | +044
        move.l  a0,0x48(a6)                     | +048
        lea     Sub_00038BE4(pc),a1             | +04c
        jsr     0x4ae.l                         | +050
        jsr     0x5dd02.l                       | +056
        move.b  #0x0,0x71(a6)                   | +05c
        move.w  #0xa,0x82(a6)                   | +062
        bset    #0x7,0x5b(a6)                   | +068
        move.l  #0x32500,0x60(a6)               | +06e
        jsr     0x267e6.l                       | +076
        clr.w   0x28(a6)                        | +07c
        lea     .L03810c(pc),a1                 | +080
        move.l  a1,(a6)                         | +084
.L03810c:
        bset    #0x6,0x8d(a6)                   | +086
        jsr     Player_FrameCommon_032ff2(pc)   | +08c
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +090
        jsr     0x27a92.l                       | +094
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +09a
        bcc.w   .L03812e                        | +09e
        lea     Player_CrouchIdleB_0381a2(pc),a1 | +0a2
        move.l  a1,(a6)                         | +0a6
.L03812e:
        jsr     0x5cef8.l                       | +0a8
        bcs.w   .L03813e                        | +0ae
        lea     Player_CrouchExit_037ec2(pc),a1 | +0b2
        move.l  a1,(a6)                         | +0b6
.L03813e:
        btst    #0x2,0x8c(a6)                   | +0b8
        bne.w   .L03817e                        | +0be
        jsr     Player_ActionSelect_0330d0(pc)  | +0c2
        bcc.w   .L03817e                        | +0c6
        cmpi.b  #0xff,d1                        | +0ca
        bne.w   .L038162                        | +0ce
        lea     Sub_00038A28(pc),a1             | +0d2
        move.l  a1,(a6)                         | +0d6
        bra.w   .L03817e                        | +0d8
.L038162:
        cmpi.b  #0x3,d1                         | +0dc
        bne.w   .L038174                        | +0e0
        lea     Sub_000388F0(pc),a1             | +0e4
        move.l  a1,(a6)                         | +0e8
        bra.w   .L03817e                        | +0ea
.L038174:
        lea     Player_CrouchShoot_03873c(pc),a1 | +0ee
        move.l  a1,(a6)                         | +0f2
        bra.w   .L03817e                        | +0f4
.L03817e:
        jsr     0x27eba.l                       | +0f8
        bcc.w   .L03818e                        | +0fe
        lea     Player_Knockback_036d64(pc),a1  | +102
        move.l  a1,(a6)                         | +106
.L03818e:
        jsr     Input_FireByMode_033034(pc)     | +108
        bcc.w   .L03819c                        | +10c
        lea     Player_JumpStart_036914(pc),a1  | +110
        move.l  a1,(a6)                         | +114
.L03819c:
        jsr     PlayerRoute_PublishState_033522(pc) | +116
        rts                                     | +11a

| ----------------------------------------------------------------------------
|  Player_CrouchIdleB_0381a2  @ $0381A2  (216 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrouchIdleB_0381a2, "ax", @progbits
        .global Player_CrouchIdleB_0381a2
Player_CrouchIdleB_0381a2:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x1,0x8c(a6)                   | +006
        bclr    #0x3,0x8c(a6)                   | +00c
        jsr     0x2abcc.l                       | +012
        bcs.w   .L0381e6                        | +018
        move.b  #0x30,0x70(a6)                  | +01c
        lea     0x279850.l,a0                   | +022
        move.l  -0x4(a0),0x74(a6)               | +028
        move.b  #0xff,0x21(a6)                  | +02e
        lea     0x279850.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        bra.w   .L03820a                        | +040
.L0381e6:
        move.b  #0x30,0x70(a6)                  | +044
        lea     0x279a82.l,a0                   | +04a
        move.l  -0x4(a0),0x74(a6)               | +050
        move.b  #0xff,0x21(a6)                  | +056
        lea     0x279a82.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
.L03820a:
        bra.w   Player_CrouchIdle_Setup_0382e2 | +068
        bclr    #0x2,0x8c(a6)                   | +06c
        bclr    #0x1,0x8c(a6)                   | +072
        bclr    #0x3,0x8c(a6)                   | +078
        jsr     0x2abcc.l                       | +07e
        bcs.w   .L038252                        | +084
        move.b  #0x30,0x70(a6)                  | +088
        lea     0x27985a.l,a0                   | +08e
        move.l  -0x4(a0),0x74(a6)               | +094
        move.b  #0xff,0x21(a6)                  | +09a
        lea     0x27985a.l,a0                   | +0a0
        jsr     0x28cd4.l                       | +0a6
        bra.w   .L038276                        | +0ac
.L038252:
        move.b  #0x30,0x70(a6)                  | +0b0
        lea     0x279a82.l,a0                   | +0b6
        move.l  -0x4(a0),0x74(a6)               | +0bc
        move.b  #0xff,0x21(a6)                  | +0c2
        lea     0x279a82.l,a0                   | +0c8
        jsr     0x28cd4.l                       | +0ce
.L038276:
        bra.w   Player_CrouchIdle_Setup_0382e2 | +0d4

| ----------------------------------------------------------------------------
|  Player_CrouchIdle_03827a  @ $03827A  (434 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrouchIdle_03827a, "ax", @progbits
        .global Player_CrouchIdle_03827a
Player_CrouchIdle_03827a:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x1,0x8c(a6)                   | +006
        bclr    #0x3,0x8c(a6)                   | +00c
        jsr     0x2abcc.l                       | +012
        bcs.w   .L0382be                        | +018
        move.b  #0x30,0x70(a6)                  | +01c
        lea     0x27983c.l,a0                   | +022
        move.l  -0x4(a0),0x74(a6)               | +028
        move.b  #0xff,0x21(a6)                  | +02e
        lea     0x27983c.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        bra.w   Player_CrouchIdle_Setup_0382e2 | +040
.L0382be:
        move.b  #0x30,0x70(a6)                  | +044
        lea     0x279a82.l,a0                   | +04a
        move.l  -0x4(a0),0x74(a6)               | +050
        move.b  #0xff,0x21(a6)                  | +056
        lea     0x279a82.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
        .global Player_CrouchIdle_Setup_0382e2
Player_CrouchIdle_Setup_0382e2:
        move.l  #0x32598,0x60(a6)               | +068
        jsr     0x267e6.l                       | +070
        clr.w   0x28(a6)                        | +076
        clr.w   0x2c(a6)                        | +07a
        lea     Sub_00032734(pc),a0             | +07e
        move.l  a0,0x48(a6)                     | +082
        lea     .L038306(pc),a1                 | +086
        move.l  a1,(a6)                         | +08a
.L038306:
        bset    #0x6,0x8d(a6)                   | +08c
        jsr     Player_FrameCommon_032ff2(pc)   | +092
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +096
        bcc.w   .L03831e                        | +09a
        lea     Player_CrouchReenterByInput_038044(pc),a1 | +09e
        move.l  a1,(a6)                         | +0a2
.L03831e:
        jsr     0x27a92.l                       | +0a4
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0aa
        btst    #0x0,0x3a(a6)                   | +0ae
        bne.w   .L038352                        | +0b4
        jsr     JmpAbsThunk_032e3c(pc)          | +0b8
        bcc.w   .L038340                        | +0bc
        lea     Player_CrawlRight_03842c(pc),a1 | +0c0
        move.l  a1,(a6)                         | +0c4
.L038340:
        jsr     Input_RightThunk_032e42(pc)     | +0c6
        bcc.w   .L03834e                        | +0ca
        lea     Player_CrawlLeft_Alt_0386e2(pc),a1 | +0ce
        move.l  a1,(a6)                         | +0d2
.L03834e:
        bra.w   .L03836e                        | +0d4
.L038352:
        jsr     JmpAbsThunk_032e3c(pc)          | +0d8
        bcc.w   .L038360                        | +0dc
        lea     Player_CrawlRight_Alt_038688(pc),a1 | +0e0
        move.l  a1,(a6)                         | +0e4
.L038360:
        jsr     Input_RightThunk_032e42(pc)     | +0e6
        bcc.w   .L03836e                        | +0ea
        lea     Player_CrawlLeft_03855a(pc),a1  | +0ee
        move.l  a1,(a6)                         | +0f2
.L03836e:
        jsr     0x5cef8.l                       | +0f4
        bcs.w   .L03837e                        | +0fa
        lea     Player_CrouchExit_037ec2(pc),a1 | +0fe
        move.l  a1,(a6)                         | +102
.L03837e:
        jsr     Player_ActionSelect_0330d0(pc)  | +104
        bcc.w   .L0383b4                        | +108
        cmpi.b  #0xff,d1                        | +10c
        bne.w   .L038398                        | +110
        lea     Sub_00038A28(pc),a1             | +114
        move.l  a1,(a6)                         | +118
        bra.w   .L0383b4                        | +11a
.L038398:
        cmpi.b  #0x3,d1                         | +11e
        bne.w   .L0383aa                        | +122
        lea     Sub_000388F0(pc),a1             | +126
        move.l  a1,(a6)                         | +12a
        bra.w   .L0383b4                        | +12c
.L0383aa:
        lea     Player_CrouchShoot_03873c(pc),a1 | +130
        move.l  a1,(a6)                         | +134
        bra.w   .L0383b4                        | +136
.L0383b4:
        cmpi.w  #0x0,0x82(a6)                   | +13a
        bne.w   .L0383ce                        | +140
        cmpi.b  #0x0,0x71(a6)                   | +144
        beq.w   .L0383ce                        | +14a
        lea     Player_CrouchWeaponEmpty_038086(pc),a1 | +14e
        move.l  a1,(a6)                         | +152
.L0383ce:
        jsr     0x27eba.l                       | +154
        bcc.w   .L0383de                        | +15a
        lea     Player_KnockbackDelay_036fc2(pc),a1 | +15e
        move.l  a1,(a6)                         | +162
.L0383de:
        jsr     Input_FireByMode_033034(pc)     | +164
        bcc.w   .L0383ec                        | +168
        lea     Player_JumpStart_036914(pc),a1  | +16c
        move.l  a1,(a6)                         | +170
.L0383ec:
        jsr     PlayerRoute_PublishState_033522(pc) | +172
        cmpi.b  #0x3,0x106ece.l                 | +176
        beq.w   .L038410                        | +17e
        movea.l #0xffffffff,a0                  | +182
        lea     Sub_000324C6(pc),a0             | +188
        jsr     0x5dd56.l                       | +18c
        bra.w   .L038420                        | +192
.L038410:
        movea.l #0xffffffff,a0                  | +196
        lea     Sub_000324BC(pc),a0             | +19c
        jsr     0x5dd56.l                       | +1a0
.L038420:
        bcc.w   .L03842a                        | +1a6
        lea     Player_DeathPit_037b8e(pc),a1   | +1aa
        move.l  a1,(a6)                         | +1ae
.L03842a:
        rts                                     | +1b0

| ----------------------------------------------------------------------------
|  Player_CrawlRight_03842c  @ $03842C  (302 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrawlRight_03842c, "ax", @progbits
        .global Player_CrawlRight_03842c
Player_CrawlRight_03842c:
        jsr     0x2abcc.l                       | +000
        bcs.w   .L03845e                        | +006
        move.b  #0x30,0x70(a6)                  | +00a
        lea     0x279a82.l,a0                   | +010
        move.l  -0x4(a0),0x74(a6)               | +016
        move.b  #0xff,0x21(a6)                  | +01c
        lea     0x279a82.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        bra.w   Player_CrawlRight_Setup_038482 | +02e
.L03845e:
        move.b  #0x30,0x70(a6)                  | +032
        lea     0x279a82.l,a0                   | +038
        move.l  -0x4(a0),0x74(a6)               | +03e
        move.b  #0xff,0x21(a6)                  | +044
        lea     0x279a82.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        .global Player_CrawlRight_Setup_038482
Player_CrawlRight_Setup_038482:
        bclr    #0x2,0x8c(a6)                   | +056
        bclr    #0x1,0x8c(a6)                   | +05c
        bclr    #0x3,0x8c(a6)                   | +062
        move.l  #0x32598,0x60(a6)               | +068
        move.w  #0x120,0x28(a6)                 | +070
        clr.w   0x2c(a6)                        | +076
        lea     Sub_00032734(pc),a0             | +07a
        move.l  a0,0x48(a6)                     | +07e
        bclr    #0x0,0x3a(a6)                   | +082
        lea     .L0384ba(pc),a1                 | +088
        move.l  a1,(a6)                         | +08c
.L0384ba:
        bset    #0x6,0x8d(a6)                   | +08e
        jsr     Player_FrameCommon_032ff2(pc)   | +094
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +098
        bcc.w   .L0384d2                        | +09c
        lea     Player_CrouchReenterByInput_038044(pc),a1 | +0a0
        move.l  a1,(a6)                         | +0a4
.L0384d2:
        move.w  #0x120,0x28(a6)                 | +0a6
        jsr     0x27a92.l                       | +0ac
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0b2
        jsr     JmpAbsThunk_032e3c(pc)          | +0b6
        bcs.w   .L0384f0                        | +0ba
        lea     Player_CrouchIdle_03827a(pc),a1 | +0be
        move.l  a1,(a6)                         | +0c2
.L0384f0:
        jsr     0x5cef8.l                       | +0c4
        bcs.w   .L038500                        | +0ca
        lea     Player_CrouchExit_037ec2(pc),a1 | +0ce
        move.l  a1,(a6)                         | +0d2
.L038500:
        jsr     Player_ActionSelect_0330d0(pc)  | +0d4
        bcc.w   .L038536                        | +0d8
        cmpi.b  #0xff,d1                        | +0dc
        bne.w   .L03851a                        | +0e0
        lea     Sub_00038A28(pc),a1             | +0e4
        move.l  a1,(a6)                         | +0e8
        bra.w   .L038536                        | +0ea
.L03851a:
        cmpi.b  #0x3,d1                         | +0ee
        bne.w   .L03852c                        | +0f2
        lea     Sub_000388F0(pc),a1             | +0f6
        move.l  a1,(a6)                         | +0fa
        bra.w   .L038536                        | +0fc
.L03852c:
        lea     Player_CrouchShoot_03873c(pc),a1 | +100
        move.l  a1,(a6)                         | +104
        bra.w   .L038536                        | +106
.L038536:
        jsr     0x27eba.l                       | +10a
        bcc.w   .L038546                        | +110
        lea     Player_KnockbackDelay_036fc2(pc),a1 | +114
        move.l  a1,(a6)                         | +118
.L038546:
        jsr     Input_FireByMode_033034(pc)     | +11a
        bcc.w   .L038554                        | +11e
        lea     Player_JumpStart_036914(pc),a1  | +122
        move.l  a1,(a6)                         | +126
.L038554:
        jsr     PlayerRoute_PublishState_033522(pc) | +128
        rts                                     | +12c

| ----------------------------------------------------------------------------
|  Player_CrawlLeft_03855a  @ $03855A  (302 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrawlLeft_03855a, "ax", @progbits
        .global Player_CrawlLeft_03855a
Player_CrawlLeft_03855a:
        jsr     0x2abcc.l                       | +000
        bcs.w   .L03858c                        | +006
        move.b  #0x30,0x70(a6)                  | +00a
        lea     0x279a82.l,a0                   | +010
        move.l  -0x4(a0),0x74(a6)               | +016
        move.b  #0xff,0x21(a6)                  | +01c
        lea     0x279a82.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        bra.w   Player_CrawlLeft_Setup_0385b0 | +02e
.L03858c:
        move.b  #0x30,0x70(a6)                  | +032
        lea     0x279a82.l,a0                   | +038
        move.l  -0x4(a0),0x74(a6)               | +03e
        move.b  #0xff,0x21(a6)                  | +044
        lea     0x279a82.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        .global Player_CrawlLeft_Setup_0385b0
Player_CrawlLeft_Setup_0385b0:
        bclr    #0x2,0x8c(a6)                   | +056
        bclr    #0x1,0x8c(a6)                   | +05c
        bclr    #0x3,0x8c(a6)                   | +062
        move.l  #0x32598,0x60(a6)               | +068
        move.w  #0xfee0,0x28(a6)                | +070
        clr.w   0x2c(a6)                        | +076
        lea     Sub_00032734(pc),a0             | +07a
        move.l  a0,0x48(a6)                     | +07e
        bset    #0x0,0x3a(a6)                   | +082
        lea     .L0385e8(pc),a1                 | +088
        move.l  a1,(a6)                         | +08c
.L0385e8:
        bset    #0x6,0x8d(a6)                   | +08e
        jsr     Player_FrameCommon_032ff2(pc)   | +094
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +098
        bcc.w   .L038600                        | +09c
        lea     Player_CrouchReenterByInput_038044(pc),a1 | +0a0
        move.l  a1,(a6)                         | +0a4
.L038600:
        move.w  #0xfee0,0x28(a6)                | +0a6
        jsr     0x27a92.l                       | +0ac
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0b2
        jsr     Input_RightThunk_032e42(pc)     | +0b6
        bcs.w   .L03861e                        | +0ba
        lea     Player_CrouchIdle_03827a(pc),a1 | +0be
        move.l  a1,(a6)                         | +0c2
.L03861e:
        jsr     0x5cef8.l                       | +0c4
        bcs.w   .L03862e                        | +0ca
        lea     Player_CrouchExit_037ec2(pc),a1 | +0ce
        move.l  a1,(a6)                         | +0d2
.L03862e:
        jsr     Player_ActionSelect_0330d0(pc)  | +0d4
        bcc.w   .L038664                        | +0d8
        cmpi.b  #0xff,d1                        | +0dc
        bne.w   .L038648                        | +0e0
        lea     Sub_00038A28(pc),a1             | +0e4
        move.l  a1,(a6)                         | +0e8
        bra.w   .L038664                        | +0ea
.L038648:
        cmpi.b  #0x3,d1                         | +0ee
        bne.w   .L03865a                        | +0f2
        lea     Sub_000388F0(pc),a1             | +0f6
        move.l  a1,(a6)                         | +0fa
        bra.w   .L038664                        | +0fc
.L03865a:
        lea     Player_CrouchShoot_03873c(pc),a1 | +100
        move.l  a1,(a6)                         | +104
        bra.w   .L038664                        | +106
.L038664:
        jsr     0x27eba.l                       | +10a
        bcc.w   .L038674                        | +110
        lea     Player_KnockbackDelay_036fc2(pc),a1 | +114
        move.l  a1,(a6)                         | +118
.L038674:
        jsr     Input_FireByMode_033034(pc)     | +11a
        bcc.w   .L038682                        | +11e
        lea     Player_JumpStart_036914(pc),a1  | +122
        move.l  a1,(a6)                         | +126
.L038682:
        jsr     PlayerRoute_PublishState_033522(pc) | +128
        rts                                     | +12c

| ----------------------------------------------------------------------------
|  Player_CrawlRight_Alt_038688  @ $038688  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrawlRight_Alt_038688, "ax", @progbits
        .global Player_CrawlRight_Alt_038688
Player_CrawlRight_Alt_038688:
        jsr     0x2abcc.l                       | +000
        bcs.w   .L0386ba                        | +006
        move.b  #0x30,0x70(a6)                  | +00a
        lea     0x279a78.l,a0                   | +010
        move.l  -0x4(a0),0x74(a6)               | +016
        move.b  #0xff,0x21(a6)                  | +01c
        lea     0x279a78.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        bra.w   .L0386de                        | +02e
.L0386ba:
        move.b  #0x30,0x70(a6)                  | +032
        lea     0x279a78.l,a0                   | +038
        move.l  -0x4(a0),0x74(a6)               | +03e
        move.b  #0xff,0x21(a6)                  | +044
        lea     0x279a78.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
.L0386de:
        bra.w   Player_CrawlRight_Setup_038482 | +056

| ----------------------------------------------------------------------------
|  Player_CrawlLeft_Alt_0386e2  @ $0386E2  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrawlLeft_Alt_0386e2, "ax", @progbits
        .global Player_CrawlLeft_Alt_0386e2
Player_CrawlLeft_Alt_0386e2:
        jsr     0x2abcc.l                       | +000
        bcs.w   .L038714                        | +006
        move.b  #0x30,0x70(a6)                  | +00a
        lea     0x279a78.l,a0                   | +010
        move.l  -0x4(a0),0x74(a6)               | +016
        move.b  #0xff,0x21(a6)                  | +01c
        lea     0x279a78.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        bra.w   .L038738                        | +02e
.L038714:
        move.b  #0x30,0x70(a6)                  | +032
        lea     0x279a78.l,a0                   | +038
        move.l  -0x4(a0),0x74(a6)               | +03e
        move.b  #0xff,0x21(a6)                  | +044
        lea     0x279a78.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
.L038738:
        bra.w   Player_CrawlLeft_Setup_0385b0 | +056

| ----------------------------------------------------------------------------
|  Player_CrouchShoot_03873c  @ $03873C  (436 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrouchShoot_03873c, "ax", @progbits
        .global Player_CrouchShoot_03873c
Player_CrouchShoot_03873c:
        bset    #0x1,0x8c(a6)                   | +000
        bclr    #0x1,0x8c(a6)                   | +006
        move.l  #0x32598,0x60(a6)               | +00c
        jsr     0x267e6.l                       | +014
        clr.w   0x28(a6)                        | +01a
        clr.w   0x2c(a6)                        | +01e
        jsr     0x2abcc.l                       | +022
        bcs.w   .L038790                        | +028
        move.b  #0x30,0x70(a6)                  | +02c
        lea     0x279d06.l,a0                   | +032
        move.l  -0x4(a0),0x74(a6)               | +038
        move.b  #0xff,0x21(a6)                  | +03e
        lea     0x279d06.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        bra.w   .L0387b4                        | +050
.L038790:
        move.b  #0x30,0x70(a6)                  | +054
        lea     0x279d06.l,a0                   | +05a
        move.l  -0x4(a0),0x74(a6)               | +060
        move.b  #0xff,0x21(a6)                  | +066
        lea     0x279d06.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
.L0387b4:
        lea     Sub_00032734(pc),a0             | +078
        move.l  a0,0x48(a6)                     | +07c
        lea     .L0387c2(pc),a1                 | +080
        move.l  a1,(a6)                         | +084
.L0387c2:
        bset    #0x6,0x8d(a6)                   | +086
        jsr     Player_FrameCommon_032ff2(pc)   | +08c
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +090
        jsr     0x27a92.l                       | +094
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +09a
        bcc.w   .L0387e4                        | +09e
        lea     Player_CrouchIdleB_0381a2(pc),a1 | +0a2
        move.l  a1,(a6)                         | +0a6
.L0387e4:
        btst    #0x2,0x8c(a6)                   | +0a8
        bne.w   .L038882                        | +0ae
        btst    #0x0,0x3a(a6)                   | +0b2
        bne.w   .L03880a                        | +0b8
        jsr     Input_RightThunk_032e42(pc)     | +0bc
        bcc.w   .L038806                        | +0c0
        lea     Player_CrawlLeft_Alt_0386e2(pc),a1 | +0c4
        move.l  a1,(a6)                         | +0c8
.L038806:
        bra.w   .L038818                        | +0ca
.L03880a:
        jsr     JmpAbsThunk_032e3c(pc)          | +0ce
        bcc.w   .L038818                        | +0d2
        lea     Player_CrawlRight_Alt_038688(pc),a1 | +0d6
        move.l  a1,(a6)                         | +0da
.L038818:
        jsr     Player_ActionSelect_0330d0(pc)  | +0dc
        bcc.w   .L03884e                        | +0e0
        cmpi.b  #0xff,d1                        | +0e4
        bne.w   .L038832                        | +0e8
        lea     Sub_00038A28(pc),a1             | +0ec
        move.l  a1,(a6)                         | +0f0
        bra.w   .L03884e                        | +0f2
.L038832:
        cmpi.b  #0x3,d1                         | +0f6
        bne.w   .L038844                        | +0fa
        lea     Sub_000388F0(pc),a1             | +0fe
        move.l  a1,(a6)                         | +102
        bra.w   .L03884e                        | +104
.L038844:
        lea     Player_CrouchShoot_03873c(pc),a1 | +108
        move.l  a1,(a6)                         | +10c
        bra.w   .L03884e                        | +10e
.L03884e:
        cmpi.b  #0x0,0x85(a6)                   | +112
        bne.w   .L038868                        | +118
        cmpi.b  #0x1,0x71(a6)                   | +11c
        bne.w   .L038868                        | +122
        lea     Sub_00038AE6(pc),a1             | +126
        move.l  a1,(a6)                         | +12a
.L038868:
        cmpi.w  #0x0,0x82(a6)                   | +12c
        bne.w   .L038882                        | +132
        cmpi.b  #0x0,0x71(a6)                   | +136
        beq.w   .L038882                        | +13c
        lea     Player_CrouchWeaponEmpty_038086(pc),a1 | +140
        move.l  a1,(a6)                         | +144
.L038882:
        jsr     0x5cef8.l                       | +146
        bcs.w   .L038892                        | +14c
        lea     Player_CrouchExit_037ec2(pc),a1 | +150
        move.l  a1,(a6)                         | +154
.L038892:
        jsr     Input_FireByMode_033034(pc)     | +156
        bcc.w   .L0388a0                        | +15a
        lea     Player_JumpStart_036914(pc),a1  | +15e
        move.l  a1,(a6)                         | +162
.L0388a0:
        jsr     0x27eba.l                       | +164
        bcc.w   .L0388b0                        | +16a
        lea     Player_KnockbackDelay_036fc2(pc),a1 | +16e
        move.l  a1,(a6)                         | +172
.L0388b0:
        jsr     PlayerRoute_PublishState_033522(pc) | +174
        cmpi.b  #0x3,0x106ece.l                 | +178
        beq.w   .L0388d4                        | +180
        movea.l #0xffffffff,a0                  | +184
        lea     Sub_000324C6(pc),a0             | +18a
        jsr     0x5dd56.l                       | +18e
        bra.w   .L0388e4                        | +194
.L0388d4:
        movea.l #0xffffffff,a0                  | +198
        lea     Sub_000324BC(pc),a0             | +19e
        jsr     0x5dd56.l                       | +1a2
.L0388e4:
        bcc.w   .L0388ee                        | +1a8
        lea     Player_DeathPit_037b8e(pc),a1   | +1ac
        move.l  a1,(a6)                         | +1b0
.L0388ee:
        rts                                     | +1b2
