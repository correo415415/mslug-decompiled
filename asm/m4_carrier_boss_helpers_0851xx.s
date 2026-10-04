| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave JJJ — Mision 4: transporte, torreta, agua y helpers de spawn del boss
|  Región: $08512C..$0865BE  (4,742 B, 65 entradas, 33 huecos)
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
|  M4_PlatformSpawn_085134  @ $085134  (90 B)
| ----------------------------------------------------------------------------
        .section .text.M4_PlatformSpawn_085134, "ax", @progbits
        .global M4_PlatformSpawn_085134
M4_PlatformSpawn_085134:
        lea     0x676ba.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.w  #0x160,0x22(a0)                 | +012
        movem.l a0,-(a7)                        | +018
        move.w  #0x0,d0                         | +01c
        move.w  #0x320,d1                       | +020
        jsr     0x44022.l                       | +024
        movem.l (a7)+,a0                        | +02a
        move.w  d1,0x24(a0)                     | +02e
        move.l  #0x0,0x98(a0)                   | +032
        move.l  #0x0,0x9c(a0)                   | +03a
        move.b  #0x0,0x98(a0)                   | +042
        move.b  #0x10,0x99(a0)                  | +048
        move.b  #0x1,0x11(a0)                   | +04e
        jmp     0x518.l                         | +054

| ----------------------------------------------------------------------------
|  Rts_08518e  @ $08518E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08518e, "ax", @progbits
        .global Rts_08518e
