| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $05AA96..$05CA2A  (8,084 B, 9 entradas, 1 huecos)
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
|  SpriteDispatchJT_05AA96  @ $05AA96  (1820 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteDispatchJT_05AA96, "ax", @progbits
        .global SpriteDispatchJT_05AA96
SpriteDispatchJT_05AA96:
        bra.w   .L05aab6                        | +000
        bra.w   .L05ab34                        | +004
        bra.w   .L05abb8                        | +008
        bra.w   .L05ac44                        | +00c
        bra.w   .L05acd6                        | +010
        bra.w   .L05ae00                        | +014
        bra.w   .L05af30                        | +018
        bra.w   .L05b06e                        | +01c
.L05aab6:
        cmpa.l  a2,a3                           | +020
        bcc.w   .L05ab32                        | +022
        move.b  (a0)+,-(a7)                     | +026
        move.b  (a0)+,d2                        | +028
        eor.b   d5,d2                           | +02a
        move.w  #0xfff,d4                       | +02c
.L05aac6:
        moveq   #0,d5                           | +030
        move.l  (a0)+,d1                        | +032
        moveq   #0,d5                           | +034
        move.b  (a0)+,d5                        | +036
        swap    d5                              | +038
        move.b  (a0)+,d5                        | +03a
        move.w  d5,d6                           | +03c
        swap    d6                              | +03e
        move.w  d5,d6                           | +040
        add.w   d0,d1                           | +042
        swap    d1                              | +044
        swap    d0                              | +046
        add.w   d0,d1                           | +048
        swap    d0                              | +04a
        andi.w  #0x1ff,d1                       | +04c
        lsl.l   #0x7,d1                         | +050
        or.l    d6,d1                           | +052
        clr.w   d5                              | +054
        swap    d5                              | +056
        subq.b  #0x1,d5                         | +058
        add.w   d6,d6                           | +05a
        move.w  #0x800,d3                       | +05c
.L05aaf6:
        cmpi.w  #0xf800,d1                      | +060
        bcc.w   .L05ab06                        | +064
        cmpi.w  #0xa000,d1                      | +068
        bcc.w   .L05ab14                        | +06c
.L05ab06:
        move.l  a0,(a3)+                        | +070
        move.w  d2,(a3)+                        | +072
        move.w  d4,(a3)+                        | +074
        move.l  d1,(a3)+                        | +076
        cmpa.l  a2,a3                           | +078
        bcc.w   .L05ab20                        | +07a
.L05ab14:
        add.w   d3,d1                           | +07e
        adda.w  d6,a0                           | +080
        dbra    d5,.L05aaf6                     | +082
        subq.b  #0x1,(a7)                       | +086
        bne.b   .L05aac6                        | +088
.L05ab20:
        addq.w  #0x2,a7                         | +08a
        lea     0x4258(a5),a2                   | +08c
        cmpa.l  a2,a3                           | +090
        beq.w   .L05ab32                        | +092
        ori.b   #0x80,-0x6(a3)                  | +096
.L05ab32:
        rts                                     | +09c
.L05ab34:
        cmpa.l  a2,a3                           | +09e
        bcc.w   .L05abb6                        | +0a0
        move.b  (a0)+,-(a7)                     | +0a4
        move.b  (a0)+,d2                        | +0a6
        eor.b   d5,d2                           | +0a8
        move.w  #0xfff,d4                       | +0aa
.L05ab44:
        moveq   #0,d5                           | +0ae
        move.l  (a0)+,d1                        | +0b0
        moveq   #0,d5                           | +0b2
        move.b  (a0)+,d5                        | +0b4
        swap    d5                              | +0b6
        move.b  (a0)+,d5                        | +0b8
        swap    d1                              | +0ba
        not.w   d1                              | +0bc
        swap    d1                              | +0be
        move.w  d5,d6                           | +0c0
        swap    d6                              | +0c2
        move.w  d5,d6                           | +0c4
        add.w   d0,d1                           | +0c6
        swap    d1                              | +0c8
        swap    d0                              | +0ca
        add.w   d0,d1                           | +0cc
        swap    d0                              | +0ce
        andi.w  #0x1ff,d1                       | +0d0
        lsl.l   #0x7,d1                         | +0d4
        or.l    d6,d1                           | +0d6
        clr.w   d5                              | +0d8
        swap    d5                              | +0da
        subq.b  #0x1,d5                         | +0dc
        add.w   d6,d6                           | +0de
        move.w  #0x800,d3                       | +0e0
.L05ab7a:
        sub.w   d3,d1                           | +0e4
        cmpi.w  #0xf800,d1                      | +0e6
        bcc.w   .L05ab8c                        | +0ea
        cmpi.w  #0xa000,d1                      | +0ee
        bcc.w   .L05ab9a                        | +0f2
.L05ab8c:
        move.l  a0,(a3)+                        | +0f6
        move.w  d2,(a3)+                        | +0f8
        move.w  d4,(a3)+                        | +0fa
        move.l  d1,(a3)+                        | +0fc
        cmpa.l  a2,a3                           | +0fe
        bcc.w   .L05aba4                        | +100
.L05ab9a:
        adda.w  d6,a0                           | +104
        dbra    d5,.L05ab7a                     | +106
        subq.b  #0x1,(a7)                       | +10a
        bne.b   .L05ab44                        | +10c
.L05aba4:
        addq.w  #0x2,a7                         | +10e
        lea     0x4258(a5),a2                   | +110
        cmpa.l  a2,a3                           | +114
        beq.w   .L05abb6                        | +116
        ori.b   #0x80,-0x6(a3)                  | +11a
.L05abb6:
        rts                                     | +120
.L05abb8:
        cmpa.l  a2,a3                           | +122
        bcc.w   .L05ac42                        | +124
        move.b  (a0)+,-(a7)                     | +128
        move.b  (a0)+,d2                        | +12a
        eor.b   d5,d2                           | +12c
        move.w  #0x1fff,d4                      | +12e
.L05abc8:
        moveq   #0,d5                           | +132
        move.l  (a0)+,d1                        | +134
        moveq   #0,d5                           | +136
        move.b  (a0)+,d5                        | +138
        swap    d5                              | +13a
        move.b  (a0)+,d5                        | +13c
        neg.w   d1                              | +13e
        move.w  d5,d6                           | +140
        lsl.w   #0x4,d6                         | +142
        subq.w  #0x1,d6                         | +144
        add.w   d6,d1                           | +146
        move.w  d5,d6                           | +148
        swap    d6                              | +14a
        move.w  d5,d6                           | +14c
        add.w   d0,d1                           | +14e
        swap    d1                              | +150
        swap    d0                              | +152
        add.w   d0,d1                           | +154
        swap    d0                              | +156
        andi.w  #0x1ff,d1                       | +158
        lsl.l   #0x7,d1                         | +15c
        or.l    d6,d1                           | +15e
        clr.w   d5                              | +160
        swap    d5                              | +162
        subq.b  #0x1,d5                         | +164
        add.w   d6,d6                           | +166
        adda.w  d6,a0                           | +168
        move.w  #0x800,d3                       | +16a
.L05ac04:
        cmpi.w  #0xf800,d1                      | +16e
        bcc.w   .L05ac14                        | +172
        cmpi.w  #0xa000,d1                      | +176
        bcc.w   .L05ac22                        | +17a
.L05ac14:
        move.l  a0,(a3)+                        | +17e
        move.w  d2,(a3)+                        | +180
        move.w  d4,(a3)+                        | +182
        move.l  d1,(a3)+                        | +184
        cmpa.l  a2,a3                           | +186
        bcc.w   .L05ac30                        | +188
.L05ac22:
        add.w   d3,d1                           | +18c
        adda.w  d6,a0                           | +18e
        dbra    d5,.L05ac04                     | +190
        suba.w  d6,a0                           | +194
        subq.b  #0x1,(a7)                       | +196
        bne.b   .L05abc8                        | +198
.L05ac30:
        addq.w  #0x2,a7                         | +19a
        lea     0x4258(a5),a2                   | +19c
        cmpa.l  a2,a3                           | +1a0
        beq.w   .L05ac42                        | +1a2
        ori.b   #0x80,-0x6(a3)                  | +1a6
.L05ac42:
        rts                                     | +1ac
.L05ac44:
        cmpa.l  a2,a3                           | +1ae
        bcc.w   .L05acd4                        | +1b0
        move.b  (a0)+,-(a7)                     | +1b4
        move.b  (a0)+,d2                        | +1b6
        eor.b   d5,d2                           | +1b8
        move.w  #0x1fff,d4                      | +1ba
.L05ac54:
        moveq   #0,d5                           | +1be
        move.l  (a0)+,d1                        | +1c0
        moveq   #0,d5                           | +1c2
        move.b  (a0)+,d5                        | +1c4
        swap    d5                              | +1c6
        move.b  (a0)+,d5                        | +1c8
        swap    d1                              | +1ca
        not.w   d1                              | +1cc
        swap    d1                              | +1ce
        neg.w   d1                              | +1d0
        move.w  d5,d6                           | +1d2
        lsl.w   #0x4,d6                         | +1d4
        subq.w  #0x1,d6                         | +1d6
        add.w   d6,d1                           | +1d8
        move.w  d5,d6                           | +1da
        swap    d6                              | +1dc
        move.w  d5,d6                           | +1de
        add.w   d0,d1                           | +1e0
        swap    d1                              | +1e2
        swap    d0                              | +1e4
        add.w   d0,d1                           | +1e6
        swap    d0                              | +1e8
        andi.w  #0x1ff,d1                       | +1ea
        lsl.l   #0x7,d1                         | +1ee
        or.l    d6,d1                           | +1f0
        clr.w   d5                              | +1f2
        swap    d5                              | +1f4
        subq.b  #0x1,d5                         | +1f6
        add.w   d6,d6                           | +1f8
        adda.w  d6,a0                           | +1fa
        move.w  #0x800,d3                       | +1fc
.L05ac96:
        sub.w   d3,d1                           | +200
        cmpi.w  #0xf800,d1                      | +202
        bcc.w   .L05aca8                        | +206
        cmpi.w  #0xa000,d1                      | +20a
        bcc.w   .L05acb6                        | +20e
.L05aca8:
        move.l  a0,(a3)+                        | +212
        move.w  d2,(a3)+                        | +214
        move.w  d4,(a3)+                        | +216
        move.l  d1,(a3)+                        | +218
        cmpa.l  a2,a3                           | +21a
        bcc.w   .L05acc2                        | +21c
.L05acb6:
        adda.w  d6,a0                           | +220
        dbra    d5,.L05ac96                     | +222
        suba.w  d6,a0                           | +226
        subq.b  #0x1,(a7)                       | +228
        bne.b   .L05ac54                        | +22a
.L05acc2:
        addq.w  #0x2,a7                         | +22c
        lea     0x4258(a5),a2                   | +22e
        cmpa.l  a2,a3                           | +232
        beq.w   .L05acd4                        | +234
        ori.b   #0x80,-0x6(a3)                  | +238
.L05acd4:
        rts                                     | +23e
.L05acd6:
        cmpa.l  a2,a3                           | +240
        bcc.w   .L05adfe                        | +242
        move.b  (a0)+,-(a7)                     | +246
        move.b  (a0)+,d2                        | +248
        eor.b   d5,d2                           | +24a
        andi.w  #0xff,d4                        | +24c
        move.w  d4,d7                           | +250
        addq.w  #0x1,d7                         | +252
        swap    d7                              | +254
        move.w  d3,d7                           | +256
        andi.w  #0xff,d7                        | +258
        addq.w  #0x1,d7                         | +25c
        lsl.w   #0x4,d4                         | +25e
        eori.b  #0xf0,d4                        | +260
        move.b  d4,0x616e(a5)                   | +264
        move.b  d3,d4                           | +268
        andi.b  #0xf0,d4                        | +26a
        move.w  d4,d6                           | +26e
        subi.w  #0x100,d4                       | +270
        swap    d4                              | +274
        move.w  d6,d4                           | +276
        andi.w  #0xf,d3                         | +278
        swap    d2                              | +27c
        move.w  d3,d2                           | +27e
        swap    d2                              | +280
.L05ad18:
        moveq   #0,d5                           | +282
        move.l  (a0)+,d1                        | +284
        moveq   #0,d5                           | +286
        move.b  (a0)+,d5                        | +288
        swap    d5                              | +28a
        move.b  (a0)+,d5                        | +28c
        moveq   #0,d3                           | +28e
        move.l  d1,d6                           | +290
        muls.w  d7,d6                           | +292
        spl.b   d3                              | +294
        add.l   d3,d6                           | +296
        asr.l   #0x8,d6                         | +298
        swap    d7                              | +29a
        swap    d1                              | +29c
        muls.w  d7,d1                           | +29e
        move.b  d1,0x616d(a5)                   | +2a0
        lsl.l   #0x8,d1                         | +2a4
        move.w  d6,d1                           | +2a6
        swap    d7                              | +2a8
        move.w  d5,d6                           | +2aa
        cmpi.b  #0x11,d6                        | +2ac
        bcc.w   .L05ad60                        | +2b0
        subq.b  #0x1,d6                         | +2b4
        lsl.b   #0x4,d6                         | +2b6
        swap    d2                              | +2b8
        or.b    d2,d6                           | +2ba
        lea     Sub_0005A8BA(pc),a4             | +2bc
        move.b  (a4,d6.w),d6                    | +2c0
        swap    d2                              | +2c4
        bra.w   .L05ad66                        | +2c6
.L05ad60:
        swap    d2                              | +2ca
        move.w  d2,d6                           | +2cc
        swap    d2                              | +2ce
.L05ad66:
        andi.b  #0xf0,d4                        | +2d0
        or.b    d4,d6                           | +2d4
        move.b  d6,d4                           | +2d6
        swap    d4                              | +2d8
        move.b  d6,d4                           | +2da
        swap    d4                              | +2dc
        move.w  d7,d6                           | +2de
        mulu.w  d5,d6                           | +2e0
        addi.w  #0xff,d6                        | +2e2
        lsr.w   #0x8,d6                         | +2e6
        swap    d6                              | +2e8
        move.w  d5,d6                           | +2ea
        add.w   d0,d1                           | +2ec
        swap    d1                              | +2ee
        swap    d0                              | +2f0
        add.w   d0,d1                           | +2f2
        swap    d0                              | +2f4
        andi.w  #0x1ff,d1                       | +2f6
        lsl.l   #0x7,d1                         | +2fa
        or.l    d6,d1                           | +2fc
        clr.w   d5                              | +2fe
        swap    d5                              | +300
        subq.b  #0x1,d5                         | +302
        add.w   d6,d6                           | +304
        movea.l d4,a4                           | +306
.L05ad9e:
        move.b  0x616e(a5),d3                   | +308
        sub.b   d3,0x616d(a5)                   | +30c
        bcc.w   .L05adb2                        | +310
        swap    d4                              | +314
        tst.w   d4                              | +316
        bmi.w   .L05adde                        | +318
.L05adb2:
        move.w  d4,d3                           | +31c
        andi.w  #0xf00,d3                       | +31e
        addi.w  #0x100,d3                       | +322
        lsr.w   #0x1,d3                         | +326
        cmpi.w  #0xf800,d1                      | +328
        bcc.w   .L05adce                        | +32c
        cmpi.w  #0xa000,d1                      | +330
        bcc.w   .L05addc                        | +334
.L05adce:
        move.l  a0,(a3)+                        | +338
        move.w  d2,(a3)+                        | +33a
        move.w  d4,(a3)+                        | +33c
        move.l  d1,(a3)+                        | +33e
        cmpa.l  a2,a3                           | +340
        bcc.w   .L05adec                        | +342
.L05addc:
        add.w   d3,d1                           | +346
.L05adde:
        move.l  a4,d4                           | +348
        adda.w  d6,a0                           | +34a
        dbra    d5,.L05ad9e                     | +34c
        subq.b  #0x1,(a7)                       | +350
        bne.w   .L05ad18                        | +352
.L05adec:
        addq.w  #0x2,a7                         | +356
        lea     0x4258(a5),a2                   | +358
        cmpa.l  a2,a3                           | +35c
        beq.w   .L05adfe                        | +35e
        ori.b   #0x80,-0x6(a3)                  | +362
.L05adfe:
        rts                                     | +368
.L05ae00:
        cmpa.l  a2,a3                           | +36a
        bcc.w   .L05af2e                        | +36c
        move.b  (a0)+,-(a7)                     | +370
        move.b  (a0)+,d2                        | +372
        eor.b   d5,d2                           | +374
        andi.w  #0xff,d4                        | +376
        move.w  d4,d7                           | +37a
        addq.w  #0x1,d7                         | +37c
        swap    d7                              | +37e
        move.w  d3,d7                           | +380
        andi.w  #0xff,d7                        | +382
        addq.w  #0x1,d7                         | +386
        lsl.w   #0x4,d4                         | +388
        eori.b  #0xf0,d4                        | +38a
        move.b  d4,0x616e(a5)                   | +38e
        move.b  d3,d4                           | +392
        andi.b  #0xf0,d4                        | +394
        move.w  d4,d6                           | +398
        subi.w  #0x100,d4                       | +39a
        swap    d4                              | +39e
        move.w  d6,d4                           | +3a0
        andi.w  #0xf,d3                         | +3a2
        swap    d2                              | +3a6
        move.w  d3,d2                           | +3a8
        swap    d2                              | +3aa
.L05ae42:
        moveq   #0,d5                           | +3ac
        move.l  (a0)+,d1                        | +3ae
        moveq   #0,d5                           | +3b0
        move.b  (a0)+,d5                        | +3b2
        swap    d5                              | +3b4
        move.b  (a0)+,d5                        | +3b6
        swap    d1                              | +3b8
        not.w   d1                              | +3ba
        swap    d1                              | +3bc
        moveq   #0,d3                           | +3be
        move.l  d1,d6                           | +3c0
        muls.w  d7,d6                           | +3c2
        spl.b   d3                              | +3c4
        add.l   d3,d6                           | +3c6
        asr.l   #0x8,d6                         | +3c8
        swap    d7                              | +3ca
        swap    d1                              | +3cc
        muls.w  d7,d1                           | +3ce
        move.b  d1,0x616d(a5)                   | +3d0
        lsl.l   #0x8,d1                         | +3d4
        move.w  d6,d1                           | +3d6
        swap    d7                              | +3d8
        move.w  d5,d6                           | +3da
        cmpi.b  #0x11,d6                        | +3dc
        bcc.w   .L05ae90                        | +3e0
        subq.b  #0x1,d6                         | +3e4
        lsl.b   #0x4,d6                         | +3e6
        swap    d2                              | +3e8
        or.b    d2,d6                           | +3ea
        lea     Sub_0005A8BA(pc),a4             | +3ec
        move.b  (a4,d6.w),d6                    | +3f0
        swap    d2                              | +3f4
        bra.w   .L05ae96                        | +3f6
.L05ae90:
        swap    d2                              | +3fa
        move.w  d2,d6                           | +3fc
        swap    d2                              | +3fe
.L05ae96:
        andi.b  #0xf0,d4                        | +400
        or.b    d4,d6                           | +404
        move.b  d6,d4                           | +406
        swap    d4                              | +408
        move.b  d6,d4                           | +40a
        swap    d4                              | +40c
        move.w  d7,d6                           | +40e
        mulu.w  d5,d6                           | +410
        addi.w  #0xff,d6                        | +412
        lsr.w   #0x8,d6                         | +416
        swap    d6                              | +418
        move.w  d5,d6                           | +41a
        add.w   d0,d1                           | +41c
        swap    d1                              | +41e
        swap    d0                              | +420
        add.w   d0,d1                           | +422
        swap    d0                              | +424
        andi.w  #0x1ff,d1                       | +426
        lsl.l   #0x7,d1                         | +42a
        or.l    d6,d1                           | +42c
        clr.w   d5                              | +42e
        swap    d5                              | +430
        subq.b  #0x1,d5                         | +432
        add.w   d6,d6                           | +434
        movea.l d4,a4                           | +436
.L05aece:
        move.b  0x616e(a5),d3                   | +438
        add.b   d3,0x616d(a5)                   | +43c
        bcc.w   .L05aee2                        | +440
        swap    d4                              | +444
        tst.w   d4                              | +446
        bmi.w   .L05af0e                        | +448
.L05aee2:
        move.w  d4,d3                           | +44c
        andi.w  #0xf00,d3                       | +44e
        addi.w  #0x100,d3                       | +452
        lsr.w   #0x1,d3                         | +456
        sub.w   d3,d1                           | +458
        cmpi.w  #0xf800,d1                      | +45a
        bcc.w   .L05af00                        | +45e
        cmpi.w  #0xa000,d1                      | +462
        bcc.w   .L05af0e                        | +466
.L05af00:
        move.l  a0,(a3)+                        | +46a
        move.w  d2,(a3)+                        | +46c
        move.w  d4,(a3)+                        | +46e
        move.l  d1,(a3)+                        | +470
        cmpa.l  a2,a3                           | +472
        bcc.w   .L05af1c                        | +474
.L05af0e:
        move.l  a4,d4                           | +478
        adda.w  d6,a0                           | +47a
        dbra    d5,.L05aece                     | +47c
        subq.b  #0x1,(a7)                       | +480
        bne.w   .L05ae42                        | +482
.L05af1c:
        addq.w  #0x2,a7                         | +486
        lea     0x4258(a5),a2                   | +488
        cmpa.l  a2,a3                           | +48c
        beq.w   .L05af2e                        | +48e
        ori.b   #0x80,-0x6(a3)                  | +492
.L05af2e:
        rts                                     | +498
.L05af30:
        cmpa.l  a2,a3                           | +49a
        bcc.w   .L05b06c                        | +49c
        move.b  (a0)+,-(a7)                     | +4a0
        move.b  (a0)+,d2                        | +4a2
        eor.b   d5,d2                           | +4a4
        andi.w  #0xff,d4                        | +4a6
        move.w  d4,d7                           | +4aa
        addq.w  #0x1,d7                         | +4ac
        swap    d7                              | +4ae
        move.w  d3,d7                           | +4b0
        andi.w  #0xff,d7                        | +4b2
        addq.w  #0x1,d7                         | +4b6
        lsl.w   #0x4,d4                         | +4b8
        eori.b  #0xf0,d4                        | +4ba
        move.b  d4,0x616e(a5)                   | +4be
        move.b  d3,d4                           | +4c2
        andi.b  #0xf0,d4                        | +4c4
        move.w  d4,d6                           | +4c8
        subi.w  #0x100,d4                       | +4ca
        swap    d4                              | +4ce
        move.w  d6,d4                           | +4d0
        ori.l   #0x10001000,d4                  | +4d2
        andi.w  #0xf,d3                         | +4d8
        swap    d2                              | +4dc
        move.w  d3,d2                           | +4de
        swap    d2                              | +4e0
.L05af78:
        moveq   #0,d5                           | +4e2
        move.l  (a0)+,d1                        | +4e4
        moveq   #0,d5                           | +4e6
        move.b  (a0)+,d5                        | +4e8
        swap    d5                              | +4ea
        move.b  (a0)+,d5                        | +4ec
        neg.w   d1                              | +4ee
        move.w  d5,d6                           | +4f0
        lsl.w   #0x4,d6                         | +4f2
        subq.w  #0x1,d6                         | +4f4
        add.w   d6,d1                           | +4f6
        moveq   #0,d3                           | +4f8
        move.l  d1,d6                           | +4fa
        muls.w  d7,d6                           | +4fc
        spl.b   d3                              | +4fe
        add.l   d3,d6                           | +500
        asr.l   #0x8,d6                         | +502
        swap    d7                              | +504
        swap    d1                              | +506
        muls.w  d7,d1                           | +508
        move.b  d1,0x616d(a5)                   | +50a
        lsl.l   #0x8,d1                         | +50e
        move.w  d6,d1                           | +510
        swap    d7                              | +512
        move.w  d5,d6                           | +514
        cmpi.b  #0x11,d6                        | +516
        bcc.w   .L05afca                        | +51a
        subq.b  #0x1,d6                         | +51e
        lsl.b   #0x4,d6                         | +520
        swap    d2                              | +522
        or.b    d2,d6                           | +524
        lea     Sub_0005A8BA(pc),a4             | +526
        move.b  (a4,d6.w),d6                    | +52a
        swap    d2                              | +52e
        bra.w   .L05afd0                        | +530
.L05afca:
        swap    d2                              | +534
        move.w  d2,d6                           | +536
        swap    d2                              | +538
.L05afd0:
        andi.b  #0xf0,d4                        | +53a
        or.b    d4,d6                           | +53e
        move.b  d6,d4                           | +540
        swap    d4                              | +542
        move.b  d6,d4                           | +544
        swap    d4                              | +546
        move.w  d7,d6                           | +548
        mulu.w  d5,d6                           | +54a
        addi.w  #0xff,d6                        | +54c
        lsr.w   #0x8,d6                         | +550
        swap    d6                              | +552
        move.w  d5,d6                           | +554
        add.w   d0,d1                           | +556
        swap    d1                              | +558
        swap    d0                              | +55a
        add.w   d0,d1                           | +55c
        swap    d0                              | +55e
        andi.w  #0x1ff,d1                       | +560
        lsl.l   #0x7,d1                         | +564
        or.l    d6,d1                           | +566
        clr.w   d5                              | +568
        swap    d5                              | +56a
        subq.b  #0x1,d5                         | +56c
        add.w   d6,d6                           | +56e
        adda.w  d6,a0                           | +570
        movea.l d4,a4                           | +572
.L05b00a:
        move.b  0x616e(a5),d3                   | +574
        sub.b   d3,0x616d(a5)                   | +578
        bcc.w   .L05b01e                        | +57c
        swap    d4                              | +580
        tst.w   d4                              | +582
        bmi.w   .L05b04a                        | +584
.L05b01e:
        move.w  d4,d3                           | +588
        andi.w  #0xf00,d3                       | +58a
        addi.w  #0x100,d3                       | +58e
        lsr.w   #0x1,d3                         | +592
        cmpi.w  #0xf800,d1                      | +594
        bcc.w   .L05b03a                        | +598
        cmpi.w  #0xa000,d1                      | +59c
        bcc.w   .L05b048                        | +5a0
.L05b03a:
        move.l  a0,(a3)+                        | +5a4
        move.w  d2,(a3)+                        | +5a6
        move.w  d4,(a3)+                        | +5a8
        move.l  d1,(a3)+                        | +5aa
        cmpa.l  a2,a3                           | +5ac
        bcc.w   .L05b05a                        | +5ae
.L05b048:
        add.w   d3,d1                           | +5b2
.L05b04a:
        move.l  a4,d4                           | +5b4
        adda.w  d6,a0                           | +5b6
        dbra    d5,.L05b00a                     | +5b8
        suba.w  d6,a0                           | +5bc
        subq.b  #0x1,(a7)                       | +5be
        bne.w   .L05af78                        | +5c0
.L05b05a:
        addq.w  #0x2,a7                         | +5c4
        lea     0x4258(a5),a2                   | +5c6
        cmpa.l  a2,a3                           | +5ca
        beq.w   .L05b06c                        | +5cc
        ori.b   #0x80,-0x6(a3)                  | +5d0
.L05b06c:
        rts                                     | +5d6
.L05b06e:
        cmpa.l  a2,a3                           | +5d8
        bcc.w   .L05b1b0                        | +5da
        move.b  (a0)+,-(a7)                     | +5de
        move.b  (a0)+,d2                        | +5e0
        eor.b   d5,d2                           | +5e2
        andi.w  #0xff,d4                        | +5e4
        move.w  d4,d7                           | +5e8
        addq.w  #0x1,d7                         | +5ea
        swap    d7                              | +5ec
        move.w  d3,d7                           | +5ee
        andi.w  #0xff,d7                        | +5f0
        addq.w  #0x1,d7                         | +5f4
        lsl.w   #0x4,d4                         | +5f6
        eori.b  #0xf0,d4                        | +5f8
        move.b  d4,0x616e(a5)                   | +5fc
        move.b  d3,d4                           | +600
        andi.b  #0xf0,d4                        | +602
        move.w  d4,d6                           | +606
        subi.w  #0x100,d4                       | +608
        swap    d4                              | +60c
        move.w  d6,d4                           | +60e
        ori.l   #0x10001000,d4                  | +610
        andi.w  #0xf,d3                         | +616
        swap    d2                              | +61a
        move.w  d3,d2                           | +61c
        swap    d2                              | +61e
.L05b0b6:
        moveq   #0,d5                           | +620
        move.l  (a0)+,d1                        | +622
        moveq   #0,d5                           | +624
        move.b  (a0)+,d5                        | +626
        swap    d5                              | +628
        move.b  (a0)+,d5                        | +62a
        swap    d1                              | +62c
        not.w   d1                              | +62e
        swap    d1                              | +630
        neg.w   d1                              | +632
        move.w  d5,d6                           | +634
        lsl.w   #0x4,d6                         | +636
        subq.w  #0x1,d6                         | +638
        add.w   d6,d1                           | +63a
        moveq   #0,d3                           | +63c
        move.l  d1,d6                           | +63e
        muls.w  d7,d6                           | +640
        spl.b   d3                              | +642
        add.l   d3,d6                           | +644
        asr.l   #0x8,d6                         | +646
        swap    d7                              | +648
        swap    d1                              | +64a
        muls.w  d7,d1                           | +64c
        move.b  d1,0x616d(a5)                   | +64e
        lsl.l   #0x8,d1                         | +652
        move.w  d6,d1                           | +654
        swap    d7                              | +656
        move.w  d5,d6                           | +658
        cmpi.b  #0x11,d6                        | +65a
        bcc.w   .L05b10e                        | +65e
        subq.b  #0x1,d6                         | +662
        lsl.b   #0x4,d6                         | +664
        swap    d2                              | +666
        or.b    d2,d6                           | +668
        lea     Sub_0005A8BA(pc),a4             | +66a
        move.b  (a4,d6.w),d6                    | +66e
        swap    d2                              | +672
        bra.w   .L05b114                        | +674
.L05b10e:
        swap    d2                              | +678
        move.w  d2,d6                           | +67a
        swap    d2                              | +67c
.L05b114:
        andi.b  #0xf0,d4                        | +67e
        or.b    d4,d6                           | +682
        move.b  d6,d4                           | +684
        swap    d4                              | +686
        move.b  d6,d4                           | +688
        swap    d4                              | +68a
        move.w  d7,d6                           | +68c
        mulu.w  d5,d6                           | +68e
        addi.w  #0xff,d6                        | +690
        lsr.w   #0x8,d6                         | +694
        swap    d6                              | +696
        move.w  d5,d6                           | +698
        add.w   d0,d1                           | +69a
        swap    d1                              | +69c
        swap    d0                              | +69e
        add.w   d0,d1                           | +6a0
        swap    d0                              | +6a2
        andi.w  #0x1ff,d1                       | +6a4
        lsl.l   #0x7,d1                         | +6a8
        or.l    d6,d1                           | +6aa
        clr.w   d5                              | +6ac
        swap    d5                              | +6ae
        subq.b  #0x1,d5                         | +6b0
        add.w   d6,d6                           | +6b2
        adda.w  d6,a0                           | +6b4
        movea.l d4,a4                           | +6b6
.L05b14e:
        move.b  0x616e(a5),d3                   | +6b8
        add.b   d3,0x616d(a5)                   | +6bc
        bcc.w   .L05b162                        | +6c0
        swap    d4                              | +6c4
        tst.w   d4                              | +6c6
        bmi.w   .L05b18e                        | +6c8
.L05b162:
        move.w  d4,d3                           | +6cc
        andi.w  #0xf00,d3                       | +6ce
        addi.w  #0x100,d3                       | +6d2
        lsr.w   #0x1,d3                         | +6d6
        sub.w   d3,d1                           | +6d8
        cmpi.w  #0xf800,d1                      | +6da
        bcc.w   .L05b180                        | +6de
        cmpi.w  #0xa000,d1                      | +6e2
        bcc.w   .L05b18e                        | +6e6
.L05b180:
        move.l  a0,(a3)+                        | +6ea
        move.w  d2,(a3)+                        | +6ec
        move.w  d4,(a3)+                        | +6ee
        move.l  d1,(a3)+                        | +6f0
        cmpa.l  a2,a3                           | +6f2
        bcc.w   .L05b19e                        | +6f4
.L05b18e:
        move.l  a4,d4                           | +6f8
        adda.w  d6,a0                           | +6fa
        dbra    d5,.L05b14e                     | +6fc
        suba.w  d6,a0                           | +700
        subq.b  #0x1,(a7)                       | +702
        bne.w   .L05b0b6                        | +704
.L05b19e:
        addq.w  #0x2,a7                         | +708
        lea     0x4258(a5),a2                   | +70a
        cmpa.l  a2,a3                           | +70e
        beq.w   .L05b1b0                        | +710
        ori.b   #0x80,-0x6(a3)                  | +714
.L05b1b0:
        rts                                     | +71a

| ----------------------------------------------------------------------------
|  Sprite_DispatchSplashHook_05b1b2  @ $05B1B2  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Sprite_DispatchSplashHook_05b1b2, "ax", @progbits
        .global Sprite_DispatchSplashHook_05b1b2
Sprite_DispatchSplashHook_05b1b2:
        movem.l d0-d5/a0-a1,-(a7)               | +000
        cmpi.b  #0x0,0x106ece.l                 | +004
        bne.w   .L05b20c                        | +00c
        move.l  0x106f50.l,d6                   | +010
        swap    d6                              | +016
        cmpi.w  #0x780,d6                       | +018
        bgt.w   .L05b20c                        | +01c
        move.w  0x22(a6),d0                     | +020
        move.w  0x24(a6),d1                     | +024
        addi.w  #0x8,d1                         | +028
        jsr     0x43f02.l                       | +02c
        move.b  d0,d6                           | +032
        cmpi.b  #0x3a,d6                        | +034
        bne.w   .L05b20c                        | +038
        movem.l (a7)+,d0-d5/a0-a1               | +03c
        movem.l d0-d5/a0-a1,-(a7)               | +040
        eori.b  #0x2,d5                         | +044
        move.b  #0x7f,d3                        | +048
        jsr     Sprite_SplashScreenY_05b212(pc) | +04c
        jsr     Sprite_Dispatch_05A9D6__L05a9ea(pc) | +050
        movem.l (a7)+,d0-d5/a0-a1               | +054
        rts                                     | +058
.L05b20c:
        movem.l (a7)+,d0-d5/a0-a1               | +05a
        rts                                     | +05e

| ----------------------------------------------------------------------------
|  Sprite_SplashScreenY_05b212  @ $05B212  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Sprite_SplashScreenY_05b212, "ax", @progbits
        .global Sprite_SplashScreenY_05b212
Sprite_SplashScreenY_05b212:
        movem.l d0-d4,-(a7)                     | +000
        move.w  d0,d6                           | +004
        addq.w  #0x2,d6                         | +006
        move.w  #0xe8,d1                        | +008
        jsr     0x96a5a.l                       | +00c
        sub.w   d1,d6                           | +012
        sub.w   d6,d1                           | +014
        move.w  d1,d6                           | +016
        movem.l (a7)+,d0-d4                     | +018
        move.w  d6,d0                           | +01c
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  SpriteQueue_SortAndRenderSCB1_05b232  @ $05B232  (318 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteQueue_SortAndRenderSCB1_05b232, "ax", @progbits
        .global SpriteQueue_SortAndRenderSCB1_05b232
SpriteQueue_SortAndRenderSCB1_05b232:
        lea     0x108080.l,a5                   | +000
        lea     0x5424(a5),a0                   | +006
        move.w  0x6148(a5),d7                   | +00a
        beq.w   .L05b2de                        | +00e
        move.w  d7,d6                           | +012
        lsr.w   #0x1,d6                         | +014
        andi.w  #0xfffc,d6                      | +016
        beq.w   .L05b2de                        | +01a
.L05b250:
        move.l  (a0,d6.w),d0                    | +01e
        move.w  d6,d5                           | +022
        move.w  d5,d4                           | +024
        add.w   d4,d4                           | +026
.L05b25a:
        move.l  (a0,d4.w),d1                    | +028
        cmp.w   d7,d4                           | +02c
        bcc.w   .L05b272                        | +02e
        move.l  0x4(a0,d4.w),d2                 | +032
        cmp.l   d1,d2                           | +036
        bcs.w   .L05b272                        | +038
        move.l  d2,d1                           | +03c
        addq.w  #0x4,d4                         | +03e
.L05b272:
        cmp.l   d0,d1                           | +040
        bcs.w   .L05b284                        | +042
        move.l  d1,(a0,d5.w)                    | +046
        move.w  d4,d5                           | +04a
        add.w   d4,d4                           | +04c
        cmp.w   d4,d7                           | +04e
        bcc.b   .L05b25a                        | +050
.L05b284:
        move.l  d0,(a0,d5.w)                    | +052
        subq.w  #0x4,d6                         | +056
        bne.b   .L05b250                        | +058
        move.w  #0x4,d6                         | +05a
.L05b290:
        move.l  0x4(a0),d1                      | +05e
        move.l  (a0,d7.w),d0                    | +062
        move.l  d1,(a0,d7.w)                    | +066
        subq.w  #0x4,d7                         | +06a
        cmp.w   d6,d7                           | +06c
        beq.w   .L05b2da                        | +06e
        move.w  d6,d5                           | +072
        move.w  d5,d4                           | +074
        add.w   d4,d4                           | +076
.L05b2aa:
        move.l  (a0,d4.w),d1                    | +078
        cmp.w   d7,d4                           | +07c
        bcc.w   .L05b2c2                        | +07e
        move.l  0x4(a0,d4.w),d2                 | +082
        cmp.l   d1,d2                           | +086
        bcs.w   .L05b2c2                        | +088
        move.l  d2,d1                           | +08c
        addq.w  #0x4,d4                         | +08e
.L05b2c2:
        cmp.l   d0,d1                           | +090
        bcs.w   .L05b2d4                        | +092
        move.l  d1,(a0,d5.w)                    | +096
        move.w  d4,d5                           | +09a
        add.w   d4,d4                           | +09c
        cmp.w   d4,d7                           | +09e
        bcc.b   .L05b2aa                        | +0a0
.L05b2d4:
        move.l  d0,(a0,d5.w)                    | +0a2
        bra.b   .L05b290                        | +0a6
.L05b2da:
        move.l  d0,0x4(a0)                      | +0a8
.L05b2de:
        lea     0x5428(a5),a0                   | +0ac
        lea     0x348(a0),a1                    | +0b0
        move.w  0x614a(a5),d7                   | +0b4
        cmpi.w  #0x344,d7                       | +0b8
        bcc.w   .L05b31a                        | +0bc
        adda.w  d7,a0                           | +0c0
        lea     0x4(a0),a2                      | +0c2
.L05b2f8:
        movea.l a2,a3                           | +0c6
        move.l  (a2)+,d0                        | +0c8
        move.l  -(a3),d1                        | +0ca
        cmp.l   d0,d1                           | +0cc
        bls.w   .L05b316                        | +0ce
.L05b304:
        move.l  d1,0x4(a3)                      | +0d2
        move.l  -(a3),d1                        | +0d6
        cmpa.l  a3,a0                           | +0d8
        bhi.b   .L05b312                        | +0da
        cmp.l   d0,d1                           | +0dc
        bhi.b   .L05b304                        | +0de
.L05b312:
        move.l  d0,0x4(a3)                      | +0e0
.L05b316:
        cmpa.l  a1,a2                           | +0e4
        bcs.b   .L05b2f8                        | +0e6
.L05b31a:
        move.l  a6,-(a7)                        | +0e8
        lea     0x3c0000.l,a4                   | +0ea
        lea     0x614c(a5),a0                   | +0f0
        move.w  0x10e1f6.l,(a0)                 | +0f4
        move.w  0x10e1fa.l,0x2(a0)              | +0fa
        lea     0x542a(a5),a1                   | +102
        move.w  0x6148(a5),d7                   | +106
        lea     (a1,d7.w),a2                    | +10a
        bsr.w   SpriteQueue_RenderRange_05b370  | +10e
        lea     0x6158(a5),a0                   | +112
        move.w  0x10e1f8.l,(a0)                 | +116
        move.w  #0x17c,0x2(a0)                  | +11c
        lea     0x542a(a5),a1                   | +122
        lea     0x348(a1),a2                    | +126
        move.w  0x614a(a5),d7                   | +12a
        adda.w  d7,a1                           | +12e
        bsr.w   SpriteQueue_RenderRange_05b370  | +130
        movea.l (a7)+,a6                        | +134
        move.b  #0xff,0x616c(a5)                | +136
        rts                                     | +13c

| ----------------------------------------------------------------------------
|  SpriteQueue_RenderRange_05b370  @ $05B370  (144 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteQueue_RenderRange_05b370, "ax", @progbits
        .global SpriteQueue_RenderRange_05b370
SpriteQueue_RenderRange_05b370:
        move.w  0x4(a0),d2                      | +000
        not.b   0x8(a0)                         | +004
        bpl.w   .L05b3be                        | +008
        move.w  (a0),d0                         | +00c
        move.w  d0,d1                           | +00e
        lsl.w   #0x6,d1                         | +010
        move.w  0x2(a0),d3                      | +012
.L05b386:
        cmpa.l  a1,a2                           | +016
        bls.w   .L05b3ba                        | +018
        move.w  (a1),d5                         | +01c
        lea     0x4258(a5),a3                   | +01e
        adda.w  d5,a3                           | +022
.L05b394:
        cmp.w   d3,d0                           | +024
        bcc.w   .L05b3fa                        | +026
        cmp.w   d2,d0                           | +02a
        bcc.w   .L05b3fa                        | +02c
        bsr.w   SCB1_WriteTileColumn_05b52e     | +030
        addq.w  #0x1,d0                         | +034
        addi.w  #0x40,d1                        | +036
        tst.b   0x6(a3)                         | +03a
        adda.w  #0xc,a3                         | +03e
        bpl.b   .L05b394                        | +042
        adda.w  #0x4,a1                         | +044
        bra.b   .L05b386                        | +048
.L05b3ba:
        bra.w   .L05b3fa                        | +04a
.L05b3be:
        move.w  0x2(a0),d0                      | +04e
        move.w  d0,d1                           | +052
        lsl.w   #0x6,d1                         | +054
        move.w  (a0),d3                         | +056
.L05b3c8:
        cmpa.l  a1,a2                           | +058
        bls.w   .L05b3fa                        | +05a
        subq.l  #0x4,a2                         | +05e
        move.w  (a2),d5                         | +060
        lea     0x4258(a5),a3                   | +062
        adda.w  d5,a3                           | +066
.L05b3d8:
        cmp.w   d0,d3                           | +068
        bcc.w   .L05b3fa                        | +06a
        cmp.w   d0,d2                           | +06e
        bcc.w   .L05b3fa                        | +070
        subq.w  #0x1,d0                         | +074
        subi.w  #0x40,d1                        | +076
        bsr.w   SCB1_WriteTileColumn_05b52e     | +07a
        tst.b   0x6(a3)                         | +07e
        adda.w  #0xc,a3                         | +082
        bpl.b   .L05b3d8                        | +086
        bra.b   .L05b3c8                        | +088
.L05b3fa:
        move.w  d0,0x6(a0)                      | +08a
        rts                                     | +08e

| ----------------------------------------------------------------------------
|  SpriteQueue_RenderSCB234_05b400  @ $05B400  (302 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteQueue_RenderSCB234_05b400, "ax", @progbits
        .global SpriteQueue_RenderSCB234_05b400
SpriteQueue_RenderSCB234_05b400:
        tst.b   0x8(a0)                         | +000
        bpl.w   .L05b496                        | +004
        move.w  (a0),d0                         | +008
        move.w  d0,d1                           | +00a
        lsl.w   #0x6,d1                         | +00c
        move.w  0x2(a0),d2                      | +00e
        move.w  #0x40,d3                        | +012
.L05b416:
        cmpa.l  a1,a2                           | +016
        bls.w   .L05b45c                        | +018
        move.w  (a1),d5                         | +01c
        lea     0x4258(a5),a3                   | +01e
        adda.w  d5,a3                           | +022
.L05b424:
        cmp.w   d2,d0                           | +024
        bcc.w   .L05b524                        | +026
        cmp.w   0x6(a0),d0                      | +02a
        bcc.w   .L05b444                        | +02e
        move.w  #0x8201,d7                      | +032
        add.w   d0,d7                           | +036
        swap    d7                              | +038
        move.w  0x8(a3),d7                      | +03a
        move.l  d7,(a4)                         | +03e
        bra.w   .L05b448                        | +040
.L05b444:
        bsr.w   SCB1_WriteTileColumnTerm_05bf76 | +044
.L05b448:
        addq.w  #0x1,d0                         | +048
        add.w   d3,d1                           | +04a
        tst.b   0x6(a3)                         | +04c
        adda.w  #0xc,a3                         | +050
        bpl.b   .L05b424                        | +054
        adda.w  #0x4,a1                         | +056
        bra.b   .L05b416                        | +05a
.L05b45c:
        move.w  d2,d1                           | +05c
        sub.w   d0,d1                           | +05e
        move.w  d1,0xa(a0)                      | +060
        move.w  0x4(a0),d1                      | +064
        bpl.w   .L05b46e                        | +068
        move.w  d2,d1                           | +06c
.L05b46e:
        cmp.w   d0,d1                           | +06e
        bcc.w   .L05b476                        | +070
        move.w  d0,d1                           | +074
.L05b476:
        sub.w   d1,d2                           | +076
        move.l  #0x8201,d7                      | +078
        add.w   d1,d7                           | +07e
        swap    d7                              | +080
        moveq   #1,d6                           | +082
        swap    d6                              | +084
        bra.w   .L05b48e                        | +086
.L05b48a:
        move.l  d7,(a4)                         | +08a
        add.l   d6,d7                           | +08c
.L05b48e:
        dbra    d2,.L05b48a                     | +08e
        bra.w   .L05b51e                        | +092
.L05b496:
        move.w  0x2(a0),d0                      | +096
        move.w  d0,d1                           | +09a
        lsl.w   #0x6,d1                         | +09c
        move.w  (a0),d2                         | +09e
        move.w  #0x40,d3                        | +0a0
.L05b4a4:
        cmpa.l  a1,a2                           | +0a4
        bls.w   .L05b4e8                        | +0a6
        subq.l  #0x4,a2                         | +0aa
        move.w  (a2),d5                         | +0ac
        lea     0x4258(a5),a3                   | +0ae
        adda.w  d5,a3                           | +0b2
.L05b4b4:
        cmp.w   d0,d2                           | +0b4
        bcc.w   .L05b524                        | +0b6
        subq.w  #0x1,d0                         | +0ba
        sub.w   d3,d1                           | +0bc
        cmp.w   0x6(a0),d0                      | +0be
        bcs.w   .L05b4d8                        | +0c2
        move.w  #0x8201,d7                      | +0c6
        add.w   d0,d7                           | +0ca
        swap    d7                              | +0cc
        move.w  0x8(a3),d7                      | +0ce
        move.l  d7,(a4)                         | +0d2
        bra.w   .L05b4dc                        | +0d4
.L05b4d8:
        bsr.w   SCB1_WriteTileColumnTerm_05bf76 | +0d8
.L05b4dc:
        tst.b   0x6(a3)                         | +0dc
        adda.w  #0xc,a3                         | +0e0
        bpl.b   .L05b4b4                        | +0e4
        bra.b   .L05b4a4                        | +0e6
.L05b4e8:
        move.w  d0,d1                           | +0e8
        sub.w   d2,d1                           | +0ea
        move.w  d1,0xa(a0)                      | +0ec
        move.w  0x4(a0),d1                      | +0f0
        bpl.w   .L05b4fa                        | +0f4
        move.w  d2,d1                           | +0f8
.L05b4fa:
        cmp.w   d0,d1                           | +0fa
        bcs.w   .L05b502                        | +0fc
        move.w  d0,d1                           | +100
.L05b502:
        sub.w   d2,d1                           | +102
        move.l  #0x8201,d7                      | +104
        add.w   d2,d7                           | +10a
        swap    d7                              | +10c
        moveq   #1,d6                           | +10e
        swap    d6                              | +110
        bra.w   .L05b51a                        | +112
.L05b516:
        move.l  d7,(a4)                         | +116
        add.l   d6,d7                           | +118
.L05b51a:
        dbra    d1,.L05b516                     | +11a
.L05b51e:
        move.w  d0,0x4(a0)                      | +11e
        rts                                     | +122
.L05b524:
        move.w  d0,0x4(a0)                      | +124
        clr.w   0xa(a0)                         | +128
        rts                                     | +12c

| ----------------------------------------------------------------------------
|  SCB1_WriteTileColumn_05b52e  @ $05B52E  (2632 B)
| ----------------------------------------------------------------------------
        .section .text.SCB1_WriteTileColumn_05b52e, "ax", @progbits
        .global SCB1_WriteTileColumn_05b52e
SCB1_WriteTileColumn_05b52e:
        move.w  0x6(a3),d7                      | +000
        addq.b  #0x1,d7                         | +004
        sne.b   d6                              | +006
        andi.b  #0x4,d6                         | +008
        rol.w   #0x7,d7                         | +00c
        andi.w  #0x8,d7                         | +00e
        or.b    d6,d7                           | +012
        jmp     .L05b546(pc,d7.w)               | +014
.L05b546:
        bra.w   .L05b556                        | +018
        bra.w   .L05b962                        | +01c
        bra.w   .L05b75c                        | +020
        bra.w   .L05bc6c                        | +024
.L05b556:
        move.w  #0x40,d7                        | +028
        add.w   d1,d7                           | +02c
        move.w  d7,d5                           | +02e
        addq.w  #0x1,d5                         | +030
        swap    d5                              | +032
        move.w  0x4(a3),d5                      | +034
        swap    d7                              | +038
        moveq   #2,d4                           | +03a
        swap    d4                              | +03c
        movea.l (a3),a6                         | +03e
        move.w  0xa(a3),d6                      | +040
        subq.b  #0x1,d6                         | +044
        andi.w  #0x1f,d6                        | +046
        add.w   d6,d6                           | +04a
        add.w   d6,d6                           | +04c
        jmp     .L05b580(pc,d6.w)               | +04e
.L05b580:
        bra.w   .L05b732                        | +052
        bra.w   .L05b728                        | +056
        bra.w   .L05b71e                        | +05a
        bra.w   .L05b714                        | +05e
        bra.w   .L05b70a                        | +062
        bra.w   .L05b700                        | +066
        bra.w   .L05b6f6                        | +06a
        bra.w   .L05b6ec                        | +06e
        bra.w   .L05b6e2                        | +072
        bra.w   .L05b6d8                        | +076
        bra.w   .L05b6ce                        | +07a
        bra.w   .L05b6c4                        | +07e
        bra.w   .L05b6ba                        | +082
        bra.w   .L05b6b0                        | +086
        bra.w   .L05b6a6                        | +08a
        bra.w   .L05b69c                        | +08e
        bra.w   .L05b692                        | +092
        bra.w   .L05b688                        | +096
        bra.w   .L05b67e                        | +09a
        bra.w   .L05b674                        | +09e
        bra.w   .L05b66a                        | +0a2
        bra.w   .L05b660                        | +0a6
        bra.w   .L05b656                        | +0aa
        bra.w   .L05b64c                        | +0ae
        bra.w   .L05b642                        | +0b2
        bra.w   .L05b638                        | +0b6
        bra.w   .L05b62e                        | +0ba
        bra.w   .L05b624                        | +0be
        bra.w   .L05b61a                        | +0c2
        bra.w   .L05b610                        | +0c6
        bra.w   .L05b606                        | +0ca
        move.w  (a6)+,d7                        | +0ce
        move.l  d7,(a4)                         | +0d0
        add.l   d4,d7                           | +0d2
        move.l  d5,(a4)                         | +0d4
        add.l   d4,d5                           | +0d6
.L05b606:
        move.w  (a6)+,d7                        | +0d8
        move.l  d7,(a4)                         | +0da
        add.l   d4,d7                           | +0dc
        move.l  d5,(a4)                         | +0de
        add.l   d4,d5                           | +0e0
.L05b610:
        move.w  (a6)+,d7                        | +0e2
        move.l  d7,(a4)                         | +0e4
        add.l   d4,d7                           | +0e6
        move.l  d5,(a4)                         | +0e8
        add.l   d4,d5                           | +0ea
.L05b61a:
        move.w  (a6)+,d7                        | +0ec
        move.l  d7,(a4)                         | +0ee
        add.l   d4,d7                           | +0f0
        move.l  d5,(a4)                         | +0f2
        add.l   d4,d5                           | +0f4
.L05b624:
        move.w  (a6)+,d7                        | +0f6
        move.l  d7,(a4)                         | +0f8
        add.l   d4,d7                           | +0fa
        move.l  d5,(a4)                         | +0fc
        add.l   d4,d5                           | +0fe
.L05b62e:
        move.w  (a6)+,d7                        | +100
        move.l  d7,(a4)                         | +102
        add.l   d4,d7                           | +104
        move.l  d5,(a4)                         | +106
        add.l   d4,d5                           | +108
.L05b638:
        move.w  (a6)+,d7                        | +10a
        move.l  d7,(a4)                         | +10c
        add.l   d4,d7                           | +10e
        move.l  d5,(a4)                         | +110
        add.l   d4,d5                           | +112
.L05b642:
        move.w  (a6)+,d7                        | +114
        move.l  d7,(a4)                         | +116
        add.l   d4,d7                           | +118
        move.l  d5,(a4)                         | +11a
        add.l   d4,d5                           | +11c
.L05b64c:
        move.w  (a6)+,d7                        | +11e
        move.l  d7,(a4)                         | +120
        add.l   d4,d7                           | +122
        move.l  d5,(a4)                         | +124
        add.l   d4,d5                           | +126
.L05b656:
        move.w  (a6)+,d7                        | +128
        move.l  d7,(a4)                         | +12a
        add.l   d4,d7                           | +12c
        move.l  d5,(a4)                         | +12e
        add.l   d4,d5                           | +130
.L05b660:
        move.w  (a6)+,d7                        | +132
        move.l  d7,(a4)                         | +134
        add.l   d4,d7                           | +136
        move.l  d5,(a4)                         | +138
        add.l   d4,d5                           | +13a
.L05b66a:
        move.w  (a6)+,d7                        | +13c
        move.l  d7,(a4)                         | +13e
        add.l   d4,d7                           | +140
        move.l  d5,(a4)                         | +142
        add.l   d4,d5                           | +144
.L05b674:
        move.w  (a6)+,d7                        | +146
        move.l  d7,(a4)                         | +148
        add.l   d4,d7                           | +14a
        move.l  d5,(a4)                         | +14c
        add.l   d4,d5                           | +14e
.L05b67e:
        move.w  (a6)+,d7                        | +150
        move.l  d7,(a4)                         | +152
        add.l   d4,d7                           | +154
        move.l  d5,(a4)                         | +156
        add.l   d4,d5                           | +158
.L05b688:
        move.w  (a6)+,d7                        | +15a
        move.l  d7,(a4)                         | +15c
        add.l   d4,d7                           | +15e
        move.l  d5,(a4)                         | +160
        add.l   d4,d5                           | +162
.L05b692:
        move.w  (a6)+,d7                        | +164
        move.l  d7,(a4)                         | +166
        add.l   d4,d7                           | +168
        move.l  d5,(a4)                         | +16a
        add.l   d4,d5                           | +16c
.L05b69c:
        move.w  (a6)+,d7                        | +16e
        move.l  d7,(a4)                         | +170
        add.l   d4,d7                           | +172
        move.l  d5,(a4)                         | +174
        add.l   d4,d5                           | +176
.L05b6a6:
        move.w  (a6)+,d7                        | +178
        move.l  d7,(a4)                         | +17a
        add.l   d4,d7                           | +17c
        move.l  d5,(a4)                         | +17e
        add.l   d4,d5                           | +180
.L05b6b0:
        move.w  (a6)+,d7                        | +182
        move.l  d7,(a4)                         | +184
        add.l   d4,d7                           | +186
        move.l  d5,(a4)                         | +188
        add.l   d4,d5                           | +18a
.L05b6ba:
        move.w  (a6)+,d7                        | +18c
        move.l  d7,(a4)                         | +18e
        add.l   d4,d7                           | +190
        move.l  d5,(a4)                         | +192
        add.l   d4,d5                           | +194
.L05b6c4:
        move.w  (a6)+,d7                        | +196
        move.l  d7,(a4)                         | +198
        add.l   d4,d7                           | +19a
        move.l  d5,(a4)                         | +19c
        add.l   d4,d5                           | +19e
.L05b6ce:
        move.w  (a6)+,d7                        | +1a0
        move.l  d7,(a4)                         | +1a2
        add.l   d4,d7                           | +1a4
        move.l  d5,(a4)                         | +1a6
        add.l   d4,d5                           | +1a8
.L05b6d8:
        move.w  (a6)+,d7                        | +1aa
        move.l  d7,(a4)                         | +1ac
        add.l   d4,d7                           | +1ae
        move.l  d5,(a4)                         | +1b0
        add.l   d4,d5                           | +1b2
.L05b6e2:
        move.w  (a6)+,d7                        | +1b4
        move.l  d7,(a4)                         | +1b6
        add.l   d4,d7                           | +1b8
        move.l  d5,(a4)                         | +1ba
        add.l   d4,d5                           | +1bc
.L05b6ec:
        move.w  (a6)+,d7                        | +1be
        move.l  d7,(a4)                         | +1c0
        add.l   d4,d7                           | +1c2
        move.l  d5,(a4)                         | +1c4
        add.l   d4,d5                           | +1c6
.L05b6f6:
        move.w  (a6)+,d7                        | +1c8
        move.l  d7,(a4)                         | +1ca
        add.l   d4,d7                           | +1cc
        move.l  d5,(a4)                         | +1ce
        add.l   d4,d5                           | +1d0
.L05b700:
        move.w  (a6)+,d7                        | +1d2
        move.l  d7,(a4)                         | +1d4
        add.l   d4,d7                           | +1d6
        move.l  d5,(a4)                         | +1d8
        add.l   d4,d5                           | +1da
.L05b70a:
        move.w  (a6)+,d7                        | +1dc
        move.l  d7,(a4)                         | +1de
        add.l   d4,d7                           | +1e0
        move.l  d5,(a4)                         | +1e2
        add.l   d4,d5                           | +1e4
.L05b714:
        move.w  (a6)+,d7                        | +1e6
        move.l  d7,(a4)                         | +1e8
        add.l   d4,d7                           | +1ea
        move.l  d5,(a4)                         | +1ec
        add.l   d4,d5                           | +1ee
.L05b71e:
        move.w  (a6)+,d7                        | +1f0
        move.l  d7,(a4)                         | +1f2
        add.l   d4,d7                           | +1f4
        move.l  d5,(a4)                         | +1f6
        add.l   d4,d5                           | +1f8
.L05b728:
        move.w  (a6)+,d7                        | +1fa
        move.l  d7,(a4)                         | +1fc
        add.l   d4,d7                           | +1fe
        move.l  d5,(a4)                         | +200
        add.l   d4,d5                           | +202
.L05b732:
        move.w  (a6)+,d7                        | +204
        move.l  d7,(a4)                         | +206
        add.l   d4,d7                           | +208
        move.l  d5,(a4)                         | +20a
        add.l   d4,d5                           | +20c
        move.w  #0x8401,d6                      | +20e
        add.w   d0,d6                           | +212
        swap    d6                              | +214
        move.w  0xa(a3),d6                      | +216
        move.l  #0x2000000,d5                   | +21a
        move.l  d6,(a4)                         | +220
        sub.l   d5,d6                           | +222
        sub.l   d5,d6                           | +224
        move.w  0x6(a3),d6                      | +226
        move.l  d6,(a4)                         | +22a
        rts                                     | +22c
.L05b75c:
        move.w  #0x40,d7                        | +22e
        add.w   d1,d7                           | +232
        move.w  d7,d5                           | +234
        addq.w  #0x1,d5                         | +236
        swap    d5                              | +238
        move.w  0x4(a3),d5                      | +23a
        swap    d7                              | +23e
        moveq   #2,d4                           | +240
        swap    d4                              | +242
        movea.l (a3),a6                         | +244
        move.w  0xa(a3),d6                      | +246
        subq.b  #0x1,d6                         | +24a
        andi.w  #0x1f,d6                        | +24c
        add.w   d6,d6                           | +250
        add.w   d6,d6                           | +252
        jmp     .L05b786(pc,d6.w)               | +254
.L05b786:
        bra.w   .L05b938                        | +258
        bra.w   .L05b92e                        | +25c
        bra.w   .L05b924                        | +260
        bra.w   .L05b91a                        | +264
        bra.w   .L05b910                        | +268
        bra.w   .L05b906                        | +26c
        bra.w   .L05b8fc                        | +270
        bra.w   .L05b8f2                        | +274
        bra.w   .L05b8e8                        | +278
        bra.w   .L05b8de                        | +27c
        bra.w   .L05b8d4                        | +280
        bra.w   .L05b8ca                        | +284
        bra.w   .L05b8c0                        | +288
        bra.w   .L05b8b6                        | +28c
        bra.w   .L05b8ac                        | +290
        bra.w   .L05b8a2                        | +294
        bra.w   .L05b898                        | +298
        bra.w   .L05b88e                        | +29c
        bra.w   .L05b884                        | +2a0
        bra.w   .L05b87a                        | +2a4
        bra.w   .L05b870                        | +2a8
        bra.w   .L05b866                        | +2ac
        bra.w   .L05b85c                        | +2b0
        bra.w   .L05b852                        | +2b4
        bra.w   .L05b848                        | +2b8
        bra.w   .L05b83e                        | +2bc
        bra.w   .L05b834                        | +2c0
        bra.w   .L05b82a                        | +2c4
        bra.w   .L05b820                        | +2c8
        bra.w   .L05b816                        | +2cc
        bra.w   .L05b80c                        | +2d0
        move.w  -(a6),d7                        | +2d4
        move.l  d7,(a4)                         | +2d6
        add.l   d4,d7                           | +2d8
        move.l  d5,(a4)                         | +2da
        add.l   d4,d5                           | +2dc
.L05b80c:
        move.w  -(a6),d7                        | +2de
        move.l  d7,(a4)                         | +2e0
        add.l   d4,d7                           | +2e2
        move.l  d5,(a4)                         | +2e4
        add.l   d4,d5                           | +2e6
.L05b816:
        move.w  -(a6),d7                        | +2e8
        move.l  d7,(a4)                         | +2ea
        add.l   d4,d7                           | +2ec
        move.l  d5,(a4)                         | +2ee
        add.l   d4,d5                           | +2f0
.L05b820:
        move.w  -(a6),d7                        | +2f2
        move.l  d7,(a4)                         | +2f4
        add.l   d4,d7                           | +2f6
        move.l  d5,(a4)                         | +2f8
        add.l   d4,d5                           | +2fa
.L05b82a:
        move.w  -(a6),d7                        | +2fc
        move.l  d7,(a4)                         | +2fe
        add.l   d4,d7                           | +300
        move.l  d5,(a4)                         | +302
        add.l   d4,d5                           | +304
.L05b834:
        move.w  -(a6),d7                        | +306
        move.l  d7,(a4)                         | +308
        add.l   d4,d7                           | +30a
        move.l  d5,(a4)                         | +30c
        add.l   d4,d5                           | +30e
.L05b83e:
        move.w  -(a6),d7                        | +310
        move.l  d7,(a4)                         | +312
        add.l   d4,d7                           | +314
        move.l  d5,(a4)                         | +316
        add.l   d4,d5                           | +318
.L05b848:
        move.w  -(a6),d7                        | +31a
        move.l  d7,(a4)                         | +31c
        add.l   d4,d7                           | +31e
        move.l  d5,(a4)                         | +320
        add.l   d4,d5                           | +322
.L05b852:
        move.w  -(a6),d7                        | +324
        move.l  d7,(a4)                         | +326
        add.l   d4,d7                           | +328
        move.l  d5,(a4)                         | +32a
        add.l   d4,d5                           | +32c
.L05b85c:
        move.w  -(a6),d7                        | +32e
        move.l  d7,(a4)                         | +330
        add.l   d4,d7                           | +332
        move.l  d5,(a4)                         | +334
        add.l   d4,d5                           | +336
.L05b866:
        move.w  -(a6),d7                        | +338
        move.l  d7,(a4)                         | +33a
        add.l   d4,d7                           | +33c
        move.l  d5,(a4)                         | +33e
        add.l   d4,d5                           | +340
.L05b870:
        move.w  -(a6),d7                        | +342
        move.l  d7,(a4)                         | +344
        add.l   d4,d7                           | +346
        move.l  d5,(a4)                         | +348
        add.l   d4,d5                           | +34a
.L05b87a:
        move.w  -(a6),d7                        | +34c
        move.l  d7,(a4)                         | +34e
        add.l   d4,d7                           | +350
        move.l  d5,(a4)                         | +352
        add.l   d4,d5                           | +354
.L05b884:
        move.w  -(a6),d7                        | +356
        move.l  d7,(a4)                         | +358
        add.l   d4,d7                           | +35a
        move.l  d5,(a4)                         | +35c
        add.l   d4,d5                           | +35e
.L05b88e:
        move.w  -(a6),d7                        | +360
        move.l  d7,(a4)                         | +362
        add.l   d4,d7                           | +364
        move.l  d5,(a4)                         | +366
        add.l   d4,d5                           | +368
.L05b898:
        move.w  -(a6),d7                        | +36a
        move.l  d7,(a4)                         | +36c
        add.l   d4,d7                           | +36e
        move.l  d5,(a4)                         | +370
        add.l   d4,d5                           | +372
.L05b8a2:
        move.w  -(a6),d7                        | +374
        move.l  d7,(a4)                         | +376
        add.l   d4,d7                           | +378
        move.l  d5,(a4)                         | +37a
        add.l   d4,d5                           | +37c
.L05b8ac:
        move.w  -(a6),d7                        | +37e
        move.l  d7,(a4)                         | +380
        add.l   d4,d7                           | +382
        move.l  d5,(a4)                         | +384
        add.l   d4,d5                           | +386
.L05b8b6:
        move.w  -(a6),d7                        | +388
        move.l  d7,(a4)                         | +38a
        add.l   d4,d7                           | +38c
        move.l  d5,(a4)                         | +38e
        add.l   d4,d5                           | +390
.L05b8c0:
        move.w  -(a6),d7                        | +392
        move.l  d7,(a4)                         | +394
        add.l   d4,d7                           | +396
        move.l  d5,(a4)                         | +398
        add.l   d4,d5                           | +39a
.L05b8ca:
        move.w  -(a6),d7                        | +39c
        move.l  d7,(a4)                         | +39e
        add.l   d4,d7                           | +3a0
        move.l  d5,(a4)                         | +3a2
        add.l   d4,d5                           | +3a4
.L05b8d4:
        move.w  -(a6),d7                        | +3a6
        move.l  d7,(a4)                         | +3a8
        add.l   d4,d7                           | +3aa
        move.l  d5,(a4)                         | +3ac
        add.l   d4,d5                           | +3ae
.L05b8de:
        move.w  -(a6),d7                        | +3b0
        move.l  d7,(a4)                         | +3b2
        add.l   d4,d7                           | +3b4
        move.l  d5,(a4)                         | +3b6
        add.l   d4,d5                           | +3b8
.L05b8e8:
        move.w  -(a6),d7                        | +3ba
        move.l  d7,(a4)                         | +3bc
        add.l   d4,d7                           | +3be
        move.l  d5,(a4)                         | +3c0
        add.l   d4,d5                           | +3c2
.L05b8f2:
        move.w  -(a6),d7                        | +3c4
        move.l  d7,(a4)                         | +3c6
        add.l   d4,d7                           | +3c8
        move.l  d5,(a4)                         | +3ca
        add.l   d4,d5                           | +3cc
.L05b8fc:
        move.w  -(a6),d7                        | +3ce
        move.l  d7,(a4)                         | +3d0
        add.l   d4,d7                           | +3d2
        move.l  d5,(a4)                         | +3d4
        add.l   d4,d5                           | +3d6
.L05b906:
        move.w  -(a6),d7                        | +3d8
        move.l  d7,(a4)                         | +3da
        add.l   d4,d7                           | +3dc
        move.l  d5,(a4)                         | +3de
        add.l   d4,d5                           | +3e0
.L05b910:
        move.w  -(a6),d7                        | +3e2
        move.l  d7,(a4)                         | +3e4
        add.l   d4,d7                           | +3e6
        move.l  d5,(a4)                         | +3e8
        add.l   d4,d5                           | +3ea
.L05b91a:
        move.w  -(a6),d7                        | +3ec
        move.l  d7,(a4)                         | +3ee
        add.l   d4,d7                           | +3f0
        move.l  d5,(a4)                         | +3f2
        add.l   d4,d5                           | +3f4
.L05b924:
        move.w  -(a6),d7                        | +3f6
        move.l  d7,(a4)                         | +3f8
        add.l   d4,d7                           | +3fa
        move.l  d5,(a4)                         | +3fc
        add.l   d4,d5                           | +3fe
.L05b92e:
        move.w  -(a6),d7                        | +400
        move.l  d7,(a4)                         | +402
        add.l   d4,d7                           | +404
        move.l  d5,(a4)                         | +406
        add.l   d4,d5                           | +408
.L05b938:
        move.w  -(a6),d7                        | +40a
        move.l  d7,(a4)                         | +40c
        add.l   d4,d7                           | +40e
        move.l  d5,(a4)                         | +410
        add.l   d4,d5                           | +412
        move.w  #0x8401,d6                      | +414
        add.w   d0,d6                           | +418
        swap    d6                              | +41a
        move.w  0xa(a3),d6                      | +41c
        move.l  #0x2000000,d5                   | +420
        move.l  d6,(a4)                         | +426
        sub.l   d5,d6                           | +428
        sub.l   d5,d6                           | +42a
        move.w  0x6(a3),d6                      | +42c
        move.l  d6,(a4)                         | +430
        rts                                     | +432
.L05b962:
        move.w  #0x40,d7                        | +434
        add.w   d1,d7                           | +438
        move.w  d7,d5                           | +43a
        addq.w  #0x1,d5                         | +43c
        swap    d5                              | +43e
        move.w  0x4(a3),d5                      | +440
        swap    d7                              | +444
        moveq   #2,d4                           | +446
        swap    d4                              | +448
        movea.l (a3),a6                         | +44a
        move.w  0xa(a3),d6                      | +44c
        subq.b  #0x1,d6                         | +450
        andi.w  #0x1f,d6                        | +452
        add.w   d6,d6                           | +456
        add.w   d6,d6                           | +458
        jmp     .L05b98c(pc,d6.w)               | +45a
.L05b98c:
        bra.w   .L05bb3e                        | +45e
        bra.w   .L05bb34                        | +462
        bra.w   .L05bb2a                        | +466
        bra.w   .L05bb20                        | +46a
        bra.w   .L05bb16                        | +46e
        bra.w   .L05bb0c                        | +472
        bra.w   .L05bb02                        | +476
        bra.w   .L05baf8                        | +47a
        bra.w   .L05baee                        | +47e
        bra.w   .L05bae4                        | +482
        bra.w   .L05bada                        | +486
        bra.w   .L05bad0                        | +48a
        bra.w   .L05bac6                        | +48e
        bra.w   .L05babc                        | +492
        bra.w   .L05bab2                        | +496
        bra.w   .L05baa8                        | +49a
        bra.w   .L05ba9e                        | +49e
        bra.w   .L05ba94                        | +4a2
        bra.w   .L05ba8a                        | +4a6
        bra.w   .L05ba80                        | +4aa
        bra.w   .L05ba76                        | +4ae
        bra.w   .L05ba6c                        | +4b2
        bra.w   .L05ba62                        | +4b6
        bra.w   .L05ba58                        | +4ba
        bra.w   .L05ba4e                        | +4be
        bra.w   .L05ba44                        | +4c2
        bra.w   .L05ba3a                        | +4c6
        bra.w   .L05ba30                        | +4ca
        bra.w   .L05ba26                        | +4ce
        bra.w   .L05ba1c                        | +4d2
        bra.w   .L05ba12                        | +4d6
        move.w  (a6)+,d7                        | +4da
        move.l  d7,(a4)                         | +4dc
        add.l   d4,d7                           | +4de
        move.l  d5,(a4)                         | +4e0
        add.l   d4,d5                           | +4e2
.L05ba12:
        move.w  (a6)+,d7                        | +4e4
        move.l  d7,(a4)                         | +4e6
        add.l   d4,d7                           | +4e8
        move.l  d5,(a4)                         | +4ea
        add.l   d4,d5                           | +4ec
.L05ba1c:
        move.w  (a6)+,d7                        | +4ee
        move.l  d7,(a4)                         | +4f0
        add.l   d4,d7                           | +4f2
        move.l  d5,(a4)                         | +4f4
        add.l   d4,d5                           | +4f6
.L05ba26:
        move.w  (a6)+,d7                        | +4f8
        move.l  d7,(a4)                         | +4fa
        add.l   d4,d7                           | +4fc
        move.l  d5,(a4)                         | +4fe
        add.l   d4,d5                           | +500
.L05ba30:
        move.w  (a6)+,d7                        | +502
        move.l  d7,(a4)                         | +504
        add.l   d4,d7                           | +506
        move.l  d5,(a4)                         | +508
        add.l   d4,d5                           | +50a
.L05ba3a:
        move.w  (a6)+,d7                        | +50c
        move.l  d7,(a4)                         | +50e
        add.l   d4,d7                           | +510
        move.l  d5,(a4)                         | +512
        add.l   d4,d5                           | +514
.L05ba44:
        move.w  (a6)+,d7                        | +516
        move.l  d7,(a4)                         | +518
        add.l   d4,d7                           | +51a
        move.l  d5,(a4)                         | +51c
        add.l   d4,d5                           | +51e
.L05ba4e:
        move.w  (a6)+,d7                        | +520
        move.l  d7,(a4)                         | +522
        add.l   d4,d7                           | +524
        move.l  d5,(a4)                         | +526
        add.l   d4,d5                           | +528
.L05ba58:
        move.w  (a6)+,d7                        | +52a
        move.l  d7,(a4)                         | +52c
        add.l   d4,d7                           | +52e
        move.l  d5,(a4)                         | +530
        add.l   d4,d5                           | +532
.L05ba62:
        move.w  (a6)+,d7                        | +534
        move.l  d7,(a4)                         | +536
        add.l   d4,d7                           | +538
        move.l  d5,(a4)                         | +53a
        add.l   d4,d5                           | +53c
.L05ba6c:
        move.w  (a6)+,d7                        | +53e
        move.l  d7,(a4)                         | +540
        add.l   d4,d7                           | +542
        move.l  d5,(a4)                         | +544
        add.l   d4,d5                           | +546
.L05ba76:
        move.w  (a6)+,d7                        | +548
        move.l  d7,(a4)                         | +54a
        add.l   d4,d7                           | +54c
        move.l  d5,(a4)                         | +54e
        add.l   d4,d5                           | +550
.L05ba80:
        move.w  (a6)+,d7                        | +552
        move.l  d7,(a4)                         | +554
        add.l   d4,d7                           | +556
        move.l  d5,(a4)                         | +558
        add.l   d4,d5                           | +55a
.L05ba8a:
        move.w  (a6)+,d7                        | +55c
        move.l  d7,(a4)                         | +55e
        add.l   d4,d7                           | +560
        move.l  d5,(a4)                         | +562
        add.l   d4,d5                           | +564
.L05ba94:
        move.w  (a6)+,d7                        | +566
        move.l  d7,(a4)                         | +568
        add.l   d4,d7                           | +56a
        move.l  d5,(a4)                         | +56c
        add.l   d4,d5                           | +56e
.L05ba9e:
        move.w  (a6)+,d7                        | +570
        move.l  d7,(a4)                         | +572
        add.l   d4,d7                           | +574
        move.l  d5,(a4)                         | +576
        add.l   d4,d5                           | +578
.L05baa8:
        move.w  (a6)+,d7                        | +57a
        move.l  d7,(a4)                         | +57c
        add.l   d4,d7                           | +57e
        move.l  d5,(a4)                         | +580
        add.l   d4,d5                           | +582
.L05bab2:
        move.w  (a6)+,d7                        | +584
        move.l  d7,(a4)                         | +586
        add.l   d4,d7                           | +588
        move.l  d5,(a4)                         | +58a
        add.l   d4,d5                           | +58c
.L05babc:
        move.w  (a6)+,d7                        | +58e
        move.l  d7,(a4)                         | +590
        add.l   d4,d7                           | +592
        move.l  d5,(a4)                         | +594
        add.l   d4,d5                           | +596
.L05bac6:
        move.w  (a6)+,d7                        | +598
        move.l  d7,(a4)                         | +59a
        add.l   d4,d7                           | +59c
        move.l  d5,(a4)                         | +59e
        add.l   d4,d5                           | +5a0
.L05bad0:
        move.w  (a6)+,d7                        | +5a2
        move.l  d7,(a4)                         | +5a4
        add.l   d4,d7                           | +5a6
        move.l  d5,(a4)                         | +5a8
        add.l   d4,d5                           | +5aa
.L05bada:
        move.w  (a6)+,d7                        | +5ac
        move.l  d7,(a4)                         | +5ae
        add.l   d4,d7                           | +5b0
        move.l  d5,(a4)                         | +5b2
        add.l   d4,d5                           | +5b4
.L05bae4:
        move.w  (a6)+,d7                        | +5b6
        move.l  d7,(a4)                         | +5b8
        add.l   d4,d7                           | +5ba
        move.l  d5,(a4)                         | +5bc
        add.l   d4,d5                           | +5be
.L05baee:
        move.w  (a6)+,d7                        | +5c0
        move.l  d7,(a4)                         | +5c2
        add.l   d4,d7                           | +5c4
        move.l  d5,(a4)                         | +5c6
        add.l   d4,d5                           | +5c8
.L05baf8:
        move.w  (a6)+,d7                        | +5ca
        move.l  d7,(a4)                         | +5cc
        add.l   d4,d7                           | +5ce
        move.l  d5,(a4)                         | +5d0
        add.l   d4,d5                           | +5d2
.L05bb02:
        move.w  (a6)+,d7                        | +5d4
        move.l  d7,(a4)                         | +5d6
        add.l   d4,d7                           | +5d8
        move.l  d5,(a4)                         | +5da
        add.l   d4,d5                           | +5dc
.L05bb0c:
        move.w  (a6)+,d7                        | +5de
        move.l  d7,(a4)                         | +5e0
        add.l   d4,d7                           | +5e2
        move.l  d5,(a4)                         | +5e4
        add.l   d4,d5                           | +5e6
.L05bb16:
        move.w  (a6)+,d7                        | +5e8
        move.l  d7,(a4)                         | +5ea
        add.l   d4,d7                           | +5ec
        move.l  d5,(a4)                         | +5ee
        add.l   d4,d5                           | +5f0
.L05bb20:
        move.w  (a6)+,d7                        | +5f2
        move.l  d7,(a4)                         | +5f4
        add.l   d4,d7                           | +5f6
        move.l  d5,(a4)                         | +5f8
        add.l   d4,d5                           | +5fa
.L05bb2a:
        move.w  (a6)+,d7                        | +5fc
        move.l  d7,(a4)                         | +5fe
        add.l   d4,d7                           | +600
        move.l  d5,(a4)                         | +602
        add.l   d4,d5                           | +604
.L05bb34:
        move.w  (a6)+,d7                        | +606
        move.l  d7,(a4)                         | +608
        add.l   d4,d7                           | +60a
        move.l  d5,(a4)                         | +60c
        add.l   d4,d5                           | +60e
.L05bb3e:
        move.w  (a6)+,d7                        | +610
        move.l  d7,(a4)                         | +612
        add.l   d4,d7                           | +614
        move.l  d5,(a4)                         | +616
        add.l   d4,d5                           | +618
        clr.w   d7                              | +61a
        moveq   #1,d4                           | +61c
        swap    d4                              | +61e
        add.w   d6,d6                           | +620
        jmp     .L05bb54(pc,d6.w)               | +622
.L05bb54:
        move.l  d7,(a4)                         | +626
        add.l   d4,d7                           | +628
        move.l  d7,(a4)                         | +62a
        add.l   d4,d7                           | +62c
        move.l  d7,(a4)                         | +62e
        add.l   d4,d7                           | +630
        move.l  d7,(a4)                         | +632
        add.l   d4,d7                           | +634
        move.l  d7,(a4)                         | +636
        add.l   d4,d7                           | +638
        move.l  d7,(a4)                         | +63a
        add.l   d4,d7                           | +63c
        move.l  d7,(a4)                         | +63e
        add.l   d4,d7                           | +640
        move.l  d7,(a4)                         | +642
        add.l   d4,d7                           | +644
        move.l  d7,(a4)                         | +646
        add.l   d4,d7                           | +648
        move.l  d7,(a4)                         | +64a
        add.l   d4,d7                           | +64c
        move.l  d7,(a4)                         | +64e
        add.l   d4,d7                           | +650
        move.l  d7,(a4)                         | +652
        add.l   d4,d7                           | +654
        move.l  d7,(a4)                         | +656
        add.l   d4,d7                           | +658
        move.l  d7,(a4)                         | +65a
        add.l   d4,d7                           | +65c
        move.l  d7,(a4)                         | +65e
        add.l   d4,d7                           | +660
        move.l  d7,(a4)                         | +662
        add.l   d4,d7                           | +664
        move.l  d7,(a4)                         | +666
        add.l   d4,d7                           | +668
        move.l  d7,(a4)                         | +66a
        add.l   d4,d7                           | +66c
        move.l  d7,(a4)                         | +66e
        add.l   d4,d7                           | +670
        move.l  d7,(a4)                         | +672
        add.l   d4,d7                           | +674
        move.l  d7,(a4)                         | +676
        add.l   d4,d7                           | +678
        move.l  d7,(a4)                         | +67a
        add.l   d4,d7                           | +67c
        move.l  d7,(a4)                         | +67e
        add.l   d4,d7                           | +680
        move.l  d7,(a4)                         | +682
        add.l   d4,d7                           | +684
        move.l  d7,(a4)                         | +686
        add.l   d4,d7                           | +688
        move.l  d7,(a4)                         | +68a
        add.l   d4,d7                           | +68c
        move.l  d7,(a4)                         | +68e
        add.l   d4,d7                           | +690
        move.l  d7,(a4)                         | +692
        add.l   d4,d7                           | +694
        move.l  d7,(a4)                         | +696
        add.l   d4,d7                           | +698
        move.l  d7,(a4)                         | +69a
        add.l   d4,d7                           | +69c
        move.l  d7,(a4)                         | +69e
        add.l   d4,d7                           | +6a0
        move.l  d7,(a4)                         | +6a2
        add.l   d4,d7                           | +6a4
        move.l  d7,(a4)                         | +6a6
        add.l   d4,d7                           | +6a8
        move.l  d7,(a4)                         | +6aa
        add.l   d4,d7                           | +6ac
        move.l  d7,(a4)                         | +6ae
        add.l   d4,d7                           | +6b0
        move.l  d7,(a4)                         | +6b2
        add.l   d4,d7                           | +6b4
        move.l  d7,(a4)                         | +6b6
        add.l   d4,d7                           | +6b8
        move.l  d7,(a4)                         | +6ba
        add.l   d4,d7                           | +6bc
        move.l  d7,(a4)                         | +6be
        add.l   d4,d7                           | +6c0
        move.l  d7,(a4)                         | +6c2
        add.l   d4,d7                           | +6c4
        move.l  d7,(a4)                         | +6c6
        add.l   d4,d7                           | +6c8
        move.l  d7,(a4)                         | +6ca
        add.l   d4,d7                           | +6cc
        move.l  d7,(a4)                         | +6ce
        add.l   d4,d7                           | +6d0
        move.l  d7,(a4)                         | +6d2
        add.l   d4,d7                           | +6d4
        move.l  d7,(a4)                         | +6d6
        add.l   d4,d7                           | +6d8
        move.l  d7,(a4)                         | +6da
        add.l   d4,d7                           | +6dc
        move.l  d7,(a4)                         | +6de
        add.l   d4,d7                           | +6e0
        move.l  d7,(a4)                         | +6e2
        add.l   d4,d7                           | +6e4
        move.l  d7,(a4)                         | +6e6
        add.l   d4,d7                           | +6e8
        move.l  d7,(a4)                         | +6ea
        add.l   d4,d7                           | +6ec
        move.l  d7,(a4)                         | +6ee
        add.l   d4,d7                           | +6f0
        move.l  d7,(a4)                         | +6f2
        add.l   d4,d7                           | +6f4
        move.l  d7,(a4)                         | +6f6
        add.l   d4,d7                           | +6f8
        move.l  d7,(a4)                         | +6fa
        add.l   d4,d7                           | +6fc
        move.l  d7,(a4)                         | +6fe
        add.l   d4,d7                           | +700
        move.l  d7,(a4)                         | +702
        add.l   d4,d7                           | +704
        move.l  d7,(a4)                         | +706
        add.l   d4,d7                           | +708
        move.l  d7,(a4)                         | +70a
        add.l   d4,d7                           | +70c
        move.l  d7,(a4)                         | +70e
        add.l   d4,d7                           | +710
        move.l  d7,(a4)                         | +712
        add.l   d4,d7                           | +714
        move.l  d7,(a4)                         | +716
        add.l   d4,d7                           | +718
        move.l  d7,(a4)                         | +71a
        add.l   d4,d7                           | +71c
        move.w  #0x8401,d6                      | +71e
        add.w   d0,d6                           | +722
        swap    d6                              | +724
        move.w  0xa(a3),d6                      | +726
        move.l  #0x2000000,d5                   | +72a
        move.l  d6,(a4)                         | +730
        sub.l   d5,d6                           | +732
        sub.l   d5,d6                           | +734
        move.w  0x6(a3),d6                      | +736
        move.l  d6,(a4)                         | +73a
        rts                                     | +73c
.L05bc6c:
        move.w  #0x40,d7                        | +73e
        add.w   d1,d7                           | +742
        move.w  d7,d5                           | +744
        addq.w  #0x1,d5                         | +746
        swap    d5                              | +748
        move.w  0x4(a3),d5                      | +74a
        swap    d7                              | +74e
        moveq   #2,d4                           | +750
        swap    d4                              | +752
        movea.l (a3),a6                         | +754
        move.w  0xa(a3),d6                      | +756
        subq.b  #0x1,d6                         | +75a
        andi.w  #0x1f,d6                        | +75c
        add.w   d6,d6                           | +760
        add.w   d6,d6                           | +762
        jmp     .L05bc96(pc,d6.w)               | +764
.L05bc96:
        bra.w   .L05be48                        | +768
        bra.w   .L05be3e                        | +76c
        bra.w   .L05be34                        | +770
        bra.w   .L05be2a                        | +774
        bra.w   .L05be20                        | +778
        bra.w   .L05be16                        | +77c
        bra.w   .L05be0c                        | +780
        bra.w   .L05be02                        | +784
        bra.w   .L05bdf8                        | +788
        bra.w   .L05bdee                        | +78c
        bra.w   .L05bde4                        | +790
        bra.w   .L05bdda                        | +794
        bra.w   .L05bdd0                        | +798
        bra.w   .L05bdc6                        | +79c
        bra.w   .L05bdbc                        | +7a0
        bra.w   .L05bdb2                        | +7a4
        bra.w   .L05bda8                        | +7a8
        bra.w   .L05bd9e                        | +7ac
        bra.w   .L05bd94                        | +7b0
        bra.w   .L05bd8a                        | +7b4
        bra.w   .L05bd80                        | +7b8
        bra.w   .L05bd76                        | +7bc
        bra.w   .L05bd6c                        | +7c0
        bra.w   .L05bd62                        | +7c4
        bra.w   .L05bd58                        | +7c8
        bra.w   .L05bd4e                        | +7cc
        bra.w   .L05bd44                        | +7d0
        bra.w   .L05bd3a                        | +7d4
        bra.w   .L05bd30                        | +7d8
        bra.w   .L05bd26                        | +7dc
        bra.w   .L05bd1c                        | +7e0
        move.w  -(a6),d7                        | +7e4
        move.l  d7,(a4)                         | +7e6
        add.l   d4,d7                           | +7e8
        move.l  d5,(a4)                         | +7ea
        add.l   d4,d5                           | +7ec
.L05bd1c:
        move.w  -(a6),d7                        | +7ee
        move.l  d7,(a4)                         | +7f0
        add.l   d4,d7                           | +7f2
        move.l  d5,(a4)                         | +7f4
        add.l   d4,d5                           | +7f6
.L05bd26:
        move.w  -(a6),d7                        | +7f8
        move.l  d7,(a4)                         | +7fa
        add.l   d4,d7                           | +7fc
        move.l  d5,(a4)                         | +7fe
        add.l   d4,d5                           | +800
.L05bd30:
        move.w  -(a6),d7                        | +802
        move.l  d7,(a4)                         | +804
        add.l   d4,d7                           | +806
        move.l  d5,(a4)                         | +808
        add.l   d4,d5                           | +80a
.L05bd3a:
        move.w  -(a6),d7                        | +80c
        move.l  d7,(a4)                         | +80e
        add.l   d4,d7                           | +810
        move.l  d5,(a4)                         | +812
        add.l   d4,d5                           | +814
.L05bd44:
        move.w  -(a6),d7                        | +816
        move.l  d7,(a4)                         | +818
        add.l   d4,d7                           | +81a
        move.l  d5,(a4)                         | +81c
        add.l   d4,d5                           | +81e
.L05bd4e:
        move.w  -(a6),d7                        | +820
        move.l  d7,(a4)                         | +822
        add.l   d4,d7                           | +824
        move.l  d5,(a4)                         | +826
        add.l   d4,d5                           | +828
.L05bd58:
        move.w  -(a6),d7                        | +82a
        move.l  d7,(a4)                         | +82c
        add.l   d4,d7                           | +82e
        move.l  d5,(a4)                         | +830
        add.l   d4,d5                           | +832
.L05bd62:
        move.w  -(a6),d7                        | +834
        move.l  d7,(a4)                         | +836
        add.l   d4,d7                           | +838
        move.l  d5,(a4)                         | +83a
        add.l   d4,d5                           | +83c
.L05bd6c:
        move.w  -(a6),d7                        | +83e
        move.l  d7,(a4)                         | +840
        add.l   d4,d7                           | +842
        move.l  d5,(a4)                         | +844
        add.l   d4,d5                           | +846
.L05bd76:
        move.w  -(a6),d7                        | +848
        move.l  d7,(a4)                         | +84a
        add.l   d4,d7                           | +84c
        move.l  d5,(a4)                         | +84e
        add.l   d4,d5                           | +850
.L05bd80:
        move.w  -(a6),d7                        | +852
        move.l  d7,(a4)                         | +854
        add.l   d4,d7                           | +856
        move.l  d5,(a4)                         | +858
        add.l   d4,d5                           | +85a
.L05bd8a:
        move.w  -(a6),d7                        | +85c
        move.l  d7,(a4)                         | +85e
        add.l   d4,d7                           | +860
        move.l  d5,(a4)                         | +862
        add.l   d4,d5                           | +864
.L05bd94:
        move.w  -(a6),d7                        | +866
        move.l  d7,(a4)                         | +868
        add.l   d4,d7                           | +86a
        move.l  d5,(a4)                         | +86c
        add.l   d4,d5                           | +86e
.L05bd9e:
        move.w  -(a6),d7                        | +870
        move.l  d7,(a4)                         | +872
        add.l   d4,d7                           | +874
        move.l  d5,(a4)                         | +876
        add.l   d4,d5                           | +878
.L05bda8:
        move.w  -(a6),d7                        | +87a
        move.l  d7,(a4)                         | +87c
        add.l   d4,d7                           | +87e
        move.l  d5,(a4)                         | +880
        add.l   d4,d5                           | +882
.L05bdb2:
        move.w  -(a6),d7                        | +884
        move.l  d7,(a4)                         | +886
        add.l   d4,d7                           | +888
        move.l  d5,(a4)                         | +88a
        add.l   d4,d5                           | +88c
.L05bdbc:
        move.w  -(a6),d7                        | +88e
        move.l  d7,(a4)                         | +890
        add.l   d4,d7                           | +892
        move.l  d5,(a4)                         | +894
        add.l   d4,d5                           | +896
.L05bdc6:
        move.w  -(a6),d7                        | +898
        move.l  d7,(a4)                         | +89a
        add.l   d4,d7                           | +89c
        move.l  d5,(a4)                         | +89e
        add.l   d4,d5                           | +8a0
.L05bdd0:
        move.w  -(a6),d7                        | +8a2
        move.l  d7,(a4)                         | +8a4
        add.l   d4,d7                           | +8a6
        move.l  d5,(a4)                         | +8a8
        add.l   d4,d5                           | +8aa
.L05bdda:
        move.w  -(a6),d7                        | +8ac
        move.l  d7,(a4)                         | +8ae
        add.l   d4,d7                           | +8b0
        move.l  d5,(a4)                         | +8b2
        add.l   d4,d5                           | +8b4
.L05bde4:
        move.w  -(a6),d7                        | +8b6
        move.l  d7,(a4)                         | +8b8
        add.l   d4,d7                           | +8ba
        move.l  d5,(a4)                         | +8bc
        add.l   d4,d5                           | +8be
.L05bdee:
        move.w  -(a6),d7                        | +8c0
        move.l  d7,(a4)                         | +8c2
        add.l   d4,d7                           | +8c4
        move.l  d5,(a4)                         | +8c6
        add.l   d4,d5                           | +8c8
.L05bdf8:
        move.w  -(a6),d7                        | +8ca
        move.l  d7,(a4)                         | +8cc
        add.l   d4,d7                           | +8ce
        move.l  d5,(a4)                         | +8d0
        add.l   d4,d5                           | +8d2
.L05be02:
        move.w  -(a6),d7                        | +8d4
        move.l  d7,(a4)                         | +8d6
        add.l   d4,d7                           | +8d8
        move.l  d5,(a4)                         | +8da
        add.l   d4,d5                           | +8dc
.L05be0c:
        move.w  -(a6),d7                        | +8de
        move.l  d7,(a4)                         | +8e0
        add.l   d4,d7                           | +8e2
        move.l  d5,(a4)                         | +8e4
        add.l   d4,d5                           | +8e6
.L05be16:
        move.w  -(a6),d7                        | +8e8
        move.l  d7,(a4)                         | +8ea
        add.l   d4,d7                           | +8ec
        move.l  d5,(a4)                         | +8ee
        add.l   d4,d5                           | +8f0
.L05be20:
        move.w  -(a6),d7                        | +8f2
        move.l  d7,(a4)                         | +8f4
        add.l   d4,d7                           | +8f6
        move.l  d5,(a4)                         | +8f8
        add.l   d4,d5                           | +8fa
.L05be2a:
        move.w  -(a6),d7                        | +8fc
        move.l  d7,(a4)                         | +8fe
        add.l   d4,d7                           | +900
        move.l  d5,(a4)                         | +902
        add.l   d4,d5                           | +904
.L05be34:
        move.w  -(a6),d7                        | +906
        move.l  d7,(a4)                         | +908
        add.l   d4,d7                           | +90a
        move.l  d5,(a4)                         | +90c
        add.l   d4,d5                           | +90e
.L05be3e:
        move.w  -(a6),d7                        | +910
        move.l  d7,(a4)                         | +912
        add.l   d4,d7                           | +914
        move.l  d5,(a4)                         | +916
        add.l   d4,d5                           | +918
.L05be48:
        move.w  -(a6),d7                        | +91a
        move.l  d7,(a4)                         | +91c
        add.l   d4,d7                           | +91e
        move.l  d5,(a4)                         | +920
        add.l   d4,d5                           | +922
        clr.w   d7                              | +924
        moveq   #1,d4                           | +926
        swap    d4                              | +928
        add.w   d6,d6                           | +92a
        jmp     .L05be5e(pc,d6.w)               | +92c
.L05be5e:
        move.l  d7,(a4)                         | +930
        add.l   d4,d7                           | +932
        move.l  d7,(a4)                         | +934
        add.l   d4,d7                           | +936
        move.l  d7,(a4)                         | +938
        add.l   d4,d7                           | +93a
        move.l  d7,(a4)                         | +93c
        add.l   d4,d7                           | +93e
        move.l  d7,(a4)                         | +940
        add.l   d4,d7                           | +942
        move.l  d7,(a4)                         | +944
        add.l   d4,d7                           | +946
        move.l  d7,(a4)                         | +948
        add.l   d4,d7                           | +94a
        move.l  d7,(a4)                         | +94c
        add.l   d4,d7                           | +94e
        move.l  d7,(a4)                         | +950
        add.l   d4,d7                           | +952
        move.l  d7,(a4)                         | +954
        add.l   d4,d7                           | +956
        move.l  d7,(a4)                         | +958
        add.l   d4,d7                           | +95a
        move.l  d7,(a4)                         | +95c
        add.l   d4,d7                           | +95e
        move.l  d7,(a4)                         | +960
        add.l   d4,d7                           | +962
        move.l  d7,(a4)                         | +964
        add.l   d4,d7                           | +966
        move.l  d7,(a4)                         | +968
        add.l   d4,d7                           | +96a
        move.l  d7,(a4)                         | +96c
        add.l   d4,d7                           | +96e
        move.l  d7,(a4)                         | +970
        add.l   d4,d7                           | +972
        move.l  d7,(a4)                         | +974
        add.l   d4,d7                           | +976
        move.l  d7,(a4)                         | +978
        add.l   d4,d7                           | +97a
        move.l  d7,(a4)                         | +97c
        add.l   d4,d7                           | +97e
        move.l  d7,(a4)                         | +980
        add.l   d4,d7                           | +982
        move.l  d7,(a4)                         | +984
        add.l   d4,d7                           | +986
        move.l  d7,(a4)                         | +988
        add.l   d4,d7                           | +98a
        move.l  d7,(a4)                         | +98c
        add.l   d4,d7                           | +98e
        move.l  d7,(a4)                         | +990
        add.l   d4,d7                           | +992
        move.l  d7,(a4)                         | +994
        add.l   d4,d7                           | +996
        move.l  d7,(a4)                         | +998
        add.l   d4,d7                           | +99a
        move.l  d7,(a4)                         | +99c
        add.l   d4,d7                           | +99e
        move.l  d7,(a4)                         | +9a0
        add.l   d4,d7                           | +9a2
        move.l  d7,(a4)                         | +9a4
        add.l   d4,d7                           | +9a6
        move.l  d7,(a4)                         | +9a8
        add.l   d4,d7                           | +9aa
        move.l  d7,(a4)                         | +9ac
        add.l   d4,d7                           | +9ae
        move.l  d7,(a4)                         | +9b0
        add.l   d4,d7                           | +9b2
        move.l  d7,(a4)                         | +9b4
        add.l   d4,d7                           | +9b6
        move.l  d7,(a4)                         | +9b8
        add.l   d4,d7                           | +9ba
        move.l  d7,(a4)                         | +9bc
        add.l   d4,d7                           | +9be
        move.l  d7,(a4)                         | +9c0
        add.l   d4,d7                           | +9c2
        move.l  d7,(a4)                         | +9c4
        add.l   d4,d7                           | +9c6
        move.l  d7,(a4)                         | +9c8
        add.l   d4,d7                           | +9ca
        move.l  d7,(a4)                         | +9cc
        add.l   d4,d7                           | +9ce
        move.l  d7,(a4)                         | +9d0
        add.l   d4,d7                           | +9d2
        move.l  d7,(a4)                         | +9d4
        add.l   d4,d7                           | +9d6
        move.l  d7,(a4)                         | +9d8
        add.l   d4,d7                           | +9da
        move.l  d7,(a4)                         | +9dc
        add.l   d4,d7                           | +9de
        move.l  d7,(a4)                         | +9e0
        add.l   d4,d7                           | +9e2
        move.l  d7,(a4)                         | +9e4
        add.l   d4,d7                           | +9e6
        move.l  d7,(a4)                         | +9e8
        add.l   d4,d7                           | +9ea
        move.l  d7,(a4)                         | +9ec
        add.l   d4,d7                           | +9ee
        move.l  d7,(a4)                         | +9f0
        add.l   d4,d7                           | +9f2
        move.l  d7,(a4)                         | +9f4
        add.l   d4,d7                           | +9f6
        move.l  d7,(a4)                         | +9f8
        add.l   d4,d7                           | +9fa
        move.l  d7,(a4)                         | +9fc
        add.l   d4,d7                           | +9fe
        move.l  d7,(a4)                         | +a00
        add.l   d4,d7                           | +a02
        move.l  d7,(a4)                         | +a04
        add.l   d4,d7                           | +a06
        move.l  d7,(a4)                         | +a08
        add.l   d4,d7                           | +a0a
        move.l  d7,(a4)                         | +a0c
        add.l   d4,d7                           | +a0e
        move.l  d7,(a4)                         | +a10
        add.l   d4,d7                           | +a12
        move.l  d7,(a4)                         | +a14
        add.l   d4,d7                           | +a16
        move.l  d7,(a4)                         | +a18
        add.l   d4,d7                           | +a1a
        move.l  d7,(a4)                         | +a1c
        add.l   d4,d7                           | +a1e
        move.l  d7,(a4)                         | +a20
        add.l   d4,d7                           | +a22
        move.l  d7,(a4)                         | +a24
        add.l   d4,d7                           | +a26
        move.w  #0x8401,d6                      | +a28
        add.w   d0,d6                           | +a2c
        swap    d6                              | +a2e
        move.w  0xa(a3),d6                      | +a30
        move.l  #0x2000000,d5                   | +a34
        move.l  d6,(a4)                         | +a3a
        sub.l   d5,d6                           | +a3c
        sub.l   d5,d6                           | +a3e
        move.w  0x6(a3),d6                      | +a40
        move.l  d6,(a4)                         | +a44
        rts                                     | +a46

| ----------------------------------------------------------------------------
|  SCB1_WriteTileColumnTerm_05bf76  @ $05BF76  (2656 B)
| ----------------------------------------------------------------------------
        .section .text.SCB1_WriteTileColumnTerm_05bf76, "ax", @progbits
        .global SCB1_WriteTileColumnTerm_05bf76
SCB1_WriteTileColumnTerm_05bf76:
        move.w  0x6(a3),d7                      | +000
        addq.b  #0x1,d7                         | +004
        sne.b   d6                              | +006
        andi.b  #0x4,d6                         | +008
        rol.w   #0x7,d7                         | +00c
        andi.w  #0x8,d7                         | +00e
        or.b    d6,d7                           | +012
        jmp     .L05bf8e(pc,d7.w)               | +014
.L05bf8e:
        bra.w   .L05bf9e                        | +018
        bra.w   .L05c3b6                        | +01c
        bra.w   .L05c1aa                        | +020
        bra.w   .L05c6c6                        | +024
.L05bf9e:
        move.w  #0x40,d7                        | +028
        add.w   d1,d7                           | +02c
        move.w  d7,d5                           | +02e
        addq.w  #0x1,d5                         | +030
        swap    d5                              | +032
        move.w  0x4(a3),d5                      | +034
        swap    d7                              | +038
        moveq   #2,d4                           | +03a
        swap    d4                              | +03c
        movea.l (a3),a6                         | +03e
        move.w  0xa(a3),d6                      | +040
        subq.b  #0x1,d6                         | +044
        andi.w  #0x1f,d6                        | +046
        add.w   d6,d6                           | +04a
        add.w   d6,d6                           | +04c
        jmp     .L05bfc8(pc,d6.w)               | +04e
.L05bfc8:
        bra.w   .L05c17a                        | +052
        bra.w   .L05c170                        | +056
        bra.w   .L05c166                        | +05a
        bra.w   .L05c15c                        | +05e
        bra.w   .L05c152                        | +062
        bra.w   .L05c148                        | +066
        bra.w   .L05c13e                        | +06a
        bra.w   .L05c134                        | +06e
        bra.w   .L05c12a                        | +072
        bra.w   .L05c120                        | +076
        bra.w   .L05c116                        | +07a
        bra.w   .L05c10c                        | +07e
        bra.w   .L05c102                        | +082
        bra.w   .L05c0f8                        | +086
        bra.w   .L05c0ee                        | +08a
        bra.w   .L05c0e4                        | +08e
        bra.w   .L05c0da                        | +092
        bra.w   .L05c0d0                        | +096
        bra.w   .L05c0c6                        | +09a
        bra.w   .L05c0bc                        | +09e
        bra.w   .L05c0b2                        | +0a2
        bra.w   .L05c0a8                        | +0a6
        bra.w   .L05c09e                        | +0aa
        bra.w   .L05c094                        | +0ae
        bra.w   .L05c08a                        | +0b2
        bra.w   .L05c080                        | +0b6
        bra.w   .L05c076                        | +0ba
        bra.w   .L05c06c                        | +0be
        bra.w   .L05c062                        | +0c2
        bra.w   .L05c058                        | +0c6
        bra.w   .L05c04e                        | +0ca
        move.w  (a6)+,d7                        | +0ce
        move.l  d7,(a4)                         | +0d0
        add.l   d4,d7                           | +0d2
        move.l  d5,(a4)                         | +0d4
        add.l   d4,d5                           | +0d6
.L05c04e:
        move.w  (a6)+,d7                        | +0d8
        move.l  d7,(a4)                         | +0da
        add.l   d4,d7                           | +0dc
        move.l  d5,(a4)                         | +0de
        add.l   d4,d5                           | +0e0
.L05c058:
        move.w  (a6)+,d7                        | +0e2
        move.l  d7,(a4)                         | +0e4
        add.l   d4,d7                           | +0e6
        move.l  d5,(a4)                         | +0e8
        add.l   d4,d5                           | +0ea
.L05c062:
        move.w  (a6)+,d7                        | +0ec
        move.l  d7,(a4)                         | +0ee
        add.l   d4,d7                           | +0f0
        move.l  d5,(a4)                         | +0f2
        add.l   d4,d5                           | +0f4
.L05c06c:
        move.w  (a6)+,d7                        | +0f6
        move.l  d7,(a4)                         | +0f8
        add.l   d4,d7                           | +0fa
        move.l  d5,(a4)                         | +0fc
        add.l   d4,d5                           | +0fe
.L05c076:
        move.w  (a6)+,d7                        | +100
        move.l  d7,(a4)                         | +102
        add.l   d4,d7                           | +104
        move.l  d5,(a4)                         | +106
        add.l   d4,d5                           | +108
.L05c080:
        move.w  (a6)+,d7                        | +10a
        move.l  d7,(a4)                         | +10c
        add.l   d4,d7                           | +10e
        move.l  d5,(a4)                         | +110
        add.l   d4,d5                           | +112
.L05c08a:
        move.w  (a6)+,d7                        | +114
        move.l  d7,(a4)                         | +116
        add.l   d4,d7                           | +118
        move.l  d5,(a4)                         | +11a
        add.l   d4,d5                           | +11c
.L05c094:
        move.w  (a6)+,d7                        | +11e
        move.l  d7,(a4)                         | +120
        add.l   d4,d7                           | +122
        move.l  d5,(a4)                         | +124
        add.l   d4,d5                           | +126
.L05c09e:
        move.w  (a6)+,d7                        | +128
        move.l  d7,(a4)                         | +12a
        add.l   d4,d7                           | +12c
        move.l  d5,(a4)                         | +12e
        add.l   d4,d5                           | +130
.L05c0a8:
        move.w  (a6)+,d7                        | +132
        move.l  d7,(a4)                         | +134
        add.l   d4,d7                           | +136
        move.l  d5,(a4)                         | +138
        add.l   d4,d5                           | +13a
.L05c0b2:
        move.w  (a6)+,d7                        | +13c
        move.l  d7,(a4)                         | +13e
        add.l   d4,d7                           | +140
        move.l  d5,(a4)                         | +142
        add.l   d4,d5                           | +144
.L05c0bc:
        move.w  (a6)+,d7                        | +146
        move.l  d7,(a4)                         | +148
        add.l   d4,d7                           | +14a
        move.l  d5,(a4)                         | +14c
        add.l   d4,d5                           | +14e
.L05c0c6:
        move.w  (a6)+,d7                        | +150
        move.l  d7,(a4)                         | +152
        add.l   d4,d7                           | +154
        move.l  d5,(a4)                         | +156
        add.l   d4,d5                           | +158
.L05c0d0:
        move.w  (a6)+,d7                        | +15a
        move.l  d7,(a4)                         | +15c
        add.l   d4,d7                           | +15e
        move.l  d5,(a4)                         | +160
        add.l   d4,d5                           | +162
.L05c0da:
        move.w  (a6)+,d7                        | +164
        move.l  d7,(a4)                         | +166
        add.l   d4,d7                           | +168
        move.l  d5,(a4)                         | +16a
        add.l   d4,d5                           | +16c
.L05c0e4:
        move.w  (a6)+,d7                        | +16e
        move.l  d7,(a4)                         | +170
        add.l   d4,d7                           | +172
        move.l  d5,(a4)                         | +174
        add.l   d4,d5                           | +176
.L05c0ee:
        move.w  (a6)+,d7                        | +178
        move.l  d7,(a4)                         | +17a
        add.l   d4,d7                           | +17c
        move.l  d5,(a4)                         | +17e
        add.l   d4,d5                           | +180
.L05c0f8:
        move.w  (a6)+,d7                        | +182
        move.l  d7,(a4)                         | +184
        add.l   d4,d7                           | +186
        move.l  d5,(a4)                         | +188
        add.l   d4,d5                           | +18a
.L05c102:
        move.w  (a6)+,d7                        | +18c
        move.l  d7,(a4)                         | +18e
        add.l   d4,d7                           | +190
        move.l  d5,(a4)                         | +192
        add.l   d4,d5                           | +194
.L05c10c:
        move.w  (a6)+,d7                        | +196
        move.l  d7,(a4)                         | +198
        add.l   d4,d7                           | +19a
        move.l  d5,(a4)                         | +19c
        add.l   d4,d5                           | +19e
.L05c116:
        move.w  (a6)+,d7                        | +1a0
        move.l  d7,(a4)                         | +1a2
        add.l   d4,d7                           | +1a4
        move.l  d5,(a4)                         | +1a6
        add.l   d4,d5                           | +1a8
.L05c120:
        move.w  (a6)+,d7                        | +1aa
        move.l  d7,(a4)                         | +1ac
        add.l   d4,d7                           | +1ae
        move.l  d5,(a4)                         | +1b0
        add.l   d4,d5                           | +1b2
.L05c12a:
        move.w  (a6)+,d7                        | +1b4
        move.l  d7,(a4)                         | +1b6
        add.l   d4,d7                           | +1b8
        move.l  d5,(a4)                         | +1ba
        add.l   d4,d5                           | +1bc
.L05c134:
        move.w  (a6)+,d7                        | +1be
        move.l  d7,(a4)                         | +1c0
        add.l   d4,d7                           | +1c2
        move.l  d5,(a4)                         | +1c4
        add.l   d4,d5                           | +1c6
.L05c13e:
        move.w  (a6)+,d7                        | +1c8
        move.l  d7,(a4)                         | +1ca
        add.l   d4,d7                           | +1cc
        move.l  d5,(a4)                         | +1ce
        add.l   d4,d5                           | +1d0
.L05c148:
        move.w  (a6)+,d7                        | +1d2
        move.l  d7,(a4)                         | +1d4
        add.l   d4,d7                           | +1d6
        move.l  d5,(a4)                         | +1d8
        add.l   d4,d5                           | +1da
.L05c152:
        move.w  (a6)+,d7                        | +1dc
        move.l  d7,(a4)                         | +1de
        add.l   d4,d7                           | +1e0
        move.l  d5,(a4)                         | +1e2
        add.l   d4,d5                           | +1e4
.L05c15c:
        move.w  (a6)+,d7                        | +1e6
        move.l  d7,(a4)                         | +1e8
        add.l   d4,d7                           | +1ea
        move.l  d5,(a4)                         | +1ec
        add.l   d4,d5                           | +1ee
.L05c166:
        move.w  (a6)+,d7                        | +1f0
        move.l  d7,(a4)                         | +1f2
        add.l   d4,d7                           | +1f4
        move.l  d5,(a4)                         | +1f6
        add.l   d4,d5                           | +1f8
.L05c170:
        move.w  (a6)+,d7                        | +1fa
        move.l  d7,(a4)                         | +1fc
        add.l   d4,d7                           | +1fe
        move.l  d5,(a4)                         | +200
        add.l   d4,d5                           | +202
.L05c17a:
        move.w  (a6)+,d7                        | +204
        move.l  d7,(a4)                         | +206
        add.l   d4,d7                           | +208
        move.l  d5,(a4)                         | +20a
        add.l   d4,d5                           | +20c
        move.w  #0x8401,d6                      | +20e
        add.w   d0,d6                           | +212
        swap    d6                              | +214
        move.w  0xa(a3),d6                      | +216
        move.l  #0x2000000,d5                   | +21a
        move.l  d6,(a4)                         | +220
        sub.l   d5,d6                           | +222
        move.w  0x8(a3),d6                      | +224
        move.l  d6,(a4)                         | +228
        sub.l   d5,d6                           | +22a
        move.w  0x6(a3),d6                      | +22c
        move.l  d6,(a4)                         | +230
        rts                                     | +232
.L05c1aa:
        move.w  #0x40,d7                        | +234
        add.w   d1,d7                           | +238
        move.w  d7,d5                           | +23a
        addq.w  #0x1,d5                         | +23c
        swap    d5                              | +23e
        move.w  0x4(a3),d5                      | +240
        swap    d7                              | +244
        moveq   #2,d4                           | +246
        swap    d4                              | +248
        movea.l (a3),a6                         | +24a
        move.w  0xa(a3),d6                      | +24c
        subq.b  #0x1,d6                         | +250
        andi.w  #0x1f,d6                        | +252
        add.w   d6,d6                           | +256
        add.w   d6,d6                           | +258
        jmp     .L05c1d4(pc,d6.w)               | +25a
.L05c1d4:
        bra.w   .L05c386                        | +25e
        bra.w   .L05c37c                        | +262
        bra.w   .L05c372                        | +266
        bra.w   .L05c368                        | +26a
        bra.w   .L05c35e                        | +26e
        bra.w   .L05c354                        | +272
        bra.w   .L05c34a                        | +276
        bra.w   .L05c340                        | +27a
        bra.w   .L05c336                        | +27e
        bra.w   .L05c32c                        | +282
        bra.w   .L05c322                        | +286
        bra.w   .L05c318                        | +28a
        bra.w   .L05c30e                        | +28e
        bra.w   .L05c304                        | +292
        bra.w   .L05c2fa                        | +296
        bra.w   .L05c2f0                        | +29a
        bra.w   .L05c2e6                        | +29e
        bra.w   .L05c2dc                        | +2a2
        bra.w   .L05c2d2                        | +2a6
        bra.w   .L05c2c8                        | +2aa
        bra.w   .L05c2be                        | +2ae
        bra.w   .L05c2b4                        | +2b2
        bra.w   .L05c2aa                        | +2b6
        bra.w   .L05c2a0                        | +2ba
        bra.w   .L05c296                        | +2be
        bra.w   .L05c28c                        | +2c2
        bra.w   .L05c282                        | +2c6
        bra.w   .L05c278                        | +2ca
        bra.w   .L05c26e                        | +2ce
        bra.w   .L05c264                        | +2d2
        bra.w   .L05c25a                        | +2d6
        move.w  -(a6),d7                        | +2da
        move.l  d7,(a4)                         | +2dc
        add.l   d4,d7                           | +2de
        move.l  d5,(a4)                         | +2e0
        add.l   d4,d5                           | +2e2
.L05c25a:
        move.w  -(a6),d7                        | +2e4
        move.l  d7,(a4)                         | +2e6
        add.l   d4,d7                           | +2e8
        move.l  d5,(a4)                         | +2ea
        add.l   d4,d5                           | +2ec
.L05c264:
        move.w  -(a6),d7                        | +2ee
        move.l  d7,(a4)                         | +2f0
        add.l   d4,d7                           | +2f2
        move.l  d5,(a4)                         | +2f4
        add.l   d4,d5                           | +2f6
.L05c26e:
        move.w  -(a6),d7                        | +2f8
        move.l  d7,(a4)                         | +2fa
        add.l   d4,d7                           | +2fc
        move.l  d5,(a4)                         | +2fe
        add.l   d4,d5                           | +300
.L05c278:
        move.w  -(a6),d7                        | +302
        move.l  d7,(a4)                         | +304
        add.l   d4,d7                           | +306
        move.l  d5,(a4)                         | +308
        add.l   d4,d5                           | +30a
.L05c282:
        move.w  -(a6),d7                        | +30c
        move.l  d7,(a4)                         | +30e
        add.l   d4,d7                           | +310
        move.l  d5,(a4)                         | +312
        add.l   d4,d5                           | +314
.L05c28c:
        move.w  -(a6),d7                        | +316
        move.l  d7,(a4)                         | +318
        add.l   d4,d7                           | +31a
        move.l  d5,(a4)                         | +31c
        add.l   d4,d5                           | +31e
.L05c296:
        move.w  -(a6),d7                        | +320
        move.l  d7,(a4)                         | +322
        add.l   d4,d7                           | +324
        move.l  d5,(a4)                         | +326
        add.l   d4,d5                           | +328
.L05c2a0:
        move.w  -(a6),d7                        | +32a
        move.l  d7,(a4)                         | +32c
        add.l   d4,d7                           | +32e
        move.l  d5,(a4)                         | +330
        add.l   d4,d5                           | +332
.L05c2aa:
        move.w  -(a6),d7                        | +334
        move.l  d7,(a4)                         | +336
        add.l   d4,d7                           | +338
        move.l  d5,(a4)                         | +33a
        add.l   d4,d5                           | +33c
.L05c2b4:
        move.w  -(a6),d7                        | +33e
        move.l  d7,(a4)                         | +340
        add.l   d4,d7                           | +342
        move.l  d5,(a4)                         | +344
        add.l   d4,d5                           | +346
.L05c2be:
        move.w  -(a6),d7                        | +348
        move.l  d7,(a4)                         | +34a
        add.l   d4,d7                           | +34c
        move.l  d5,(a4)                         | +34e
        add.l   d4,d5                           | +350
.L05c2c8:
        move.w  -(a6),d7                        | +352
        move.l  d7,(a4)                         | +354
        add.l   d4,d7                           | +356
        move.l  d5,(a4)                         | +358
        add.l   d4,d5                           | +35a
.L05c2d2:
        move.w  -(a6),d7                        | +35c
        move.l  d7,(a4)                         | +35e
        add.l   d4,d7                           | +360
        move.l  d5,(a4)                         | +362
        add.l   d4,d5                           | +364
.L05c2dc:
        move.w  -(a6),d7                        | +366
        move.l  d7,(a4)                         | +368
        add.l   d4,d7                           | +36a
        move.l  d5,(a4)                         | +36c
        add.l   d4,d5                           | +36e
.L05c2e6:
        move.w  -(a6),d7                        | +370
        move.l  d7,(a4)                         | +372
        add.l   d4,d7                           | +374
        move.l  d5,(a4)                         | +376
        add.l   d4,d5                           | +378
.L05c2f0:
        move.w  -(a6),d7                        | +37a
        move.l  d7,(a4)                         | +37c
        add.l   d4,d7                           | +37e
        move.l  d5,(a4)                         | +380
        add.l   d4,d5                           | +382
.L05c2fa:
        move.w  -(a6),d7                        | +384
        move.l  d7,(a4)                         | +386
        add.l   d4,d7                           | +388
        move.l  d5,(a4)                         | +38a
        add.l   d4,d5                           | +38c
.L05c304:
        move.w  -(a6),d7                        | +38e
        move.l  d7,(a4)                         | +390
        add.l   d4,d7                           | +392
        move.l  d5,(a4)                         | +394
        add.l   d4,d5                           | +396
.L05c30e:
        move.w  -(a6),d7                        | +398
        move.l  d7,(a4)                         | +39a
        add.l   d4,d7                           | +39c
        move.l  d5,(a4)                         | +39e
        add.l   d4,d5                           | +3a0
.L05c318:
        move.w  -(a6),d7                        | +3a2
        move.l  d7,(a4)                         | +3a4
        add.l   d4,d7                           | +3a6
        move.l  d5,(a4)                         | +3a8
        add.l   d4,d5                           | +3aa
.L05c322:
        move.w  -(a6),d7                        | +3ac
        move.l  d7,(a4)                         | +3ae
        add.l   d4,d7                           | +3b0
        move.l  d5,(a4)                         | +3b2
        add.l   d4,d5                           | +3b4
.L05c32c:
        move.w  -(a6),d7                        | +3b6
        move.l  d7,(a4)                         | +3b8
        add.l   d4,d7                           | +3ba
        move.l  d5,(a4)                         | +3bc
        add.l   d4,d5                           | +3be
.L05c336:
        move.w  -(a6),d7                        | +3c0
        move.l  d7,(a4)                         | +3c2
        add.l   d4,d7                           | +3c4
        move.l  d5,(a4)                         | +3c6
        add.l   d4,d5                           | +3c8
.L05c340:
        move.w  -(a6),d7                        | +3ca
        move.l  d7,(a4)                         | +3cc
        add.l   d4,d7                           | +3ce
        move.l  d5,(a4)                         | +3d0
        add.l   d4,d5                           | +3d2
.L05c34a:
        move.w  -(a6),d7                        | +3d4
        move.l  d7,(a4)                         | +3d6
        add.l   d4,d7                           | +3d8
        move.l  d5,(a4)                         | +3da
        add.l   d4,d5                           | +3dc
.L05c354:
        move.w  -(a6),d7                        | +3de
        move.l  d7,(a4)                         | +3e0
        add.l   d4,d7                           | +3e2
        move.l  d5,(a4)                         | +3e4
        add.l   d4,d5                           | +3e6
.L05c35e:
        move.w  -(a6),d7                        | +3e8
        move.l  d7,(a4)                         | +3ea
        add.l   d4,d7                           | +3ec
        move.l  d5,(a4)                         | +3ee
        add.l   d4,d5                           | +3f0
.L05c368:
        move.w  -(a6),d7                        | +3f2
        move.l  d7,(a4)                         | +3f4
        add.l   d4,d7                           | +3f6
        move.l  d5,(a4)                         | +3f8
        add.l   d4,d5                           | +3fa
.L05c372:
        move.w  -(a6),d7                        | +3fc
        move.l  d7,(a4)                         | +3fe
        add.l   d4,d7                           | +400
        move.l  d5,(a4)                         | +402
        add.l   d4,d5                           | +404
.L05c37c:
        move.w  -(a6),d7                        | +406
        move.l  d7,(a4)                         | +408
        add.l   d4,d7                           | +40a
        move.l  d5,(a4)                         | +40c
        add.l   d4,d5                           | +40e
.L05c386:
        move.w  -(a6),d7                        | +410
        move.l  d7,(a4)                         | +412
        add.l   d4,d7                           | +414
        move.l  d5,(a4)                         | +416
        add.l   d4,d5                           | +418
        move.w  #0x8401,d6                      | +41a
        add.w   d0,d6                           | +41e
        swap    d6                              | +420
        move.w  0xa(a3),d6                      | +422
        move.l  #0x2000000,d5                   | +426
        move.l  d6,(a4)                         | +42c
        sub.l   d5,d6                           | +42e
        move.w  0x8(a3),d6                      | +430
        move.l  d6,(a4)                         | +434
        sub.l   d5,d6                           | +436
        move.w  0x6(a3),d6                      | +438
        move.l  d6,(a4)                         | +43c
        rts                                     | +43e
.L05c3b6:
        move.w  #0x40,d7                        | +440
        add.w   d1,d7                           | +444
        move.w  d7,d5                           | +446
        addq.w  #0x1,d5                         | +448
        swap    d5                              | +44a
        move.w  0x4(a3),d5                      | +44c
        swap    d7                              | +450
        moveq   #2,d4                           | +452
        swap    d4                              | +454
        movea.l (a3),a6                         | +456
        move.w  0xa(a3),d6                      | +458
        subq.b  #0x1,d6                         | +45c
        andi.w  #0x1f,d6                        | +45e
        add.w   d6,d6                           | +462
        add.w   d6,d6                           | +464
        jmp     .L05c3e0(pc,d6.w)               | +466
.L05c3e0:
        bra.w   .L05c592                        | +46a
        bra.w   .L05c588                        | +46e
        bra.w   .L05c57e                        | +472
        bra.w   .L05c574                        | +476
        bra.w   .L05c56a                        | +47a
        bra.w   .L05c560                        | +47e
        bra.w   .L05c556                        | +482
        bra.w   .L05c54c                        | +486
        bra.w   .L05c542                        | +48a
        bra.w   .L05c538                        | +48e
        bra.w   .L05c52e                        | +492
        bra.w   .L05c524                        | +496
        bra.w   .L05c51a                        | +49a
        bra.w   .L05c510                        | +49e
        bra.w   .L05c506                        | +4a2
        bra.w   .L05c4fc                        | +4a6
        bra.w   .L05c4f2                        | +4aa
        bra.w   .L05c4e8                        | +4ae
        bra.w   .L05c4de                        | +4b2
        bra.w   .L05c4d4                        | +4b6
        bra.w   .L05c4ca                        | +4ba
        bra.w   .L05c4c0                        | +4be
        bra.w   .L05c4b6                        | +4c2
        bra.w   .L05c4ac                        | +4c6
        bra.w   .L05c4a2                        | +4ca
        bra.w   .L05c498                        | +4ce
        bra.w   .L05c48e                        | +4d2
        bra.w   .L05c484                        | +4d6
        bra.w   .L05c47a                        | +4da
        bra.w   .L05c470                        | +4de
        bra.w   .L05c466                        | +4e2
        move.w  (a6)+,d7                        | +4e6
        move.l  d7,(a4)                         | +4e8
        add.l   d4,d7                           | +4ea
        move.l  d5,(a4)                         | +4ec
        add.l   d4,d5                           | +4ee
.L05c466:
        move.w  (a6)+,d7                        | +4f0
        move.l  d7,(a4)                         | +4f2
        add.l   d4,d7                           | +4f4
        move.l  d5,(a4)                         | +4f6
        add.l   d4,d5                           | +4f8
.L05c470:
        move.w  (a6)+,d7                        | +4fa
        move.l  d7,(a4)                         | +4fc
        add.l   d4,d7                           | +4fe
        move.l  d5,(a4)                         | +500
        add.l   d4,d5                           | +502
.L05c47a:
        move.w  (a6)+,d7                        | +504
        move.l  d7,(a4)                         | +506
        add.l   d4,d7                           | +508
        move.l  d5,(a4)                         | +50a
        add.l   d4,d5                           | +50c
.L05c484:
        move.w  (a6)+,d7                        | +50e
        move.l  d7,(a4)                         | +510
        add.l   d4,d7                           | +512
        move.l  d5,(a4)                         | +514
        add.l   d4,d5                           | +516
.L05c48e:
        move.w  (a6)+,d7                        | +518
        move.l  d7,(a4)                         | +51a
        add.l   d4,d7                           | +51c
        move.l  d5,(a4)                         | +51e
        add.l   d4,d5                           | +520
.L05c498:
        move.w  (a6)+,d7                        | +522
        move.l  d7,(a4)                         | +524
        add.l   d4,d7                           | +526
        move.l  d5,(a4)                         | +528
        add.l   d4,d5                           | +52a
.L05c4a2:
        move.w  (a6)+,d7                        | +52c
        move.l  d7,(a4)                         | +52e
        add.l   d4,d7                           | +530
        move.l  d5,(a4)                         | +532
        add.l   d4,d5                           | +534
.L05c4ac:
        move.w  (a6)+,d7                        | +536
        move.l  d7,(a4)                         | +538
        add.l   d4,d7                           | +53a
        move.l  d5,(a4)                         | +53c
        add.l   d4,d5                           | +53e
.L05c4b6:
        move.w  (a6)+,d7                        | +540
        move.l  d7,(a4)                         | +542
        add.l   d4,d7                           | +544
        move.l  d5,(a4)                         | +546
        add.l   d4,d5                           | +548
.L05c4c0:
        move.w  (a6)+,d7                        | +54a
        move.l  d7,(a4)                         | +54c
        add.l   d4,d7                           | +54e
        move.l  d5,(a4)                         | +550
        add.l   d4,d5                           | +552
.L05c4ca:
        move.w  (a6)+,d7                        | +554
        move.l  d7,(a4)                         | +556
        add.l   d4,d7                           | +558
        move.l  d5,(a4)                         | +55a
        add.l   d4,d5                           | +55c
.L05c4d4:
        move.w  (a6)+,d7                        | +55e
        move.l  d7,(a4)                         | +560
        add.l   d4,d7                           | +562
        move.l  d5,(a4)                         | +564
        add.l   d4,d5                           | +566
.L05c4de:
        move.w  (a6)+,d7                        | +568
        move.l  d7,(a4)                         | +56a
        add.l   d4,d7                           | +56c
        move.l  d5,(a4)                         | +56e
        add.l   d4,d5                           | +570
.L05c4e8:
        move.w  (a6)+,d7                        | +572
        move.l  d7,(a4)                         | +574
        add.l   d4,d7                           | +576
        move.l  d5,(a4)                         | +578
        add.l   d4,d5                           | +57a
.L05c4f2:
        move.w  (a6)+,d7                        | +57c
        move.l  d7,(a4)                         | +57e
        add.l   d4,d7                           | +580
        move.l  d5,(a4)                         | +582
        add.l   d4,d5                           | +584
.L05c4fc:
        move.w  (a6)+,d7                        | +586
        move.l  d7,(a4)                         | +588
        add.l   d4,d7                           | +58a
        move.l  d5,(a4)                         | +58c
        add.l   d4,d5                           | +58e
.L05c506:
        move.w  (a6)+,d7                        | +590
        move.l  d7,(a4)                         | +592
        add.l   d4,d7                           | +594
        move.l  d5,(a4)                         | +596
        add.l   d4,d5                           | +598
.L05c510:
        move.w  (a6)+,d7                        | +59a
        move.l  d7,(a4)                         | +59c
        add.l   d4,d7                           | +59e
        move.l  d5,(a4)                         | +5a0
        add.l   d4,d5                           | +5a2
.L05c51a:
        move.w  (a6)+,d7                        | +5a4
        move.l  d7,(a4)                         | +5a6
        add.l   d4,d7                           | +5a8
        move.l  d5,(a4)                         | +5aa
        add.l   d4,d5                           | +5ac
.L05c524:
        move.w  (a6)+,d7                        | +5ae
        move.l  d7,(a4)                         | +5b0
        add.l   d4,d7                           | +5b2
        move.l  d5,(a4)                         | +5b4
        add.l   d4,d5                           | +5b6
.L05c52e:
        move.w  (a6)+,d7                        | +5b8
        move.l  d7,(a4)                         | +5ba
        add.l   d4,d7                           | +5bc
        move.l  d5,(a4)                         | +5be
        add.l   d4,d5                           | +5c0
.L05c538:
        move.w  (a6)+,d7                        | +5c2
        move.l  d7,(a4)                         | +5c4
        add.l   d4,d7                           | +5c6
        move.l  d5,(a4)                         | +5c8
        add.l   d4,d5                           | +5ca
.L05c542:
        move.w  (a6)+,d7                        | +5cc
        move.l  d7,(a4)                         | +5ce
        add.l   d4,d7                           | +5d0
        move.l  d5,(a4)                         | +5d2
        add.l   d4,d5                           | +5d4
.L05c54c:
        move.w  (a6)+,d7                        | +5d6
        move.l  d7,(a4)                         | +5d8
        add.l   d4,d7                           | +5da
        move.l  d5,(a4)                         | +5dc
        add.l   d4,d5                           | +5de
.L05c556:
        move.w  (a6)+,d7                        | +5e0
        move.l  d7,(a4)                         | +5e2
        add.l   d4,d7                           | +5e4
        move.l  d5,(a4)                         | +5e6
        add.l   d4,d5                           | +5e8
.L05c560:
        move.w  (a6)+,d7                        | +5ea
        move.l  d7,(a4)                         | +5ec
        add.l   d4,d7                           | +5ee
        move.l  d5,(a4)                         | +5f0
        add.l   d4,d5                           | +5f2
.L05c56a:
        move.w  (a6)+,d7                        | +5f4
        move.l  d7,(a4)                         | +5f6
        add.l   d4,d7                           | +5f8
        move.l  d5,(a4)                         | +5fa
        add.l   d4,d5                           | +5fc
.L05c574:
        move.w  (a6)+,d7                        | +5fe
        move.l  d7,(a4)                         | +600
        add.l   d4,d7                           | +602
        move.l  d5,(a4)                         | +604
        add.l   d4,d5                           | +606
.L05c57e:
        move.w  (a6)+,d7                        | +608
        move.l  d7,(a4)                         | +60a
        add.l   d4,d7                           | +60c
        move.l  d5,(a4)                         | +60e
        add.l   d4,d5                           | +610
.L05c588:
        move.w  (a6)+,d7                        | +612
        move.l  d7,(a4)                         | +614
        add.l   d4,d7                           | +616
        move.l  d5,(a4)                         | +618
        add.l   d4,d5                           | +61a
.L05c592:
        move.w  (a6)+,d7                        | +61c
        move.l  d7,(a4)                         | +61e
        add.l   d4,d7                           | +620
        move.l  d5,(a4)                         | +622
        add.l   d4,d5                           | +624
        clr.w   d7                              | +626
        moveq   #1,d4                           | +628
        swap    d4                              | +62a
        add.w   d6,d6                           | +62c
        jmp     .L05c5a8(pc,d6.w)               | +62e
.L05c5a8:
        move.l  d7,(a4)                         | +632
        add.l   d4,d7                           | +634
        move.l  d7,(a4)                         | +636
        add.l   d4,d7                           | +638
        move.l  d7,(a4)                         | +63a
        add.l   d4,d7                           | +63c
        move.l  d7,(a4)                         | +63e
        add.l   d4,d7                           | +640
        move.l  d7,(a4)                         | +642
        add.l   d4,d7                           | +644
        move.l  d7,(a4)                         | +646
        add.l   d4,d7                           | +648
        move.l  d7,(a4)                         | +64a
        add.l   d4,d7                           | +64c
        move.l  d7,(a4)                         | +64e
        add.l   d4,d7                           | +650
        move.l  d7,(a4)                         | +652
        add.l   d4,d7                           | +654
        move.l  d7,(a4)                         | +656
        add.l   d4,d7                           | +658
        move.l  d7,(a4)                         | +65a
        add.l   d4,d7                           | +65c
        move.l  d7,(a4)                         | +65e
        add.l   d4,d7                           | +660
        move.l  d7,(a4)                         | +662
        add.l   d4,d7                           | +664
        move.l  d7,(a4)                         | +666
        add.l   d4,d7                           | +668
        move.l  d7,(a4)                         | +66a
        add.l   d4,d7                           | +66c
        move.l  d7,(a4)                         | +66e
        add.l   d4,d7                           | +670
        move.l  d7,(a4)                         | +672
        add.l   d4,d7                           | +674
        move.l  d7,(a4)                         | +676
        add.l   d4,d7                           | +678
        move.l  d7,(a4)                         | +67a
        add.l   d4,d7                           | +67c
        move.l  d7,(a4)                         | +67e
        add.l   d4,d7                           | +680
        move.l  d7,(a4)                         | +682
        add.l   d4,d7                           | +684
        move.l  d7,(a4)                         | +686
        add.l   d4,d7                           | +688
        move.l  d7,(a4)                         | +68a
        add.l   d4,d7                           | +68c
        move.l  d7,(a4)                         | +68e
        add.l   d4,d7                           | +690
        move.l  d7,(a4)                         | +692
        add.l   d4,d7                           | +694
        move.l  d7,(a4)                         | +696
        add.l   d4,d7                           | +698
        move.l  d7,(a4)                         | +69a
        add.l   d4,d7                           | +69c
        move.l  d7,(a4)                         | +69e
        add.l   d4,d7                           | +6a0
        move.l  d7,(a4)                         | +6a2
        add.l   d4,d7                           | +6a4
        move.l  d7,(a4)                         | +6a6
        add.l   d4,d7                           | +6a8
        move.l  d7,(a4)                         | +6aa
        add.l   d4,d7                           | +6ac
        move.l  d7,(a4)                         | +6ae
        add.l   d4,d7                           | +6b0
        move.l  d7,(a4)                         | +6b2
        add.l   d4,d7                           | +6b4
        move.l  d7,(a4)                         | +6b6
        add.l   d4,d7                           | +6b8
        move.l  d7,(a4)                         | +6ba
        add.l   d4,d7                           | +6bc
        move.l  d7,(a4)                         | +6be
        add.l   d4,d7                           | +6c0
        move.l  d7,(a4)                         | +6c2
        add.l   d4,d7                           | +6c4
        move.l  d7,(a4)                         | +6c6
        add.l   d4,d7                           | +6c8
        move.l  d7,(a4)                         | +6ca
        add.l   d4,d7                           | +6cc
        move.l  d7,(a4)                         | +6ce
        add.l   d4,d7                           | +6d0
        move.l  d7,(a4)                         | +6d2
        add.l   d4,d7                           | +6d4
        move.l  d7,(a4)                         | +6d6
        add.l   d4,d7                           | +6d8
        move.l  d7,(a4)                         | +6da
        add.l   d4,d7                           | +6dc
        move.l  d7,(a4)                         | +6de
        add.l   d4,d7                           | +6e0
        move.l  d7,(a4)                         | +6e2
        add.l   d4,d7                           | +6e4
        move.l  d7,(a4)                         | +6e6
        add.l   d4,d7                           | +6e8
        move.l  d7,(a4)                         | +6ea
        add.l   d4,d7                           | +6ec
        move.l  d7,(a4)                         | +6ee
        add.l   d4,d7                           | +6f0
        move.l  d7,(a4)                         | +6f2
        add.l   d4,d7                           | +6f4
        move.l  d7,(a4)                         | +6f6
        add.l   d4,d7                           | +6f8
        move.l  d7,(a4)                         | +6fa
        add.l   d4,d7                           | +6fc
        move.l  d7,(a4)                         | +6fe
        add.l   d4,d7                           | +700
        move.l  d7,(a4)                         | +702
        add.l   d4,d7                           | +704
        move.l  d7,(a4)                         | +706
        add.l   d4,d7                           | +708
        move.l  d7,(a4)                         | +70a
        add.l   d4,d7                           | +70c
        move.l  d7,(a4)                         | +70e
        add.l   d4,d7                           | +710
        move.l  d7,(a4)                         | +712
        add.l   d4,d7                           | +714
        move.l  d7,(a4)                         | +716
        add.l   d4,d7                           | +718
        move.l  d7,(a4)                         | +71a
        add.l   d4,d7                           | +71c
        move.l  d7,(a4)                         | +71e
        add.l   d4,d7                           | +720
        move.l  d7,(a4)                         | +722
        add.l   d4,d7                           | +724
        move.l  d7,(a4)                         | +726
        add.l   d4,d7                           | +728
        move.w  #0x8401,d6                      | +72a
        add.w   d0,d6                           | +72e
        swap    d6                              | +730
        move.w  0xa(a3),d6                      | +732
        move.l  #0x2000000,d5                   | +736
        move.l  d6,(a4)                         | +73c
        sub.l   d5,d6                           | +73e
        move.w  0x8(a3),d6                      | +740
        move.l  d6,(a4)                         | +744
        sub.l   d5,d6                           | +746
        move.w  0x6(a3),d6                      | +748
        move.l  d6,(a4)                         | +74c
        rts                                     | +74e
.L05c6c6:
        move.w  #0x40,d7                        | +750
        add.w   d1,d7                           | +754
        move.w  d7,d5                           | +756
        addq.w  #0x1,d5                         | +758
        swap    d5                              | +75a
        move.w  0x4(a3),d5                      | +75c
        swap    d7                              | +760
        moveq   #2,d4                           | +762
        swap    d4                              | +764
        movea.l (a3),a6                         | +766
        move.w  0xa(a3),d6                      | +768
        subq.b  #0x1,d6                         | +76c
        andi.w  #0x1f,d6                        | +76e
        add.w   d6,d6                           | +772
        add.w   d6,d6                           | +774
        jmp     .L05c6f0(pc,d6.w)               | +776
.L05c6f0:
        bra.w   .L05c8a2                        | +77a
        bra.w   .L05c898                        | +77e
        bra.w   .L05c88e                        | +782
        bra.w   .L05c884                        | +786
        bra.w   .L05c87a                        | +78a
        bra.w   .L05c870                        | +78e
        bra.w   .L05c866                        | +792
        bra.w   .L05c85c                        | +796
        bra.w   .L05c852                        | +79a
        bra.w   .L05c848                        | +79e
        bra.w   .L05c83e                        | +7a2
        bra.w   .L05c834                        | +7a6
        bra.w   .L05c82a                        | +7aa
        bra.w   .L05c820                        | +7ae
        bra.w   .L05c816                        | +7b2
        bra.w   .L05c80c                        | +7b6
        bra.w   .L05c802                        | +7ba
        bra.w   .L05c7f8                        | +7be
        bra.w   .L05c7ee                        | +7c2
        bra.w   .L05c7e4                        | +7c6
        bra.w   .L05c7da                        | +7ca
        bra.w   .L05c7d0                        | +7ce
        bra.w   .L05c7c6                        | +7d2
        bra.w   .L05c7bc                        | +7d6
        bra.w   .L05c7b2                        | +7da
        bra.w   .L05c7a8                        | +7de
        bra.w   .L05c79e                        | +7e2
        bra.w   .L05c794                        | +7e6
        bra.w   .L05c78a                        | +7ea
        bra.w   .L05c780                        | +7ee
        bra.w   .L05c776                        | +7f2
        move.w  -(a6),d7                        | +7f6
        move.l  d7,(a4)                         | +7f8
        add.l   d4,d7                           | +7fa
        move.l  d5,(a4)                         | +7fc
        add.l   d4,d5                           | +7fe
.L05c776:
        move.w  -(a6),d7                        | +800
        move.l  d7,(a4)                         | +802
        add.l   d4,d7                           | +804
        move.l  d5,(a4)                         | +806
        add.l   d4,d5                           | +808
.L05c780:
        move.w  -(a6),d7                        | +80a
        move.l  d7,(a4)                         | +80c
        add.l   d4,d7                           | +80e
        move.l  d5,(a4)                         | +810
        add.l   d4,d5                           | +812
.L05c78a:
        move.w  -(a6),d7                        | +814
        move.l  d7,(a4)                         | +816
        add.l   d4,d7                           | +818
        move.l  d5,(a4)                         | +81a
        add.l   d4,d5                           | +81c
.L05c794:
        move.w  -(a6),d7                        | +81e
        move.l  d7,(a4)                         | +820
        add.l   d4,d7                           | +822
        move.l  d5,(a4)                         | +824
        add.l   d4,d5                           | +826
.L05c79e:
        move.w  -(a6),d7                        | +828
        move.l  d7,(a4)                         | +82a
        add.l   d4,d7                           | +82c
        move.l  d5,(a4)                         | +82e
        add.l   d4,d5                           | +830
.L05c7a8:
        move.w  -(a6),d7                        | +832
        move.l  d7,(a4)                         | +834
        add.l   d4,d7                           | +836
        move.l  d5,(a4)                         | +838
        add.l   d4,d5                           | +83a
.L05c7b2:
        move.w  -(a6),d7                        | +83c
        move.l  d7,(a4)                         | +83e
        add.l   d4,d7                           | +840
        move.l  d5,(a4)                         | +842
        add.l   d4,d5                           | +844
.L05c7bc:
        move.w  -(a6),d7                        | +846
        move.l  d7,(a4)                         | +848
        add.l   d4,d7                           | +84a
        move.l  d5,(a4)                         | +84c
        add.l   d4,d5                           | +84e
.L05c7c6:
        move.w  -(a6),d7                        | +850
        move.l  d7,(a4)                         | +852
        add.l   d4,d7                           | +854
        move.l  d5,(a4)                         | +856
        add.l   d4,d5                           | +858
.L05c7d0:
        move.w  -(a6),d7                        | +85a
        move.l  d7,(a4)                         | +85c
        add.l   d4,d7                           | +85e
        move.l  d5,(a4)                         | +860
        add.l   d4,d5                           | +862
.L05c7da:
        move.w  -(a6),d7                        | +864
        move.l  d7,(a4)                         | +866
        add.l   d4,d7                           | +868
        move.l  d5,(a4)                         | +86a
        add.l   d4,d5                           | +86c
.L05c7e4:
        move.w  -(a6),d7                        | +86e
        move.l  d7,(a4)                         | +870
        add.l   d4,d7                           | +872
        move.l  d5,(a4)                         | +874
        add.l   d4,d5                           | +876
.L05c7ee:
        move.w  -(a6),d7                        | +878
        move.l  d7,(a4)                         | +87a
        add.l   d4,d7                           | +87c
        move.l  d5,(a4)                         | +87e
        add.l   d4,d5                           | +880
.L05c7f8:
        move.w  -(a6),d7                        | +882
        move.l  d7,(a4)                         | +884
        add.l   d4,d7                           | +886
        move.l  d5,(a4)                         | +888
        add.l   d4,d5                           | +88a
.L05c802:
        move.w  -(a6),d7                        | +88c
        move.l  d7,(a4)                         | +88e
        add.l   d4,d7                           | +890
        move.l  d5,(a4)                         | +892
        add.l   d4,d5                           | +894
.L05c80c:
        move.w  -(a6),d7                        | +896
        move.l  d7,(a4)                         | +898
        add.l   d4,d7                           | +89a
        move.l  d5,(a4)                         | +89c
        add.l   d4,d5                           | +89e
.L05c816:
        move.w  -(a6),d7                        | +8a0
        move.l  d7,(a4)                         | +8a2
        add.l   d4,d7                           | +8a4
        move.l  d5,(a4)                         | +8a6
        add.l   d4,d5                           | +8a8
.L05c820:
        move.w  -(a6),d7                        | +8aa
        move.l  d7,(a4)                         | +8ac
        add.l   d4,d7                           | +8ae
        move.l  d5,(a4)                         | +8b0
        add.l   d4,d5                           | +8b2
.L05c82a:
        move.w  -(a6),d7                        | +8b4
        move.l  d7,(a4)                         | +8b6
        add.l   d4,d7                           | +8b8
        move.l  d5,(a4)                         | +8ba
        add.l   d4,d5                           | +8bc
.L05c834:
        move.w  -(a6),d7                        | +8be
        move.l  d7,(a4)                         | +8c0
        add.l   d4,d7                           | +8c2
        move.l  d5,(a4)                         | +8c4
        add.l   d4,d5                           | +8c6
.L05c83e:
        move.w  -(a6),d7                        | +8c8
        move.l  d7,(a4)                         | +8ca
        add.l   d4,d7                           | +8cc
        move.l  d5,(a4)                         | +8ce
        add.l   d4,d5                           | +8d0
.L05c848:
        move.w  -(a6),d7                        | +8d2
        move.l  d7,(a4)                         | +8d4
        add.l   d4,d7                           | +8d6
        move.l  d5,(a4)                         | +8d8
        add.l   d4,d5                           | +8da
.L05c852:
        move.w  -(a6),d7                        | +8dc
        move.l  d7,(a4)                         | +8de
        add.l   d4,d7                           | +8e0
        move.l  d5,(a4)                         | +8e2
        add.l   d4,d5                           | +8e4
.L05c85c:
        move.w  -(a6),d7                        | +8e6
        move.l  d7,(a4)                         | +8e8
        add.l   d4,d7                           | +8ea
        move.l  d5,(a4)                         | +8ec
        add.l   d4,d5                           | +8ee
.L05c866:
        move.w  -(a6),d7                        | +8f0
        move.l  d7,(a4)                         | +8f2
        add.l   d4,d7                           | +8f4
        move.l  d5,(a4)                         | +8f6
        add.l   d4,d5                           | +8f8
.L05c870:
        move.w  -(a6),d7                        | +8fa
        move.l  d7,(a4)                         | +8fc
        add.l   d4,d7                           | +8fe
        move.l  d5,(a4)                         | +900
        add.l   d4,d5                           | +902
.L05c87a:
        move.w  -(a6),d7                        | +904
        move.l  d7,(a4)                         | +906
        add.l   d4,d7                           | +908
        move.l  d5,(a4)                         | +90a
        add.l   d4,d5                           | +90c
.L05c884:
        move.w  -(a6),d7                        | +90e
        move.l  d7,(a4)                         | +910
        add.l   d4,d7                           | +912
        move.l  d5,(a4)                         | +914
        add.l   d4,d5                           | +916
.L05c88e:
        move.w  -(a6),d7                        | +918
        move.l  d7,(a4)                         | +91a
        add.l   d4,d7                           | +91c
        move.l  d5,(a4)                         | +91e
        add.l   d4,d5                           | +920
.L05c898:
        move.w  -(a6),d7                        | +922
        move.l  d7,(a4)                         | +924
        add.l   d4,d7                           | +926
        move.l  d5,(a4)                         | +928
        add.l   d4,d5                           | +92a
.L05c8a2:
        move.w  -(a6),d7                        | +92c
        move.l  d7,(a4)                         | +92e
        add.l   d4,d7                           | +930
        move.l  d5,(a4)                         | +932
        add.l   d4,d5                           | +934
        clr.w   d7                              | +936
        moveq   #1,d4                           | +938
        swap    d4                              | +93a
        add.w   d6,d6                           | +93c
        jmp     .L05c8b8(pc,d6.w)               | +93e
.L05c8b8:
        move.l  d7,(a4)                         | +942
        add.l   d4,d7                           | +944
        move.l  d7,(a4)                         | +946
        add.l   d4,d7                           | +948
        move.l  d7,(a4)                         | +94a
        add.l   d4,d7                           | +94c
        move.l  d7,(a4)                         | +94e
        add.l   d4,d7                           | +950
        move.l  d7,(a4)                         | +952
        add.l   d4,d7                           | +954
        move.l  d7,(a4)                         | +956
        add.l   d4,d7                           | +958
        move.l  d7,(a4)                         | +95a
        add.l   d4,d7                           | +95c
        move.l  d7,(a4)                         | +95e
        add.l   d4,d7                           | +960
        move.l  d7,(a4)                         | +962
        add.l   d4,d7                           | +964
        move.l  d7,(a4)                         | +966
        add.l   d4,d7                           | +968
        move.l  d7,(a4)                         | +96a
        add.l   d4,d7                           | +96c
        move.l  d7,(a4)                         | +96e
        add.l   d4,d7                           | +970
        move.l  d7,(a4)                         | +972
        add.l   d4,d7                           | +974
        move.l  d7,(a4)                         | +976
        add.l   d4,d7                           | +978
        move.l  d7,(a4)                         | +97a
        add.l   d4,d7                           | +97c
        move.l  d7,(a4)                         | +97e
        add.l   d4,d7                           | +980
        move.l  d7,(a4)                         | +982
        add.l   d4,d7                           | +984
        move.l  d7,(a4)                         | +986
        add.l   d4,d7                           | +988
        move.l  d7,(a4)                         | +98a
        add.l   d4,d7                           | +98c
        move.l  d7,(a4)                         | +98e
        add.l   d4,d7                           | +990
        move.l  d7,(a4)                         | +992
        add.l   d4,d7                           | +994
        move.l  d7,(a4)                         | +996
        add.l   d4,d7                           | +998
        move.l  d7,(a4)                         | +99a
        add.l   d4,d7                           | +99c
        move.l  d7,(a4)                         | +99e
        add.l   d4,d7                           | +9a0
        move.l  d7,(a4)                         | +9a2
        add.l   d4,d7                           | +9a4
        move.l  d7,(a4)                         | +9a6
        add.l   d4,d7                           | +9a8
        move.l  d7,(a4)                         | +9aa
        add.l   d4,d7                           | +9ac
        move.l  d7,(a4)                         | +9ae
        add.l   d4,d7                           | +9b0
        move.l  d7,(a4)                         | +9b2
        add.l   d4,d7                           | +9b4
        move.l  d7,(a4)                         | +9b6
        add.l   d4,d7                           | +9b8
        move.l  d7,(a4)                         | +9ba
        add.l   d4,d7                           | +9bc
        move.l  d7,(a4)                         | +9be
        add.l   d4,d7                           | +9c0
        move.l  d7,(a4)                         | +9c2
        add.l   d4,d7                           | +9c4
        move.l  d7,(a4)                         | +9c6
        add.l   d4,d7                           | +9c8
        move.l  d7,(a4)                         | +9ca
        add.l   d4,d7                           | +9cc
        move.l  d7,(a4)                         | +9ce
        add.l   d4,d7                           | +9d0
        move.l  d7,(a4)                         | +9d2
        add.l   d4,d7                           | +9d4
        move.l  d7,(a4)                         | +9d6
        add.l   d4,d7                           | +9d8
        move.l  d7,(a4)                         | +9da
        add.l   d4,d7                           | +9dc
        move.l  d7,(a4)                         | +9de
        add.l   d4,d7                           | +9e0
        move.l  d7,(a4)                         | +9e2
        add.l   d4,d7                           | +9e4
        move.l  d7,(a4)                         | +9e6
        add.l   d4,d7                           | +9e8
        move.l  d7,(a4)                         | +9ea
        add.l   d4,d7                           | +9ec
        move.l  d7,(a4)                         | +9ee
        add.l   d4,d7                           | +9f0
        move.l  d7,(a4)                         | +9f2
        add.l   d4,d7                           | +9f4
        move.l  d7,(a4)                         | +9f6
        add.l   d4,d7                           | +9f8
        move.l  d7,(a4)                         | +9fa
        add.l   d4,d7                           | +9fc
        move.l  d7,(a4)                         | +9fe
        add.l   d4,d7                           | +a00
        move.l  d7,(a4)                         | +a02
        add.l   d4,d7                           | +a04
        move.l  d7,(a4)                         | +a06
        add.l   d4,d7                           | +a08
        move.l  d7,(a4)                         | +a0a
        add.l   d4,d7                           | +a0c
        move.l  d7,(a4)                         | +a0e
        add.l   d4,d7                           | +a10
        move.l  d7,(a4)                         | +a12
        add.l   d4,d7                           | +a14
        move.l  d7,(a4)                         | +a16
        add.l   d4,d7                           | +a18
        move.l  d7,(a4)                         | +a1a
        add.l   d4,d7                           | +a1c
        move.l  d7,(a4)                         | +a1e
        add.l   d4,d7                           | +a20
        move.l  d7,(a4)                         | +a22
        add.l   d4,d7                           | +a24
        move.l  d7,(a4)                         | +a26
        add.l   d4,d7                           | +a28
        move.l  d7,(a4)                         | +a2a
        add.l   d4,d7                           | +a2c
        move.l  d7,(a4)                         | +a2e
        add.l   d4,d7                           | +a30
        move.l  d7,(a4)                         | +a32
        add.l   d4,d7                           | +a34
        move.l  d7,(a4)                         | +a36
        add.l   d4,d7                           | +a38
        move.w  #0x8401,d6                      | +a3a
        add.w   d0,d6                           | +a3e
        swap    d6                              | +a40
        move.w  0xa(a3),d6                      | +a42
        move.l  #0x2000000,d5                   | +a46
        move.l  d6,(a4)                         | +a4c
        sub.l   d5,d6                           | +a4e
        move.w  0x8(a3),d6                      | +a50
        move.l  d6,(a4)                         | +a54
        sub.l   d5,d6                           | +a56
        move.w  0x6(a3),d6                      | +a58
        move.l  d6,(a4)                         | +a5c
        rts                                     | +a5e

| ----------------------------------------------------------------------------
|  Vblank_FlushSpriteQueue_05c9d6  @ $05C9D6  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Vblank_FlushSpriteQueue_05c9d6, "ax", @progbits
        .global Vblank_FlushSpriteQueue_05c9d6
Vblank_FlushSpriteQueue_05c9d6:
        tst.b   0x10e1ec.l                      | +000
        beq.w   .L05ca28                        | +006
        clr.b   0x10e1ec.l                      | +00a
        lea     0x3c0000.l,a4                   | +010
        lea     0x108080.l,a5                   | +016
        lea     0x614c(a5),a0                   | +01c
        lea     0x542a(a5),a1                   | +020
        move.w  0x6148(a5),d7                   | +024
        movea.l a1,a2                           | +028
        adda.w  d7,a2                           | +02a
        bsr.w   SpriteQueue_RenderSCB234_05b400 | +02c
        move.w  0xa(a0),0x6168(a5)              | +030
        lea     0x6158(a5),a0                   | +036
        lea     0x542a(a5),a1                   | +03a
        lea     0x348(a1),a2                    | +03e
        move.w  0x614a(a5),d7                   | +042
        adda.w  d7,a1                           | +046
        bsr.w   SpriteQueue_RenderSCB234_05b400 | +048
        move.w  0xa(a0),0x616a(a5)              | +04c
.L05ca28:
        rts                                     | +052
