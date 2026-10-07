| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $04FA50..$051914  (7,396 B, 97 entradas, 36 huecos)
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
|  FixTile_Set11C1_04fa58  @ $04FA58  (16 B)
| ----------------------------------------------------------------------------
        .section .text.FixTile_Set11C1_04fa58, "ax", @progbits
        .global FixTile_Set11C1_04fa58
FixTile_Set11C1_04fa58:
        move.w  #0x11,d1                        | +000
        move.w  #0xc1,d2                        | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c

| ----------------------------------------------------------------------------
|  Prop_OffscreenLeftCheck_04fa70  @ $04FA70  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_OffscreenLeftCheck_04fa70, "ax", @progbits
        .global Prop_OffscreenLeftCheck_04fa70
Prop_OffscreenLeftCheck_04fa70:
        move.w  0x70(a6),d0                     | +000
        neg.w   d0                              | +004
        cmp.w   0x22(a6),d0                     | +006
        ble.w   ClearC_04fa84                   | +00a

| ----------------------------------------------------------------------------
|  Prop_TowerBlitState10_04fa8a  @ $04FA8A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerBlitState10_04fa8a, "ax", @progbits
        .global Prop_TowerBlitState10_04fa8a
Prop_TowerBlitState10_04fa8a:
        cmpi.b  #0x10,0x21(a6)                  | +000
        bne.w   Prop_TowerBlitState11_04faa0    | +006
        lea     0x29557e.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Prop_TowerBlitState11_04faa0  @ $04FAA0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerBlitState11_04faa0, "ax", @progbits
        .global Prop_TowerBlitState11_04faa0
Prop_TowerBlitState11_04faa0:
        cmpi.b  #0x11,0x21(a6)                  | +000
        bne.w   Prop_TowerBlitState12_04fab6    | +006
        lea     0x295592.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Prop_TowerBlitState12_04fab6  @ $04FAB6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerBlitState12_04fab6, "ax", @progbits
        .global Prop_TowerBlitState12_04fab6
Prop_TowerBlitState12_04fab6:
        cmpi.b  #0x12,0x21(a6)                  | +000
        bne.w   Prop_TowerBlitState01_04facc    | +006
        lea     0x2955a6.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Prop_TowerBlitState01_04facc  @ $04FACC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerBlitState01_04facc, "ax", @progbits
        .global Prop_TowerBlitState01_04facc
Prop_TowerBlitState01_04facc:
        cmpi.b  #0x1,0x21(a6)                   | +000
        bne.w   Prop_TowerBlitState02_04fae2    | +006
        lea     0x2955ba.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Prop_TowerBlitState02_04fae2  @ $04FAE2  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerBlitState02_04fae2, "ax", @progbits
        .global Prop_TowerBlitState02_04fae2
Prop_TowerBlitState02_04fae2:
        cmpi.b  #0x2,0x21(a6)                   | +000
        bne.w   JsrPcRts_04faf6                 | +006
        lea     0x2955ce.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Prop_BunkerBlitState10_04faf8  @ $04FAF8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BunkerBlitState10_04faf8, "ax", @progbits
        .global Prop_BunkerBlitState10_04faf8
Prop_BunkerBlitState10_04faf8:
        cmpi.b  #0x10,0x21(a6)                  | +000
        bne.w   Prop_BunkerBlitState01_04fb0e   | +006
        lea     0x295646.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Prop_BunkerBlitState01_04fb0e  @ $04FB0E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BunkerBlitState01_04fb0e, "ax", @progbits
        .global Prop_BunkerBlitState01_04fb0e
Prop_BunkerBlitState01_04fb0e:
        cmpi.b  #0x1,0x21(a6)                   | +000
        bne.w   Prop_BunkerBlitState11_04fb24   | +006
        lea     0x29565a.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Prop_BunkerBlitState11_04fb24  @ $04FB24  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BunkerBlitState11_04fb24, "ax", @progbits
        .global Prop_BunkerBlitState11_04fb24
Prop_BunkerBlitState11_04fb24:
        cmpi.b  #0x11,0x21(a6)                  | +000
        bne.w   Stub_0004FB3A                   | +006
        lea     0x29566e.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Prop_NestBlitByFacing_04fb3c  @ $04FB3C  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_NestBlitByFacing_04fb3c, "ax", @progbits
        .global Prop_NestBlitByFacing_04fb3c
Prop_NestBlitByFacing_04fb3c:
        btst    #0x0,0x3a(a6)                   | +000
        beq.w   .L04fb5e                        | +006
        lea     0x2956d2.l,a2                   | +00a
        jsr     Sprite_InvokeBlit8Params(pc)    | +010
        lea     0x295736.l,a2                   | +014
        jsr     Sprite_InvokeBlit8Params(pc)    | +01a
        bra.w   JsrPcRts_04fb72                 | +01e
.L04fb5e:
        lea     0x295696.l,a2                   | +022
        jsr     Sprite_InvokeBlit8Params(pc)    | +028
        lea     0x29570e.l,a2                   | +02c

| ----------------------------------------------------------------------------
|  Prop_NestBlitPair_04fb74  @ $04FB74  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_NestBlitPair_04fb74, "ax", @progbits
        .global Prop_NestBlitPair_04fb74
Prop_NestBlitPair_04fb74:
        lea     0x29575e.l,a2                   | +000
        jsr     Sprite_InvokeBlit8Params(pc)    | +006
        lea     0x295772.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Barrier_SpawnPiece01_04fb8a  @ $04FB8A  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Barrier_SpawnPiece01_04fb8a, "ax", @progbits
        .global Barrier_SpawnPiece01_04fb8a
Barrier_SpawnPiece01_04fb8a:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x1,0x21(a0)                   | +010
        lea     0x295826.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        lea     0x29583a.l,a2                   | +020
        move.l  a2,0x74(a0)                     | +026
        move.l  #0xffffffff,0x78(a0)            | +02a
        addi.w  #0x40,0x22(a0)                  | +032
        addi.w  #0x70,0x24(a0)                  | +038
        movem.l a6,-(a7)                        | +03e
        movea.l a0,a6                           | +042
        lea     0x2946a2.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        movem.l (a7)+,a6                        | +050
        rts                                     | +054

| ----------------------------------------------------------------------------
|  Barrier_SpawnPiece02_04fbe0  @ $04FBE0  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Barrier_SpawnPiece02_04fbe0, "ax", @progbits
        .global Barrier_SpawnPiece02_04fbe0
Barrier_SpawnPiece02_04fbe0:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x2,0x21(a0)                   | +010
        lea     0x295862.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        lea     0x295876.l,a2                   | +020
        move.l  a2,0x74(a0)                     | +026
        move.l  #0xffffffff,0x78(a0)            | +02a
        addi.w  #0x20,0x22(a0)                  | +032
        addi.w  #0x50,0x24(a0)                  | +038
        movem.l a6,-(a7)                        | +03e
        movea.l a0,a6                           | +042
        lea     0x2946a2.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        movem.l (a7)+,a6                        | +050
        rts                                     | +054

| ----------------------------------------------------------------------------
|  Barrier_SpawnPiece04_04fc36  @ $04FC36  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Barrier_SpawnPiece04_04fc36, "ax", @progbits
        .global Barrier_SpawnPiece04_04fc36
Barrier_SpawnPiece04_04fc36:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x4,0x21(a0)                   | +010
        lea     0x29588a.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        lea     0x29589e.l,a2                   | +020
        move.l  a2,0x74(a0)                     | +026
        move.l  #0xffffffff,0x78(a0)            | +02a
        addi.w  #0x40,0x22(a0)                  | +032
        addi.w  #0x50,0x24(a0)                  | +038
        movem.l a6,-(a7)                        | +03e
        movea.l a0,a6                           | +042
        lea     0x2946a2.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        movem.l (a7)+,a6                        | +050
        rts                                     | +054

| ----------------------------------------------------------------------------
|  Barrier_SpawnPiece08_04fc8c  @ $04FC8C  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Barrier_SpawnPiece08_04fc8c, "ax", @progbits
        .global Barrier_SpawnPiece08_04fc8c
Barrier_SpawnPiece08_04fc8c:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x8,0x21(a0)                   | +010
        lea     0x2958b2.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x60,0x22(a0)                  | +028
        addi.w  #0x50,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movea.l a0,a6                           | +038
        lea     0x2946a2.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        movem.l (a7)+,a6                        | +046
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  Barrier_SpawnPieceLeft_04fcd8  @ $04FCD8  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Barrier_SpawnPieceLeft_04fcd8, "ax", @progbits
        .global Barrier_SpawnPieceLeft_04fcd8
Barrier_SpawnPieceLeft_04fcd8:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x0,0x21(a0)                   | +010
        lea     0x295af6.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x20,0x22(a0)                  | +028
        addi.w  #0x50,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946ce.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Barrier_SpawnPieceRight_04fd2c  @ $04FD2C  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Barrier_SpawnPieceRight_04fd2c, "ax", @progbits
        .global Barrier_SpawnPieceRight_04fd2c
Barrier_SpawnPieceRight_04fd2c:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x0,0x21(a0)                   | +010
        lea     0x295b1e.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        lea     0x295b32.l,a2                   | +020
        move.l  a2,0x74(a0)                     | +026
        move.l  #0xffffffff,0x78(a0)            | +02a
        addi.w  #0x40,0x22(a0)                  | +032
        addi.w  #0x30,0x24(a0)                  | +038
        movem.l a6,-(a7)                        | +03e
        movem.l a0,-(a7)                        | +042
        movea.l a0,a6                           | +046
        lea     0x2946ce.l,a0                   | +048
        jsr     0x28cd4.l                       | +04e
        movem.l (a7)+,a0                        | +054
        movem.l (a7)+,a6                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  Gatehouse_SpawnPiece01_04fd8a  @ $04FD8A  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Gatehouse_SpawnPiece01_04fd8a, "ax", @progbits
        .global Gatehouse_SpawnPiece01_04fd8a
Gatehouse_SpawnPiece01_04fd8a:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x1,0x21(a0)                   | +010
        lea     0x2958ee.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x10,0x22(a0)                  | +028
        addi.w  #0x90,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946a2.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Gatehouse_SpawnPiece02_04fdde  @ $04FDDE  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Gatehouse_SpawnPiece02_04fdde, "ax", @progbits
        .global Gatehouse_SpawnPiece02_04fdde
Gatehouse_SpawnPiece02_04fdde:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x2,0x21(a0)                   | +010
        lea     0x295916.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x30,0x22(a0)                  | +028
        addi.w  #0x90,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946a2.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Gatehouse_SpawnPiece04_04fe32  @ $04FE32  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Gatehouse_SpawnPiece04_04fe32, "ax", @progbits
        .global Gatehouse_SpawnPiece04_04fe32
Gatehouse_SpawnPiece04_04fe32:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x4,0x21(a0)                   | +010
        lea     0x29593e.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x50,0x22(a0)                  | +028
        addi.w  #0x90,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946a2.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Gatehouse_SpawnPiece08_04fe86  @ $04FE86  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Gatehouse_SpawnPiece08_04fe86, "ax", @progbits
        .global Gatehouse_SpawnPiece08_04fe86