Rts_08518e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  M4_Carrier_Init_085190  @ $085190  (88 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Carrier_Init_085190, "ax", @progbits
        .global M4_Carrier_Init_085190
M4_Carrier_Init_085190:
        move.w  #0x1c5,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2c029a.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.w  d0,0x66(a6)                     | +016
        lea     M4_Turret_Init_085394(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd22.l                       | +024
        move.b  #0x0,0x20(a6)                   | +02a
        move.b  #0x0,0x21(a6)                   | +030
        move.b  #0x0,0x89(a6)                   | +036
        lea     .L0851d2(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L0851d2:
        move.l  0x106f5c.l,d0                   | +042
        swap    d0                              | +048
        cmpi.w  #0x1f0,d0                       | +04a
        bgt.w   M4_Carrier_Approach_0851f0              | +04e
        clr.b   0x10e39a.l                      | +052

| ----------------------------------------------------------------------------
|  M4_Carrier_Approach_0851f0  @ $0851F0  (120 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Carrier_Approach_0851f0, "ax", @progbits
        .global M4_Carrier_Approach_0851f0
M4_Carrier_Approach_0851f0:
        lea     M4_SoldierDropperB_085608(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        lea     0x2e93fa.l,a0                   | +00a
        move.l  a0,0x48(a6)                     | +010
        lea     0x2e78ce.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L085216(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L085216:
        clr.b   0x10e39a.l                      | +026
        jsr     0x2783a.l                       | +02c
        move.b  #0x0,0x89(a6)                   | +032
        jsr     0x2870a.l                       | +038
        bcc.w   JsrAbsThunk_085268              | +03e
        move.b  #0xff,0x89(a6)                  | +042
        lea     0x2e78b4.l,a0                   | +048
        jsr     0x28cd4.l                       | +04e
        lea     0x5e766.l,a0                    | +054
        jsr     0x5e798.l                       | +05a
        bclr    #0x3,0x13(a6)                   | +060
        lea     0x2e994e.l,a1                   | +066
        jsr     0x77c7e.l                       | +06c
        lea     M4_Carrier_Fight_085270(pc),a1       | +072
        move.l  a1,(a6)                         | +076

| ----------------------------------------------------------------------------
|  M4_Carrier_Fight_085270  @ $085270  (192 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Carrier_Fight_085270, "ax", @progbits
        .global M4_Carrier_Fight_085270
M4_Carrier_Fight_085270:
        move.w  #0x102d,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ea858.l,a2                   | +00a
        jsr     0x5022a.l                       | +010
        lea     .L08528c(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L08528c:
        clr.b   0x10e39a.l                      | +01c
        jsr     0x2783a.l                       | +022
        move.b  #0x0,0x89(a6)                   | +028
        jsr     0x2870a.l                       | +02e
        bcc.w   .L0852cc                        | +034
        move.b  #0xff,0x89(a6)                  | +038
        lea     0x2e78de.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        lea     0x5e766.l,a0                    | +04a
        jsr     0x5e798.l                       | +050
        bclr    #0x3,0x13(a6)                   | +056
.L0852cc:
        jsr     0x28758.l                       | +05c
        bcc.w   JsrAbsThunk_085330              | +062
        lea     0xffff.w,a0                     | +066
        move.l  a0,0x48(a6)                     | +06a
        move.b  #0xff,0x20(a6)                  | +06e
        lea     0x2eaf02.l,a1                   | +074
        jsr     0x43fac.l                       | +07a
        lea     0x2e9960.l,a1                   | +080
        jsr     0x77c7e.l                       | +086
        lea     0x2e9972.l,a1                   | +08c
        jsr     0x77c7e.l                       | +092
        move.b  #0xff,0x74(a6)                  | +098
        lea     0x2e9790.l,a0                   | +09e
        move.l  a0,0x4c(a6)                     | +0a4
        jsr     0x283ca.l                       | +0a8
        jsr     0x283ca.l                       | +0ae
        jsr     0x283d8.l                       | +0b4
        lea     M4_Carrier_Wreck_085338(pc),a1       | +0ba
        move.l  a1,(a6)                         | +0be

| ----------------------------------------------------------------------------
|  M4_Carrier_Wreck_085338  @ $085338  (92 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Carrier_Wreck_085338, "ax", @progbits
        .global M4_Carrier_Wreck_085338
M4_Carrier_Wreck_085338:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x4c(a6)                     | +004
        jsr     0x283ca.l                       | +008
        move.w  #0x1028,d0                      | +00e
        jsr     0x2352.l                        | +012
        jsr     M4_SetupSprites4_085b7e(pc)          | +018
        move.w  #0x74,d0                        | +01c
        move.b  #0x0,0x74(a6)                   | +020
        lea     0xec6c8.l,a1                    | +026
        jsr     0x4429e.l                       | +02c
        lea     0x2ea86c.l,a2                   | +032
        jsr     0x5022a.l                       | +038
        lea     .L08537c(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L08537c:
        clr.b   0x10e39a.l                      | +044
        cmpi.b  #0xff,0x21(a6)                  | +04a
        bne.w   .L085392                        | +050
        jmp     0x518.l                         | +054
.L085392:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  M4_Turret_Init_085394  @ $085394  (118 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Turret_Init_085394, "ax", @progbits
        .global M4_Turret_Init_085394
M4_Turret_Init_085394:
        lea     M4_Wheel_0856aa(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        movea.l 0xc(a6),a1                      | +00a
        move.w  0x22(a1),0x22(a0)               | +00e
        move.w  0x24(a1),0x24(a0)               | +014
        move.w  #0x100,d0                       | +01a
        move.w  #0x2f6,d1                       | +01e
        jsr     0x44022.l                       | +022
        move.w  d0,0x22(a6)                     | +028
        move.w  d1,0x24(a6)                     | +02c
        move.w  #0x1c8,d1                       | +030
        jsr     0x236e.l                        | +034
        lea     0x2e7908.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        move.w  #0x100,0x38(a6)                 | +046
        move.l  #0x2eb0b8,0x60(a6)              | +04c
        lea     .L0853ee(pc),a1                 | +054
        move.l  a1,(a6)                         | +058
.L0853ee:
        move.l  0x106f5c.l,d0                   | +05a
        swap    d0                              | +060
        cmpi.w  #0x1f0,d0                       | +062
        bgt.w   M4_Turret_Active_085412              | +066
        jsr     0x28998.l                       | +06a
        jsr     0x2783a.l                       | +070

| ----------------------------------------------------------------------------
|  M4_Turret_Active_085412  @ $085412  (106 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Turret_Active_085412, "ax", @progbits
        .global M4_Turret_Active_085412
M4_Turret_Active_085412:
        jsr     0x267e2.l                       | +000
        bclr    #0x1,0x12(a6)                   | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x4c(a6)                     | +010
        jsr     0x283ca.l                       | +014
        lea     M4_TurretMount_085526(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        lea     .L08543c(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L08543c:
        jsr     0x28998.l                       | +02a
        jsr     0x27cee.l                       | +030
        jsr     M4_Turret_EdgeCheck_085d32(pc)          | +036
        jsr     M4_Turret_Shake_085ebc(pc)          | +03a
        jsr     0x28d70.l                       | +03e
        jsr     M4_Turret_Knockback_085da8(pc)          | +044
        movea.l 0xc(a6),a0                      | +048
        cmpi.b  #0xff,0x20(a0)                  | +04c
        bne.w   SetHandlerRts_085482            | +052
        lea     0xffff.w,a0                     | +056
        move.l  a0,0x48(a6)                     | +05a
        lea     0x2e9984.l,a1                   | +05e
        jsr     0x77c7e.l                       | +064

| ----------------------------------------------------------------------------
|  M4_Turret_Death_085484  @ $085484  (100 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Turret_Death_085484, "ax", @progbits
        .global M4_Turret_Death_085484
M4_Turret_Death_085484:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        move.w  #0x1033,d0                      | +008
        jsr     0x2352.l                        | +00c
        lea     0x2e9996.l,a1                   | +012
        jsr     0x77c7e.l                       | +018
        jsr     0x267e2.l                       | +01e
        move.w  #0xffe0,0x2e(a6)                | +024
        lea     0x2e7940.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L0854c0(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L0854c0:
        jsr     0x27cee.l                       | +03c
        jsr     0x28d70.l                       | +042
        cmpi.w  #0x20,0x24(a6)                  | +048
        bgt.w   .L0854e6                        | +04e
        movea.l 0xc(a6),a0                      | +052
        move.b  #0xff,0x21(a0)                  | +056
        jmp     0x518.l                         | +05c
.L0854e6:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  M4_SmokePuff_0854e8  @ $0854E8  (62 B)
| ----------------------------------------------------------------------------
        .section .text.M4_SmokePuff_0854e8, "ax", @progbits
        .global M4_SmokePuff_0854e8
M4_SmokePuff_0854e8:
        move.w  #0x2,0x72(a6)                   | +000
        lea     .L0854f4(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0854f4:
        jsr     0x2783a.l                       | +00c
        lea     0x2e96ec.l,a0                   | +012
        move.l  a0,0x4c(a6)                     | +018
        jsr     0x283ca.l                       | +01c
        jsr     0x283ca.l                       | +022
        jsr     0x283d8.l                       | +028
        subq.w  #0x1,0x72(a6)                   | +02e
        bpl.w   .L085524                        | +032
        jmp     0x518.l                         | +036
.L085524:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  M4_TurretMount_085526  @ $085526  (94 B)
| ----------------------------------------------------------------------------
        .section .text.M4_TurretMount_085526, "ax", @progbits
        .global M4_TurretMount_085526
M4_TurretMount_085526:
        lea     0x2e7a10.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L085538(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L085538:
        movea.l 0xc(a6),a0                      | +012
        move.w  0x22(a0),0x22(a6)               | +016
        move.w  0x24(a0),0x24(a6)               | +01c
        jsr     0x28d70.l                       | +022
        jsr     0x2870a.l                       | +028
        bcc.w   .L08556a                        | +02e
        lea     0x5e766.l,a0                    | +032
        jsr     0x5e798.l                       | +038
        bclr    #0x3,0x13(a6)                   | +03e
.L08556a:
        movea.l 0xc(a6),a0                      | +044
        movea.l 0xc(a0),a0                      | +048
        cmpi.b  #0xff,0x21(a0)                  | +04c
        bne.w   .L085582                        | +052
        jmp     0x518.l                         | +056
.L085582:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  M4_SoldierDropper_085584  @ $085584  (126 B)
| ----------------------------------------------------------------------------
        .section .text.M4_SoldierDropper_085584, "ax", @progbits
        .global M4_SoldierDropper_085584
M4_SoldierDropper_085584:
        move.w  #0x180,d0                       | +000
        move.w  #0x258,d1                       | +004
        jsr     0x44022.l                       | +008
        move.w  d0,0x22(a6)                     | +00e
        move.w  d1,0x24(a6)                     | +012
        move.w  #0x0,0x72(a6)                   | +016
        lea     .L0855a6(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L0855a6:
        jsr     0x2783a.l                       | +022
        movea.l 0xc(a6),a0                      | +028
        cmpi.b  #0xff,0x20(a0)                  | +02c
        bne.w   .L0855c2                        | +032
        jmp     0x518.l                         | +036
        rts                                     | +03c
.L0855c2:
        jsr     0x6f0.l                         | +03e
        bcs.w   SetTaskWRts_085606              | +044
        cmpi.w  #0x0,0x72(a6)                   | +048
        ble.w   .L0855de                        | +04e
        subq.w  #0x1,0x72(a6)                   | +052
        bra.w   SetTaskWRts_085606              | +056
.L0855de:
        lea     0x483e2.l,a1                    | +05a
        jsr     0x4ae.l                         | +060
        jsr     0x5dd22.l                       | +066
        move.b  #0x0,0x11(a0)                   | +06c
        lea     0x2c0420.l,a0                   | +072
        jsr     0x799de.l                       | +078

| ----------------------------------------------------------------------------
|  M4_SoldierDropperB_085608  @ $085608  (156 B)
| ----------------------------------------------------------------------------
        .section .text.M4_SoldierDropperB_085608, "ax", @progbits
        .global M4_SoldierDropperB_085608
M4_SoldierDropperB_085608:
        move.w  #0x180,d0                       | +000
        move.w  #0x2ff,d1                       | +004
        jsr     0x44022.l                       | +008
        move.w  d0,0x22(a6)                     | +00e
        move.w  d1,0x24(a6)                     | +012
        move.w  #0x0,0x72(a6)                   | +016
        lea     .L08562a(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L08562a:
        jsr     0x2783a.l                       | +022
        movea.l 0xc(a6),a0                      | +028
        cmpi.b  #0xff,0x20(a0)                  | +02c
        bne.w   .L085646                        | +032
        jmp     0x518.l                         | +036
        rts                                     | +03c
.L085646:
        jsr     0x6f0.l                         | +03e
        bcs.w   SetTaskWRts_0856a8              | +044
        cmpi.w  #0x0,0x72(a6)                   | +048
        ble.w   .L085662                        | +04e
        subq.w  #0x1,0x72(a6)                   | +052
        bra.w   SetTaskWRts_0856a8              | +056
.L085662:
        lea     0x483e2.l,a1                    | +05a
        jsr     0x4ae.l                         | +060
        jsr     0x5dd22.l                       | +066
        move.b  #0x0,0x11(a0)                   | +06c
        move.b  #0xf,0x98(a0)                   | +072
        move.b  #0x0,0x99(a0)                   | +078
        move.b  #0x0,0x9a(a0)                   | +07e
        move.b  #0x1,0x9b(a0)                   | +084
        move.b  #0x1e,0x9c(a0)                  | +08a
        lea     0x2c0420.l,a0                   | +090
        jsr     0x799de.l                       | +096

| ----------------------------------------------------------------------------
|  M4_Wheel_0856aa  @ $0856AA  (144 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Wheel_0856aa, "ax", @progbits
        .global M4_Wheel_0856aa
M4_Wheel_0856aa:
        move.w  #0x1d0,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1d1,d1                       | +00a
        jsr     0x236e.l                        | +00e
        addi.w  #0x30,0x22(a6)                  | +014
        addi.w  #0x61,0x24(a6)                  | +01a
        clr.l   d0                              | +020
        move.b  d0,0x88(a6)                     | +022
        movea.l #0x2e7bde,a0                    | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L0856ec                        | +038
        jsr     0x28cd4.l                       | +03c
.L0856ec:
        move.w  #0x0,0x38(a6)                   | +042
        lea     .L0856f8(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L0856f8:
        jsr     0x2783a.l                       | +04e
        move.w  0x16(a6),0x14(a6)               | +054
        movea.l 0xc(a6),a0                      | +05a
        movea.l 0xc(a0),a0                      | +05e
        cmpi.b  #0xff,0x89(a0)                  | +062
        bne.w   .L08571c                        | +068
        move.w  0x18(a6),0x14(a6)               | +06c
.L08571c:
        jsr     M4_Wheel_Anim_085c32(pc)          | +072
        movea.l 0xc(a6),a0                      | +076
        movea.l 0xc(a0),a0                      | +07a
        cmpi.b  #0xff,0x20(a0)                  | +07e
        bne.w   .L085738                        | +084
        jmp     0x518.l                         | +088
.L085738:
        rts                                     | +08e

| ----------------------------------------------------------------------------
|  M4_SplashFx_08573a  @ $08573A  (64 B)
| ----------------------------------------------------------------------------
        .section .text.M4_SplashFx_08573a, "ax", @progbits
        .global M4_SplashFx_08573a
M4_SplashFx_08573a:
        move.w  #0x1b3,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2e7950.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x0,0x38(a6)                   | +016
        jsr     0x267e2.l                       | +01c
        lea     .L085762(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L085762:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L085778                        | +034
        jmp     0x518.l                         | +038
.L085778:
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  M4_Rail_SpawnRow_08577a  @ $08577A  (56 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Rail_SpawnRow_08577a, "ax", @progbits
        .global M4_Rail_SpawnRow_08577a
M4_Rail_SpawnRow_08577a:
        clr.b   d0                              | +000
.L08577c:
        movem.w d0,-(a7)                        | +002
        lea     M4_Rail_Segment_0857b4(pc),a1       | +006
        jsr     0x4ae.l                         | +00a
        jsr     0x5dd22.l                       | +010
        movem.w (a7)+,d0                        | +016
        move.b  d0,0x21(a0)                     | +01a
        addi.w  #0x10,0x22(a6)                  | +01e
        addq.b  #0x1,d0                         | +024
        cmpi.b  #0x14,d0                        | +026
        blt.b   .L08577c                        | +02a
        lea     .L0857ac(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L0857ac:
        jmp     0x518.l                         | +032

| ----------------------------------------------------------------------------
|  Rts_0857b2  @ $0857B2  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_0857b2, "ax", @progbits
        .global Rts_0857b2
Rts_0857b2:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  M4_Rail_Segment_0857b4  @ $0857B4  (62 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Rail_Segment_0857b4, "ax", @progbits
        .global M4_Rail_Segment_0857b4
M4_Rail_Segment_0857b4:
        lea     .L0857ba(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L0857ba:
        jsr     0x2783a.l                       | +006
        movea.l #0xffffffff,a0                  | +00c
        lea     0x2e986a.l,a0                   | +012
        jsr     0x5dd5c.l                       | +018
        bcs.w   .L0857ea                        | +01e
        lea     0x2eb104.l,a0                   | +022
        jsr     0x5e086.l                       | +028
        bcs.w   .L0857f0                        | +02e
        jsr     M4_Rail_Blit_085d04(pc)          | +032
.L0857ea:
        jmp     0x518.l                         | +036
.L0857f0:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  M4_Water_Spawn2_0857f2  @ $0857F2  (44 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Water_Spawn2_0857f2, "ax", @progbits
        .global M4_Water_Spawn2_0857f2
M4_Water_Spawn2_0857f2:
        lea     M4_Water_InitP1_085820(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        lea     M4_Water_InitP2_08582a(pc),a1       | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd22.l                       | +01a
        lea     .L085818(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L085818:
        jmp     0x518.l                         | +026

| ----------------------------------------------------------------------------
|  Rts_08581e  @ $08581E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08581e, "ax", @progbits
        .global Rts_08581e
Rts_08581e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  M4_Water_InitP1_085820  @ $085820  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Water_InitP1_085820, "ax", @progbits
        .global M4_Water_InitP1_085820
M4_Water_InitP1_085820:
        move.b  #0x0,0x20(a6)                   | +000
        bra.w   M4_Water_Body_085830     | +006

| ----------------------------------------------------------------------------
|  M4_Water_InitP2_08582a  @ $08582A  (166 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Water_InitP2_08582a, "ax", @progbits
        .global M4_Water_InitP2_08582a
M4_Water_InitP2_08582a:
        move.b  #0x1,0x20(a6)                   | +000
        .global M4_Water_Body_085830
M4_Water_Body_085830:
        lea     0x2e79fa.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L085842(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L085842:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        movea.l #0xffffffff,a0                  | +024
        lea     0x2e986a.l,a0                   | +02a
        jsr     0x5dd5c.l                       | +030
        bcc.w   .L08586c                        | +036
        jmp     0x518.l                         | +03a
        rts                                     | +040
.L08586c:
        jsr     0x6f0.l                         | +042
        bcs.w   .L0858ce                        | +048
        lea     0x100440.l,a0                   | +04c
        movea.l a0,a4                           | +052
        cmpi.b  #0x0,0x20(a6)                   | +054
        beq.w   .L085890                        | +05a
        lea     0x1004e0.l,a0                   | +05e
        movea.l a0,a4                           | +064
.L085890:
        lea     0x2eb10e.l,a1                   | +066
        jsr     0x5e260.l                       | +06c
        bcs.w   .L0858ce                        | +072
        cmpi.w  #0x0,0x28(a4)                   | +076
        beq.w   .L0858ce                        | +07c
        movem.l a4,-(a7)                        | +080
        lea     M4_SplashFx_08573a(pc),a1       | +084
        jsr     0x4ae.l                         | +088
        movem.l (a7)+,a4                        | +08e
        move.w  0x22(a4),0x22(a0)               | +092
        move.w  0x24(a4),d0                     | +098
        subi.w  #0x8,d0                         | +09c
        move.w  d0,0x24(a0)                     | +0a0
.L0858ce:
        rts                                     | +0a4

| ----------------------------------------------------------------------------
|  M4_Debris_Init_0858d0  @ $0858D0  (34 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Debris_Init_0858d0, "ax", @progbits
        .global M4_Debris_Init_0858d0
M4_Debris_Init_0858d0:
        move.w  #0x1c9,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x0,0x38(a6)                   | +00a
        jsr     0x267e2.l                       | +010
        move.b  0x99(a6),d0                     | +016
        andi.b  #0x1,d0                         | +01a
        move.b  d0,0x3a(a6)                     | +01e

| ----------------------------------------------------------------------------
|  M4_Debris_Idle_0858f2  @ $0858F2  (108 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Debris_Idle_0858f2, "ax", @progbits
        .global M4_Debris_Idle_0858f2
M4_Debris_Idle_0858f2:
        clr.l   d0                              | +000
        move.b  0x98(a6),d0                     | +002
        andi.b  #0x3,d0                         | +006
        movea.l #0x2e7b2e,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L085918                        | +01c
        jsr     0x28cd4.l                       | +020
.L085918:
        lea     .L08591e(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L08591e:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        lea     0x2eb118.l,a0                   | +038
        jsr     0x5e086.l                       | +03e
        bcs.w   .L085940                        | +044
        lea     M4_Debris_Hit_08595e(pc),a1       | +048
        move.l  a1,(a6)                         | +04c
.L085940:
        movea.l #0xffffffff,a0                  | +04e
        lea     0x2e9874.l,a0                   | +054
        jsr     0x5dd5c.l                       | +05a
        bcc.w   .L08595c                        | +060
        jmp     0x518.l                         | +064
.L08595c:
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  M4_Debris_Hit_08595e  @ $08595E  (118 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Debris_Hit_08595e, "ax", @progbits
        .global M4_Debris_Hit_08595e
M4_Debris_Hit_08595e:
        clr.l   d0                              | +000
        move.b  0x98(a6),d0                     | +002
        andi.b  #0x3,d0                         | +006
        movea.l #0x2e7b3e,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L085984                        | +01c
        jsr     0x28cd4.l                       | +020
.L085984:
        lea     .L08598a(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L08598a:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        bcc.w   .L0859b6                        | +038
        lea     M4_Debris_Hit_08595e(pc),a1       | +03c
        move.l  a1,(a6)                         | +040
        lea     0x2eb118.l,a0                   | +042
        jsr     0x5e086.l                       | +048
        bcc.w   .L0859b6                        | +04e
        lea     M4_Debris_Idle_0858f2(pc),a1       | +052
        move.l  a1,(a6)                         | +056
.L0859b6:
        movea.l #0xffffffff,a0                  | +058
        lea     0x2e9874.l,a0                   | +05e
        jsr     0x5dd5c.l                       | +064
        bcc.w   .L0859d2                        | +06a
        jmp     0x518.l                         | +06e
.L0859d2:
        rts                                     | +074

| ----------------------------------------------------------------------------
|  M4_CamFloor_Init_0859d4  @ $0859D4  (52 B)
| ----------------------------------------------------------------------------
        .section .text.M4_CamFloor_Init_0859d4, "ax", @progbits
        .global M4_CamFloor_Init_0859d4
M4_CamFloor_Init_0859d4:
        move.b  #0x0,0x20(a6)                   | +000
        move.w  #0x430,0x10816a.l               | +006
        move.w  #0x430,0x10816e.l               | +00e
        move.w  #0xa0,0x22(a6)                  | +016
        btst    #0x0,0x100001.l                 | +01c
        beq.w   M4_CamFloor_Step_085a08              | +024
        lea     0x2e7bfe.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e

| ----------------------------------------------------------------------------
|  M4_CamFloor_Step_085a08  @ $085A08  (190 B)
| ----------------------------------------------------------------------------
        .section .text.M4_CamFloor_Step_085a08, "ax", @progbits
        .global M4_CamFloor_Step_085a08
M4_CamFloor_Step_085a08:
        clr.l   d0                              | +000
        move.b  0x20(a6),d0                     | +002
        asl.l   #0x1,d0                         | +006
        lea     0xec882.l,a0                    | +008
        move.w  (a0,d0.w),d1                    | +00e
        cmpi.w  #0xffff,d1                      | +012
        bne.w   .L085a2a                        | +016
        jmp     0x518.l                         | +01a
        rts                                     | +020
.L085a2a:
        jsr     0x44022.l                       | +022
        move.w  d1,0x24(a6)                     | +028
        lea     .L085a3a(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L085a3a:
        move.w  0x22(a6),d0                     | +032
        move.w  0x24(a6),d1                     | +036
        movem.w d0,-(a7)                        | +03a
        movem.w d1,-(a7)                        | +03e
        move.w  #0x0,0x22(a6)                   | +042
        move.w  #0xfc,0x24(a6)                  | +048
        lea     0x2e97e4.l,a0                   | +04e
        move.l  a0,0x4c(a6)                     | +054
        jsr     0x283ca.l                       | +058
        jsr     0x283ca.l                       | +05e
        jsr     0x283d8.l                       | +064
        movem.w (a7)+,d1                        | +06a
        movem.w (a7)+,d0                        | +06e
        move.w  d0,0x22(a6)                     | +072
        move.w  d1,0x24(a6)                     | +076
        jsr     0x2783a.l                       | +07a
        btst    #0x0,0x100001.l                 | +080
        beq.w   .L085a9a                        | +088
        jsr     0x28d70.l                       | +08c
.L085a9a:
        jsr     M4_PlayersBelowY_085ace(pc)          | +092
        bcc.w   SetHandlerRts_085acc            | +096
        move.w  0x24(a6),d1                     | +09a
        jsr     0x4400e.l                       | +09e
        subi.w  #0x100,d1                       | +0a4
        bpl.w   .L085ab6                        | +0a8
        clr.w   d1                              | +0ac
.L085ab6:
        move.w  d1,0x10816a.l                   | +0ae
        move.w  d1,0x10816e.l                   | +0b4
        addq.b  #0x1,0x20(a6)                   | +0ba

| ----------------------------------------------------------------------------
|  M4_PlayersBelowY_085ace  @ $085ACE  (142 B)
| ----------------------------------------------------------------------------
        .section .text.M4_PlayersBelowY_085ace, "ax", @progbits
        .global M4_PlayersBelowY_085ace
M4_PlayersBelowY_085ace:
        jsr     0x5e55c.l                       | +000
        cmpi.b  #0x0,d0                         | +006
        beq.w   ClearXN_085b62                  | +00a
        cmpi.b  #0x1,d0                         | +00e
        bne.w   .L085b02                        | +012
        lea     0x100440.l,a0                   | +016
        jsr     M4_SlotAlive_085b68(pc)          | +01c
        bcc.w   ClearXN_085b62                  | +020
        move.w  0x24(a0),d1                     | +024
        cmp.w   0x24(a6),d1                     | +028
        blt.w   ClearXN_085b62                  | +02c
        bra.w   SetXN_085b5c                    | +030
.L085b02:
        cmpi.b  #0x2,d0                         | +034
        bne.w   .L085b28                        | +038
        lea     0x1004e0.l,a0                   | +03c
        jsr     M4_SlotAlive_085b68(pc)          | +042
        bcc.w   ClearXN_085b62                  | +046
        move.w  0x24(a0),d1                     | +04a
        cmp.w   0x24(a6),d1                     | +04e
        blt.w   ClearXN_085b62                  | +052
        bra.w   SetXN_085b5c                    | +056
.L085b28:
        lea     0x100440.l,a0                   | +05a
        jsr     M4_SlotAlive_085b68(pc)          | +060
        bcc.w   ClearXN_085b62                  | +064
        move.w  0x24(a0),d1                     | +068
        cmp.w   0x24(a6),d1                     | +06c
        blt.w   ClearXN_085b62                  | +070
        lea     0x1004e0.l,a0                   | +074
        jsr     M4_SlotAlive_085b68(pc)          | +07a
        bcc.w   ClearXN_085b62                  | +07e
        move.w  0x24(a0),d1                     | +082
        cmp.w   0x24(a6),d1                     | +086
        blt.w   ClearXN_085b62                  | +08a

| ----------------------------------------------------------------------------
|  M4_SlotAlive_085b68  @ $085B68  (10 B)
| ----------------------------------------------------------------------------
        .section .text.M4_SlotAlive_085b68, "ax", @progbits
        .global M4_SlotAlive_085b68
M4_SlotAlive_085b68:
        jsr     0x32e08.l                       | +000
        bcs.w   ClearXN_085b78                  | +006

| ----------------------------------------------------------------------------
|  M4_SetupSprites4_085b7e  @ $085B7E  (82 B)
| ----------------------------------------------------------------------------
        .section .text.M4_SetupSprites4_085b7e, "ax", @progbits
        .global M4_SetupSprites4_085b7e
M4_SetupSprites4_085b7e:
        move.w  #0x4e,d1                        | +000
        move.w  #0x1f8,d2                       | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x4f,d1                        | +016
        move.w  #0x1f9,d2                       | +01a
        move.w  #0xffff,d3                      | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  #0x50,d1                        | +02c
        move.w  #0x1fa,d2                       | +030
        move.w  #0xffff,d3                      | +034
        move.w  #0x1,d4                         | +038
        jsr     0x2c26.l                        | +03c
        move.w  #0x51,d1                        | +042
        move.w  #0x1fb,d2                       | +046
        move.w  #0xffff,d3                      | +04a
        move.w  #0x1,d4                         | +04e

| ----------------------------------------------------------------------------
|  M4_SetupSprites4B_085bd8  @ $085BD8  (82 B)
| ----------------------------------------------------------------------------
        .section .text.M4_SetupSprites4B_085bd8, "ax", @progbits
        .global M4_SetupSprites4B_085bd8
M4_SetupSprites4B_085bd8:
        move.w  #0x4e,d1                        | +000
        move.w  #0x1f7,d2                       | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x4f,d1                        | +016
        move.w  #0x1fc,d2                       | +01a
        move.w  #0xffff,d3                      | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  #0x50,d1                        | +02c
        move.w  #0x1fd,d2                       | +030
        move.w  #0xffff,d3                      | +034
        move.w  #0x1,d4                         | +038
        jsr     0x2c26.l                        | +03c
        move.w  #0x51,d1                        | +042
        move.w  #0x1fe,d2                       | +046
        move.w  #0xffff,d3                      | +04a
        move.w  #0x1,d4                         | +04e

| ----------------------------------------------------------------------------
|  M4_Wheel_Anim_085c32  @ $085C32  (202 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Wheel_Anim_085c32, "ax", @progbits
        .global M4_Wheel_Anim_085c32
M4_Wheel_Anim_085c32:
        movea.l 0xc(a6),a0                      | +000
        cmpi.w  #0x0,0x2a(a0)                   | +004
        beq.w   .L085cd6                        | +00a
        clr.l   d0                              | +00e
        move.w  0x2a(a0),d0                     | +010
        btst    #0xf,d0                         | +014
        bne.w   .L085c86                        | +018
        move.b  0x88(a6),d0                     | +01c
        subq.b  #0x1,d0                         | +020
        andi.b  #0x3,d0                         | +022
        move.b  d0,0x88(a6)                     | +026
        clr.l   d0                              | +02a
        move.b  0x88(a6),d0                     | +02c
        andi.b  #0x3,d0                         | +030
        movea.l #0x2e7bee,a0                    | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L085c82                        | +046
        jsr     0x28cd4.l                       | +04a
.L085c82:
        bra.w   JsrAbsThunk_085cfc              | +050
.L085c86:
        jsr     0x28d70.l                       | +054
        bcc.w   JsrAbsRts_085d02                | +05a
        move.b  0x88(a6),d0                     | +05e
        addq.b  #0x1,d0                         | +062
        andi.b  #0x3,d0                         | +064
        move.b  d0,0x88(a6)                     | +068
        clr.l   d0                              | +06c
        move.b  0x88(a6),d0                     | +06e
        andi.b  #0x3,d0                         | +072
        movea.l #0x2e7bde,a0                    | +076
        lsl.w   #0x2,d0                         | +07c
        movea.l (a0,d0.w),a0                    | +07e
        cmpa.l  #0xffffffff,a0                  | +082
        beq.w   .L085cc4                        | +088
        jsr     0x28cd4.l                       | +08c
.L085cc4:
        bra.w   JsrAbsRts_085d02                | +092
        move.b  0x88(a6),d0                     | +096
        addq.b  #0x1,d0                         | +09a
        andi.b  #0x3,d0                         | +09c
        move.b  d0,0x88(a6)                     | +0a0
.L085cd6:
        clr.l   d0                              | +0a4
        move.b  0x88(a6),d0                     | +0a6
        andi.b  #0x3,d0                         | +0aa
        movea.l #0x2e7bde,a0                    | +0ae
        lsl.w   #0x2,d0                         | +0b4
        movea.l (a0,d0.w),a0                    | +0b6
        cmpa.l  #0xffffffff,a0                  | +0ba
        beq.w   JsrAbsThunk_085cfc              | +0c0
        jsr     0x28cd4.l                       | +0c4

| ----------------------------------------------------------------------------
|  M4_Rail_Blit_085d04  @ $085D04  (30 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Rail_Blit_085d04, "ax", @progbits
        .global M4_Rail_Blit_085d04
M4_Rail_Blit_085d04:
        lea     0x2eaa10.l,a0                   | +000
        clr.l   d0                              | +006
        move.b  0x21(a6),d0                     | +008
        cmpi.b  #0x14,d0                        | +00c
        blt.w   .L085d1c                        | +010
        move.b  #0x0,d0                         | +014
.L085d1c:
        asl.l   #0x2,d0                         | +018
        movea.l (a0,d0.w),a2                    | +01a

| ----------------------------------------------------------------------------
|  M4_Turret_EdgeCheck_085d32  @ $085D32  (106 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Turret_EdgeCheck_085d32, "ax", @progbits
        .global M4_Turret_EdgeCheck_085d32
M4_Turret_EdgeCheck_085d32:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        jsr     0x4400e.l                       | +008
        clr.l   d0                              | +00e
        move.w  0x2a(a6),d0                     | +010
        btst    #0xf,d0                         | +014
        bne.w   .L085d5a                        | +018
        cmpi.w  #0x2d0,d1                       | +01c
        bgt.w   ClearXN_085da2                  | +020
        bra.w   SetXN_085d9c                    | +024
.L085d5a:
        cmpi.w  #0x310,d1                       | +028
        blt.w   ClearXN_085da2                  | +02c
        lea     0x2e99a8.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        subi.w  #0x8,0x24(a0)                   | +03c
        lea     0x2e99ba.l,a1                   | +042
        jsr     0x77c7e.l                       | +048
        subi.w  #0x8,0x38(a0)                   | +04e
        move.b  #0xa,0x75(a6)                   | +054
        lea     M4_SmokePuff_0854e8(pc),a1       | +05a
        jsr     0x4ae.l                         | +05e
        jsr     0x5dd02.l                       | +064

| ----------------------------------------------------------------------------
|  M4_Turret_Knockback_085da8  @ $085DA8  (102 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Turret_Knockback_085da8, "ax", @progbits
        .global M4_Turret_Knockback_085da8
M4_Turret_Knockback_085da8:
        lea     0x2e94a2.l,a0                   | +000
        move.l  a0,0x48(a6)                     | +006
        move.w  #0x7fff,0x66(a6)                | +00a
        jsr     0x2870a.l                       | +010
        bcc.w   M4_Turret_Recoil_085e14              | +016
        lea     0x2c031c.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        move.w  d0,0x2a(a6)                     | +026
        lea     0x2e791e.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        bclr    #0x3,0x13(a6)                   | +036
        move.w  #0x108d,d0                      | +03c
        jsr     0x2352.l                        | +040
        move.w  0x22(a6),d0                     | +046
        move.w  0x24(a6),d1                     | +04a
        jsr     0x4400e.l                       | +04e
        cmpi.w  #0x2d0,d1                       | +054
        ble.w   M4_Turret_RecoilLeft_085e2a     | +058
        move.b  0x78(a6),d0                     | +05c
        asl.b   #0x1,d0                         | +060
        bclr    #0x0,d0                         | +062

| ----------------------------------------------------------------------------
|  M4_Turret_Recoil_085e14  @ $085E14  (52 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Turret_Recoil_085e14, "ax", @progbits
        .global M4_Turret_Recoil_085e14
M4_Turret_Recoil_085e14:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        jsr     0x4400e.l                       | +008
        cmpi.w  #0x310,d1                       | +00e
        bge.w   M4_Turret_RecoilStop_085e4e              | +012
        .global M4_Turret_RecoilLeft_085e2a
M4_Turret_RecoilLeft_085e2a:
        lea     0x2c039e.l,a0                   | +016
        jsr     0x799de.l                       | +01c
        move.w  d0,d0                           | +022
        neg.w   d0                              | +024
        move.w  d0,0x2a(a6)                     | +026
        move.b  0x78(a6),d0                     | +02a
        asl.b   #0x1,d0                         | +02e
        bclr    #0x0,d0                         | +030

| ----------------------------------------------------------------------------
|  M4_Turret_RecoilStop_085e4e  @ $085E4E  (22 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Turret_RecoilStop_085e4e, "ax", @progbits
        .global M4_Turret_RecoilStop_085e4e
M4_Turret_RecoilStop_085e4e:
        move.b  0x78(a6),d0                     | +000
        asl.b   #0x1,d0                         | +004
        bset    #0x0,d0                         | +006
        move.b  d0,0x78(a6)                     | +00a
        move.w  #0x0,0x2a(a6)                   | +00e
        rts                                     | +014

| ----------------------------------------------------------------------------
|  M4_PickTargetSlot_085e64  @ $085E64  (76 B)
| ----------------------------------------------------------------------------
        .section .text.M4_PickTargetSlot_085e64, "ax", @progbits
        .global M4_PickTargetSlot_085e64
M4_PickTargetSlot_085e64:
        jsr     0x5e1aa.l                       | +000
        bcs.w   ClearXN_085eb6                  | +006
        cmpi.w  #0x1,d0                         | +00a
        bne.w   .L085e80                        | +00e
        lea     0x100440.l,a0                   | +012
        bra.w   SetXN_085eb0                    | +018
.L085e80:
        cmpi.w  #0x2,d0                         | +01c
        bne.w   .L085e92                        | +020
        lea     0x1004e0.l,a0                   | +024
        bra.w   SetXN_085eb0                    | +02a
.L085e92:
        move.b  0x106f28.l,d0                   | +02e
        btst    #0x7,d0                         | +034
        bne.w   .L085eaa                        | +038
        lea     0x100440.l,a0                   | +03c
        bra.w   SetXN_085eb0                    | +042
.L085eaa:
        lea     0x1004e0.l,a0                   | +046

| ----------------------------------------------------------------------------
|  M4_Turret_Shake_085ebc  @ $085EBC  (44 B)
| ----------------------------------------------------------------------------
        .section .text.M4_Turret_Shake_085ebc, "ax", @progbits
        .global M4_Turret_Shake_085ebc
M4_Turret_Shake_085ebc:
        cmpi.b  #0x0,0x75(a6)                   | +000
        ble.w   .L085ee6                        | +006
        subq.b  #0x1,0x75(a6)                   | +00a
        move.w  #0x1,d1                         | +00e
        move.b  0x75(a6),d0                     | +012
        btst    #0x0,d0                         | +016
        bne.w   .L085ee2                        | +01a
        add.w   d1,0x24(a6)                     | +01e
        bra.w   .L085ee6                        | +022
.L085ee2:
        sub.w   d1,0x24(a6)                     | +026
.L085ee6:
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  Flight_WobbleArm_085ee8  @ $085EE8  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Flight_WobbleArm_085ee8, "ax", @progbits
        .global Flight_WobbleArm_085ee8
Flight_WobbleArm_085ee8:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bgt.w   .L085f06                        | +006
        jsr     0x5e9b6.l                       | +00a
        andi.w  #0x1f,d0                        | +010
        bne.w   .L085f06                        | +014
        move.w  #0x2,0x72(a6)                   | +018
.L085f06:
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  Flight_WobbleStep_085f08  @ $085F08  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Flight_WobbleStep_085f08, "ax", @progbits
        .global Flight_WobbleStep_085f08
Flight_WobbleStep_085f08:
        cmpi.w  #0x0,0x72(a6)                   | +000
        ble.w   .L085f42                        | +006
        subq.w  #0x1,0x72(a6)                   | +00a
        move.w  #0x100,d1                       | +00e
        move.w  0x72(a6),d0                     | +012
        btst    #0x0,d0                         | +016
        bne.w   .L085f28                        | +01a
        neg.w   d1                              | +01e
.L085f28:
        move.w  0x2a(a6),d2                     | +020
        movem.w d2,-(a7)                        | +024
        move.w  d1,0x2a(a6)                     | +028
        jsr     0x27cee.l                       | +02c
        movem.w (a7)+,d2                        | +032
        move.w  d2,0x2a(a6)                     | +036
.L085f42:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Flight_HitboxParams_085f44  @ $085F44  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Flight_HitboxParams_085f44, "ax", @progbits
        .global Flight_HitboxParams_085f44
Flight_HitboxParams_085f44:
        move.w  0x22(a6),d0                     | +000
        subi.w  #0x60,d0                        | +004
        move.w  0x24(a6),d1                     | +008
        subi.w  #0x0,d1                         | +00c
        move.w  #0xc0,d2                        | +010

| ----------------------------------------------------------------------------
|  Flight_AltitudeCheck_085f60  @ $085F60  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Flight_AltitudeCheck_085f60, "ax", @progbits
        .global Flight_AltitudeCheck_085f60
Flight_AltitudeCheck_085f60:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        jsr     0x44022.l                       | +008
        clr.w   d2                              | +00e
        clr.w   d3                              | +010
        move.b  0x98(a6),d2                     | +012
        move.b  0x99(a6),d3                     | +016
        asl.w   #0x4,d2                         | +01a
        asl.w   #0x4,d3                         | +01c
        clr.l   d4                              | +01e
        move.w  0x2a(a6),d4                     | +020
        btst    #0xf,d4                         | +024
        beq.w   Flight_AltitudeCheckUp_085f9e              | +028
        cmp.w   d1,d3                           | +02c
        ble.w   ClearXN_085f98                  | +02e

| ----------------------------------------------------------------------------
|  Flight_AltitudeCheckUp_085f9e  @ $085F9E  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Flight_AltitudeCheckUp_085f9e, "ax", @progbits
        .global Flight_AltitudeCheckUp_085f9e
Flight_AltitudeCheckUp_085f9e:
        cmp.w   d1,d2                           | +000
        bge.w   SetXN_085faa                    | +002

| ----------------------------------------------------------------------------
|  Boss_RandomDropSpawn_085fb0  @ $085FB0  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_RandomDropSpawn_085fb0, "ax", @progbits
        .global Boss_RandomDropSpawn_085fb0
Boss_RandomDropSpawn_085fb0:
        jsr     0x5e9b6.l                       | +000
        andi.b  #0x7,d0                         | +006
        bne.w   .L08601a                        | +00a
        move.w  #0x2,d0                         | +00e
        jsr     0x5e9e4.l                       | +012
        addi.w  #0x1,d0                         | +018
.L085fcc:
        cmpi.w  #0x0,d0                         | +01c
        beq.w   .L08601a                        | +020
        movem.w d0,-(a7)                        | +024
        lea     0x3fec6.l,a1                    | +028
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd22.l                       | +034
        movem.l a0,-(a7)                        | +03a
        move.w  #0x200,0x24(a0)                 | +03e
        move.w  #0x30,d0                        | +044
        jsr     0x5e9e4.l                       | +048
        btst    #0x0,d0                         | +04e
        beq.w   .L086008                        | +052
        neg.w   d0                              | +056
.L086008:
        movem.l (a7)+,a0                        | +058
        add.w   d0,0x22(a0)                     | +05c
        movem.w (a7)+,d0                        | +060
        subi.w  #0x1,d0                         | +064
        bra.b   .L085fcc                        | +068
.L08601a:
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  Boss_PhaseJingle_08601c  @ $08601C  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_PhaseJingle_08601c, "ax", @progbits
        .global Boss_PhaseJingle_08601c
Boss_PhaseJingle_08601c:
        move.b  0x21(a6),d0                     | +000
        cmpi.b  #0x1,d0                         | +004
        beq.w   .L086030                        | +008
        cmpi.b  #0x5,d0                         | +00c
        bne.w   Boss_PhaseJingle8_08603c              | +010
.L086030:
        move.w  #0x1026,d0                      | +014

| ----------------------------------------------------------------------------
|  Boss_PhaseJingle8_08603c  @ $08603C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_PhaseJingle8_08603c, "ax", @progbits
        .global Boss_PhaseJingle8_08603c
Boss_PhaseJingle8_08603c:
        cmpi.b  #0x8,d0                         | +000
        bne.w   JsrAbsRts_08604e                | +004
        move.w  #0x1032,d0                      | +008

| ----------------------------------------------------------------------------
|  Boss_SineBob_086050  @ $086050  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_SineBob_086050, "ax", @progbits
        .global Boss_SineBob_086050
Boss_SineBob_086050:
        addi.w  #0x8,0x34(a6)                   | +000
        andi.w  #0xff,0x34(a6)                  | +006
        move.w  0x34(a6),d0                     | +00c
        move.w  #0x80,d1                        | +010
        jsr     0x13c0e.l                       | +014
        move.w  d2,0x2a(a6)                     | +01a

| ----------------------------------------------------------------------------
|  Boss_SpawnGuardList_086076  @ $086076  (110 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_SpawnGuardList_086076, "ax", @progbits
        .global Boss_SpawnGuardList_086076
Boss_SpawnGuardList_086076:
        lea     0x2eaf1c.l,a1                   | +000
        cmpi.b  #0xb,0x106ece.l                 | +006
        bne.w   .L08608e                        | +00e
        lea     0x2eaf7e.l,a1                   | +012
.L08608e:
        clr.l   d0                              | +018
.L086090:
        movem.l a1,-(a7)                        | +01a
        movem.l d0,-(a7)                        | +01e
        lea     TaskHandler_083c02(pc),a1       | +022
        jsr     0x6fe.l                         | +026
        jsr     0x5dd02.l                       | +02c
        movem.l (a7)+,d0                        | +032
        movem.l (a7)+,a1                        | +036
        move.w  (a1,d0.w),d1                    | +03a
        add.w   d1,0x22(a0)                     | +03e
        move.w  0x2(a1,d0.w),d1                 | +042
        add.w   d1,0x24(a0)                     | +046
        move.w  0x4(a1,d0.w),0x38(a0)           | +04a
        move.b  0x6(a1,d0.w),0x21(a0)           | +050
        move.b  0x7(a1,d0.w),d1                 | +056
        or.b    d1,0x3a(a0)                     | +05a
        addi.l  #0x8,d0                         | +05e
        cmpi.w  #0xffff,(a1,d0.w)               | +064
        bne.b   .L086090                        | +06a
        rts                                     | +06c

| ----------------------------------------------------------------------------
|  Boss_Spawn45Children_0860e4  @ $0860E4  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_Spawn45Children_0860e4, "ax", @progbits
        .global Boss_Spawn45Children_0860e4
Boss_Spawn45Children_0860e4:
        clr.b   d0                              | +000
.L0860e6:
        movem.w d0,-(a7)                        | +002
        lea     TaskHandler_0845ec(pc),a1       | +006
        jsr     0x6fe.l                         | +00a
        jsr     0x5dd02.l                       | +010
        movem.w (a7)+,d0                        | +016
        move.b  d0,0x21(a0)                     | +01a
        addq.b  #0x1,d0                         | +01e
        cmpi.b  #0x2d,d0                        | +020
        blt.b   .L0860e6                        | +024
        rts                                     | +026

| ----------------------------------------------------------------------------
|  Boss_SpawnStepList_08610c  @ $08610C  (138 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_SpawnStepList_08610c, "ax", @progbits
        .global Boss_SpawnStepList_08610c
Boss_SpawnStepList_08610c:
        lea     0x2eafe2.l,a1                   | +000
        clr.l   d0                              | +006
.L086114:
        cmpi.l  #0x38,d0                        | +008
        bge.w   .L086142                        | +00e
        movem.l a1,-(a7)                        | +012
        movem.l d0,-(a7)                        | +016
        lea     TaskHandler_084836(pc),a1       | +01a
        jsr     0x6fe.l                         | +01e
        jsr     0x5dd02.l                       | +024
        movem.l (a7)+,d0                        | +02a
        movem.l (a7)+,a1                        | +02e
        bra.w   .L086162                        | +032
.L086142:
        movem.l a1,-(a7)                        | +036
        movem.l d0,-(a7)                        | +03a
        lea     TaskHandler_084836(pc),a1       | +03e
        jsr     0x4ae.l                         | +042
        jsr     0x5dd22.l                       | +048
        movem.l (a7)+,d0                        | +04e
        movem.l (a7)+,a1                        | +052
.L086162:
        move.w  (a1,d0.w),d1                    | +056
        add.w   d1,0x22(a0)                     | +05a
        move.w  0x2(a1,d0.w),d1                 | +05e
        add.w   d1,0x24(a0)                     | +062
        move.w  0x4(a1,d0.w),0x38(a0)           | +066
        move.b  0x6(a1,d0.w),0x21(a0)           | +06c
        move.b  0x7(a1,d0.w),d1                 | +072
        or.b    d1,0x3a(a0)                     | +076
        addi.l  #0x8,d0                         | +07a
        cmpi.w  #0xffff,(a1,d0.w)               | +080
        bne.b   .L086114                        | +086
        rts                                     | +088

| ----------------------------------------------------------------------------
|  Boss_SpawnTenEscorts_086196  @ $086196  (362 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_SpawnTenEscorts_086196, "ax", @progbits
        .global Boss_SpawnTenEscorts_086196
Boss_SpawnTenEscorts_086196:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd22.l                       | +00c
        addi.w  #0x30,0x22(a0)                  | +012
        addi.w  #0x20,0x24(a0)                  | +018
        move.w  #0xc000,0x38(a0)                | +01e
        lea     0x77fd6.l,a1                    | +024
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd22.l                       | +030
        addi.w  #0x20,0x22(a0)                  | +036
        addi.w  #0x50,0x24(a0)                  | +03c
        move.w  #0xc000,0x38(a0)                | +042
        lea     0x77fd6.l,a1                    | +048
        jsr     0x4ae.l                         | +04e
        jsr     0x5dd22.l                       | +054
        addi.w  #0x40,0x22(a0)                  | +05a
        addi.w  #0x70,0x24(a0)                  | +060
        move.w  #0xc000,0x38(a0)                | +066
        lea     0x77fd6.l,a1                    | +06c
        jsr     0x4ae.l                         | +072
        jsr     0x5dd22.l                       | +078
        addi.w  #0x60,0x22(a0)                  | +07e
        addi.w  #0x10,0x24(a0)                  | +084
        move.w  #0xc000,0x38(a0)                | +08a
        lea     0x77fd6.l,a1                    | +090
        jsr     0x4ae.l                         | +096
        jsr     0x5dd22.l                       | +09c
        addi.w  #0x80,0x22(a0)                  | +0a2
        addi.w  #0x40,0x24(a0)                  | +0a8
        move.w  #0xc000,0x38(a0)                | +0ae
        lea     0x77fd6.l,a1                    | +0b4
        jsr     0x4ae.l                         | +0ba
        jsr     0x5dd22.l                       | +0c0
        addi.w  #0x90,0x22(a0)                  | +0c6
        addi.w  #0x70,0x24(a0)                  | +0cc
        move.w  #0xc000,0x38(a0)                | +0d2
        lea     0x77fd6.l,a1                    | +0d8
        jsr     0x4ae.l                         | +0de
        jsr     0x5dd22.l                       | +0e4
        addi.w  #0xb0,0x22(a0)                  | +0ea
        addi.w  #0x20,0x24(a0)                  | +0f0
        move.w  #0xc000,0x38(a0)                | +0f6
        lea     0x77fd6.l,a1                    | +0fc
        jsr     0x4ae.l                         | +102
        jsr     0x5dd22.l                       | +108
        addi.w  #0xd0,0x22(a0)                  | +10e
        addi.w  #0x60,0x24(a0)                  | +114
        move.w  #0xc000,0x38(a0)                | +11a
        lea     0x77fd6.l,a1                    | +120
        jsr     0x4ae.l                         | +126
        jsr     0x5dd22.l                       | +12c
        addi.w  #0xe0,0x22(a0)                  | +132
        addi.w  #0x20,0x24(a0)                  | +138
        move.w  #0xc000,0x38(a0)                | +13e
        lea     0x77fd6.l,a1                    | +144
        jsr     0x4ae.l                         | +14a
        jsr     0x5dd22.l                       | +150
        addi.w  #0xf0,0x22(a0)                  | +156
        addi.w  #0x70,0x24(a0)                  | +15c
        move.w  #0xc000,0x38(a0)                | +162
        rts                                     | +168

| ----------------------------------------------------------------------------
|  Boss_Spawn4Finale_086300  @ $086300  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_Spawn4Finale_086300, "ax", @progbits
        .global Boss_Spawn4Finale_086300
Boss_Spawn4Finale_086300:
        clr.b   d0                              | +000
.L086302:
        movem.w d0,-(a7)                        | +002
        lea     TaskHandler_0846e0(pc),a1       | +006
        jsr     0x6fe.l                         | +00a
        jsr     0x5dd02.l                       | +010
        movem.w (a7)+,d0                        | +016
        move.b  d0,0x21(a0)                     | +01a
        addq.b  #0x1,d0                         | +01e
        cmpi.b  #0x4,d0                         | +020
        blt.b   .L086302                        | +024
        rts                                     | +026

| ----------------------------------------------------------------------------
|  Boss_SpawnRow8_086328  @ $086328  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_SpawnRow8_086328, "ax", @progbits
        .global Boss_SpawnRow8_086328
Boss_SpawnRow8_086328:
        clr.b   d0                              | +000
        move.w  0x22(a6),d1                     | +002
.L08632e:
        movem.l d0,-(a7)                        | +006
        movem.l d1,-(a7)                        | +00a
        lea     TaskHandler_083fd0(pc),a1       | +00e
        jsr     0x4ae.l                         | +012
        jsr     0x5dd22.l                       | +018
        movem.l (a7)+,d1                        | +01e
        movem.l (a7)+,d0                        | +022
        move.b  d0,0x21(a0)                     | +026
        move.w  d1,0x22(a0)                     | +02a
        addq.b  #0x1,d0                         | +02e
        addi.w  #0x20,d1                        | +030
        cmpi.b  #0x8,d0                         | +034
        blt.b   .L08632e                        | +038
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Boss_SpawnRow9_086364  @ $086364  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_SpawnRow9_086364, "ax", @progbits
        .global Boss_SpawnRow9_086364
Boss_SpawnRow9_086364:
        clr.b   d0                              | +000
        move.w  0x22(a6),d1                     | +002
.L08636a:
        movem.l d0,-(a7)                        | +006
        movem.l d1,-(a7)                        | +00a
        lea     TaskHandler_084282(pc),a1       | +00e
        jsr     0x4ae.l                         | +012
        jsr     0x5dd22.l                       | +018
        movem.l (a7)+,d1                        | +01e
        movem.l (a7)+,d0                        | +022
        move.b  d0,0x21(a0)                     | +026
        move.w  d1,0x22(a0)                     | +02a
        addq.b  #0x1,d0                         | +02e
        addi.w  #0x10,d1                        | +030
        cmpi.b  #0x9,d0                         | +034
        blt.b   .L08636a                        | +038
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Entity_BobY1_0863a0  @ $0863A0  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_BobY1_0863a0, "ax", @progbits
        .global Entity_BobY1_0863a0
Entity_BobY1_0863a0:
        addq.b  #0x1,0x72(a6)                   | +000
        btst    #0x0,0x72(a6)                   | +004
        bne.w   .L0863b6                        | +00a
        subi.w  #0x1,0x24(a6)                   | +00e
        rts                                     | +014
.L0863b6:
        addi.w  #0x1,0x24(a6)                   | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Entity_TickIfState2_0863be  @ $0863BE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_TickIfState2_0863be, "ax", @progbits
        .global Entity_TickIfState2_0863be
Entity_TickIfState2_0863be:
        cmpi.b  #0x2,0x21(a6)                   | +000
        bne.w   JsrAbsRts_0863d4                | +006
        jsr     0x283ca.l                       | +00a

| ----------------------------------------------------------------------------
|  Boss_BlitTable_A_0864b6  @ $0864B6  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_BlitTable_A_0864b6, "ax", @progbits
        .global Boss_BlitTable_A_0864b6
Boss_BlitTable_A_0864b6:
        lea     0x2eaa60.l,a0                   | +000
        clr.l   d0                              | +006
        move.b  0x21(a6),d0                     | +008
        asl.l   #0x4,d0                         | +00c
        movea.l 0x4(a0,d0.w),a2                 | +00e

| ----------------------------------------------------------------------------
|  Boss_BlitTable_B_0864d0  @ $0864D0  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_BlitTable_B_0864d0, "ax", @progbits
        .global Boss_BlitTable_B_0864d0
Boss_BlitTable_B_0864d0:
        lea     0x2eaa60.l,a0                   | +000
        clr.l   d0                              | +006
        move.b  0x21(a6),d0                     | +008
        asl.l   #0x4,d0                         | +00c
        movea.l 0xc(a0,d0.w),a2                 | +00e

| ----------------------------------------------------------------------------
|  Boss_BlitTable_C_0864ea  @ $0864EA  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_BlitTable_C_0864ea, "ax", @progbits
        .global Boss_BlitTable_C_0864ea
Boss_BlitTable_C_0864ea:
        lea     0x2eaae0.l,a0                   | +000
        clr.l   d0                              | +006
        move.b  0x21(a6),d0                     | +008
        asl.l   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a2                    | +00e

| ----------------------------------------------------------------------------
|  Boss_BlitTable_D_086504  @ $086504  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_BlitTable_D_086504, "ax", @progbits
        .global Boss_BlitTable_D_086504
Boss_BlitTable_D_086504:
        lea     0x2eab04.l,a0                   | +000
        clr.l   d0                              | +006
        move.b  0x21(a6),d0                     | +008
        asl.l   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a2                    | +00e

| ----------------------------------------------------------------------------
|  Boss_BlitTable_E_08651e  @ $08651E  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_BlitTable_E_08651e, "ax", @progbits
        .global Boss_BlitTable_E_08651e
Boss_BlitTable_E_08651e:
        lea     0x2eab24.l,a0                   | +000
        clr.l   d0                              | +006
        move.b  0x21(a6),d0                     | +008
        asl.l   #0x3,d0                         | +00c
        movea.l (a0,d0.w),a2                    | +00e

| ----------------------------------------------------------------------------
|  Boss_BlitTable_F_086538  @ $086538  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_BlitTable_F_086538, "ax", @progbits
        .global Boss_BlitTable_F_086538
Boss_BlitTable_F_086538:
        lea     0x2eab24.l,a0                   | +000
        clr.l   d0                              | +006
        move.b  0x21(a6),d0                     | +008
        asl.l   #0x3,d0                         | +00c
        movea.l 0x4(a0,d0.w),a2                 | +00e

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_086552  @ $086552  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_086552, "ax", @progbits
        .global Entity_CmpPrioWithSibling_086552
Entity_CmpPrioWithSibling_086552:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_086568                    | +00c

| ----------------------------------------------------------------------------
|  Boss_Shadow_Init_08656e  @ $08656E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_Shadow_Init_08656e, "ax", @progbits
        .global Boss_Shadow_Init_08656e
Boss_Shadow_Init_08656e:
        lea     0x2ecec4.l,a0                   | +000
        move.l  a0,0x4c(a6)                     | +006
        jsr     0x283ca.l                       | +00a
        bra.w   .L086596                        | +010
        bra.w   .L086596                        | +014
        lea     0x2ecd80.l,a0                   | +018
        move.l  a0,0x4c(a6)                     | +01e
        jsr     0x283ca.l                       | +022
.L086596:
        move.w  #0x0,0x22(a6)                   | +028
        move.w  #0x100,0x24(a6)                 | +02e
        jsr     0x283ca.l                       | +034
        jsr     0x283d8.l                       | +03a
        move.b  #0x1,0x10e39e.l                 | +040
