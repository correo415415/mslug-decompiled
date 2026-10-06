| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $0539F0..$053F96  (1,316 B, 20 entradas, 12 huecos)
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
|  Prop_IndestructibleChild_0539f0  @ $0539F0  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_IndestructibleChild_0539f0, "ax", @progbits
        .global Prop_IndestructibleChild_0539f0
Prop_IndestructibleChild_0539f0:
        move.w  #0x1af,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        move.w  #0xf000,0x38(a6)                | +010
        lea     0x2977d0.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L053a18(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L053a18:
        jsr     0x2783a.l                       | +028
        move.w  0x107fe8.l,d0                   | +02e
        sub.w   d0,0x22(a6)                     | +034
        move.w  0x107fea.l,d0                   | +038
        add.w   d0,0x24(a6)                     | +03e
        jsr     0x28d70.l                       | +042
        bcc.w   JsrPcRts_053a40                 | +048

| ----------------------------------------------------------------------------
|  Prop_TrapFlame_053a42  @ $053A42  (348 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TrapFlame_053a42, "ax", @progbits
        .global Prop_TrapFlame_053a42
Prop_TrapFlame_053a42:
        move.w  #0x1e4,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        move.w  #0x4000,0x38(a6)                | +010
        jsr     0x27cee.l                       | +016
        lea     0x2977e0.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        move.w  #0x0,0x72(a6)                   | +028
        move.b  #0x0,0x20(a6)                   | +02e
        lea     Prop_TrapFlameBase_053b9e(pc),a1 | +034
        jsr     0x4ae.l                         | +038
        lea     .L053a86(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L053a86:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x5e9b6.l                       | +050
        andi.b  #0x7,d0                         | +056
        bne.w   .L053aaa                        | +05a
        move.w  #0x1098,d0                      | +05e
        jsr     0x2352.l                        | +062
.L053aaa:
        cmpi.w  #0x0,0x72(a6)                   | +068
        ble.w   .L053abc                        | +06e
        subq.w  #0x1,0x72(a6)                   | +072
        bra.w   .L053b7a                        | +076
.L053abc:
        lea     0x298896.l,a0                   | +07a
        move.l  a0,0x4c(a6)                     | +080
        jsr     0x283ca.l                       | +084
        jsr     0x283ca.l                       | +08a
        jsr     0x283d8.l                       | +090
        btst    #0x1,0x13(a6)                   | +096
        beq.w   .L053af4                        | +09c
        bclr    #0x1,0x13(a6)                   | +0a0
        move.l  #0xffffffff,0x50(a6)            | +0a6
        bra.w   .L053b58                        | +0ae
.L053af4:
        lea     0x2987ee.l,a0                   | +0b2
        move.l  a0,0x4c(a6)                     | +0b8
        jsr     0x283ca.l                       | +0bc
        jsr     0x283ca.l                       | +0c2
        jsr     0x283d8.l                       | +0c8
        btst    #0x1,0x13(a6)                   | +0ce
        beq.w   .L053b28                        | +0d4
        lea     0x100440.l,a0                   | +0d8
        move.l  a0,0x50(a6)                     | +0de
        bra.w   .L053b58                        | +0e2
.L053b28:
        lea     0x298842.l,a0                   | +0e6
        move.l  a0,0x4c(a6)                     | +0ec
        jsr     0x283ca.l                       | +0f0
        jsr     0x283ca.l                       | +0f6
        jsr     0x283d8.l                       | +0fc
        btst    #0x1,0x13(a6)                   | +102
        beq.w   .L053b7a                        | +108
        lea     0x1004e0.l,a0                   | +10c
        move.l  a0,0x50(a6)                     | +112
.L053b58:
        bclr    #0x1,0x13(a6)                   | +116
        lea     Prop_TrapFlameHitSpawn_053c44(pc),a1 | +11c
        jsr     0x4ae.l                         | +120
        jsr     0x5dd02.l                       | +126
        move.l  0x50(a6),0x50(a0)               | +12c
        move.w  #0xc,0x72(a6)                   | +132
.L053b7a:
        movea.l #0xffffffff,a0                  | +138
        lea     0x298740.l,a0                   | +13e
        jsr     0x5dd5c.l                       | +144
        bcc.w   .L053b9c                        | +14a
        move.b  #0xff,0x20(a6)                  | +14e
        jmp     0x518.l                         | +154
.L053b9c:
        rts                                     | +15a

| ----------------------------------------------------------------------------
|  Prop_TrapFlameBase_053b9e  @ $053B9E  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TrapFlameBase_053b9e, "ax", @progbits
        .global Prop_TrapFlameBase_053b9e
Prop_TrapFlameBase_053b9e:
        move.w  #0x1e4,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        move.w  #0xf000,0x38(a6)                | +010
        jsr     0x27cee.l                       | +016
        lea     0x297856.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        move.w  #0x0,0x72(a6)                   | +028
        lea     .L053bd2(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L053bd2:
        movea.l 0xc(a6),a0                      | +034
        move.w  0x22(a0),0x22(a6)               | +038
        move.w  0x24(a0),0x24(a6)               | +03e
        jsr     0x28d70.l                       | +044
        movea.l 0xc(a6),a0                      | +04a
        cmpi.b  #0xff,0x20(a0)                  | +04e
        bne.w   .L053bfc                        | +054
        jmp     0x518.l                         | +058
.L053bfc:
        rts                                     | +05e

| ----------------------------------------------------------------------------
|  Prop_TrapFlameHitFx_053bfe  @ $053BFE  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TrapFlameHitFx_053bfe, "ax", @progbits
        .global Prop_TrapFlameHitFx_053bfe
Prop_TrapFlameHitFx_053bfe:
        move.w  #0x1e4,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        move.w  #0x8000,0x38(a6)                | +010
        jsr     0x27cee.l                       | +016
        lea     0x2978cc.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L053c2c(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L053c2c:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        bcc.w   .L053c42                        | +03a
        jmp     0x518.l                         | +03e
.L053c42:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Prop_TrapFlameHitSpawn_053c44  @ $053C44  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TrapFlameHitSpawn_053c44, "ax", @progbits
        .global Prop_TrapFlameHitSpawn_053c44
Prop_TrapFlameHitSpawn_053c44:
        lea     Prop_TrapFlameHitFx_053bfe(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a

| ----------------------------------------------------------------------------
|  Prop_BurnFollowVictim_053c64  @ $053C64  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BurnFollowVictim_053c64, "ax", @progbits
        .global Prop_BurnFollowVictim_053c64
Prop_BurnFollowVictim_053c64:
        movea.l 0x50(a6),a0                     | +000
        move.l  a0,d0                           | +004
        cmpi.l  #0xffffffff,d0                  | +006
        bne.w   .L053c7c                        | +00c
        jmp     0x518.l                         | +010
        rts                                     | +016
.L053c7c:
        move.b  0x45(a0),d0                     | +018
        cmpi.b  #0x0,d0                         | +01c
        bgt.w   .L053c9e                        | +020
        move.w  0x28(a0),d0                     | +024
        asl.w   #0x1,d0                         | +028
        move.w  d0,0x28(a0)                     | +02a
        move.w  #0x1000,0x2a(a0)                | +02e
        move.w  #0x19,0x72(a6)                  | +034
.L053c9e:
        lea     .L053ca4(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L053ca4:
        movea.l 0x50(a6),a0                     | +040
        move.w  0x22(a0),0x22(a6)               | +044
        move.w  0x24(a0),0x24(a6)               | +04a
        move.w  0x38(a0),0x38(a6)               | +050
        move.w  0x72(a6),d0                     | +056
        andi.w  #0x3,d0                         | +05a
        bne.w   .L053ce2                        | +05e
        lea     Prop_BurnSmokePuff_053cf2(pc),a1 | +062
        jsr     0x4ae.l                         | +066
        jsr     0x5dd22.l                       | +06c
        addi.w  #0x10,0x24(a0)                  | +072
        move.w  0x38(a6),0x38(a0)               | +078
.L053ce2:
        subq.w  #0x1,0x72(a6)                   | +07e
        bpl.w   .L053cf0                        | +082
        jmp     0x518.l                         | +086
.L053cf0:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  Prop_BurnSmokePuff_053cf2  @ $053CF2  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BurnSmokePuff_053cf2, "ax", @progbits
        .global Prop_BurnSmokePuff_053cf2
Prop_BurnSmokePuff_053cf2:
        move.w  #0x2,d1                         | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        lea     0x297922.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L053d14(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L053d14:
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        bcc.w   .L053d2a                        | +02e
        jmp     0x518.l                         | +032
.L053d2a:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  Prop_SpawnBonusOnce_053d2c  @ $053D2C  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_SpawnBonusOnce_053d2c, "ax", @progbits
        .global Prop_SpawnBonusOnce_053d2c
Prop_SpawnBonusOnce_053d2c:
        cmpi.b  #0x0,0x21(a6)                   | +000
        bne.w   .L053d4e                        | +006
        move.b  #0xff,0x21(a6)                  | +00a
        jsr     0x9ba56.l                       | +010
        subi.w  #0x18,0x22(a0)                  | +016
        addi.w  #0x30,0x24(a0)                  | +01c
.L053d4e:
        rts                                     | +022

| ----------------------------------------------------------------------------
|  Prop_ToggleSpriteByTimer_053d50  @ $053D50  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_ToggleSpriteByTimer_053d50, "ax", @progbits
        .global Prop_ToggleSpriteByTimer_053d50
Prop_ToggleSpriteByTimer_053d50:
        cmpi.w  #0x0,0x72(a6)                   | +000
        ble.w   .L053d78                        | +006
        clr.l   d0                              | +00a
        move.w  0x72(a6),d0                     | +00c
        btst    #0x0,d0                         | +010
        beq.w   .L053d70                        | +014
        move.w  0x16(a6),0x14(a6)               | +018
        rts                                     | +01e
.L053d70:
        move.w  0x18(a6),0x14(a6)               | +020
        rts                                     | +026
.L053d78:
        move.w  0x16(a6),0x14(a6)               | +028
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Prop_PickSpriteByVictimDir_053d80  @ $053D80  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_PickSpriteByVictimDir_053d80, "ax", @progbits
        .global Prop_PickSpriteByVictimDir_053d80
Prop_PickSpriteByVictimDir_053d80:
        movea.l 0x50(a6),a0                     | +000
        clr.l   d0                              | +004
        move.w  0x28(a0),d0                     | +006
        cmpi.w  #0x0,d0                         | +00a
        beq.w   JsrAbsRts_053dc8                | +00e
        cmpi.b  #0x0,0x20(a6)                   | +012
        beq.w   .L053db4                        | +018
        btst    #0xf,d0                         | +01c
        beq.w   JsrAbsRts_053dc8                | +020
        lea     0x2976d2.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        bra.w   JsrAbsRts_053dc8                | +030
.L053db4:
        btst    #0xf,d0                         | +034
        bne.w   JsrAbsRts_053dc8                | +038
        lea     0x297668.l,a0                   | +03c

| ----------------------------------------------------------------------------
|  Prop_SyncSpriteWithParent_053dca  @ $053DCA  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_SyncSpriteWithParent_053dca, "ax", @progbits
        .global Prop_SyncSpriteWithParent_053dca
Prop_SyncSpriteWithParent_053dca:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x20(a0),d0                     | +004
        cmp.b   0x20(a6),d0                     | +008
        beq.w   .L053e00                        | +00c
        cmpi.b  #0x0,0x20(a0)                   | +010
        bne.w   .L053df4                        | +016
        lea     0x297798.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        bra.w   .L053e00                        | +026
.L053df4:
        lea     0x297750.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
.L053e00:
        movea.l 0xc(a6),a0                      | +036
        move.b  0x20(a0),0x20(a6)               | +03a
        rts                                     | +040

| ----------------------------------------------------------------------------
|  Prop_PlayBreakMusicByPhase_053e0c  @ $053E0C  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_PlayBreakMusicByPhase_053e0c, "ax", @progbits
        .global Prop_PlayBreakMusicByPhase_053e0c
Prop_PlayBreakMusicByPhase_053e0c:
        cmpi.b  #0x0,0x21(a6)                   | +000
        bne.w   .L053e24                        | +006
        move.w  #0x102c,d0                      | +00a
        jsr     0x2352.l                        | +00e
        bra.w   JsrAbsRts_053e76                | +014
.L053e24:
        cmpi.b  #0x1,0x21(a6)                   | +018
        bne.w   .L053e3c                        | +01e
        move.w  #0x102c,d0                      | +022
        jsr     0x2352.l                        | +026
        bra.w   JsrAbsRts_053e76                | +02c
.L053e3c:
        cmpi.b  #0x2,0x21(a6)                   | +030
        bne.w   .L053e54                        | +036
        move.w  #0x1023,d0                      | +03a
        jsr     0x2352.l                        | +03e
        bra.w   JsrAbsRts_053e76                | +044
.L053e54:
        cmpi.b  #0x3,0x21(a6)                   | +048
        bne.w   .L053e6c                        | +04e
        move.w  #0x1035,d0                      | +052
        jsr     0x2352.l                        | +056
        bra.w   JsrAbsRts_053e76                | +05c
.L053e6c:
        move.w  #0x1023,d0                      | +060

| ----------------------------------------------------------------------------
|  Prop_RunDebrisScriptByPhase_053e78  @ $053E78  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_RunDebrisScriptByPhase_053e78, "ax", @progbits
        .global Prop_RunDebrisScriptByPhase_053e78
Prop_RunDebrisScriptByPhase_053e78:
        lea     0x297f50.l,a0                   | +000
        move.b  0x21(a6),d0                     | +006
        andi.w  #0xff,d0                        | +00a
        asl.w   #0x2,d0                         | +00e
        move.w  0x38(a6),d1                     | +010
        asl.w   #0x4,d1                         | +014
        add.w   d1,d0                           | +016
        movea.l (a0,d0.w),a2                    | +018

| ----------------------------------------------------------------------------
|  Prop_PickRandomItemPtr_053e9c  @ $053E9C  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_PickRandomItemPtr_053e9c, "ax", @progbits
        .global Prop_PickRandomItemPtr_053e9c
Prop_PickRandomItemPtr_053e9c:
        move.w  #0x3,d0                         | +000
        jsr     0x5ea1c.l                       | +004
        asl.w   #0x2,d0                         | +00a
        lea     0x2980fc.l,a0                   | +00c
        movea.l (a0,d0.w),a0                    | +012

| ----------------------------------------------------------------------------
|  Prop_RunDebrisScriptByPrio_053eba  @ $053EBA  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_RunDebrisScriptByPrio_053eba, "ax", @progbits
        .global Prop_RunDebrisScriptByPrio_053eba
Prop_RunDebrisScriptByPrio_053eba:
        lea     0x298074.l,a1                   | +000
        cmpi.w  #0x0,0x38(a6)                   | +006
        beq.w   JsrAbsThunk_053eda              | +00c
        cmpi.w  #0x6,0x38(a6)                   | +010
        beq.w   JsrAbsThunk_053eda              | +016
        lea     0x298062.l,a1                   | +01a

| ----------------------------------------------------------------------------
|  Prop_GateDebrisA_053ee2  @ $053EE2  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GateDebrisA_053ee2, "ax", @progbits
        .global Prop_GateDebrisA_053ee2
Prop_GateDebrisA_053ee2:
        lea     0x29860c.l,a2                   | +000
        jsr     0x5022a.l                       | +006
        lea     0x298620.l,a2                   | +00c
        jsr     0x5022a.l                       | +012

| ----------------------------------------------------------------------------
|  Prop_GateDebrisB_053f08  @ $053F08  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GateDebrisB_053f08, "ax", @progbits
        .global Prop_GateDebrisB_053f08
Prop_GateDebrisB_053f08:
        lea     0x298648.l,a2                   | +000
        jsr     0x5022a.l                       | +006
        lea     0x29865c.l,a2                   | +00c
        jsr     0x5022a.l                       | +012

| ----------------------------------------------------------------------------
|  Prop_GateDebrisC_053f2e  @ $053F2E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GateDebrisC_053f2e, "ax", @progbits
        .global Prop_GateDebrisC_053f2e
Prop_GateDebrisC_053f2e:
        lea     0x298684.l,a2                   | +000
        jsr     0x5022a.l                       | +006
        lea     0x298698.l,a2                   | +00c
        jsr     0x5022a.l                       | +012

| ----------------------------------------------------------------------------
|  Prop_GateDebrisD_053f54  @ $053F54  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GateDebrisD_053f54, "ax", @progbits
        .global Prop_GateDebrisD_053f54
Prop_GateDebrisD_053f54:
        lea     0x2986c0.l,a2                   | +000
        jsr     0x5022a.l                       | +006
        lea     0x2986d4.l,a2                   | +00c
        jsr     0x5022a.l                       | +012

| ----------------------------------------------------------------------------
|  Entity_CmpField10WithLink8_053f7a  @ $053F7A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpField10WithLink8_053f7a, "ax", @progbits
        .global Entity_CmpField10WithLink8_053f7a
Entity_CmpField10WithLink8_053f7a:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_053f90                    | +00c