Gatehouse_SpawnPiece08_04fe86:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x8,0x21(a0)                   | +010
        lea     0x295966.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x70,0x22(a0)                  | +028
        addi.w  #0x90,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946a2.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Gatehouse_SpawnPiece10_04feda  @ $04FEDA  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Gatehouse_SpawnPiece10_04feda, "ax", @progbits
        .global Gatehouse_SpawnPiece10_04feda
Gatehouse_SpawnPiece10_04feda:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x10,0x21(a0)                  | +010
        lea     0x29598e.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x10,0x22(a0)                  | +028
        addi.w  #0x70,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946a2.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Gatehouse_SpawnPiece20_04ff2e  @ $04FF2E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Gatehouse_SpawnPiece20_04ff2e, "ax", @progbits
        .global Gatehouse_SpawnPiece20_04ff2e
Gatehouse_SpawnPiece20_04ff2e:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x20,0x21(a0)                  | +010
        lea     0x2959b6.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x30,0x22(a0)                  | +028
        addi.w  #0x70,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946a2.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Gatehouse_SpawnPiece40_04ff82  @ $04FF82  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Gatehouse_SpawnPiece40_04ff82, "ax", @progbits
        .global Gatehouse_SpawnPiece40_04ff82
Gatehouse_SpawnPiece40_04ff82:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x40,0x21(a0)                  | +010
        lea     0x2959de.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x50,0x22(a0)                  | +028
        addi.w  #0x70,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946a2.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Gatehouse_SpawnPiece80_04ffd6  @ $04FFD6  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Gatehouse_SpawnPiece80_04ffd6, "ax", @progbits
        .global Gatehouse_SpawnPiece80_04ffd6
Gatehouse_SpawnPiece80_04ffd6:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x80,0x21(a0)                  | +010
        lea     0x295a06.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x70,0x22(a0)                  | +028
        addi.w  #0x70,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946a2.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Fortress_SpawnPiece01_05002a  @ $05002A  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Fortress_SpawnPiece01_05002a, "ax", @progbits
        .global Fortress_SpawnPiece01_05002a
Fortress_SpawnPiece01_05002a:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x1,0x21(a0)                   | +010
        lea     0x295a6a.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x0,0x22(a0)                   | +028
        addi.w  #0x90,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x2946fa.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Fortress_SpawnPiece02_05007e  @ $05007E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Fortress_SpawnPiece02_05007e, "ax", @progbits
        .global Fortress_SpawnPiece02_05007e
Fortress_SpawnPiece02_05007e:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x2,0x21(a0)                   | +010
        lea     0x295a7e.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x20,0x22(a0)                  | +028
        addi.w  #0x90,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x294710.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Fortress_SpawnPiece04_0500d2  @ $0500D2  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Fortress_SpawnPiece04_0500d2, "ax", @progbits
        .global Fortress_SpawnPiece04_0500d2
Fortress_SpawnPiece04_0500d2:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x4,0x21(a0)                   | +010
        lea     0x295a92.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x20,0x22(a0)                  | +028
        addi.w  #0x70,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x294726.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Fortress_SpawnPiece08_050126  @ $050126  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Fortress_SpawnPiece08_050126, "ax", @progbits
        .global Fortress_SpawnPiece08_050126
Fortress_SpawnPiece08_050126:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x8,0x21(a0)                   | +010
        lea     0x295aa6.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        move.l  #0xffffffff,0x74(a0)            | +020
        addi.w  #0x40,0x22(a0)                  | +028
        addi.w  #0x70,0x24(a0)                  | +02e
        movem.l a6,-(a7)                        | +034
        movem.l a0,-(a7)                        | +038
        movea.l a0,a6                           | +03c
        lea     0x29473c.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        movem.l (a7)+,a0                        | +04a
        movem.l (a7)+,a6                        | +04e
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Fortress_SpawnPiece10_05017a  @ $05017A  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Fortress_SpawnPiece10_05017a, "ax", @progbits
        .global Fortress_SpawnPiece10_05017a
Fortress_SpawnPiece10_05017a:
        lea     Prop_BlitSequence_04f1f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        move.b  #0x10,0x21(a0)                  | +010
        lea     0x295aba.l,a2                   | +016
        move.l  a2,0x70(a0)                     | +01c
        lea     0x295ace.l,a2                   | +020
        move.l  a2,0x74(a0)                     | +026
        move.l  #0xffffffff,0x78(a0)            | +02a
        addi.w  #0x60,0x22(a0)                  | +032
        addi.w  #0x10,0x24(a0)                  | +038
        movem.l a6,-(a7)                        | +03e
        movem.l a0,-(a7)                        | +042
        movea.l a0,a6                           | +046
        lea     0x2946b8.l,a0                   | +048
        jsr     0x28cd4.l                       | +04e
        movem.l (a7)+,a0                        | +054
        movem.l (a7)+,a6                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  Gatehouse_BlitDamaged_0501d8  @ $0501D8  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Gatehouse_BlitDamaged_0501d8, "ax", @progbits
        .global Gatehouse_BlitDamaged_0501d8
Gatehouse_BlitDamaged_0501d8:
        lea     0x295902.l,a2                   | +000
        jsr     Sprite_InvokeBlit8Params(pc)    | +006
        lea     0x29592a.l,a2                   | +00a
        jsr     Sprite_InvokeBlit8Params(pc)    | +010
        lea     0x295952.l,a2                   | +014
        jsr     Sprite_InvokeBlit8Params(pc)    | +01a
        lea     0x29597a.l,a2                   | +01e
        jsr     Sprite_InvokeBlit8Params(pc)    | +024
        lea     0x2959a2.l,a2                   | +028
        jsr     Sprite_InvokeBlit8Params(pc)    | +02e
        lea     0x2959ca.l,a2                   | +032
        jsr     Sprite_InvokeBlit8Params(pc)    | +038
        lea     0x2959f2.l,a2                   | +03c
        jsr     Sprite_InvokeBlit8Params(pc)    | +042
        lea     0x295a1a.l,a2                   | +046

| ----------------------------------------------------------------------------
|  SlotPrioCheck_050250  @ $050250  (16 B)
| ----------------------------------------------------------------------------
        .section .text.SlotPrioCheck_050250, "ax", @progbits
        .global SlotPrioCheck_050250
SlotPrioCheck_050250:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_050266                    | +00c

| ----------------------------------------------------------------------------
|  Entity_StepMoveWithProbe_05029c  @ $05029C  (196 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_StepMoveWithProbe_05029c, "ax", @progbits
        .global Entity_StepMoveWithProbe_05029c
Entity_StepMoveWithProbe_05029c:
        move.l  #0x27bc8,-(a7)                  | +000
        bra.w   .L0502ac                        | +006
        .global Entity_StepMoveWithProbe_05029c__L0502a6
Entity_StepMoveWithProbe_05029c__L0502a6:
.L0502a6:
        move.l  #0x27d50,-(a7)                  | +00a
.L0502ac:
        move.w  0x2a(a6),-(a7)                  | +010
        move.w  0x28(a6),-(a7)                  | +014
        move.w  0x2e(a6),-(a7)                  | +018
        move.w  0x2c(a6),-(a7)                  | +01c
        clr.w   0x2c(a6)                        | +020
        clr.w   0x2e(a6)                        | +024
        move.w  0x28(a6),-(a7)                  | +028
        bmi.w   .L0502d6                        | +02c
        move.w  #0x800,0x28(a6)                 | +030
        bra.w   .L0502dc                        | +036
.L0502d6:
        move.w  #0xf800,0x28(a6)                | +03a
.L0502dc:
        move.w  0x2a(a6),-(a7)                  | +040
        bmi.w   .L0502ee                        | +044
        move.w  #0x800,0x2a(a6)                 | +048
        bra.w   .L0502f4                        | +04e
.L0502ee:
        move.w  #0xf800,0x2a(a6)                | +052
.L0502f4:
        move.w  0x2(a7),d0                      | +058
        smi.b   d1                              | +05c
        sub.w   0x28(a6),d0                     | +05e
        smi.b   d2                              | +062
        eor.b   d1,d2                           | +064
        beq.w   .L05030c                        | +066
        add.w   d0,0x28(a6)                     | +06a
        clr.w   d0                              | +06e
.L05030c:
        move.w  d0,0x2(a7)                      | +070
        move.w  (a7),d0                         | +074
        smi.b   d1                              | +076
        sub.w   0x2a(a6),d0                     | +078
        smi.b   d2                              | +07c
        eor.b   d1,d2                           | +07e
        beq.w   .L050326                        | +080
        add.w   d0,0x2a(a6)                     | +084
        clr.w   d0                              | +088
.L050326:
        move.w  d0,(a7)                         | +08a
        movea.l 0xc(a7),a0                      | +08c
        jsr     (a0)                            | +090
        move.w  sr,d7                           | +092
        bcs.w   .L05033c                        | +094
        move.w  (a7),d0                         | +098
        and.w   0x2(a7),d0                      | +09a
        bne.b   .L0502f4                        | +09e
.L05033c:
        adda.w  #0x4,a7                         | +0a0
        move.w  (a7)+,d0                        | +0a4
        move.w  d0,0x2c(a6)                     | +0a6
        move.w  (a7)+,d1                        | +0aa
        move.w  d1,0x2e(a6)                     | +0ac
        add.w   (a7)+,d0                        | +0b0
        move.w  d0,0x28(a6)                     | +0b2
        add.w   (a7)+,d1                        | +0b6
        move.w  d1,0x2a(a6)                     | +0b8
        adda.w  #0x4,a7                         | +0bc
        move.w  d7,sr                           | +0c0
        rts                                     | +0c2

| ----------------------------------------------------------------------------
|  Allen_Physics_050360  @ $050360  (138 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Physics_050360, "ax", @progbits
        .global Allen_Physics_050360
Allen_Physics_050360:
        tst.b   0x70(a6)                        | +000
        beq.w   .L0503ce                        | +004
        move.w  0x24(a6),d1                     | +008
        jsr     0x4400e.l                       | +00c
        cmpi.w  #0xef,d1                        | +012
        blt.w   .L050380                        | +016
        move.b  #0xff,0x71(a6)                  | +01a
.L050380:
        tst.b   0x71(a6)                        | +020
        beq.w   .L050394                        | +024
        jsr     Entity_StepMoveWithProbe_05029c(pc) | +028
        scc.b   0x70(a6)                        | +02c
        bra.w   .L0503ca                        | +030
.L050394:
        move.w  0x22(a6),d1                     | +034
        move.w  0x24(a6),d2                     | +038
        jsr     0x280c6.l                       | +03c
        bcc.w   .L0503c0                        | +042
        move.w  0x22(a6),d1                     | +046
        move.w  0x24(a6),d2                     | +04a
        subq.w  #0x8,d2                         | +04e
        jsr     0x280c6.l                       | +050
        bcs.w   .L0503c0                        | +056
        move.b  #0xff,0x71(a6)                  | +05a
.L0503c0:
        jsr     0x27d50.l                       | +060
        scc.b   0x70(a6)                        | +066
.L0503ca:
        bra.w   .L0503e8                        | +06a
.L0503ce:
        jsr     0x27a92.l                       | +06e
        jsr     0x27eba.l                       | +074
        bcc.w   .L0503e8                        | +07a
        move.w  #0xffc0,0x2e(a6)                | +07e
        st.b    0x70(a6)                        | +084
.L0503e8:
        rts                                     | +088

| ----------------------------------------------------------------------------
|  Allen_AcquireTarget_0503ea  @ $0503EA  (166 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_AcquireTarget_0503ea, "ax", @progbits
        .global Allen_AcquireTarget_0503ea
Allen_AcquireTarget_0503ea:
        .dc.w   0xff00                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +002  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +006  (dato / opcode no decodificado)
        .global Allen_AcquireTarget_0503ea__L0503f2
Allen_AcquireTarget_0503ea__L0503f2:
.L0503f2:
        move.b  0x7b(a6),d0                     | +008
        beq.w   .L050400                        | +00c
        subq.b  #0x1,d0                         | +010
        move.b  d0,0x7b(a6)                     | +012
.L050400:
        move.l  0x72(a6),d0                     | +016
        bmi.w   .L05043a                        | +01a
        movea.l d0,a0                           | +01e
        jsr     0x5e338.l                       | +020
        bcc.w   .L050420                        | +026
        move.l  #0xffffffff,0x72(a6)            | +02a
        bra.w   .L05043a                        | +032
.L050420:
        lea     Allen_AcquireTarget_0503ea(pc),a0 | +036
        jsr     0x5e086.l                       | +03a
        scc.b   0x79(a6)                        | +040
        bcs.w   .L050436                        | +044
        move.l  a0,0x72(a6)                     | +048
.L050436:
        bra.w   .L050462                        | +04c
.L05043a:
        lea     Allen_AcquireTarget_0503ea(pc),a0 | +050
        jsr     0x5e086.l                       | +054
        scc.b   0x79(a6)                        | +05a
        bcs.w   .L050454                        | +05e
        move.l  a0,0x72(a6)                     | +062
        bra.w   .L050462                        | +066
.L050454:
        jsr     0x5e0d4.l                       | +06a
        bcs.w   .L050462                        | +070
        move.l  a0,0x72(a6)                     | +074
.L050462:
        move.l  0x72(a6),d0                     | +078
        bmi.w   .L050488                        | +07c
        movea.l d0,a0                           | +080
        move.w  0x22(a0),d0                     | +082
        sub.w   0x22(a6),d0                     | +086
        smi.b   d0                              | +08a
        btst    #0x0,0x3a(a6)                   | +08c
        sne.b   d1                              | +092
        eor.b   d0,d1                           | +094
        move.b  d1,0x77(a6)                     | +096
        bra.w   .L05048e                        | +09a
.L050488:
        move.b  #0xff,0x77(a6)                  | +09e
.L05048e:
        rts                                     | +0a4

| ----------------------------------------------------------------------------
|  Allen_CheckTargetAbove_050490  @ $050490  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_CheckTargetAbove_050490, "ax", @progbits
        .global Allen_CheckTargetAbove_050490
Allen_CheckTargetAbove_050490:
        move.w  0x22(a6),d0                     | +000
        subi.w  #0x20,d0                        | +004
        cmpi.w  #0x100,d0                       | +008
        bcc.w   ClearC_0504ea                   | +00c
        tst.b   0x7b(a6)                        | +010
        bne.w   ClearC_0504ea                   | +014
        move.l  0x72(a6),d0                     | +018
        bmi.w   ClearC_0504ea                   | +01c
        movea.l d0,a0                           | +020
        move.w  0x22(a0),d0                     | +022
        sub.w   0x22(a6),d0                     | +026
        smi.b   d1                              | +02a
        move.b  0x3a(a6),d2                     | +02c
        eor.b   d2,d1                           | +030
        btst    #0x0,d1                         | +032
        beq.w   ClearC_0504ea                   | +036
        tst.w   d0                              | +03a
        bpl.w   .L0504d2                        | +03c
        neg.w   d0                              | +040
.L0504d2:
        move.w  0x24(a0),d1                     | +042
        sub.w   0x24(a6),d1                     | +046
        bmi.w   ClearC_0504ea                   | +04a
        sub.w   d0,d1                           | +04e
        subi.w  #0x10,d1                        | +050
        cmpi.w  #0x20,d1                        | +054
        rts                                     | +058

| ----------------------------------------------------------------------------
|  Allen_CheckTargetFront_0504f0  @ $0504F0  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_CheckTargetFront_0504f0, "ax", @progbits
        .global Allen_CheckTargetFront_0504f0
Allen_CheckTargetFront_0504f0:
        move.w  0x22(a6),d0                     | +000
        subi.w  #0x20,d0                        | +004
        cmpi.w  #0x100,d0                       | +008
        bcc.w   ClearC_05051a                   | +00c
        move.b  0x7b(a6),d0                     | +010
        bne.w   ClearC_05051a                   | +014
        move.b  0x77(a6),d0                     | +018
        and.b   0x79(a6),d0                     | +01c
        beq.w   ClearC_05051a                   | +020

| ----------------------------------------------------------------------------
|  Allen_CheckScreenEdge_050520  @ $050520  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_CheckScreenEdge_050520, "ax", @progbits
        .global Allen_CheckScreenEdge_050520
Allen_CheckScreenEdge_050520:
        move.w  0x28(a6),d0                     | +000
        bmi.w   .L050536                        | +004
        cmpi.w  #0x110,0x22(a6)                 | +008
        bpl.w   SetC_050546                     | +00e
        bra.w   ClearC_050540                   | +012
.L050536:
        cmpi.w  #0x30,0x22(a6)                  | +016
        bmi.w   SetC_050546                     | +01c

| ----------------------------------------------------------------------------
|  Allen_CheckJumpRandom_05054c  @ $05054C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_CheckJumpRandom_05054c, "ax", @progbits
        .global Allen_CheckJumpRandom_05054c
Allen_CheckJumpRandom_05054c:
        move.l  0x72(a6),d0                     | +000
        bmi.w   ClearC_050594                   | +004
        move.w  0x24(a6),d1                     | +008
        jsr     0x4400e.l                       | +00c
        cmpi.w  #0x8f,d1                        | +012
        ble.w   ClearC_050594                   | +016
        tst.b   0x7e(a6)                        | +01a
        bne.w   ClearC_050594                   | +01e
        tst.b   0x79(a6)                        | +022
        beq.w   .L050582                        | +026
        move.b  0x7c(a6),d0                     | +02a
        or.b    0x7d(a6),d0                     | +02e
        bne.w   SetC_05059a                     | +032
.L050582:
        jsr     0x5e9b6.l                       | +036
        andi.b  #0xf,d0                         | +03c
        cmp.b   0x7a(a6),d0                     | +040
        bcs.w   SetC_05059a                     | +044

| ----------------------------------------------------------------------------
|  Allen_CheckTargetFar_0505a0  @ $0505A0  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_CheckTargetFar_0505a0, "ax", @progbits
        .global Allen_CheckTargetFar_0505a0
Allen_CheckTargetFar_0505a0:
        tst.b   0x86(a6)                        | +000
        beq.w   .L0505ac                        | +004
        subq.b  #0x1,0x86(a6)                   | +008
.L0505ac:
        move.l  0x72(a6),d0                     | +00c
        bmi.w   ClearC_0505ea                   | +010
        tst.b   0x77(a6)                        | +014
        beq.w   ClearC_0505ea                   | +018
        movea.l d0,a0                           | +01c
        move.w  0x24(a6),d0                     | +01e
        sub.w   0x24(a0),d0                     | +022
        beq.w   ClearC_0505ea                   | +026
        bmi.w   ClearC_0505ea                   | +02a
        move.w  0x22(a6),d1                     | +02e
        sub.w   0x22(a0),d1                     | +032
        bpl.w   .L0505dc                        | +036
        neg.w   d1                              | +03a
.L0505dc:
        add.w   d1,d1                           | +03c
        cmp.w   d0,d1                           | +03e
        bcs.w   ClearC_0505ea                   | +040

| ----------------------------------------------------------------------------
|  Allen_KnifeHitCheck_0505f0  @ $0505F0  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_KnifeHitCheck_0505f0, "ax", @progbits
        .global Allen_KnifeHitCheck_0505f0
Allen_KnifeHitCheck_0505f0:
        tst.b   0x45(a6)                        | +000
        bne.w   ClearC_050638                   | +004
        move.w  0x22(a6),d0                     | +008
        cmpi.w  #0x20,d0                        | +00c
        bmi.w   ClearC_050638                   | +010
        cmpi.w  #0x300,d0                       | +014
        bpl.w   ClearC_050638                   | +018
        lea     0x296dca.l,a0                   | +01c
        move.l  a0,0x4c(a6)                     | +022
        jsr     0x283ca.l                       | +026
        jsr     0x283ca.l                       | +02c
        jsr     0x283d8.l                       | +032
        btst    #0x1,0x13(a6)                   | +038
        beq.w   ClearC_050638                   | +03e

| ----------------------------------------------------------------------------
|  Allen_ThrowGrenade_05063e  @ $05063E  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_ThrowGrenade_05063e, "ax", @progbits
        .global Allen_ThrowGrenade_05063e
Allen_ThrowGrenade_05063e:
        move.l  0x72(a6),d0                     | +000
        bmi.w   .L0506a0                        | +004
        move.l  a6,-(a7)                        | +008
        lea     0x100800.l,a6                   | +00a
        lea     Allen_Grenade_051208__L0512b4(pc),a1 | +010
        jsr     0x4ae.l                         | +014
        movea.l (a7)+,a6                        | +01a
        move.w  0x22(a6),0x22(a0)               | +01c
        move.w  0x24(a6),d0                     | +022
        addi.w  #0x10,d0                        | +026
        move.w  d0,0x24(a0)                     | +02a
        move.l  a0,-(a7)                        | +02e
        jsr     0x5e9b6.l                       | +030
        andi.w  #0x1f,d0                        | +036
        subi.w  #0x10,d0                        | +03a
        movea.l (a7)+,a0                        | +03e
        movea.l 0x72(a6),a1                     | +040
        add.w   0x22(a1),d0                     | +044
        sub.w   0x22(a0),d0                     | +048
        move.w  0x24(a1),d1                     | +04c
        sub.w   0x24(a0),d1                     | +050
        move.l  a0,-(a7)                        | +054
        jsr     0x5e018.l                       | +056
        movea.l (a7)+,a0                        | +05c
        move.w  d0,0x34(a0)                     | +05e
.L0506a0:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  Allen_CheckJumpHigh_0506a2  @ $0506A2  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_CheckJumpHigh_0506a2, "ax", @progbits
        .global Allen_CheckJumpHigh_0506a2
Allen_CheckJumpHigh_0506a2:
        move.l  0x72(a6),d0                     | +000
        bmi.w   ClearC_050594                   | +004
        move.w  0x24(a6),d1                     | +008
        jsr     0x4400e.l                       | +00c
        cmpi.w  #0xef,d1                        | +012
        bge.w   ClearC_0506ea                   | +016
        tst.b   0x7f(a6)                        | +01a
        bne.w   ClearC_0506ea                   | +01e
        tst.b   0x79(a6)                        | +022
        beq.w   .L0506d8                        | +026
        move.b  0x7c(a6),d0                     | +02a
        or.b    0x7d(a6),d0                     | +02e
        bne.w   SetC_0506f0                     | +032
.L0506d8:
        jsr     0x5e9b6.l                       | +036
        andi.b  #0xf,d0                         | +03c
        cmp.b   0x7a(a6),d0                     | +040
        bcs.w   SetC_0506f0                     | +044

| ----------------------------------------------------------------------------
|  Allen_CheckTouchKnife_0506f6  @ $0506F6  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_CheckTouchKnife_0506f6, "ax", @progbits
        .global Allen_CheckTouchKnife_0506f6
Allen_CheckTouchKnife_0506f6:
        tst.b   0x45(a6)                        | +000
        bne.w   ClearC_05071a                   | +004
        tst.b   0x78(a6)                        | +008
        beq.w   .L050712                        | +00c
        tst.b   0x7c(a6)                        | +010
        bne.w   SetC_050720                     | +014
        bra.w   ClearC_05071a                   | +018
.L050712:
        tst.b   0x7d(a6)                        | +01c
        bne.w   SetC_050720                     | +020

| ----------------------------------------------------------------------------
|  Allen_CommonTick_050726  @ $050726  (228 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_CommonTick_050726, "ax", @progbits
        .global Allen_CommonTick_050726
Allen_CommonTick_050726:
        .dc.w   0xffe0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +002  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +006  (dato / opcode no decodificado)
        .global Allen_CommonTick_050726__L05072e
Allen_CommonTick_050726__L05072e:
.L05072e:
        tst.b   0x87(a6)                        | +008
        beq.w   .L05073a                        | +00c
        subq.b  #0x1,0x87(a6)                   | +010
.L05073a:
        jsr     0x2870a.l                       | +014
        bcc.w   .L0507de                        | +01a
        bclr    #0x3,0x13(a6)                   | +01e
        tst.b   0x87(a6)                        | +024
        bne.w   .L05075e                        | +028
        jsr     0x4aae0.l                       | +02c
        move.b  #0xc,0x87(a6)                   | +032
.L05075e:
        move.w  0x66(a6),d0                     | +038
        move.w  0x84(a6),d1                     | +03c
        subi.w  #0x32,d1                        | +040
        cmp.w   d1,d0                           | +044
        bge.w   .L050776                        | +046
        move.w  d1,d0                           | +04a
        move.w  d1,0x66(a6)                     | +04c
.L050776:
        cmp.w   0x80(a6),d0                     | +050
        bcc.w   .L0507c2                        | +054
        move.w  0x80(a6),d0                     | +058
        sub.w   0x82(a6),d0                     | +05c
        move.w  d0,0x80(a6)                     | +060
        move.b  0x76(a6),d0                     | +064
        andi.w  #0xff,d0                        | +068
        add.w   d0,d0                           | +06c
        lea     0x296e72.l,a0                   | +06e
        move.w  (a0,d0.w),d2                    | +074
        move.w  0x14(a6),d1                     | +078
        move.w  #0xffff,d3                      | +07c
        move.w  #0xffff,d4                      | +080
        jsr     0x2c30.l                        | +084
        move.b  0x76(a6),d0                     | +08a
        cmpi.b  #0x4,d0                         | +08e
        bcc.w   .L0507c2                        | +092
        addq.b  #0x1,d0                         | +096
        move.b  d0,0x76(a6)                     | +098
.L0507c2:
        cmpi.w  #0x0,0x66(a6)                   | +09c
        ble.w   .L0507d2                        | +0a2
        bclr    #0x0,0x13(a6)                   | +0a6
.L0507d2:
        lea     0x5e766.l,a0                    | +0ac
        jsr     0x5e770.l                       | +0b2
.L0507de:
        jsr     0x28758.l                       | +0b8
        bcc.w   .L0507ee                        | +0be
        lea     Allen_Death_050e40(pc),a1       | +0c2
        move.l  a1,(a6)                         | +0c6
        .global Allen_CommonTick_050726__L0507ee
Allen_CommonTick_050726__L0507ee:
.L0507ee:
        move.w  0x66(a6),0x84(a6)               | +0c8
        lea     Allen_CommonTick_050726(pc),a0  | +0ce
        jsr     0x5dd56.l                       | +0d2
        bcc.w   .L050808                        | +0d8
        jmp     0x518.l                         | +0dc
.L050808:
        rts                                     | +0e2

| ----------------------------------------------------------------------------
|  Allen_SpawnMuzzleFlashA_05080a  @ $05080A  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_SpawnMuzzleFlashA_05080a, "ax", @progbits
        .global Allen_SpawnMuzzleFlashA_05080a
Allen_SpawnMuzzleFlashA_05080a:
        lea     Allen_MuzzleFlash_05100e(pc),a1 | +000

| ----------------------------------------------------------------------------
|  Allen_SpawnMuzzleFlashB_050816  @ $050816  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_SpawnMuzzleFlashB_050816, "ax", @progbits
        .global Allen_SpawnMuzzleFlashB_050816
Allen_SpawnMuzzleFlashB_050816:
        lea     Allen_MuzzleFlash_05100e__L05101e(pc),a1 | +000

| ----------------------------------------------------------------------------
|  Allen_SpawnBulletLevel_050822  @ $050822  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_SpawnBulletLevel_050822, "ax", @progbits
        .global Allen_SpawnBulletLevel_050822
Allen_SpawnBulletLevel_050822:
        move.w  #0x10f4,d0                      | +000
        jsr     0x2352.l                        | +004
        move.l  a6,-(a7)                        | +00a
        lea     0x100800.l,a6                   | +00c
        lea     Allen_Bullet_05148c__L051530(pc),a1 | +012
        jsr     0x4ae.l                         | +016
        movea.l (a7)+,a6                        | +01c
        move.b  0x3a(a6),0x3a(a0)               | +01e
        move.w  #0x38,d0                        | +024
        btst    #0x0,0x3a(a6)                   | +028
        bne.w   .L050860                        | +02e
        neg.w   d0                              | +032
        move.w  #0x80,0x34(a0)                  | +034
        bra.w   .L050866                        | +03a
.L050860:
        move.w  #0x0,0x34(a0)                   | +03e
.L050866:
        add.w   0x22(a6),d0                     | +044
        move.w  d0,0x22(a0)                     | +048
        move.w  0x24(a6),d0                     | +04c
        addi.w  #0x18,d0                        | +050
        move.w  d0,0x24(a0)                     | +054
        rts                                     | +058

| ----------------------------------------------------------------------------
|  Allen_SpawnBulletDiag_05087c  @ $05087C  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_SpawnBulletDiag_05087c, "ax", @progbits
        .global Allen_SpawnBulletDiag_05087c
Allen_SpawnBulletDiag_05087c:
        move.w  #0x10f4,d0                      | +000
        jsr     0x2352.l                        | +004
        move.l  a6,-(a7)                        | +00a
        lea     0x100800.l,a6                   | +00c
        lea     Allen_Bullet_05148c__L051530(pc),a1 | +012
        jsr     0x4ae.l                         | +016
        movea.l (a7)+,a6                        | +01c
        move.b  0x3a(a6),0x3a(a0)               | +01e
        move.w  #0x28,d0                        | +024
        btst    #0x0,0x3a(a6)                   | +028
        bne.w   .L0508ba                        | +02e
        neg.w   d0                              | +032
        move.w  #0x60,0x34(a0)                  | +034
        bra.w   .L0508c0                        | +03a
.L0508ba:
        move.w  #0x20,0x34(a0)                  | +03e
.L0508c0:
        add.w   0x22(a6),d0                     | +044
        move.w  d0,0x22(a0)                     | +048
        move.w  0x24(a6),d0                     | +04c
        addi.w  #0x40,d0                        | +050
        move.w  d0,0x24(a0)                     | +054
        rts                                     | +058

| ----------------------------------------------------------------------------
|  Allen_Init_0508d6  @ $0508D6  (152 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Init_0508d6, "ax", @progbits
        .global Allen_Init_0508d6
Allen_Init_0508d6:
        lea     0x296e7a.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x66(a6)                     | +00c
        move.w  d0,0x84(a6)                     | +010
        move.w  d0,d1                           | +014
        tst.w   d0                              | +016
        bne.w   .L0508fa                        | +018
        move.w  d6,d6                           | +01c
        move.w  d6,d6                           | +01e
        move.w  d6,d6                           | +020
        trap    #0xf                            | +022
.L0508fa:
        andi.l  #0xffff,d0                      | +024
        divu.w  #0x5,d0                         | +02a
        move.w  d0,0x82(a6)                     | +02e
        sub.w   d0,d1                           | +032
        move.w  d1,0x80(a6)                     | +034
        move.w  #0x18e,d1                       | +038
        jsr     0x29f2.l                        | +03c
        clr.b   0x76(a6)                        | +042
        lea     Allen_TouchSensor_050f7e__L050fa6(pc),a1 | +046
        jsr     0x4ae.l                         | +04a
        lea     Allen_TouchSensor_050f7e__L050fba(pc),a1 | +050
        jsr     0x4ae.l                         | +054
        lea     Allen_TouchSensor_050f7e(pc),a1 | +05a
        jsr     0x4ae.l                         | +05e
        lea     Allen_TouchSensor_050f7e__L050f92(pc),a1 | +064
        jsr     0x4ae.l                         | +068
        clr.b   0x7b(a6)                        | +06e
        clr.b   0x7a(a6)                        | +072
        clr.b   0x86(a6)                        | +076
        clr.b   0x87(a6)                        | +07a
        move.w  0x22(a6),d0                     | +07e
        cmpi.w  #0xa0,d0                        | +082
        bpl.w   .L05096a                        | +086
        bset    #0x0,0x3a(a6)                   | +08a
        clr.b   0x78(a6)                        | +090
.L05096a:
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +094

| ----------------------------------------------------------------------------
|  Allen_Spawn_050976  @ $050976  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Spawn_050976, "ax", @progbits
        .global Allen_Spawn_050976
Allen_Spawn_050976:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        lea     0x296894.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        tst.b   0x78(a6)                        | +014
        beq.w   .L05099c                        | +018
        move.w  #0xfa00,0x28(a6)                | +01c
        bra.w   .L0509a2                        | +022
.L05099c:
        move.w  #0x600,0x28(a6)                 | +026
.L0509a2:
        lea     .L0509a8(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L0509a8:
        move.b  #0x1e,0x45(a6)                  | +032
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +038
        bsr.w   Allen_Physics_050360            | +03c
        jsr     0x28d70.l                       | +040
        move.w  0x22(a6),d0                     | +046
        subi.w  #0x30,d0                        | +04a
        cmpi.w  #0xe0,d0                        | +04e
        bcc.w   .L0509d2                        | +052
        lea     Allen_Intro_0509de(pc),a1       | +056
        move.l  a1,(a6)                         | +05a
.L0509d2:
        bra.w   Allen_CommonTick_050726__L05072e | +05c

| ----------------------------------------------------------------------------
|  Allen_TauntVoiceTbl_0509d6  @ $0509D6  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_TauntVoiceTbl_0509d6, "ax", @progbits
        .global Allen_TauntVoiceTbl_0509d6
Allen_TauntVoiceTbl_0509d6:
        .dc.w   0x113c                        | +000  (dato / opcode no decodificado)
        .dc.w   0x113d                        | +002  (dato / opcode no decodificado)
        .dc.w   0x113e                        | +004  (dato / opcode no decodificado)
        .dc.w   0x113c                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Allen_Intro_0509de  @ $0509DE  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Intro_0509de, "ax", @progbits
        .global Allen_Intro_0509de
Allen_Intro_0509de:
        lea     0x296bd8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.w  #0x5a,0x30(a6)                  | +00c
        clr.w   0x28(a6)                        | +012
        lea     .L0509fa(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0509fa:
        move.b  #0x1e,0x45(a6)                  | +01c
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +022
        bsr.w   Allen_Physics_050360            | +026
        jsr     0x28d70.l                       | +02a
        subq.w  #0x1,0x30(a6)                   | +030
        bne.w   .L050a26                        | +034
        lea     0x2963c4.l,a0                   | +038
        move.l  a0,0x48(a6)                     | +03e
        lea     Allen_Main_050a2a(pc),a1        | +042
        move.l  a1,(a6)                         | +046
.L050a26:
        bra.w   Allen_CommonTick_050726__L05072e | +048

| ----------------------------------------------------------------------------
|  Allen_Main_050a2a  @ $050A2A  (248 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Main_050a2a, "ax", @progbits
        .global Allen_Main_050a2a
Allen_Main_050a2a:
        tst.b   0x78(a6)                        | +000
        beq.w   .L050a3c                        | +004
        move.w  #0xfa00,0x28(a6)                | +008
        bra.w   .L050a42                        | +00e
.L050a3c:
        move.w  #0x600,0x28(a6)                 | +012
.L050a42:
        move.b  0x78(a6),d0                     | +018
        move.b  0x3a(a6),d1                     | +01c
        eor.b   d1,d0                           | +020
        btst    #0x0,d0                         | +022
        beq.w   .L050a64                        | +026
        lea     0x296894.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        bra.w   .L050a70                        | +036
.L050a64:
        lea     0x296908.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
.L050a70:
        lea     .L050a76(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L050a76:
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +04c
        bsr.w   Allen_Physics_050360            | +050
        jsr     0x28d70.l                       | +054
        move.l  0x72(a6),d0                     | +05a
        bpl.w   .L050a92                        | +05e
        lea     Allen_IdleNoTarget_050c18(pc),a1 | +062
        move.l  a1,(a6)                         | +066
.L050a92:
        tst.b   0x77(a6)                        | +068
        bne.w   .L050aa0                        | +06c
        lea     Allen_Turn_050b22(pc),a1        | +070
        move.l  a1,(a6)                         | +074
.L050aa0:
        jsr     Allen_CheckTargetAbove_050490(pc) | +076
        bcc.w   .L050aae                        | +07a
        lea     Allen_FireMGLevel_050bd2(pc),a1 | +07e
        move.l  a1,(a6)                         | +082
.L050aae:
        jsr     Allen_CheckTargetFront_0504f0(pc) | +084
        bcc.w   .L050abc                        | +088
        lea     Allen_FireMGHigh_050bc2(pc),a1  | +08c
        move.l  a1,(a6)                         | +090
.L050abc:
        jsr     Allen_KnifeHitCheck_0505f0(pc)  | +092
        bcc.w   .L050aca                        | +096
        lea     Allen_KnifeAttack_050dfc(pc),a1 | +09a
        move.l  a1,(a6)                         | +09e
.L050aca:
        jsr     Allen_CheckTargetFar_0505a0(pc) | +0a0
        bcc.w   .L050ad8                        | +0a4
        lea     Allen_Taunt_050b6a(pc),a1       | +0a8
        move.l  a1,(a6)                         | +0ac
.L050ad8:
        jsr     Allen_CheckScreenEdge_050520(pc) | +0ae
        bcc.w   .L050ae6                        | +0b2
        lea     Allen_ToggleRetreat_050b5e(pc),a1 | +0b6
        move.l  a1,(a6)                         | +0ba
.L050ae6:
        jsr     Allen_CheckJumpRandom_05054c(pc) | +0bc
        bcc.w   .L050af4                        | +0c0
        lea     Allen_JumpToward_050d1a(pc),a1  | +0c4
        move.l  a1,(a6)                         | +0c8
.L050af4:
        jsr     Allen_CheckJumpHigh_0506a2(pc)  | +0ca
        bcc.w   .L050b02                        | +0ce
        lea     Allen_JumpHigh_050dbe(pc),a1    | +0d2
        move.l  a1,(a6)                         | +0d6
.L050b02:
        jsr     Allen_CheckTouchKnife_0506f6(pc) | +0d8
        bcc.w   .L050b10                        | +0dc
        lea     Allen_ToggleRetreat_050b5e(pc),a1 | +0e0
        move.l  a1,(a6)                         | +0e4
.L050b10:
        tst.b   0x70(a6)                        | +0e6
        beq.w   .L050b1e                        | +0ea
        lea     Allen_JumpDecide_050c88(pc),a1  | +0ee
        move.l  a1,(a6)                         | +0f2
.L050b1e:
        bra.w   Allen_CommonTick_050726__L05072e | +0f4

| ----------------------------------------------------------------------------
|  Allen_Turn_050b22  @ $050B22  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Turn_050b22, "ax", @progbits
        .global Allen_Turn_050b22
Allen_Turn_050b22:
        clr.b   0x86(a6)                        | +000
        lea     0x296954.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        clr.w   0x28(a6)                        | +010
        lea     .L050b3c(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L050b3c:
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +01a
        bsr.w   Allen_Physics_050360            | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L050b5a                        | +028
        bchg    #0x0,0x3a(a6)                   | +02c
        lea     Allen_Main_050a2a(pc),a1        | +032
        move.l  a1,(a6)                         | +036
.L050b5a:
        bra.w   Allen_CommonTick_050726__L05072e | +038

| ----------------------------------------------------------------------------
|  Allen_ToggleRetreat_050b5e  @ $050B5E  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_ToggleRetreat_050b5e, "ax", @progbits
        .global Allen_ToggleRetreat_050b5e
Allen_ToggleRetreat_050b5e:
        addq.b  #0x1,0x7a(a6)                   | +000
        not.b   0x78(a6)                        | +004
        bra.w   Allen_Main_050a2a               | +008

| ----------------------------------------------------------------------------
|  Allen_Taunt_050b6a  @ $050B6A  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Taunt_050b6a, "ax", @progbits
        .global Allen_Taunt_050b6a
Allen_Taunt_050b6a:
        tst.b   0x86(a6)                        | +000
        bne.w   .L050b90                        | +004
        move.b  #0x1e,0x86(a6)                  | +008
        jsr     0x5e9b6.l                       | +00e
        andi.w  #0x6,d0                         | +014
        lea     Allen_TauntVoiceTbl_0509d6(pc),a0 | +018
        move.w  (a0,d0.w),d0                    | +01c
        jsr     0x2352.l                        | +020
.L050b90:
        lea     0x296b76.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        clr.w   0x28(a6)                        | +032
        lea     .L050ba6(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L050ba6:
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +03c
        bsr.w   Allen_Physics_050360            | +040
        jsr     0x28d70.l                       | +044
        bcc.w   .L050bbe                        | +04a
        lea     Allen_Main_050a2a(pc),a1        | +04e
        move.l  a1,(a6)                         | +052
.L050bbe:
        bra.w   Allen_CommonTick_050726__L05072e | +054

| ----------------------------------------------------------------------------
|  Allen_FireMGHigh_050bc2  @ $050BC2  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_FireMGHigh_050bc2, "ax", @progbits
        .global Allen_FireMGHigh_050bc2
Allen_FireMGHigh_050bc2:
        lea     0x29658a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   Allen_FireMGLevel_050bd2__L050bde | +00c

| ----------------------------------------------------------------------------
|  Allen_FireMGLevel_050bd2  @ $050BD2  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_FireMGLevel_050bd2, "ax", @progbits
        .global Allen_FireMGLevel_050bd2
Allen_FireMGLevel_050bd2:
        lea     0x29668c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global Allen_FireMGLevel_050bd2__L050bde
Allen_FireMGLevel_050bd2__L050bde:
.L050bde:
        clr.b   0x86(a6)                        | +00c
        clr.w   0x28(a6)                        | +010
        lea     .L050bec(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L050bec:
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +01a
        bsr.w   Allen_Physics_050360            | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L050c14                        | +028
        lea     0x296efc.l,a0                   | +02c
        jsr     0x799de.l                       | +032
        move.b  d0,0x7b(a6)                     | +038
        lea     Allen_Main_050a2a(pc),a1        | +03c
        move.l  a1,(a6)                         | +040
.L050c14:
        bra.w   Allen_CommonTick_050726__L05072e | +042

| ----------------------------------------------------------------------------
|  Allen_IdleNoTarget_050c18  @ $050C18  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_IdleNoTarget_050c18, "ax", @progbits
        .global Allen_IdleNoTarget_050c18
Allen_IdleNoTarget_050c18:
        clr.w   0x28(a6)                        | +000
        lea     0x296bd8.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L050c2e(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L050c2e:
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +016
        bsr.w   Allen_Physics_050360            | +01a
        jsr     0x28d70.l                       | +01e
        move.l  0x72(a6),d0                     | +024
        bmi.w   .L050c4a                        | +028
        lea     Allen_Main_050a2a(pc),a1        | +02c
        move.l  a1,(a6)                         | +030
.L050c4a:
        bra.w   Allen_CommonTick_050726__L05072e | +032

| ----------------------------------------------------------------------------
|  Allen_Land_050c4e  @ $050C4E  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Land_050c4e, "ax", @progbits
        .global Allen_Land_050c4e
Allen_Land_050c4e:
        clr.b   0x86(a6)                        | +000
        lea     0x296a04.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        clr.b   0x7a(a6)                        | +010
        clr.w   0x28(a6)                        | +014
        lea     .L050c6c(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L050c6c:
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +01e
        bsr.w   Allen_Physics_050360            | +022
        jsr     0x28d70.l                       | +026
        bcc.w   .L050c84                        | +02c
        lea     Allen_Main_050a2a(pc),a1        | +030
        move.l  a1,(a6)                         | +034
.L050c84:
        bra.w   Allen_CommonTick_050726__L05072e | +036

| ----------------------------------------------------------------------------
|  Allen_JumpDecide_050c88  @ $050C88  (146 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_JumpDecide_050c88, "ax", @progbits
        .global Allen_JumpDecide_050c88
Allen_JumpDecide_050c88:
        move.w  0x24(a6),d1                     | +000
        jsr     0x4400e.l                       | +004
        cmpi.w  #0x8f,d1                        | +00a
        ble.w   .L050cb4                        | +00e
        move.l  0x72(a6),d0                     | +012
        bmi.w   .L050cb4                        | +016
        movea.l d0,a0                           | +01a
        move.w  0x24(a0),d0                     | +01c
        cmp.w   0x24(a6),d0                     | +020
        bcs.w   Allen_JumpToward_050d1a         | +024
        bra.w   .L050cf6                        | +028
.L050cb4:
        tst.b   0x79(a6)                        | +02c
        bne.w   .L050cf6                        | +030
        lea     0x2969bc.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        move.w  #0x886,0x2a(a6)                 | +040
        move.w  #0xfedd,0x2e(a6)                | +046
        move.w  #0x444,d0                       | +04c
        tst.b   0x78(a6)                        | +050
        beq.w   .L050ce2                        | +054
        neg.w   d0                              | +058
.L050ce2:
        move.w  d0,0x28(a6)                     | +05a
        move.b  #0xff,0x70(a6)                  | +05e
        lea     Allen_Airborne_050d90(pc),a1    | +064
        move.l  a1,(a6)                         | +068
        bra.w   Allen_Airborne_050d90           | +06a
.L050cf6:
        lea     0x2969bc.l,a0                   | +06e
        jsr     0x28cd4.l                       | +074
        clr.w   0x28(a6)                        | +07a
        clr.w   0x2a(a6)                        | +07e
        move.w  #0xffc0,0x2e(a6)                | +082
        lea     Allen_Airborne_050d90(pc),a1    | +088
        move.l  a1,(a6)                         | +08c
        bra.w   Allen_Airborne_050d90           | +08e

| ----------------------------------------------------------------------------
|  Allen_JumpToward_050d1a  @ $050D1A  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_JumpToward_050d1a, "ax", @progbits
        .global Allen_JumpToward_050d1a
Allen_JumpToward_050d1a:
        lea     0x2969bc.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.l  #0x1111,d0                      | +00c
        move.w  #0x180,d1                       | +012
        mulu.w  d0,d1                           | +016
        lsr.l   #0x8,d1                         | +018
        mulu.w  d0,d1                           | +01a
        swap    d1                              | +01c
        neg.w   d1                              | +01e
        move.w  d1,0x2e(a6)                     | +020
        move.w  #0xf0,d1                        | +024
        mulu.w  d0,d1                           | +028
        lsr.l   #0x8,d1                         | +02a
        move.w  d1,0x2a(a6)                     | +02c
        move.w  0x24(a6),d1                     | +030
        jsr     0x4400e.l                       | +034
        cmpi.w  #0xbf,d1                        | +03a
        ble.w   .L050d64                        | +03e
        move.w  #0x30,d0                        | +042
        bra.w   .L050d68                        | +046
.L050d64:
        move.w  #0x60,d0                        | +04a
.L050d68:
        cmpi.w  #0xa0,0x22(a6)                  | +04e
        bmi.w   .L050d78                        | +054
        neg.w   d0                              | +058
        addi.w  #0x140,d0                       | +05a
.L050d78:
        sub.w   0x22(a6),d0                     | +05e
        muls.w  #0x11,d0                        | +062
        move.w  d0,0x28(a6)                     | +066
        move.b  #0xff,0x70(a6)                  | +06a
        lea     Allen_Airborne_050d90(pc),a1    | +070
        move.l  a1,(a6)                         | +074

| ----------------------------------------------------------------------------
|  Allen_Airborne_050d90  @ $050D90  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Airborne_050d90, "ax", @progbits
        .global Allen_Airborne_050d90
Allen_Airborne_050d90:
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +000
        bsr.w   Allen_Physics_050360            | +004
        jsr     0x28d70.l                       | +008
        tst.b   0x70(a6)                        | +00e
        bne.w   .L050dac                        | +012
        lea     Allen_Land_050c4e(pc),a1        | +016
        move.l  a1,(a6)                         | +01a
.L050dac:
        jsr     Allen_KnifeHitCheck_0505f0(pc)  | +01c
        bcc.w   .L050dba                        | +020
        lea     Allen_KnifeAttack_050dfc(pc),a1 | +024
        move.l  a1,(a6)                         | +028
.L050dba:
        bra.w   Allen_CommonTick_050726__L05072e | +02a

| ----------------------------------------------------------------------------
|  Allen_JumpHigh_050dbe  @ $050DBE  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_JumpHigh_050dbe, "ax", @progbits
        .global Allen_JumpHigh_050dbe
Allen_JumpHigh_050dbe:
        lea     0x2969bc.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.w  #0x0,d0                         | +00c
        jsr     0x5dca4.l                       | +010
        move.w  d0,0x28(a6)                     | +016
        move.w  #0x922,0x2a(a6)                 | +01a
        move.w  #0xfd64,0x2e(a6)                | +020
        move.w  #0x0,0x2c(a6)                   | +026
        move.b  #0xff,0x70(a6)                  | +02c
        clr.b   0x71(a6)                        | +032
        lea     Allen_Airborne_050d90(pc),a1    | +036
        move.l  a1,(a6)                         | +03a
        bra.b   Allen_Airborne_050d90           | +03c

| ----------------------------------------------------------------------------
|  Allen_KnifeAttack_050dfc  @ $050DFC  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_KnifeAttack_050dfc, "ax", @progbits
        .global Allen_KnifeAttack_050dfc
Allen_KnifeAttack_050dfc:
        clr.b   0x86(a6)                        | +000
        lea     0x296c82.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        jsr     0x283ca.l                       | +010
        lea     .L050e18(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L050e18:
        bsr.w   Allen_AcquireTarget_0503ea__L0503f2 | +01c
        bsr.w   Allen_Physics_050360            | +020
        jsr     0x28d70.l                       | +024
        bcc.w   .L050e30                        | +02a
        lea     Allen_Main_050a2a(pc),a1        | +02e
        move.l  a1,(a6)                         | +032
.L050e30:
        tst.b   0x70(a6)                        | +034
        bne.w   .L050e3c                        | +038
        clr.w   0x28(a6)                        | +03c
.L050e3c:
        bra.w   Allen_CommonTick_050726__L05072e | +040

| ----------------------------------------------------------------------------
|  Allen_Death_050e40  @ $050E40  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Death_050e40, "ax", @progbits
        .global Allen_Death_050e40
Allen_Death_050e40:
        jsr     0x4aad2.l                       | +000
        move.w  #0x1140,d0                      | +006
        jsr     0x2352.l                        | +00a
        move.w  0x14(a6),d1                     | +010
        move.w  #0x18e,d2                       | +014
        move.w  #0x1,d3                         | +018
        move.w  #0x1,d4                         | +01c
        jsr     0x2c30.l                        | +020
        tst.b   0x70(a6)                        | +026
        bne.w   Allen_DeathFall_050e84__L050ea2 | +02a
        lea     0x296a4c.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        clr.w   0x28(a6)                        | +03a
        lea     Allen_DeathFall_050e84(pc),a1   | +03e
        move.l  a1,(a6)                         | +042

| ----------------------------------------------------------------------------
|  Allen_DeathFall_050e84  @ $050E84  (114 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_DeathFall_050e84, "ax", @progbits
        .global Allen_DeathFall_050e84
Allen_DeathFall_050e84:
        move.b  #0x2,0x45(a6)                   | +000
        bsr.w   Allen_Physics_050360            | +006
        jsr     0x28d70.l                       | +00a
        bcc.w   .L050e9e                        | +010
        lea     Allen_DeathFade_050f56(pc),a1   | +014
        move.l  a1,(a6)                         | +018
.L050e9e:
        bra.w   Allen_CommonTick_050726__L0507ee | +01a
        .global Allen_DeathFall_050e84__L050ea2
Allen_DeathFall_050e84__L050ea2:
.L050ea2:
        lea     0x296abc.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        move.w  #0x0,d0                         | +02a
        jsr     0x5dca4.l                       | +02e
        move.w  d0,0x28(a6)                     | +034
        move.w  #0x400,0x2a(a6)                 | +038
        move.w  #0xff00,0x2e(a6)                | +03e
        move.w  #0x0,0x2c(a6)                   | +044
        lea     .L050ed4(pc),a1                 | +04a
        move.l  a1,(a6)                         | +04e
.L050ed4:
        move.b  #0x2,0x45(a6)                   | +050
        jsr     Allen_Physics_050360(pc)        | +056
        jsr     0x28d70.l                       | +05a
        tst.b   0x70(a6)                        | +060
        bne.w   .L050ef2                        | +064
        lea     Allen_DeathLand_050ef6(pc),a1   | +068
        move.l  a1,(a6)                         | +06c
.L050ef2:
        bra.w   Allen_CommonTick_050726__L0507ee | +06e

| ----------------------------------------------------------------------------
|  Allen_DeathLand_050ef6  @ $050EF6  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_DeathLand_050ef6, "ax", @progbits
        .global Allen_DeathLand_050ef6
Allen_DeathLand_050ef6:
        lea     0x296adc.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x28(a6)                        | +00c
        lea     Allen_DeathFall_050e84(pc),a1   | +010
        move.l  a1,(a6)                         | +014
        bra.w   Allen_DeathFall_050e84          | +016

| ----------------------------------------------------------------------------
|  Allen_Stagger_050f10  @ $050F10  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Stagger_050f10, "ax", @progbits
        .global Allen_Stagger_050f10
Allen_Stagger_050f10:
        lea     0x296b24.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x28(a6)                        | +00c
        lea     .L050f26(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L050f26:
        move.b  #0x2,0x45(a6)                   | +016
        bsr.w   Allen_Physics_050360            | +01c
        jsr     0x28d70.l                       | +020
        bcc.w   .L050f52                        | +026
        move.b  #0xf,0x45(a6)                   | +02a
        move.b  0x45(a6),0x59(a6)               | +030
        bclr    #0x3,0x13(a6)                   | +036
        lea     Allen_Main_050a2a(pc),a1        | +03c
        move.l  a1,(a6)                         | +040
.L050f52:
        bra.w   Allen_CommonTick_050726__L0507ee | +042

| ----------------------------------------------------------------------------
|  Allen_DeathFade_050f56  @ $050F56  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_DeathFade_050f56, "ax", @progbits
        .global Allen_DeathFade_050f56
Allen_DeathFade_050f56:
        move.b  #0x1e,0x59(a6)                  | +000
        lea     .L050f62(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L050f62:
        bsr.w   Allen_Physics_050360            | +00c
        jsr     0x28d70.l                       | +010
        tst.b   0x59(a6)                        | +016
        bne.w   .L050f7a                        | +01a
        jmp     0x518.l                         | +01e
.L050f7a:
        bra.w   Allen_CommonTick_050726__L0507ee | +024

| ----------------------------------------------------------------------------
|  Allen_TouchSensor_050f7e  @ $050F7E  (144 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_TouchSensor_050f7e, "ax", @progbits
        .global Allen_TouchSensor_050f7e
Allen_TouchSensor_050f7e:
        lea     0x29643a.l,a0                   | +000
        move.l  a0,0x48(a6)                     | +006
        move.w  #0x7e,0x5c(a6)                  | +00a
        bra.w   .L050fca                        | +010
        .global Allen_TouchSensor_050f7e__L050f92
Allen_TouchSensor_050f7e__L050f92:
.L050f92:
        lea     0x29648e.l,a0                   | +014
        move.l  a0,0x48(a6)                     | +01a
        move.w  #0x7f,0x5c(a6)                  | +01e
        bra.w   .L050fca                        | +024
        .global Allen_TouchSensor_050f7e__L050fa6
Allen_TouchSensor_050f7e__L050fa6:
.L050fa6:
        lea     0x2964e2.l,a0                   | +028
        move.l  a0,0x48(a6)                     | +02e
        move.w  #0x7c,0x5c(a6)                  | +032
        bra.w   .L050fca                        | +038
        .global Allen_TouchSensor_050f7e__L050fba
Allen_TouchSensor_050f7e__L050fba:
.L050fba:
        lea     0x296536.l,a0                   | +03c
        move.l  a0,0x48(a6)                     | +042
        move.w  #0x7d,0x5c(a6)                  | +046
.L050fca:
        lea     .L050fd0(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L050fd0:
        movea.l 0xc(a6),a0                      | +052
        move.w  0x22(a0),0x22(a6)               | +056
        move.w  0x24(a0),0x24(a6)               | +05c
        tst.b   0x87(a0)                        | +062
        bne.w   .L050ffc                        | +066
        jsr     0x2870a.l                       | +06a
        movea.l 0xc(a6),a0                      | +070
        adda.w  0x5c(a6),a0                     | +074
        scs.b   (a0)                            | +078
        bra.w   .L051006                        | +07a
.L050ffc:
        movea.l 0xc(a6),a0                      | +07e
        adda.w  0x5c(a6),a0                     | +082
        clr.b   (a0)                            | +086
.L051006:
        bclr    #0x3,0x13(a6)                   | +088
        rts                                     | +08e

| ----------------------------------------------------------------------------
|  Allen_MuzzleFlash_05100e  @ $05100E  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_MuzzleFlash_05100e, "ax", @progbits
        .global Allen_MuzzleFlash_05100e
Allen_MuzzleFlash_05100e:
        lea     0x29663a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L05102a                        | +00c
        .global Allen_MuzzleFlash_05100e__L05101e
Allen_MuzzleFlash_05100e__L05101e:
.L05101e:
        lea     0x296784.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
.L05102a:
        move.w  #0x18f,d1                       | +01c
        jsr     0x236e.l                        | +020
        lea     .L05103a(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L05103a:
        movea.l 0xc(a6),a0                      | +02c
        move.b  0x3a(a0),0x3a(a6)               | +030
        move.w  0x38(a0),0x38(a6)               | +036
        move.w  0x22(a0),0x22(a6)               | +03c
        move.w  0x24(a0),0x24(a6)               | +042
        jsr     0x28d70.l                       | +048
        bcc.w   .L051066                        | +04e
        jmp     0x518.l                         | +052
.L051066:
        rts                                     | +058

| ----------------------------------------------------------------------------
|  Allen_GrenadeFrameOffsets_051068  @ $051068  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_GrenadeFrameOffsets_051068, "ax", @progbits
        .global Allen_GrenadeFrameOffsets_051068
Allen_GrenadeFrameOffsets_051068:
        .dc.w   0x0016                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +004  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Allen_GrenadeAttackTbl_051070  @ $051070  (408 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_GrenadeAttackTbl_051070, "ax", @progbits
        .global Allen_GrenadeAttackTbl_051070
Allen_GrenadeAttackTbl_051070:
        .dc.w   0x0006                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfffa                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0306                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +054  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +064  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +070  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)
        .global Allen_GrenadeAttackTbl_051070__L051114
Allen_GrenadeAttackTbl_051070__L051114:
.L051114:
        .dc.w   0x0018                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0064                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +0de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0064                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +100  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +108  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +10a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +114  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +116  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +118  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +120  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +122  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +124  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +126  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +12e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +138  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +13a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +13e  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +140  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +142  (dato / opcode no decodificado)
        .dc.w   0x0418                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0064                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +148  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +14c  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +150  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +154  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +156  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +158  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +15a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +160  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +164  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +166  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +168  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +170  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +172  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +174  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +176  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +178  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +17e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +180  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +184  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +186  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +188  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +18a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +18e  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +190  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +192  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +194  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +196  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Allen_Grenade_051208  @ $051208  (386 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Grenade_051208, "ax", @progbits
        .global Allen_Grenade_051208
Allen_Grenade_051208:
        .dc.w   0x0800                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x83ca                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0d54                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0d74                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0d94                        | +020  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0db4                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0dd8                        | +034  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0dfc                        | +03e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0e1c                        | +048  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0e44                        | +052  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +056  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0d54                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +060  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0d74                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0d94                        | +070  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +074  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0db4                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0dd8                        | +084  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +088  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0dfc                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +092  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0e1c                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0e44                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x120e                        | +0aa  (dato / opcode no decodificado)
        .global Allen_Grenade_051208__L0512b4
Allen_Grenade_051208__L0512b4:
.L0512b4:
        bset    #0x4,0x6b(a6)                   | +0ac
        move.w  #0xd000,0x38(a6)                | +0b2
        lea     0x296f7e.l,a0                   | +0b8
        jsr     0x799de.l                       | +0be
        move.w  d0,d1                           | +0c4
        move.w  0x34(a6),d0                     | +0c6
        jsr     0x13c0e.l                       | +0ca
        move.w  d1,0x28(a6)                     | +0d0
        move.w  d2,0x2a(a6)                     | +0d4
        clr.w   0x2c(a6)                        | +0d8
        clr.w   0x2e(a6)                        | +0dc
        move.w  #0x15d,d1                       | +0e0
        jsr     0x236e.l                        | +0e4
        move.w  #0x164,d1                       | +0ea
        jsr     0x236e.l                        | +0ee
        move.w  #0x165,d1                       | +0f4
        jsr     0x236e.l                        | +0f8
        lea     Allen_Grenade_051208(pc),a0     | +0fe
        jsr     0x28cd4.l                       | +102
        lea     Allen_GrenadeAttackTbl_051070(pc),a0 | +108
        move.l  a0,0x4c(a6)                     | +10c
        jsr     0x283ca.l                       | +110
        lea     .L051324(pc),a1                 | +116
        move.l  a1,(a6)                         | +11a
.L051324:
        jsr     Entity_StepMoveWithProbe_05029c__L0502a6(pc) | +11c
        bcc.w   .L051332                        | +120
        lea     Allen_GrenadeExplode_051392(pc),a1 | +124
        move.l  a1,(a6)                         | +128
.L051332:
        moveq   #0,d0                           | +12a
        move.b  0x106f28.l,d0                   | +12c
        andi.w  #0x3,d0                         | +132
        lsl.w   #0x1,d0                         | +136
        lea     Allen_GrenadeFrameOffsets_051068(pc),a0 | +138
        move.w  (a0,d0.w),d1                    | +13c
        move.w  (a6,d1.w),0x14(a6)              | +140
        jsr     0x28d70.l                       | +146
        btst    #0x1,0x13(a6)                   | +14c
        beq.w   .L051364                        | +152
        lea     Allen_GrenadeExplode_051392__L0513da(pc),a1 | +156
        move.l  a1,(a6)                         | +15a
.L051364:
        jsr     0x2870a.l                       | +15c
        bcc.w   .L051374                        | +162
        lea     Allen_GrenadeExplode_051392__L0513b6(pc),a1 | +166
        move.l  a1,(a6)                         | +16a
.L051374:
        movea.l #0xffffffff,a0                  | +16c
        lea     0x55cc8.l,a0                    | +172
        jsr     0x5dd56.l                       | +178
        bcc.w   SetHandlerRts_051390            | +17e

| ----------------------------------------------------------------------------
|  Allen_GrenadeExplode_051392  @ $051392  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_GrenadeExplode_051392, "ax", @progbits
        .global Allen_GrenadeExplode_051392
Allen_GrenadeExplode_051392:
        move.w  #0x2000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x10,0x38(a6)                  | +010
        move.w  #0x1027,d0                      | +016
        jsr     0x2352.l                        | +01a
        bra.w   .L0513fa                        | +020
        .global Allen_GrenadeExplode_051392__L0513b6
Allen_GrenadeExplode_051392__L0513b6:
.L0513b6:
        move.w  #0x8000,d0                      | +024
        jsr     0x28134.l                       | +028
        andi.w  #0xffe3,0x38(a6)                | +02e
        ori.w   #0x4,0x38(a6)                   | +034
        move.w  #0x1027,d0                      | +03a
        jsr     0x2352.l                        | +03e
        bra.w   .L0513fa                        | +044
        .global Allen_GrenadeExplode_051392__L0513da
Allen_GrenadeExplode_051392__L0513da:
.L0513da:
        move.w  #0x8000,d0                      | +048
        jsr     0x28134.l                       | +04c
        andi.w  #0xffe3,0x38(a6)                | +052
        ori.w   #0x10,0x38(a6)                  | +058
        move.w  #0xffff,d0                      | +05e
        jsr     0x2352.l                        | +062
.L0513fa:
        move.l  #0xffffffff,0x48(a6)            | +068
        bclr    #0x1,0x13(a6)                   | +070
        bclr    #0x3,0x13(a6)                   | +076
        bclr    #0x0,0x13(a6)                   | +07c
        jsr     0x13600.l                       | +082
        move.w  #0x1e0,d1                       | +088
        jsr     0x236e.l                        | +08c
        lea     0x29e76c.l,a0                   | +092
        jsr     0x28cd4.l                       | +098
        lea     Allen_GrenadeAttackTbl_051070__L051114(pc),a0 | +09e
        move.l  a0,0x4c(a6)                     | +0a2
        jsr     0x283ca.l                       | +0a6
        jsr     0x283ca.l                       | +0ac
        jsr     0x283d8.l                       | +0b2

| ----------------------------------------------------------------------------
|  Allen_GrenadeExplodeTick_051452  @ $051452  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_GrenadeExplodeTick_051452, "ax", @progbits
        .global Allen_GrenadeExplodeTick_051452
Allen_GrenadeExplodeTick_051452:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L05146e                        | +00c
        jsr     0x5b6.l                         | +010
        jmp     0x518.l                         | +016
.L05146e:
        lea     0xffff.w,a0                     | +01c
        move.l  a0,0x4c(a6)                     | +020

| ----------------------------------------------------------------------------
|  Allen_Bullet_05148c  @ $05148C  (366 B)
| ----------------------------------------------------------------------------
        .section .text.Allen_Bullet_05148c, "ax", @progbits
        .global Allen_Bullet_05148c
Allen_Bullet_05148c:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfffc                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0307                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +054  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +064  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +070  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)
        .global Allen_Bullet_05148c__L051530
Allen_Bullet_05148c__L051530:
.L051530:
        move.w  #0x18f,d1                       | +0a4
        jsr     0x236e.l                        | +0a8
        lea     0x2967d6.l,a0                   | +0ae
        jsr     0x28cd4.l                       | +0b4
        lea     0x297000.l,a0                   | +0ba
        jsr     0x799de.l                       | +0c0
        move.w  d0,d1                           | +0c6
        move.w  0x34(a6),d0                     | +0c8
        jsr     0x13c0e.l                       | +0cc
        move.w  d1,0x28(a6)                     | +0d2
        move.w  d2,0x2a(a6)                     | +0d6
        lea     Allen_Bullet_05148c(pc),a0      | +0da
        move.l  a0,0x4c(a6)                     | +0de
        jsr     0x283ca.l                       | +0e2
        lea     .L05157a(pc),a1                 | +0e8
        move.l  a1,(a6)                         | +0ec
.L05157a:
        jsr     Entity_StepMoveWithProbe_05029c__L0502a6(pc) | +0ee
        jsr     0x28d70.l                       | +0f2
        jsr     0x283d8.l                       | +0f8
        bcs.w   .L0515b0                        | +0fe
.L05158e:
        move.w  0x22(a6),d0                     | +102
        addi.w  #0x10,d0                        | +106
        cmpi.w  #0x160,d0                       | +10a
        bcc.w   .L0515f4                        | +10e
        move.w  0x24(a6),d0                     | +112
        subi.w  #0xf0,d0                        | +116
        cmpi.w  #0x120,d0                       | +11a
        bcc.w   .L0515f4                        | +11e
        rts                                     | +122
.L0515b0:
        jsr     0x236e.l                        | +124
        move.w  0x2a(a6),d0                     | +12a
        bne.w   .L0515ce                        | +12e
        lea     0x296d26.l,a0                   | +132
        jsr     0x28cd4.l                       | +138
        bra.w   .L0515da                        | +13e
.L0515ce:
        lea     0x296d78.l,a0                   | +142
        jsr     0x28cd4.l                       | +148
.L0515da:
        clr.w   0x28(a6)                        | +14e
        clr.w   0x2a(a6)                        | +152
        lea     .L0515e8(pc),a1                 | +156
        move.l  a1,(a6)                         | +15a
.L0515e8:
        jsr     0x28d70.l                       | +15c
        bcs.w   .L0515f4                        | +162
        bra.b   .L05158e                        | +166
.L0515f4:
        jmp     0x518.l                         | +168

| ----------------------------------------------------------------------------
|  Rts_0515fa  @ $0515FA  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_0515fa, "ax", @progbits
        .global Rts_0515fa
Rts_0515fa:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  SlotPrioCheck_0515fc  @ $0515FC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.SlotPrioCheck_0515fc, "ax", @progbits
        .global SlotPrioCheck_0515fc
SlotPrioCheck_0515fc:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_051612                    | +00c

| ----------------------------------------------------------------------------
|  MemCard_OpenLoadDialog_051618  @ $051618  (50 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_OpenLoadDialog_051618, "ax", @progbits
        .global MemCard_OpenLoadDialog_051618
MemCard_OpenLoadDialog_051618:
        .dc.w   0x4d45                        | +000  (dato / opcode no decodificado)
        .dc.w   0x5441                        | +002  (dato / opcode no decodificado)
        .dc.w   0x4c20                        | +004  (dato / opcode no decodificado)
        .dc.w   0x534c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x5547                        | +008  (dato / opcode no decodificado)
        .dc.w   0x2020                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x2020                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2020                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x2020                        | +010  (dato / opcode no decodificado)
        .dc.w   0x2020                        | +012  (dato / opcode no decodificado)
        move.b  #0x1,0x106ed2.l                 | +014
        lea     0x98288.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        move.l  #0x5164a,0x98(a0)               | +028
        rts                                     | +030

| ----------------------------------------------------------------------------
|  MemCard_LoadScene_05164a  @ $05164A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_LoadScene_05164a, "ax", @progbits
        .global MemCard_LoadScene_05164a
MemCard_LoadScene_05164a:
        lea     0x2970f8.l,a0                   | +000
        jsr     0x43562.l                       | +006
        moveq   #0,d0                           | +00c
        moveq   #0,d1                           | +00e
        jmp     0x437da.l                       | +010

| ----------------------------------------------------------------------------
|  MemCard_DetectAndSave_051660  @ $051660  (46 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_DetectAndSave_051660, "ax", @progbits
        .global MemCard_DetectAndSave_051660
MemCard_DetectAndSave_051660:
        jsr     0x9826e.l                       | +000
        bcs.w   .L051674                        | +006
        clr.b   0x106ed2.l                      | +00a
        bra.w   JsrAbsRts_051694                | +010
.L051674:
        move.b  #0x1,0x106ed2.l                 | +014
        jsr     MemCard_LoadScene_05164a(pc)    | +01c
        move.b  #0xff,0x10e3b6.l                | +020
        lea     0x9840c.l,a1                    | +028

| ----------------------------------------------------------------------------
|  MemCard_CopySlotFlag_051696  @ $051696  (14 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_CopySlotFlag_051696, "ax", @progbits
        .global MemCard_CopySlotFlag_051696
MemCard_CopySlotFlag_051696:
        move.b  0x10e3b7.l,d0                   | +000
        move.b  d0,0x106ed0.l                   | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  MemCard_IsPending_0516a4  @ $0516A4  (10 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_IsPending_0516a4, "ax", @progbits
        .global MemCard_IsPending_0516a4
MemCard_IsPending_0516a4:
        tst.b   0x10e3b6.l                      | +000
        bne.w   SetC_0516b4                     | +006

| ----------------------------------------------------------------------------
|  Players_SnapshotMods_0516ba  @ $0516BA  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Players_SnapshotMods_0516ba, "ax", @progbits
        .global Players_SnapshotMods_0516ba
Players_SnapshotMods_0516ba:
        lea     0x10e3a2.l,a4                   | +000
        move.b  0x106ed0.l,0x15(a4)             | +006
        move.b  0x10fdb6.l,0x16(a4)             | +00e
        move.b  0x10fdb7.l,0x17(a4)             | +016
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  Counters_AddSaturate_0516da  @ $0516DA  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Counters_AddSaturate_0516da, "ax", @progbits
        .global Counters_AddSaturate_0516da
Counters_AddSaturate_0516da:
        movem.w d1-d2,-(a7)                     | +000
        add.b   0x10e3bc.l,d1                   | +004
        bcc.w   .L0516ec                        | +00a
        move.b  #0xff,d1                        | +00e
.L0516ec:
        move.b  d1,0x10e3bc.l                   | +012
        add.b   0x10e3bd.l,d2                   | +018
        bcc.w   .L051700                        | +01e
        move.b  #0xff,d2                        | +022
.L051700:
        move.b  d2,0x10e3bd.l                   | +026
        movem.w (a7)+,d1-d2                     | +02c
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Players_ActiveMask_05174c  @ $05174C  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Players_ActiveMask_05174c, "ax", @progbits
        .global Players_ActiveMask_05174c
Players_ActiveMask_05174c:
        lea     0x10e3a2.l,a0                   | +000
        clr.b   d0                              | +006
        clr.b   d1                              | +008
        clr.b   d2                              | +00a
        cmpi.b  #0x1,0x16(a0)                   | +00c
        bne.w   .L05176a                        | +012
        move.b  #0xff,d0                        | +016
        move.b  0x18(a0),d2                     | +01a
.L05176a:
        cmpi.b  #0x1,0x17(a0)                   | +01e
        bne.w   .L05177c                        | +024
        move.b  #0xff,d1                        | +028
        add.b   0x19(a0),d2                     | +02c
.L05177c:
        and.b   d0,d1                           | +030
        bne.w   ClearC_051788                   | +032

| ----------------------------------------------------------------------------
|  SlotPrioCheck_05178e  @ $05178E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.SlotPrioCheck_05178e, "ax", @progbits
        .global SlotPrioCheck_05178e
SlotPrioCheck_05178e:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0517a4                    | +00c

| ----------------------------------------------------------------------------
|  Player_SetIndexFromParent_0517aa  @ $0517AA  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SetIndexFromParent_0517aa, "ax", @progbits
        .global Player_SetIndexFromParent_0517aa
Player_SetIndexFromParent_0517aa:
        movea.l 0xc(a6),a0                      | +000
        cmpa.l  #0x100300,a0                    | +004
        bne.w   .L0517c6                        | +00a
        move.b  #0x0,0x68(a6)                   | +00e
        move.b  #0x1,0x6d(a6)                   | +014
        rts                                     | +01a
.L0517c6:
        cmpa.l  #0x1003a0,a0                    | +01c
        bne.w   .L0517de                        | +022
        move.b  #0x1,0x68(a6)                   | +026
        move.b  #0x2,0x6d(a6)                   | +02c
        rts                                     | +032
.L0517de:
        move.b  #0xff,0x6d(a6)                  | +034
        move.b  0x68(a6),d0                     | +03a
        cmpi.b  #0xff,d0                        | +03e
        beq.w   .L0517fc                        | +042
        nop                                     | +046
        nop                                     | +048
        cmpi.b  #0xff,d0                        | +04a
        nop                                     | +04e
        trap    #0xf                            | +050
.L0517fc:
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Clear8Bytes_05180c  @ $05180C  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Clear8Bytes_05180c, "ax", @progbits
        .global Clear8Bytes_05180c
Clear8Bytes_05180c:
        clr.l   (a1)                            | +000
        clr.l   0x4(a1)                         | +002
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Nibbles_Unpack4Store_051814  @ $051814  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Nibbles_Unpack4Store_051814, "ax", @progbits
        .global Nibbles_Unpack4Store_051814
Nibbles_Unpack4Store_051814:
        move.l  a1,-(a7)                        | +000
        movea.l a1,a0                           | +002
        lea     0x1081b6.l,a1                   | +004
        move.l  d0,(a1)                         | +00a
        bsr.w   Nibbles_Unpack4_051828          | +00c
        movea.l (a7)+,a1                        | +010
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Nibbles_Unpack4_051828  @ $051828  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Nibbles_Unpack4_051828, "ax", @progbits
        .global Nibbles_Unpack4_051828
Nibbles_Unpack4_051828:
        move.b  (a1)+,d0                        | +000
        move.b  d0,d1                           | +002
        lsr.b   #0x4,d1                         | +004
        andi.b  #0xf,d0                         | +006
        move.b  d1,(a0)+                        | +00a
        move.b  d0,(a0)+                        | +00c
        move.b  (a1)+,d0                        | +00e
        move.b  d0,d1                           | +010
        lsr.b   #0x4,d1                         | +012
        andi.b  #0xf,d0                         | +014
        move.b  d1,(a0)+                        | +018
        move.b  d0,(a0)+                        | +01a
        move.b  (a1)+,d0                        | +01c
        move.b  d0,d1                           | +01e
        lsr.b   #0x4,d1                         | +020
        andi.b  #0xf,d0                         | +022
        move.b  d1,(a0)+                        | +026
        move.b  d0,(a0)+                        | +028
        move.b  (a1)+,d0                        | +02a
        move.b  d0,d1                           | +02c
        lsr.b   #0x4,d1                         | +02e
        andi.b  #0xf,d0                         | +030
        move.b  d1,(a0)+                        | +034
        move.b  d0,(a0)+                        | +036
        rts                                     | +038

| ----------------------------------------------------------------------------
|  Nibbles_Pack8_051862  @ $051862  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Nibbles_Pack8_051862, "ax", @progbits
        .global Nibbles_Pack8_051862
Nibbles_Pack8_051862:
        move.b  (a0)+,d0                        | +000
        move.b  (a0)+,d1                        | +002
        lsl.b   #0x4,d0                         | +004
        or.b    d1,d0                           | +006
        move.b  d0,(a1)+                        | +008
        move.b  (a0)+,d0                        | +00a
        move.b  (a0)+,d1                        | +00c
        lsl.b   #0x4,d0                         | +00e
        or.b    d1,d0                           | +010
        move.b  d0,(a1)+                        | +012
        move.b  (a0)+,d0                        | +014
        move.b  (a0)+,d1                        | +016
        lsl.b   #0x4,d0                         | +018
        or.b    d1,d0                           | +01a
        move.b  d0,(a1)+                        | +01c
        move.b  (a0)+,d0                        | +01e
        move.b  (a0)+,d1                        | +020
        lsl.b   #0x4,d0                         | +022
        or.b    d1,d0                           | +024
        move.b  d0,(a1)+                        | +026
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerState_FlagTable_05188c  @ $05188C  (136 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerState_FlagTable_05188c, "ax", @progbits
        .global PlayerState_FlagTable_05188c
PlayerState_FlagTable_05188c:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0500                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0050                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0050                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0050                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0300                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0050                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +064  (dato / opcode no decodificado)
        .dc.w   0x1000                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +070  (dato / opcode no decodificado)
        .dc.w   0x1000                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +086  (dato / opcode no decodificado)
