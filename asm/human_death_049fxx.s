| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $049FF2..$04BB8E  (6,990 B, 42 entradas, 10 huecos)
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
|  HumanDeath_Dispatch_049ff2  @ $049FF2  (26 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_Dispatch_049ff2, "ax", @progbits
        .global HumanDeath_Dispatch_049ff2
HumanDeath_Dispatch_049ff2:
        bsr.b   Sub_00049FBA                    | +000
        bcc.w   SetHandlerRts_04a012            | +002
        jsr     0x27eba.l                       | +006
        bcc.w   SetTaskHandler_04a00c           | +00c
        lea     HumanDeath_EntryKind2_04a034(pc),a1 | +010
        move.l  a1,(a6)                         | +014
        bra.w   SetHandlerRts_04a012            | +016

| ----------------------------------------------------------------------------
|  HumanDeath_EntryKind0_04a014  @ $04A014  (16 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_EntryKind0_04a014, "ax", @progbits
        .global HumanDeath_EntryKind0_04a014
HumanDeath_EntryKind0_04a014:
        jsr     0x27f60.l                       | +000
        scc.b   0x70(a6)                        | +006
        moveq   #0,d7                           | +00a
        bra.w   HumanDeath_EntryKind2_04a034__L04a050 | +00c

| ----------------------------------------------------------------------------
|  HumanDeath_EntryKind1_04a024  @ $04A024  (16 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_EntryKind1_04a024, "ax", @progbits
        .global HumanDeath_EntryKind1_04a024
HumanDeath_EntryKind1_04a024:
        jsr     0x27f60.l                       | +000
        scc.b   0x70(a6)                        | +006
        moveq   #1,d7                           | +00a
        bra.w   HumanDeath_EntryKind2_04a034__L04a050 | +00c

| ----------------------------------------------------------------------------
|  HumanDeath_EntryKind2_04a034  @ $04A034  (104 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_EntryKind2_04a034, "ax", @progbits
        .global HumanDeath_EntryKind2_04a034
HumanDeath_EntryKind2_04a034:
        jsr     0x27f60.l                       | +000
        scc.b   0x70(a6)                        | +006
        moveq   #2,d7                           | +00a
        bra.w   .L04a050                        | +00c
        .global HumanDeath_EntryKind2_04a034__L04a044
HumanDeath_EntryKind2_04a034__L04a044:
.L04a044:
        jsr     0x27f60.l                       | +010
        scc.b   0x70(a6)                        | +016
        moveq   #3,d7                           | +01a
        .global HumanDeath_EntryKind2_04a034__L04a050
HumanDeath_EntryKind2_04a034__L04a050:
.L04a050:
        move.w  0x22(a6),d0                     | +01c
        sub.w   0x54(a6),d0                     | +020
        scc.b   d0                              | +024
        btst    #0x0,0x3a(a6)                   | +026
        sne.b   d1                              | +02c
        eor.b   d0,d1                           | +02e
        move.b  d1,0x71(a6)                     | +030
        andi.w  #0xff,d7                        | +034
        lsl.w   #0x2,d7                         | +038
        lea     Sub_00049FAA(pc),a0             | +03a
        movea.l (a0,d7.w),a0                    | +03e
        moveq   #0,d0                           | +042
        move.b  0x58(a6),d0                     | +044
        cmpi.w  #0x22,d0                        | +048
        bcs.w   .L04a088                        | +04c
        move.w  #0xf,d0                         | +050
.L04a088:
        lsl.w   #0x2,d0                         | +054
        move.l  (a0,d0.w),-(a7)                 | +056
        bsr.w   HumanDeath_ResetBody_04a09c     | +05a
        jsr     0x8f308.l                       | +05e
        movea.l (a7)+,a0                        | +064
        jmp     (a0)                            | +066

| ----------------------------------------------------------------------------
|  HumanDeath_ResetBody_04a09c  @ $04A09C  (56 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_ResetBody_04a09c, "ax", @progbits
        .global HumanDeath_ResetBody_04a09c
HumanDeath_ResetBody_04a09c:
        jsr     0x2783a.l                       | +000
        bsr.w   HumanDeath_PlayCryByKind_04a0d4 | +006
        bclr    #0x1,0x12(a6)                   | +00a
        move.w  #0xffff,0x34(a6)                | +010
        move.l  #0xffffffff,0x60(a6)            | +016
        jsr     0x27f60.l                       | +01e
        scc.b   0x70(a6)                        | +024
        clr.w   0x2c(a6)                        | +028
        clr.w   0x2e(a6)                        | +02c
        move.w  #0xc000,0x38(a6)                | +030
        rts                                     | +036

| ----------------------------------------------------------------------------
|  HumanDeath_PlayCryByKind_04a0d4  @ $04A0D4  (146 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_PlayCryByKind_04a0d4, "ax", @progbits
        .global HumanDeath_PlayCryByKind_04a0d4
HumanDeath_PlayCryByKind_04a0d4:
        move.w  #0xe,d1                         | +000
        tst.b   0x10fd8f.l                      | +004
        bne.w   .L04a0e6                        | +00a
        move.w  #0x2f,d1                        | +00e
.L04a0e6:
        bra.w   .L04a0f4                        | +012
        move.w  #0x1a,d1                        | +016
        bra.w   .L04a0f4                        | +01a
        nop                                     | +01e
.L04a0f4:
        movem.w d1,-(a7)                        | +020
        jsr     0x13600.l                       | +024
        movem.w (a7)+,d1                        | +02a
        jmp     0x236e.l                        | +02e
        move.w  #0x14,d1                        | +034
        bra.w   .L04a132                        | +038
        move.w  #0x15c,d1                       | +03c
        bra.w   .L04a132                        | +040
        move.w  #0x21,d1                        | +044
        bra.w   .L04a132                        | +048
        move.w  #0xe,d1                         | +04c
        bra.w   .L04a132                        | +050
        move.w  #0x185,d1                       | +054
        bra.w   .L04a132                        | +058
        nop                                     | +05c
.L04a132:
        movem.w d1,-(a7)                        | +05e
        jsr     0x13600.l                       | +062
        movem.w (a7)+,d1                        | +068
        jmp     0x236e.l                        | +06c
        jsr     0x27f60.l                       | +072
        scc.b   0x70(a6)                        | +078
        bra.w   HumanDeath_TumbleBackLand_04a4a2 | +07c
        jsr     0x27f60.l                       | +080
        scc.b   0x70(a6)                        | +086
        bsr.w   HumanDeath_PlayCryByKind_04a0d4 | +08a
        bra.w   HumanDeath_Collapse_04a54a__L04a5a6 | +08e

| ----------------------------------------------------------------------------
|  HumanDeath_SpawnCorpseA_04a16e  @ $04A16E  (30 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_SpawnCorpseA_04a16e, "ax", @progbits
        .global HumanDeath_SpawnCorpseA_04a16e
HumanDeath_SpawnCorpseA_04a16e:
        lea     HumanDeath_CorpseA_04a18c(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  0x22(a6),0x54(a0)               | +010
        move.w  0x24(a6),0x54(a0)               | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  HumanDeath_CorpseA_04a18c  @ $04A18C  (8 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_CorpseA_04a18c, "ax", @progbits
        .global HumanDeath_CorpseA_04a18c
HumanDeath_CorpseA_04a18c:
        bsr.w   HumanDeath_ResetBody_04a09c     | +000
        bra.w   HumanDeath_InitBurst_04a6b2__L04a72e | +004

| ----------------------------------------------------------------------------
|  HumanDeath_SpawnCorpseB_04a194  @ $04A194  (30 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_SpawnCorpseB_04a194, "ax", @progbits
        .global HumanDeath_SpawnCorpseB_04a194
HumanDeath_SpawnCorpseB_04a194:
        lea     HumanDeath_CorpseB_04a1b2(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  0x22(a6),0x54(a0)               | +010
        move.w  0x24(a6),0x54(a0)               | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  HumanDeath_CorpseB_04a1b2  @ $04A1B2  (8 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_CorpseB_04a1b2, "ax", @progbits
        .global HumanDeath_CorpseB_04a1b2
HumanDeath_CorpseB_04a1b2:
        bsr.w   HumanDeath_ResetBody_04a09c     | +000
        bra.w   HumanDeath_Launched_04a7f4__L04a802 | +004

| ----------------------------------------------------------------------------
|  HumanDeath_LoadTimer_04a1ba  @ $04A1BA  (4 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_LoadTimer_04a1ba, "ax", @progbits
        .global HumanDeath_LoadTimer_04a1ba
HumanDeath_LoadTimer_04a1ba:
        move.w  0x74(a6),d0                     | +000

| ----------------------------------------------------------------------------
|  HumanDeath_DampVelocity_04a1c6  @ $04A1C6  (42 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_DampVelocity_04a1c6, "ax", @progbits
        .global HumanDeath_DampVelocity_04a1c6
HumanDeath_DampVelocity_04a1c6:
        move.w  0x28(a6),d0                     | +000
        beq.w   .L04a1da                        | +004
        asr.w   #0x5,d0                         | +008
        seq.b   d1                              | +00a
        ext.w   d1                              | +00c
        sub.w   d1,d0                           | +00e
        sub.w   d0,0x28(a6)                     | +010
.L04a1da:
        move.w  0x2a(a6),d0                     | +014
        beq.w   .L04a1ee                        | +018
        asr.w   #0x5,d0                         | +01c
        seq.b   d1                              | +01e
        ext.w   d1                              | +020
        sub.w   d1,d0                           | +022
        sub.w   d0,0x2a(a6)                     | +024
.L04a1ee:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  HumanDeath_SetVelXByFacing_04a1f0  @ $04A1F0  (12 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_SetVelXByFacing_04a1f0, "ax", @progbits
        .global HumanDeath_SetVelXByFacing_04a1f0
HumanDeath_SetVelXByFacing_04a1f0:
        btst    #0x0,0x3a(a6)                   | +000
        bne.w   SetTaskW_04a1fc                 | +006
        neg.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  HumanDeath_RandBelowY_04a202  @ $04A202  (14 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_RandBelowY_04a202, "ax", @progbits
        .global HumanDeath_RandBelowY_04a202
HumanDeath_RandBelowY_04a202:
        jsr     0x5e9b6.l                       | +000
        move.w  0x24(a6),d1                     | +006
        cmp.b   d1,d0                           | +00a
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  HumanDeath_PhysicsAir_04a218  @ $04A218  (72 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_PhysicsAir_04a218, "ax", @progbits
        .global HumanDeath_PhysicsAir_04a218
HumanDeath_PhysicsAir_04a218:
        jsr     0x27f08.l                       | +000
        bcc.w   .L04a22a                        | +006
        move.b  d3,0x70(a6)                     | +00a
        bra.w   .L04a240                        | +00e
.L04a22a:
        cmp.b   0x70(a6),d0                     | +012
        bne.w   .L04a23a                        | +016
        move.b  d3,0x70(a6)                     | +01a
        bra.w   .L04a240                        | +01e
.L04a23a:
        move.b  #0xff,0x70(a6)                  | +022
.L04a240:
        bsr.b   HumanDeath_DampVelocity_04a1c6  | +028
        tst.b   0x70(a6)                        | +02a
        bne.w   .L04a25c                        | +02e
        move.w  #0xff40,0x2e(a6)                | +032
        jsr     0x27bc8.l                       | +038
        scs.b   0x70(a6)                        | +03e
        rts                                     | +042
.L04a25c:
        bsr.w   HumanDeath_DampVelocity_04a1c6  | +044

| ----------------------------------------------------------------------------
|  HumanDeath_PhysicsGround_04a268  @ $04A268  (86 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_PhysicsGround_04a268, "ax", @progbits
        .global HumanDeath_PhysicsGround_04a268
HumanDeath_PhysicsGround_04a268:
        jsr     0x27fd8.l                       | +000
        bcc.w   .L04a27a                        | +006
        move.b  d3,0x70(a6)                     | +00a
        bra.w   .L04a290                        | +00e
.L04a27a:
        cmp.b   0x70(a6),d0                     | +012
        bne.w   .L04a28a                        | +016
        move.b  d3,0x70(a6)                     | +01a
        bra.w   .L04a290                        | +01e
.L04a28a:
        move.b  #0xff,0x70(a6)                  | +022
.L04a290:
        bsr.w   HumanDeath_DampVelocity_04a1c6  | +028
        tst.b   0x70(a6)                        | +02c
        bne.w   .L04a2a8                        | +030
        jsr     0x27d50.l                       | +034
        scs.b   0x70(a6)                        | +03a
        rts                                     | +03e
.L04a2a8:
        bsr.w   HumanDeath_DampVelocity_04a1c6  | +040
        jsr     0x27a92.l                       | +044
        jsr     0x27fac.l                       | +04a
        scc.b   0x70(a6)                        | +050
        rts                                     | +054

| ----------------------------------------------------------------------------
|  HumanDeath_PhysicsFall_04a2be  @ $04A2BE  (224 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_PhysicsFall_04a2be, "ax", @progbits
        .global HumanDeath_PhysicsFall_04a2be
HumanDeath_PhysicsFall_04a2be:
        jsr     0x27f08.l                       | +000
        bcc.w   .L04a2d0                        | +006
        move.b  d3,0x70(a6)                     | +00a
        bra.w   .L04a2e6                        | +00e
.L04a2d0:
        cmp.b   0x70(a6),d0                     | +012
        bne.w   .L04a2e0                        | +016
        move.b  d3,0x70(a6)                     | +01a
        bra.w   .L04a2e6                        | +01e
.L04a2e0:
        move.b  #0xff,0x70(a6)                  | +022
.L04a2e6:
        tst.b   0x70(a6)                        | +028
        bne.w   .L04a2fe                        | +02c
        move.w  #0xff40,0x2e(a6)                | +030
        jsr     0x27bc8.l                       | +036
        scs.b   0x70(a6)                        | +03c
.L04a2fe:
        jsr     0x27a92.l                       | +040
        jsr     0x27eba.l                       | +046
        scc.b   0x70(a6)                        | +04c
        rts                                     | +050
        jsr     0x27fd8.l                       | +052
        bcc.w   .L04a322                        | +058
        move.b  d3,0x70(a6)                     | +05c
        bra.w   .L04a338                        | +060
.L04a322:
        cmp.b   0x70(a6),d0                     | +064
        bne.w   .L04a332                        | +068
        move.b  d3,0x70(a6)                     | +06c
        bra.w   .L04a338                        | +070
.L04a332:
        move.b  #0xff,0x70(a6)                  | +074
.L04a338:
        tst.b   0x70(a6)                        | +07a
        bne.w   .L04a34c                        | +07e
        jsr     0x27d50.l                       | +082
        scs.b   0x70(a6)                        | +088
        rts                                     | +08c
.L04a34c:
        jsr     0x27a92.l                       | +08e
        jsr     0x27fac.l                       | +094
        scc.b   0x70(a6)                        | +09a
        rts                                     | +09e
        .global HumanDeath_PhysicsFall_04a2be__L04a35e
HumanDeath_PhysicsFall_04a2be__L04a35e:
.L04a35e:
        move.w  0x22(a6),d0                     | +0a0
        addi.w  #0x20,d0                        | +0a4
        cmpi.w  #0x180,d0                       | +0a8
        bcs.w   .L04a372                        | +0ac
        jmp     JmpToScheduler_04a210(pc)       | +0b0
.L04a372:
        cmpi.w  #0xe0,0x24(a6)                  | +0b4
        bcc.w   .L04a380                        | +0ba
        jmp     JmpToScheduler_04a210(pc)       | +0be
.L04a380:
        rts                                     | +0c2
        .global HumanDeath_PhysicsFall_04a2be__L04a382
HumanDeath_PhysicsFall_04a2be__L04a382:
.L04a382:
        lea     .L04a388(pc),a1                 | +0c4
        move.l  a1,(a6)                         | +0c8
.L04a388:
        bsr.w   HumanDeath_PhysicsAir_04a218    | +0ca
        jsr     0x28d70.l                       | +0ce
        bcc.w   .L04a39c                        | +0d4
        lea     HumanDeath_FadeOut_04a39e(pc),a1 | +0d8
        move.l  a1,(a6)                         | +0dc
.L04a39c:
        bra.b   .L04a35e                        | +0de

| ----------------------------------------------------------------------------
|  HumanDeath_FadeOut_04a39e  @ $04A39E  (106 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_FadeOut_04a39e, "ax", @progbits
        .global HumanDeath_FadeOut_04a39e
HumanDeath_FadeOut_04a39e:
        move.b  #0xa,0x5c(a6)                   | +000
        lea     .L04a3aa(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L04a3aa:
        bsr.w   HumanDeath_PhysicsAir_04a218    | +00c
        jsr     0x28d70.l                       | +010
        subq.b  #0x1,0x5c(a6)                   | +016
        bne.w   .L04a3c2                        | +01a
        lea     .L04a3c4(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L04a3c2:
        bra.b   HumanDeath_PhysicsFall_04a2be__L04a35e | +024
.L04a3c4:
        move.b  #0xa,0x5c(a6)                   | +026
        lea     .L04a3d0(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L04a3d0:
        jsr     HumanDeath_PhysicsAir_04a218(pc) | +032
        subq.b  #0x1,0x5c(a6)                   | +036
        bne.w   .L04a3e4                        | +03a
        jmp     JmpToScheduler_04a210(pc)       | +03e
        bra.w   .L04a3f4                        | +042
.L04a3e4:
        btst    #0x0,0x5c(a6)                   | +046
        bne.w   .L04a3f4                        | +04c
        jsr     0x28d70.l                       | +050
.L04a3f4:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +056
        .global HumanDeath_FadeOut_04a39e__L04a3f8
HumanDeath_FadeOut_04a39e__L04a3f8:
.L04a3f8:
        bsr.w   HumanDeath_SpawnBloodSplash_04aad2__L04aae0 | +05a
        tst.b   0x71(a6)                        | +05e
        bne.w   HumanDeath_TumbleBackStart_04a420 | +062
        bra.w   HumanDeath_TumbleFwdStart_04a4c8 | +066

| ----------------------------------------------------------------------------
|  HumanDeath_TumbleBackSpriteTbl_04a408  @ $04A408  (24 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_TumbleBackSpriteTbl_04a408, "ax", @progbits
        .global HumanDeath_TumbleBackSpriteTbl_04a408
HumanDeath_TumbleBackSpriteTbl_04a408:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb404                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0xb440                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        .dc.w   0xb460                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xb480                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0xb4a0                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HumanDeath_TumbleBackStart_04a420  @ $04A420  (24 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_TumbleBackStart_04a420, "ax", @progbits
        .global HumanDeath_TumbleBackStart_04a420
HumanDeath_TumbleBackStart_04a420:
        bsr.w   HumanDeath_RandBelowY_04a202    | +000
        bcs.w   HumanDeath_Collapse_04a54a__L04a620 | +004
        jsr     0x5e9b6.l                       | +008
        tst.w   d0                              | +00e
        bmi.w   HumanDeath_Collapse_04a54a__L04a568 | +010
        clr.b   0x72(a6)                        | +014

| ----------------------------------------------------------------------------
|  HumanDeath_TumbleBack_04a438  @ $04A438  (106 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_TumbleBack_04a438, "ax", @progbits
        .global HumanDeath_TumbleBack_04a438
HumanDeath_TumbleBack_04a438:
        lea     HumanDeath_TumbleBackSpriteTbl_04a408(pc),a0 | +000
        moveq   #0,d0                           | +004
        move.b  0x72(a6),d0                     | +006
        lsl.w   #0x2,d0                         | +00a
        move.l  (a0,d0.w),d0                    | +00c
        beq.w   HumanDeath_TumbleBackLand_04a4a2 | +010
        movea.l d0,a0                           | +014
        jsr     0x28cd4.l                       | +016
        addq.b  #0x1,0x72(a6)                   | +01c
        bclr    #0x3,0x13(a6)                   | +020
        move.w  #0xff80,d0                      | +026
        bsr.w   HumanDeath_SetVelXByFacing_04a1f0 | +02a
        addi.w  #0x80,0x2a(a6)                  | +02e
        addq.w  #0x2,0x24(a6)                   | +034
        lea     .L04a476(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L04a476:
        bsr.w   HumanDeath_PhysicsAir_04a218    | +03e
        jsr     0x28d70.l                       | +042
        bcc.w   .L04a49e                        | +048
        jsr     0x2870a.l                       | +04c
        bcc.w   .L04a498                        | +052
        lea     HumanDeath_TumbleBack_04a438(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
        bra.w   .L04a49e                        | +05c
.L04a498:
        lea     HumanDeath_TumbleBackLand_04a4a2(pc),a1 | +060
        move.l  a1,(a6)                         | +064
.L04a49e:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +066

| ----------------------------------------------------------------------------
|  HumanDeath_TumbleBackLand_04a4a2  @ $04A4A2  (14 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_TumbleBackLand_04a4a2, "ax", @progbits
        .global HumanDeath_TumbleBackLand_04a4a2
HumanDeath_TumbleBackLand_04a4a2:
        lea     HumanDeath_SpriteTbls_04ac56__L04b6b8(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +00a

| ----------------------------------------------------------------------------
|  HumanDeath_TumbleFwdSpriteTbl_04a4b0  @ $04A4B0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_TumbleFwdSpriteTbl_04a4b0, "ax", @progbits
        .global HumanDeath_TumbleFwdSpriteTbl_04a4b0
HumanDeath_TumbleFwdSpriteTbl_04a4b0:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb4c0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0xb4fc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        .dc.w   0xb51c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xb53c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0xb55c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HumanDeath_TumbleFwdStart_04a4c8  @ $04A4C8  (24 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_TumbleFwdStart_04a4c8, "ax", @progbits
        .global HumanDeath_TumbleFwdStart_04a4c8
HumanDeath_TumbleFwdStart_04a4c8:
        bsr.w   HumanDeath_RandBelowY_04a202    | +000
        bcs.w   HumanDeath_Collapse_04a54a__L04a620 | +004
        jsr     0x5e9b6.l                       | +008
        tst.w   d0                              | +00e
        bmi.w   HumanDeath_Collapse_04a54a__L04a568 | +010
        clr.b   0x72(a6)                        | +014

| ----------------------------------------------------------------------------
|  HumanDeath_TumbleFwd_04a4e0  @ $04A4E0  (106 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_TumbleFwd_04a4e0, "ax", @progbits
        .global HumanDeath_TumbleFwd_04a4e0
HumanDeath_TumbleFwd_04a4e0:
        lea     HumanDeath_TumbleFwdSpriteTbl_04a4b0(pc),a0 | +000
        moveq   #0,d0                           | +004
        move.b  0x72(a6),d0                     | +006
        lsl.w   #0x2,d0                         | +00a
        move.l  (a0,d0.w),d0                    | +00c
        beq.w   HumanDeath_Collapse_04a54a      | +010
        movea.l d0,a0                           | +014
        jsr     0x28cd4.l                       | +016
        addq.b  #0x1,0x72(a6)                   | +01c
        bclr    #0x3,0x13(a6)                   | +020
        move.w  #0x80,d0                        | +026
        bsr.w   HumanDeath_SetVelXByFacing_04a1f0 | +02a
        addi.w  #0x80,0x2a(a6)                  | +02e
        addq.w  #0x2,0x24(a6)                   | +034
        lea     .L04a51e(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L04a51e:
        bsr.w   HumanDeath_PhysicsAir_04a218    | +03e
        jsr     0x28d70.l                       | +042
        bcc.w   .L04a546                        | +048
        jsr     0x2870a.l                       | +04c
        bcc.w   .L04a540                        | +052
        lea     HumanDeath_TumbleFwd_04a4e0(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
        bra.w   .L04a546                        | +05c
.L04a540:
        lea     HumanDeath_Collapse_04a54a(pc),a1 | +060
        move.l  a1,(a6)                         | +064
.L04a546:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +066

| ----------------------------------------------------------------------------
|  HumanDeath_Collapse_04a54a  @ $04A54A  (296 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_Collapse_04a54a, "ax", @progbits
        .global HumanDeath_Collapse_04a54a
HumanDeath_Collapse_04a54a:
        jsr     0x5e9b6.l                       | +000
        lea     HumanDeath_SpriteTbls_04ac56__L04b57c(pc),a0 | +006
        tst.w   d0                              | +00a
        bpl.w   .L04a55e                        | +00c
        lea     HumanDeath_SpriteTbls_04ac56__L04b61a(pc),a0 | +010
.L04a55e:
        jsr     0x28cd4.l                       | +014
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +01a
        .global HumanDeath_Collapse_04a54a__L04a568
HumanDeath_Collapse_04a54a__L04a568:
.L04a568:
        tst.b   0x71(a6)                        | +01e
        beq.w   .L04a586                        | +022
        lea     HumanDeath_SpriteTbls_04ac56__L04b1f6(pc),a0 | +026
        jsr     0x28cd4.l                       | +02a
        move.w  #0xfc00,d0                      | +030
        bsr.w   HumanDeath_SetVelXByFacing_04a1f0 | +034
        bra.w   .L04a598                        | +038
.L04a586:
        lea     HumanDeath_SpriteTbls_04ac56__L04b28c(pc),a0 | +03c
        jsr     0x28cd4.l                       | +040
        move.w  #0x400,d0                       | +046
        bsr.w   HumanDeath_SetVelXByFacing_04a1f0 | +04a
.L04a598:
        move.w  #0x100,0x2a(a6)                 | +04e
        addq.w  #0x2,0x24(a6)                   | +054
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +058
        .global HumanDeath_Collapse_04a54a__L04a5a6
HumanDeath_Collapse_04a54a__L04a5a6:
.L04a5a6:
        lea     HumanDeath_SpriteTbls_04ac56__L04b34a(pc),a0 | +05c
        jsr     0x28cd4.l                       | +060
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +066
        lea     HumanDeath_SpriteTbls_04ac56__L04ad90(pc),a0 | +06a
        tst.b   0x71(a6)                        | +06e
        beq.w   .L04a5c4                        | +072
        lea     HumanDeath_SpriteTbls_04ac56__L04ae22(pc),a0 | +076
.L04a5c4:
        jsr     0x28cd4.l                       | +07a
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +080
        jsr     0x27f60.l                       | +084
        scc.b   0x70(a6)                        | +08a
        jsr     0x13600.l                       | +08e
        move.w  #0x154,d1                       | +094
        tst.b   0x10fd8f.l                      | +098
        bne.w   .L04a5f0                        | +09e
        move.w  #0x155,d1                       | +0a2
.L04a5f0:
        jsr     0x236e.l                        | +0a6
        lea     HumanDeath_SpriteTbls_04ac56__L04b13c(pc),a0 | +0ac
        jsr     0x28cd4.l                       | +0b0
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +0b6
        lea     HumanDeath_SpriteTbls_04ac56__L04b136(pc),a0 | +0ba
        jsr     0x28cd4.l                       | +0be
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +0c4
        .global HumanDeath_Collapse_04a54a__L04a612
HumanDeath_Collapse_04a54a__L04a612:
.L04a612:
        lea     HumanDeath_SpriteTbls_04ac56__L04b78c(pc),a0 | +0c8
        jsr     0x28cd4.l                       | +0cc
        bra.w   .L04a62a                        | +0d2
        .global HumanDeath_Collapse_04a54a__L04a620
HumanDeath_Collapse_04a54a__L04a620:
.L04a620:
        lea     HumanDeath_SpriteTbls_04ac56__L04b74c(pc),a0 | +0d6
        jsr     0x28cd4.l                       | +0da
.L04a62a:
        tst.b   0x71(a6)                        | +0e0
        bne.w   .L04a638                        | +0e4
        bchg    #0x0,0x3a(a6)                   | +0e8
.L04a638:
        move.w  #0xff38,d0                      | +0ee
        bsr.w   HumanDeath_SetVelXByFacing_04a1f0 | +0f2
        move.w  #0xff40,0x2e(a6)                | +0f6
        move.w  #0x0,0x2a(a6)                   | +0fc
        lea     .L04a652(pc),a1                 | +102
        move.l  a1,(a6)                         | +106
.L04a652:
        bsr.w   HumanDeath_PhysicsGround_04a268 | +108
        jsr     0x28d70.l                       | +10c
        bcc.w   .L04a66e                        | +112
        tst.b   0x70(a6)                        | +116
        beq.w   .L04a66e                        | +11a
        lea     HumanDeath_Knockdown_04a672(pc),a1 | +11e
        move.l  a1,(a6)                         | +122
.L04a66e:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +124

| ----------------------------------------------------------------------------
|  HumanDeath_Knockdown_04a672  @ $04A672  (64 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_Knockdown_04a672, "ax", @progbits
        .global HumanDeath_Knockdown_04a672
HumanDeath_Knockdown_04a672:
        lea     HumanDeath_SpriteTbls_04ac56__L04b7b8(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +00a
        jsr     0x5e9b6.l                       | +00e
        tst.w   d0                              | +014
        bmi.w   HumanDeath_Collapse_04a54a__L04a612 | +016
        lea     HumanDeath_SpriteTbls_04ac56__L04acfe(pc),a0 | +01a
        jsr     0x28cd4.l                       | +01e
        lea     .L04a69c(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L04a69c:
        bsr.w   HumanDeath_PhysicsGround_04a268 | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L04a6ae                        | +034
        jmp     JmpToScheduler_04a210(pc)       | +038
.L04a6ae:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +03c

| ----------------------------------------------------------------------------
|  HumanDeath_InitBurst_04a6b2  @ $04A6B2  (230 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_InitBurst_04a6b2, "ax", @progbits
        .global HumanDeath_InitBurst_04a6b2
HumanDeath_InitBurst_04a6b2:
        move.w  #0x0,d0                         | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0xcc6,0x2a(a6)                 | +00e
        move.w  #0xff93,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        jsr     0x5e9b6.l                       | +020
        andi.w  #0x3ff,d0                       | +026
        move.w  0x22(a6),d1                     | +02a
        sub.w   0x54(a6),d1                     | +02e
        smi.b   d1                              | +032
        ext.w   d1                              | +034
        eor.w   d1,d0                           | +036
        move.w  d0,0x28(a6)                     | +038
        jsr     0x5e9b6.l                       | +03c
        andi.w  #0x3ff,d0                       | +042
        add.w   d0,0x2a(a6)                     | +046
        clr.b   0x70(a6)                        | +04a
        rts                                     | +04e
        jsr     0x5e9b6.l                       | +050
        andi.b  #0x7f,d0                        | +056
        beq.w   HumanDeath_Launched_04a7f4__L04a802 | +05a
        jsr     0x5e9b6.l                       | +05e
        tst.w   d0                              | +064
        bmi.w   HumanDeath_FadeOut_04a39e__L04a3f8 | +066
        jsr     0x5e9b6.l                       | +06a
        andi.w  #0x3,d0                         | +070
        beq.w   HumanDeath_BurstLand_04a798__L04a7b0 | +074
        bra.w   .L04a732                        | +078
        .global HumanDeath_InitBurst_04a6b2__L04a72e
HumanDeath_InitBurst_04a6b2__L04a72e:
.L04a72e:
        addq.w  #0x4,0x24(a6)                   | +07c
.L04a732:
        lea     HumanDeath_SpriteTbls_04ac56__L04af08(pc),a0 | +080
        jsr     0x28cd4.l                       | +084
        bsr.w   HumanDeath_InitBurst_04a6b2     | +08a
        lea     0xffff.w,a0                     | +08e
        move.l  a0,0x48(a6)                     | +092
        bclr    #0x3,0x13(a6)                   | +096
        lea     .L04a754(pc),a1                 | +09c
        move.l  a1,(a6)                         | +0a0
.L04a754:
        move.w  0x2a(a6),d0                     | +0a2
        bpl.w   .L04a76a                        | +0a6
        lea     HumanDeath_SpriteTbls_04ac56__L04aeb4(pc),a0 | +0aa
        move.l  a0,0x48(a6)                     | +0ae
        lea     .L04a76a(pc),a1                 | +0b2
        move.l  a1,(a6)                         | +0b6
.L04a76a:
        jsr     HumanDeath_PhysicsGround_04a268(pc) | +0b8
        jsr     0x5e9b6.l                       | +0bc
        jsr     HumanDeath_PickGibPtr_04aba8(pc) | +0c2
        jsr     0x2870a.l                       | +0c6
        bcc.w   .L04a786                        | +0cc
        bsr.w   HumanDeath_EntryKind2_04a034__L04a044 | +0d0
.L04a786:
        tst.b   0x70(a6)                        | +0d4
        beq.w   .L04a794                        | +0d8
        lea     HumanDeath_BurstLand_04a798(pc),a1 | +0dc
        move.l  a1,(a6)                         | +0e0
.L04a794:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +0e2

| ----------------------------------------------------------------------------
|  HumanDeath_BurstLand_04a798  @ $04A798  (92 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_BurstLand_04a798, "ax", @progbits
        .global HumanDeath_BurstLand_04a798
HumanDeath_BurstLand_04a798:
        lea     HumanDeath_SpriteTbls_04ac56__L04af9e(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +00a
        jsr     0x27f60.l                       | +00e
        scc.b   0x70(a6)                        | +014
        .global HumanDeath_BurstLand_04a798__L04a7b0
HumanDeath_BurstLand_04a798__L04a7b0:
.L04a7b0:
        addq.w  #0x4,0x24(a6)                   | +018
        lea     HumanDeath_SpriteTbls_04ac56__L04b078(pc),a0 | +01c
        jsr     0x28cd4.l                       | +020
        bsr.w   HumanDeath_InitBurst_04a6b2     | +026
        jsr     0x13600.l                       | +02a
        move.w  #0x19,d1                        | +030
        jsr     0x236e.l                        | +034
        lea     .L04a7d8(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L04a7d8:
        jsr     HumanDeath_PhysicsGround_04a268(pc) | +040
        jsr     0x28d70.l                       | +044
        tst.b   0x70(a6)                        | +04a
        beq.w   .L04a7f0                        | +04e
        lea     HumanDeath_Launched_04a7f4(pc),a1 | +052
        move.l  a1,(a6)                         | +056
.L04a7f0:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +058

| ----------------------------------------------------------------------------
|  HumanDeath_Launched_04a7f4  @ $04A7F4  (444 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_Launched_04a7f4, "ax", @progbits
        .global HumanDeath_Launched_04a7f4
HumanDeath_Launched_04a7f4:
        lea     HumanDeath_SpriteTbls_04ac56__L04b0c6(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +00a
        .global HumanDeath_Launched_04a7f4__L04a802
HumanDeath_Launched_04a7f4__L04a802:
.L04a802:
        jsr     0x27f60.l                       | +00e
        scc.b   0x70(a6)                        | +014
        jsr     0x13600.l                       | +018
        move.w  #0x8a,d1                        | +01e
        jsr     0x236e.l                        | +022
        move.w  #0xfffc,d0                      | +028
        jsr     0x5dca4.l                       | +02c
        move.w  d0,0x28(a6)                     | +032
        move.w  #0x870,0x2a(a6)                 | +036
        move.w  #0xffb8,0x2e(a6)                | +03c
        move.w  #0x0,0x2c(a6)                   | +042
        lea     HumanDeath_SpriteTbls_04ac56__L04b7ec(pc),a0 | +048
        jsr     0x28cd4.l                       | +04c
        move.w  #0x2800,0x5c(a6)                | +052
        move.w  #0xffff,0x38(a6)                | +058
        bset    #0x6,0x12(a6)                   | +05e
        lea     .L04a85e(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L04a85e:
        move.w  0x5c(a6),d0                     | +06a
        move.w  d0,d1                           | +06e
        lsr.w   #0x5,d0                         | +070
        add.w   d0,d1                           | +072
        bcc.w   .L04a86e                        | +074
        moveq   #-1,d1                          | +078
.L04a86e:
        move.w  d1,0x5c(a6)                     | +07a
        lsr.w   #0x8,d1                         | +07e
        move.b  d1,0x32(a6)                     | +080
        move.b  d1,0x33(a6)                     | +084
        jsr     0x27cee.l                       | +088
        jsr     0x28d70.l                       | +08e
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +094
        jsr     0x27f60.l                       | +098
        scc.b   0x70(a6)                        | +09e
        jmp     HumanDeath_BurstLand_04a798__L04a7b0(pc) | +0a2
        lea     HumanDeath_SpriteTbls_04ac56__L04b1f6(pc),a0 | +0a6
        jsr     0x28cd4.l                       | +0aa
        movea.l 0x50(a6),a0                     | +0b0
        cmpi.w  #0x0,0x5c(a0)                   | +0b4
        beq.w   .L04a8bc                        | +0ba
        bset    #0x0,0x3a(a6)                   | +0be
        bra.w   .L04a8c2                        | +0c4
.L04a8bc:
        bclr    #0x0,0x3a(a6)                   | +0c8
.L04a8c2:
        move.w  #0xf800,d0                      | +0ce
        bsr.w   HumanDeath_SetVelXByFacing_04a1f0 | +0d2
        move.w  #0x100,0x2a(a6)                 | +0d6
        addq.w  #0x2,0x24(a6)                   | +0dc
        bsr.w   HumanDeath_SpawnBloodSplash_04aad2 | +0e0
        lea     .L04a8de(pc),a1                 | +0e4
        move.l  a1,(a6)                         | +0e8
.L04a8de:
        bsr.w   HumanDeath_PhysicsAir_04a218    | +0ea
        bsr.w   HumanDeath_PhysicsFall_04a2be   | +0ee
        jsr     0x28d70.l                       | +0f2
        bcc.w   .L04a8f6                        | +0f8
        lea     HumanDeath_FadeOut_04a39e(pc),a1 | +0fc
        move.l  a1,(a6)                         | +100
.L04a8f6:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +102
        clr.w   0x28(a6)                        | +106
        lea     HumanDeath_SpriteTbls_04ac56__L04af08(pc),a0 | +10a
        jsr     0x28cd4.l                       | +10e
        move.w  #0x800,0x2a(a6)                 | +114
        addq.w  #0x2,0x24(a6)                   | +11a
        clr.w   0x28(a6)                        | +11e
        bsr.w   HumanDeath_SpawnBloodSplash_04aad2 | +122
        lea     .L04a920(pc),a1                 | +126
        move.l  a1,(a6)                         | +12a
.L04a920:
        jsr     HumanDeath_PhysicsAir_04a218(pc) | +12c
        clr.w   0x2e(a6)                        | +130
        bsr.w   HumanDeath_PhysicsFall_04a2be   | +134
        jsr     0x5e9b6.l                       | +138
        jsr     HumanDeath_PickGibPtr_04aba8(pc) | +13e
        tst.b   0x70(a6)                        | +142
        beq.w   .L04a944                        | +146
        lea     HumanDeath_BurstLand_04a798(pc),a1 | +14a
        move.l  a1,(a6)                         | +14e
.L04a944:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +150
        move.w  #0x8000,d0                      | +154
        jsr     0x28134.l                       | +158
        andi.w  #0xffe3,0x38(a6)                | +15e
        ori.w   #0x0,0x38(a6)                   | +164
        jsr     0x5e9b6.l                       | +16a
        andi.b  #0x3,d0                         | +170
        beq.w   HumanDeath_Burning_04a9b0__L04a9be | +174
        lea     HumanDeath_SpriteTbls_04ac56__L04b82a(pc),a0 | +178
        jsr     0x28cd4.l                       | +17c
        move.w  #0x1e,0x30(a6)                  | +182
        lea     .L04a982(pc),a1                 | +188
        move.l  a1,(a6)                         | +18c
.L04a982:
        bsr.w   HumanDeath_PhysicsAir_04a218    | +18e
        jsr     0x28d70.l                       | +192
        move.w  0x30(a6),d0                     | +198
        beq.w   .L04a99e                        | +19c
        subq.w  #0x1,d0                         | +1a0
        move.w  d0,0x30(a6)                     | +1a2
        bra.w   .L04a9ac                        | +1a6
.L04a99e:
        tst.b   0x70(a6)                        | +1aa
        beq.w   .L04a9ac                        | +1ae
        lea     HumanDeath_Burning_04a9b0(pc),a1 | +1b2
        move.l  a1,(a6)                         | +1b6
.L04a9ac:
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a35e | +1b8

| ----------------------------------------------------------------------------
|  HumanDeath_Burning_04a9b0  @ $04A9B0  (178 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_Burning_04a9b0, "ax", @progbits
        .global HumanDeath_Burning_04a9b0
HumanDeath_Burning_04a9b0:
        lea     HumanDeath_SpriteTbls_04ac56__L04b898(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +00a
        .global HumanDeath_Burning_04a9b0__L04a9be
HumanDeath_Burning_04a9b0__L04a9be:
.L04a9be:
        lea     0x2b604a.l,a0                   | +00e
        jsr     0x28cd4.l                       | +014
        move.w  #0x1042,d0                      | +01a
        jsr     0x2352.l                        | +01e
        clr.b   0x21(a6)                        | +024
        lea     HumanDeath_FlameChild_04aa76(pc),a1 | +028
        jsr     0x4ae.l                         | +02c
        move.w  #0x200,d0                       | +032
        bsr.w   HumanDeath_SetVelXByFacing_04a1f0 | +036
        move.w  #0x28,0x30(a6)                  | +03a
        jsr     0x5e9b6.l                       | +040
        tst.w   d0                              | +046
        bpl.w   .L04aa06                        | +048
        move.b  #0xa,0x72(a6)                   | +04c
        bra.w   .L04aa0c                        | +052
.L04aa06:
        move.b  #0xff,0x72(a6)                  | +056
.L04aa0c:
        move.b  0x72(a6),0x73(a6)               | +05c
        move.w  #0xffc0,0x2e(a6)                | +062
        lea     .L04aa1e(pc),a1                 | +068
        move.l  a1,(a6)                         | +06c
.L04aa1e:
        jsr     HumanDeath_PhysicsGround_04a268(pc) | +06e
        jsr     0x28d70.l                       | +072
        btst    #0x5,0x5a(a6)                   | +078
        beq.w   .L04aa38                        | +07e
        lea     HumanDeath_BurnedDown_04aa62(pc),a1 | +082
        move.l  a1,(a6)                         | +086
.L04aa38:
        subq.w  #0x1,0x30(a6)                   | +088
        bne.w   .L04aa46                        | +08c
        lea     HumanDeath_BurnedDown_04aa62(pc),a1 | +090
        move.l  a1,(a6)                         | +094
.L04aa46:
        subq.b  #0x1,0x73(a6)                   | +096
        bne.w   .L04aa5e                        | +09a
        bchg    #0x0,0x3a(a6)                   | +09e
        move.b  0x72(a6),0x73(a6)               | +0a4
        neg.w   0x28(a6)                        | +0aa
.L04aa5e:
        jmp     HumanDeath_PhysicsFall_04a2be__L04a35e(pc) | +0ae

| ----------------------------------------------------------------------------
|  HumanDeath_BurnedDown_04aa62  @ $04AA62  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_BurnedDown_04aa62, "ax", @progbits
        .global HumanDeath_BurnedDown_04aa62
HumanDeath_BurnedDown_04aa62:
        lea     HumanDeath_SpriteTbls_04ac56__L04b0c6(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        move.b  #0xff,0x21(a6)                  | +00a
        bra.w   HumanDeath_PhysicsFall_04a2be__L04a382 | +010

| ----------------------------------------------------------------------------
|  HumanDeath_FlameChild_04aa76  @ $04AA76  (84 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_FlameChild_04aa76, "ax", @progbits
        .global HumanDeath_FlameChild_04aa76
HumanDeath_FlameChild_04aa76:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     HumanDeath_SpriteTbls_04ac56__L04b926(pc),a0 | +00a
        jsr     0x28cd4.l                       | +00e
        lea     .L04aa90(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L04aa90:
        jsr     0x7b2.l                         | +01a
        bcc.w   .L04aaa0                        | +020
        jmp     0x518.l                         | +024
.L04aaa0:
        movea.l 0xc(a6),a0                      | +02a
        tst.b   0x21(a0)                        | +02e
        beq.w   .L04aab2                        | +032
        jmp     0x518.l                         | +036
.L04aab2:
        move.w  0x22(a0),0x22(a6)               | +03c
        move.w  0x24(a0),d0                     | +042
        addi.w  #0x1e,d0                        | +046
        move.w  d0,0x24(a6)                     | +04a
        move.w  0x38(a0),0x38(a6)               | +04e

| ----------------------------------------------------------------------------
|  HumanDeath_SpawnBloodSplash_04aad2  @ $04AAD2  (62 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_SpawnBloodSplash_04aad2, "ax", @progbits
        .global HumanDeath_SpawnBloodSplash_04aad2
HumanDeath_SpawnBloodSplash_04aad2:
        lea     HumanDeath_BloodSplashA_04ab10(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        bra.w   .L04aaea                        | +00a
        .global HumanDeath_SpawnBloodSplash_04aad2__L04aae0
HumanDeath_SpawnBloodSplash_04aad2__L04aae0:
.L04aae0:
        lea     HumanDeath_BloodSplashB_04ab52(pc),a1 | +00e
        jsr     0x4ae.l                         | +012
.L04aaea:
        move.w  0x24(a6),0x24(a0)               | +018
        move.w  #0x8,d0                         | +01e
        btst    #0x0,0x3a(a6)                   | +022
        bne.w   .L04ab06                        | +028
        bset    #0x0,0x3a(a0)                   | +02c
        neg.w   d0                              | +032
.L04ab06:
        add.w   0x22(a6),d0                     | +034
        move.w  d0,0x22(a0)                     | +038
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  HumanDeath_BloodSplashA_04ab10  @ $04AB10  (66 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_BloodSplashA_04ab10, "ax", @progbits
        .global HumanDeath_BloodSplashA_04ab10
HumanDeath_BloodSplashA_04ab10:
        move.w  #0x17a,d1                       | +000
        tst.b   0x10fd8f.l                      | +004
        bne.w   .L04ab22                        | +00a
        move.w  #0x1ba,d1                       | +00e
.L04ab22:
        jsr     0x236e.l                        | +012
        move.w  #0xc000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x10,0x38(a6)                  | +028
        lea     HumanDeath_SpriteTbls_04ac56__L04bad2(pc),a0 | +02e
        jsr     0x28cd4.l                       | +032
        lea     HumanDeath_BloodSplash_Loop_04ab90(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
        bra.w   HumanDeath_BloodSplash_Loop_04ab90 | +03e

| ----------------------------------------------------------------------------
|  HumanDeath_BloodSplashB_04ab52  @ $04AB52  (62 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_BloodSplashB_04ab52, "ax", @progbits
        .global HumanDeath_BloodSplashB_04ab52
HumanDeath_BloodSplashB_04ab52:
        move.w  #0x17a,d1                       | +000
        tst.b   0x10fd8f.l                      | +004
        bne.w   .L04ab64                        | +00a
        move.w  #0x1ba,d1                       | +00e
.L04ab64:
        jsr     0x236e.l                        | +012
        move.w  #0xc000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x10,0x38(a6)                  | +028
        lea     HumanDeath_SpriteTbls_04ac56__L04b9f0(pc),a0 | +02e
        jsr     0x28cd4.l                       | +032
        lea     HumanDeath_BloodSplash_Loop_04ab90(pc),a1 | +038
        move.l  a1,(a6)                         | +03c

| ----------------------------------------------------------------------------
|  HumanDeath_BloodSplash_Loop_04ab90  @ $04AB90  (24 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_BloodSplash_Loop_04ab90, "ax", @progbits
        .global HumanDeath_BloodSplash_Loop_04ab90
HumanDeath_BloodSplash_Loop_04ab90:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L04aba6                        | +00c
        jmp     0x518.l                         | +010
.L04aba6:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  HumanDeath_PickGibPtr_04aba8  @ $04ABA8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_PickGibPtr_04aba8, "ax", @progbits
        .global HumanDeath_PickGibPtr_04aba8
HumanDeath_PickGibPtr_04aba8:
        andi.w  #0x7,d0                         | +000
        lea     HumanDeath_SpriteTbls_04ac56__L04af5a(pc),a0 | +004
        asl.w   #0x2,d0                         | +008
        move.l  (a0,d0.w),0x76(a6)              | +00a

| ----------------------------------------------------------------------------
|  HumanDeath_SpawnSmokePair_04abc0  @ $04ABC0  (106 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_SpawnSmokePair_04abc0, "ax", @progbits
        .global HumanDeath_SpawnSmokePair_04abc0
HumanDeath_SpawnSmokePair_04abc0:
        lea     HumanDeath_SmokeChild_04ac3a(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0x18,0x24(a0)                  | +010
        jsr     0x13600.l                       | +016
        move.w  #0x19f,d1                       | +01c
        tst.b   0x10fd8f.l                      | +020
        bne.w   .L04abee                        | +026
        move.w  #0x1a3,d1                       | +02a
.L04abee:
        jsr     0x236e.l                        | +02e
        lea     HumanDeath_SpriteTbls_04ac56__L04bb44(pc),a0 | +034
        jsr     0x28cd4.l                       | +038
        lea     .L04ac04(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
        .global HumanDeath_SpawnSmokePair_04abc0__L04ac04
HumanDeath_SpawnSmokePair_04abc0__L04ac04:
.L04ac04:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        bcc.w   .L04ac1a                        | +050
        lea     JmpToScheduler_04ac32(pc),a1    | +054
        move.l  a1,(a6)                         | +058
.L04ac1a:
        movea.l #0xffffffff,a0                  | +05a
        jsr     0x5dd56.l                       | +060
        bcc.w   SetHandlerRts_04ac30            | +066

| ----------------------------------------------------------------------------
|  HumanDeath_SmokeChild_04ac3a  @ $04AC3A  (28 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_SmokeChild_04ac3a, "ax", @progbits
        .global HumanDeath_SmokeChild_04ac3a
HumanDeath_SmokeChild_04ac3a:
        move.w  #0xd,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     HumanDeath_SmokeSprite_04bb4a(pc),a0 | +00a
        jsr     0x28cd4.l                       | +00e
        lea     HumanDeath_SpawnSmokePair_04abc0__L04ac04(pc),a1 | +014
        move.l  a1,(a6)                         | +018
        bra.b   HumanDeath_SpawnSmokePair_04abc0__L04ac04 | +01a

| ----------------------------------------------------------------------------
|  HumanDeath_SpriteTbls_04ac56  @ $04AC56  (3828 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_SpriteTbls_04ac56, "ax", @progbits
        .global HumanDeath_SpriteTbls_04ac56
HumanDeath_SpriteTbls_04ac56:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +054  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +068  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +074  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +076  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +078  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +080  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +084  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +098  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a6  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04acfe
HumanDeath_SpriteTbls_04ac56__L04acfe:
.L04acfe:
        .dc.w   0x0800                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0xa0d4                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x1043                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x279a                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x27ae                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x27c8                        | +0da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x27e2                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x27fc                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x2816                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +100  (dato / opcode no decodificado)
        .dc.w   0x2830                        | +102  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x284a                        | +10c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +114  (dato / opcode no decodificado)
        .dc.w   0x2864                        | +116  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x287e                        | +120  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +128  (dato / opcode no decodificado)
        .dc.w   0x2898                        | +12a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +132  (dato / opcode no decodificado)
        .dc.w   0x28b2                        | +134  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +136  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +138  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04ad90
HumanDeath_SpriteTbls_04ac56__L04ad90:
.L04ad90:
        .dc.w   0x0800                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +13c  (dato / opcode no decodificado)
        .dc.w   0xa0ea                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +140  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +142  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +146  (dato / opcode no decodificado)
        .dc.w   0x1040                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +14e  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +154  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +156  (dato / opcode no decodificado)
        .dc.w   0x1f88                        | +158  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +160  (dato / opcode no decodificado)
        .dc.w   0x1f9c                        | +162  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +164  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +166  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +168  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x1fb0                        | +16c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +170  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +174  (dato / opcode no decodificado)
        .dc.w   0x1fc4                        | +176  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +178  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x1fd8                        | +180  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +184  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +186  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +188  (dato / opcode no decodificado)
        .dc.w   0x1fec                        | +18a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +190  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +192  (dato / opcode no decodificado)
        .dc.w   0x2000                        | +194  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +196  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +198  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +19c  (dato / opcode no decodificado)
        .dc.w   0x2014                        | +19e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0x2028                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0x203c                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0x2050                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1be  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0x2064                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +1ca  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04ae22
HumanDeath_SpriteTbls_04ac56__L04ae22:
.L04ae22:
        .dc.w   0x0800                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0xa0ea                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0x103f                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0x2072                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0x2086                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0x209a                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +200  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +202  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +204  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +206  (dato / opcode no decodificado)
        .dc.w   0x20ae                        | +208  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +20c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +20e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +210  (dato / opcode no decodificado)
        .dc.w   0x20c2                        | +212  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +214  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +216  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +218  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +21a  (dato / opcode no decodificado)
        .dc.w   0x20d6                        | +21c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +21e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +220  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +222  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +224  (dato / opcode no decodificado)
        .dc.w   0x20ea                        | +226  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +228  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +22a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +22c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +22e  (dato / opcode no decodificado)
        .dc.w   0x20fe                        | +230  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +232  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +234  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +236  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +238  (dato / opcode no decodificado)
        .dc.w   0x2112                        | +23a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +23c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +23e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +240  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +242  (dato / opcode no decodificado)
        .dc.w   0x2126                        | +244  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +246  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +248  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +24a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +24c  (dato / opcode no decodificado)
        .dc.w   0x213a                        | +24e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +250  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +252  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +254  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +256  (dato / opcode no decodificado)
        .dc.w   0x214e                        | +258  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +25a  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +25c  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04aeb4
HumanDeath_SpriteTbls_04ac56__L04aeb4:
.L04aeb4:
        .dc.w   0x0001                        | +25e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +260  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +262  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +264  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +266  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +268  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +26a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +26c  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +26e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +270  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +272  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +274  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +276  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +278  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +27a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +27c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +27e  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +280  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +282  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +284  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +286  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +288  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +28a  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +28c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +28e  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +290  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +292  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +294  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +296  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +298  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +29a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +29c  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +29e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +2a0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +2a2  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +2a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2a6  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +2a8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +2aa  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +2ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2ae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2b0  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04af08
HumanDeath_SpriteTbls_04ac56__L04af08:
.L04af08:
        .dc.w   0x0600                        | +2b2  (dato / opcode no decodificado)
        .dc.w   0x1042                        | +2b4  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +2b6  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +2b8  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +2ba  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +2bc  (dato / opcode no decodificado)
        .dc.w   0x0500                        | +2be  (dato / opcode no decodificado)
        .dc.w   0x003b                        | +2c0  (dato / opcode no decodificado)
        .dc.w   0x1a76                        | +2c2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2c4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2c6  (dato / opcode no decodificado)
        .dc.w   0x1c78                        | +2c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2ca  (dato / opcode no decodificado)
        .dc.w   0x1a76                        | +2cc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2ce  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2d0  (dato / opcode no decodificado)
        .dc.w   0x1c92                        | +2d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2d4  (dato / opcode no decodificado)
        .dc.w   0x1a76                        | +2d6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2d8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2da  (dato / opcode no decodificado)
        .dc.w   0x1cac                        | +2dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2de  (dato / opcode no decodificado)
        .dc.w   0x1a76                        | +2e0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2e2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2e4  (dato / opcode no decodificado)
        .dc.w   0x1cc6                        | +2e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2e8  (dato / opcode no decodificado)
        .dc.w   0x1a76                        | +2ea  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2ec  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2ee  (dato / opcode no decodificado)
        .dc.w   0x1cac                        | +2f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2f2  (dato / opcode no decodificado)
        .dc.w   0x1a76                        | +2f4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2f6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2f8  (dato / opcode no decodificado)
        .dc.w   0x1c92                        | +2fa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2fc  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +2fe  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +300  (dato / opcode no decodificado)
        .dc.w   0xaf14                        | +302  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04af5a
HumanDeath_SpriteTbls_04ac56__L04af5a:
.L04af5a:
        .dc.w   0x0004                        | +304  (dato / opcode no decodificado)
        .dc.w   0xaf7a                        | +306  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +308  (dato / opcode no decodificado)
        .dc.w   0xaf80                        | +30a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +30c  (dato / opcode no decodificado)
        .dc.w   0xaf86                        | +30e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +310  (dato / opcode no decodificado)
        .dc.w   0xaf86                        | +312  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +314  (dato / opcode no decodificado)
        .dc.w   0xaf8c                        | +316  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +318  (dato / opcode no decodificado)
        .dc.w   0xaf92                        | +31a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +31c  (dato / opcode no decodificado)
        .dc.w   0xaf98                        | +31e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +320  (dato / opcode no decodificado)
        .dc.w   0xaf98                        | +322  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +324  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +326  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +328  (dato / opcode no decodificado)
        .dc.w   0x0202                        | +32a  (dato / opcode no decodificado)
        .dc.w   0x0202                        | +32c  (dato / opcode no decodificado)
        .dc.w   0x0202                        | +32e  (dato / opcode no decodificado)
        .dc.w   0x0203                        | +330  (dato / opcode no decodificado)
        .dc.w   0x0203                        | +332  (dato / opcode no decodificado)
        .dc.w   0x0203                        | +334  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +336  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +338  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +33a  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +33c  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +33e  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +340  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +342  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +344  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +346  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04af9e
HumanDeath_SpriteTbls_04ac56__L04af9e:
.L04af9e:
        .dc.w   0x0900                        | +348  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +34a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +34c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +34e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +350  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +352  (dato / opcode no decodificado)
        .dc.w   0x1acc                        | +354  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +356  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +358  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +35a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +35c  (dato / opcode no decodificado)
        .dc.w   0x1ae0                        | +35e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +360  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +362  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +364  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +366  (dato / opcode no decodificado)
        .dc.w   0x1af4                        | +368  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +36a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +36c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +36e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +370  (dato / opcode no decodificado)
        .dc.w   0x1b08                        | +372  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +374  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +376  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +378  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +37a  (dato / opcode no decodificado)
        .dc.w   0x1b18                        | +37c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +37e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +380  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +382  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +384  (dato / opcode no decodificado)
        .dc.w   0x1b28                        | +386  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +388  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +38a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +38c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +38e  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +390  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +392  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +394  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +396  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +398  (dato / opcode no decodificado)
        .dc.w   0x1b48                        | +39a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +39c  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +39e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +3a0  (dato / opcode no decodificado)
        .dc.w   0xb052                        | +3a2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +3a4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3a6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +3a8  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +3aa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3ac  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +3ae  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3b0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +3b2  (dato / opcode no decodificado)
        .dc.w   0x1b28                        | +3b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3b6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +3b8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3ba  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +3bc  (dato / opcode no decodificado)
        .dc.w   0x1b18                        | +3be  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3c0  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +3c2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3c4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +3c6  (dato / opcode no decodificado)
        .dc.w   0x1b28                        | +3c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3ca  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +3cc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3ce  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +3d0  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +3d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3d4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +3d6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3d8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +3da  (dato / opcode no decodificado)
        .dc.w   0x1b48                        | +3dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3de  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +3e0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3e2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +3e4  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +3e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3e8  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +3ea  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3ec  (dato / opcode no decodificado)
        .dc.w   0x67e2                        | +3ee  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +3f0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3f2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +3f4  (dato / opcode no decodificado)
        .dc.w   0x1b48                        | +3f6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3f8  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +3fa  (dato / opcode no decodificado)
        .dc.w   0x546e                        | +3fc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +3fe  (dato / opcode no decodificado)
        .dc.w   0x303c                        | +400  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +402  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +404  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +406  (dato / opcode no decodificado)
        .dc.w   0xdca4                        | +408  (dato / opcode no decodificado)
        .dc.w   0x3d40                        | +40a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +40c  (dato / opcode no decodificado)
        .dc.w   0x3d7c                        | +40e  (dato / opcode no decodificado)
        .dc.w   0x0180                        | +410  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +412  (dato / opcode no decodificado)
        .dc.w   0x3d7c                        | +414  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +416  (dato / opcode no decodificado)
        .dc.w   0x002e                        | +418  (dato / opcode no decodificado)
        .dc.w   0x3d7c                        | +41a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +41c  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +41e  (dato / opcode no decodificado)
        .dc.w   0x4e75                        | +420  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b078
HumanDeath_SpriteTbls_04ac56__L04b078:
.L04b078:
        .dc.w   0x0600                        | +422  (dato / opcode no decodificado)
        .dc.w   0x1042                        | +424  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +426  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +428  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +42a  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +42c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +42e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +430  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +432  (dato / opcode no decodificado)
        .dc.w   0x1e66                        | +434  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +436  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +438  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +43a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +43c  (dato / opcode no decodificado)
        .dc.w   0x1e80                        | +43e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +440  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +442  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +444  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +446  (dato / opcode no decodificado)
        .dc.w   0x1e9a                        | +448  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +44a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +44c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +44e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +450  (dato / opcode no decodificado)
        .dc.w   0x1eb4                        | +452  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +454  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +456  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +458  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +45a  (dato / opcode no decodificado)
        .dc.w   0x1ece                        | +45c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +45e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +460  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +462  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +464  (dato / opcode no decodificado)
        .dc.w   0x1e9a                        | +466  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +468  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +46a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +46c  (dato / opcode no decodificado)
        .dc.w   0xb084                        | +46e  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b0c6
HumanDeath_SpriteTbls_04ac56__L04b0c6:
.L04b0c6:
        .dc.w   0x0001                        | +470  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +472  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +474  (dato / opcode no decodificado)
        .dc.w   0x1ee8                        | +476  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +478  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +47a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +47c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +47e  (dato / opcode no decodificado)
        .dc.w   0x1efc                        | +480  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +482  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +484  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +486  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +488  (dato / opcode no decodificado)
        .dc.w   0x1f10                        | +48a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +48c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +48e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +490  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +492  (dato / opcode no decodificado)
        .dc.w   0x1f24                        | +494  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +496  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +498  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +49a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +49c  (dato / opcode no decodificado)
        .dc.w   0x1f38                        | +49e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4a0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4a2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +4a4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +4a6  (dato / opcode no decodificado)
        .dc.w   0x1f48                        | +4a8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4aa  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4ac  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +4ae  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +4b0  (dato / opcode no decodificado)
        .dc.w   0x1f58                        | +4b2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4b4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4b6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +4b8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +4ba  (dato / opcode no decodificado)
        .dc.w   0x1f68                        | +4bc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4be  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4c0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +4c2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +4c4  (dato / opcode no decodificado)
        .dc.w   0x1f78                        | +4c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4c8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4ca  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +4cc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +4ce  (dato / opcode no decodificado)
        .dc.w   0x1f68                        | +4d0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4d2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4d4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +4d6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +4d8  (dato / opcode no decodificado)
        .dc.w   0x1f78                        | +4da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4dc  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +4de  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b136
HumanDeath_SpriteTbls_04ac56__L04b136:
.L04b136:
        .dc.w   0x0800                        | +4e0  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +4e2  (dato / opcode no decodificado)
        .dc.w   0xa0d4                        | +4e4  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b13c
HumanDeath_SpriteTbls_04ac56__L04b13c:
.L04b13c:
        .dc.w   0x0900                        | +4e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4e8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +4ea  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +4ec  (dato / opcode no decodificado)
        .dc.w   0x1043                        | +4ee  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +4f0  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +4f2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +4f4  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +4f6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +4f8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +4fa  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +4fc  (dato / opcode no decodificado)
        .dc.w   0x215c                        | +4fe  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +500  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +502  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +504  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +506  (dato / opcode no decodificado)
        .dc.w   0x2170                        | +508  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +50a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +50c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +50e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +510  (dato / opcode no decodificado)
        .dc.w   0x2184                        | +512  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +514  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +516  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +518  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +51a  (dato / opcode no decodificado)
        .dc.w   0x2198                        | +51c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +51e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +520  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +522  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +524  (dato / opcode no decodificado)
        .dc.w   0x21b2                        | +526  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +528  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +52a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +52c  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +52e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +530  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +532  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +534  (dato / opcode no decodificado)
        .dc.w   0x21cc                        | +536  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +538  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +53a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +53c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +53e  (dato / opcode no decodificado)
        .dc.w   0x21e6                        | +540  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +542  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +544  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +546  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +548  (dato / opcode no decodificado)
        .dc.w   0x2200                        | +54a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +54c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +54e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +550  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +552  (dato / opcode no decodificado)
        .dc.w   0x221a                        | +554  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +556  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +558  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +55a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +55c  (dato / opcode no decodificado)
        .dc.w   0x2234                        | +55e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +560  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +562  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +564  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +566  (dato / opcode no decodificado)
        .dc.w   0x224e                        | +568  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +56a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +56c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +56e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +570  (dato / opcode no decodificado)
        .dc.w   0x2268                        | +572  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +574  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +576  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +578  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +57a  (dato / opcode no decodificado)
        .dc.w   0x2282                        | +57c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +57e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +580  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +582  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +584  (dato / opcode no decodificado)
        .dc.w   0x229c                        | +586  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +588  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +58a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +58c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +58e  (dato / opcode no decodificado)
        .dc.w   0x22b0                        | +590  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +592  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +594  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +596  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +598  (dato / opcode no decodificado)
        .dc.w   0x22c4                        | +59a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +59c  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +59e  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b1f6
HumanDeath_SpriteTbls_04ac56__L04b1f6:
.L04b1f6:
        .dc.w   0x0600                        | +5a0  (dato / opcode no decodificado)
        .dc.w   0x103f                        | +5a2  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +5a4  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +5a6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +5a8  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +5aa  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +5ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5ae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5b0  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +5b2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +5b4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +5b6  (dato / opcode no decodificado)
        .dc.w   0x1cfa                        | +5b8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5ba  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +5bc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5be  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +5c0  (dato / opcode no decodificado)
        .dc.w   0x1d0e                        | +5c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5c4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5c8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5ca  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5cc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +5ce  (dato / opcode no decodificado)
        .dc.w   0x1d28                        | +5d0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5d2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5d4  (dato / opcode no decodificado)
        .dc.w   0xfffe                        | +5d6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +5d8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5da  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +5dc  (dato / opcode no decodificado)
        .dc.w   0x1d42                        | +5de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5e0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5e2  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +5e4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +5e6  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5e8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +5ea  (dato / opcode no decodificado)
        .dc.w   0x1d56                        | +5ec  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5ee  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5f0  (dato / opcode no decodificado)
        .dc.w   0xfffb                        | +5f2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +5f4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +5f6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +5f8  (dato / opcode no decodificado)
        .dc.w   0x1d6a                        | +5fa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +5fc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +5fe  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +600  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +602  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +604  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +606  (dato / opcode no decodificado)
        .dc.w   0x1d7e                        | +608  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +60a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +60c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +60e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +610  (dato / opcode no decodificado)
        .dc.w   0x1d8c                        | +612  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +614  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +616  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +618  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +61a  (dato / opcode no decodificado)
        .dc.w   0x1d7e                        | +61c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +61e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +620  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +622  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +624  (dato / opcode no decodificado)
        .dc.w   0x1d8c                        | +626  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +628  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +62a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +62c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +62e  (dato / opcode no decodificado)
        .dc.w   0x1d7e                        | +630  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +632  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +634  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b28c
HumanDeath_SpriteTbls_04ac56__L04b28c:
.L04b28c:
        .dc.w   0x0600                        | +636  (dato / opcode no decodificado)
        .dc.w   0x103f                        | +638  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +63a  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +63c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +63e  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +640  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +642  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +644  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +646  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +648  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +64a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +64c  (dato / opcode no decodificado)
        .dc.w   0x2c26                        | +64e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +650  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +652  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +654  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +656  (dato / opcode no decodificado)
        .dc.w   0x2c40                        | +658  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +65a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +65c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +65e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +660  (dato / opcode no decodificado)
        .dc.w   0x2c5a                        | +662  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +664  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +666  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +668  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +66a  (dato / opcode no decodificado)
        .dc.w   0x2c74                        | +66c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +66e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +670  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +672  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +674  (dato / opcode no decodificado)
        .dc.w   0x2c8e                        | +676  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +678  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +67a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +67c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +67e  (dato / opcode no decodificado)
        .dc.w   0x2ca8                        | +680  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +682  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +684  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +686  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +688  (dato / opcode no decodificado)
        .dc.w   0x2cc2                        | +68a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +68c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +68e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +690  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +692  (dato / opcode no decodificado)
        .dc.w   0x2cdc                        | +694  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +696  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +698  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +69a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +69c  (dato / opcode no decodificado)
        .dc.w   0x2cf0                        | +69e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6a0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6a2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6a4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +6a6  (dato / opcode no decodificado)
        .dc.w   0x2d04                        | +6a8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6aa  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6ac  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6ae  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +6b0  (dato / opcode no decodificado)
        .dc.w   0x2d18                        | +6b2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6b4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6b6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6b8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +6ba  (dato / opcode no decodificado)
        .dc.w   0x2d2c                        | +6bc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6be  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6c0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6c2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +6c4  (dato / opcode no decodificado)
        .dc.w   0x2d40                        | +6c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6c8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6ca  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6cc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +6ce  (dato / opcode no decodificado)
        .dc.w   0x2d4e                        | +6d0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6d2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6d4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6d6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +6d8  (dato / opcode no decodificado)
        .dc.w   0x2d5c                        | +6da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6dc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6de  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6e0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +6e2  (dato / opcode no decodificado)
        .dc.w   0x2d6a                        | +6e4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6e6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +6e8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +6ea  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +6ec  (dato / opcode no decodificado)
        .dc.w   0x2d5c                        | +6ee  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +6f0  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +6f2  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b34a
HumanDeath_SpriteTbls_04ac56__L04b34a:
.L04b34a:
        .dc.w   0x0800                        | +6f4  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +6f6  (dato / opcode no decodificado)
        .dc.w   0xa0d4                        | +6f8  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +6fa  (dato / opcode no decodificado)
        .dc.w   0x1043                        | +6fc  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +6fe  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +700  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +702  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +704  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +706  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +708  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +70a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +70c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +70e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +710  (dato / opcode no decodificado)
        .dc.w   0x265e                        | +712  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +714  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +716  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +718  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +71a  (dato / opcode no decodificado)
        .dc.w   0x2672                        | +71c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +71e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +720  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +722  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +724  (dato / opcode no decodificado)
        .dc.w   0x2686                        | +726  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +728  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +72a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +72c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +72e  (dato / opcode no decodificado)
        .dc.w   0x26a0                        | +730  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +732  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +734  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +736  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +738  (dato / opcode no decodificado)
        .dc.w   0x26ba                        | +73a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +73c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +73e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +740  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +742  (dato / opcode no decodificado)
        .dc.w   0x26d4                        | +744  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +746  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +748  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +74a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +74c  (dato / opcode no decodificado)
        .dc.w   0x26ee                        | +74e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +750  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +752  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +754  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +756  (dato / opcode no decodificado)
        .dc.w   0x2708                        | +758  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +75a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +75c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +75e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +760  (dato / opcode no decodificado)
        .dc.w   0x2722                        | +762  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +764  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +766  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +768  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +76a  (dato / opcode no decodificado)
        .dc.w   0x273c                        | +76c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +76e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +770  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +772  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +774  (dato / opcode no decodificado)
        .dc.w   0x2756                        | +776  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +778  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +77a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +77c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +77e  (dato / opcode no decodificado)
        .dc.w   0x2770                        | +780  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +782  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +784  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +786  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +788  (dato / opcode no decodificado)
        .dc.w   0x277e                        | +78a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +78c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +78e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +790  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +792  (dato / opcode no decodificado)
        .dc.w   0x2770                        | +794  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +796  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +798  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +79a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +79c  (dato / opcode no decodificado)
        .dc.w   0x278c                        | +79e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7a0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +7a2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +7a4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +7a6  (dato / opcode no decodificado)
        .dc.w   0x2770                        | +7a8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7aa  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +7ac  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +7ae  (dato / opcode no decodificado)
        .dc.w   0x1041                        | +7b0  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +7b2  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +7b4  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +7b6  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +7b8  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +7ba  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +7bc  (dato / opcode no decodificado)
        .dc.w   0xacaa                        | +7be  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +7c0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +7c2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +7c4  (dato / opcode no decodificado)
        .dc.w   0x1a90                        | +7c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7c8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +7ca  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +7cc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +7ce  (dato / opcode no decodificado)
        .dc.w   0x1aa4                        | +7d0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7d2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +7d4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +7d6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +7d8  (dato / opcode no decodificado)
        .dc.w   0x1a90                        | +7da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7dc  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +7de  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +7e0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +7e2  (dato / opcode no decodificado)
        .dc.w   0x1aa4                        | +7e4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7e6  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +7e8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +7ea  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +7ec  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +7ee  (dato / opcode no decodificado)
        .dc.w   0x1ab8                        | +7f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7f2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +7f4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +7f6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +7f8  (dato / opcode no decodificado)
        .dc.w   0x1aa4                        | +7fa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +7fc  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +7fe  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +800  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +802  (dato / opcode no decodificado)
        .dc.w   0x1ab8                        | +804  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +806  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +808  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +80a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +80c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +80e  (dato / opcode no decodificado)
        .dc.w   0x1aa4                        | +810  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +812  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +814  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +816  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +818  (dato / opcode no decodificado)
        .dc.w   0x1ab8                        | +81a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +81c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +81e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +820  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +822  (dato / opcode no decodificado)
        .dc.w   0x1acc                        | +824  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +826  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +828  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +82a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +82c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +82e  (dato / opcode no decodificado)
        .dc.w   0x1ab8                        | +830  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +832  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +834  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +836  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +838  (dato / opcode no decodificado)
        .dc.w   0x1acc                        | +83a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +83c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +83e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +840  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +842  (dato / opcode no decodificado)
        .dc.w   0x1ae0                        | +844  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +846  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +848  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +84a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +84c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +84e  (dato / opcode no decodificado)
        .dc.w   0x1acc                        | +850  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +852  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +854  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +856  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +858  (dato / opcode no decodificado)
        .dc.w   0x1ae0                        | +85a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +85c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +85e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +860  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +862  (dato / opcode no decodificado)
        .dc.w   0x1af4                        | +864  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +866  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +868  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +86a  (dato / opcode no decodificado)
        .dc.w   0x1041                        | +86c  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +86e  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +870  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +872  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +874  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +876  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +878  (dato / opcode no decodificado)
        .dc.w   0xacaa                        | +87a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +87c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +87e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +880  (dato / opcode no decodificado)
        .dc.w   0x1b58                        | +882  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +884  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +886  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +888  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +88a  (dato / opcode no decodificado)
        .dc.w   0x1b6c                        | +88c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +88e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +890  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +892  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +894  (dato / opcode no decodificado)
        .dc.w   0x1b58                        | +896  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +898  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +89a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +89c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +89e  (dato / opcode no decodificado)
        .dc.w   0x1b6c                        | +8a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8a2  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +8a4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8a6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8a8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +8aa  (dato / opcode no decodificado)
        .dc.w   0x1b80                        | +8ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8ae  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8b0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8b2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +8b4  (dato / opcode no decodificado)
        .dc.w   0x1b6c                        | +8b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8b8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +8ba  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8bc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +8be  (dato / opcode no decodificado)
        .dc.w   0x1b80                        | +8c0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8c2  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +8c4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8c6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8c8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +8ca  (dato / opcode no decodificado)
        .dc.w   0x1b6c                        | +8cc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8ce  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8d0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8d2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +8d4  (dato / opcode no decodificado)
        .dc.w   0x1b80                        | +8d6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8d8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +8da  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8dc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +8de  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +8e0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8e2  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +8e4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8e6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8e8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +8ea  (dato / opcode no decodificado)
        .dc.w   0x1b80                        | +8ec  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8ee  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +8f0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8f2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +8f4  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +8f6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +8f8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +8fa  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +8fc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +8fe  (dato / opcode no decodificado)
        .dc.w   0x1ba8                        | +900  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +902  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +904  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +906  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +908  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +90a  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +90c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +90e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +910  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +912  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +914  (dato / opcode no decodificado)
        .dc.w   0x1ba8                        | +916  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +918  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +91a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +91c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +91e  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +920  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +922  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +924  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b57c
HumanDeath_SpriteTbls_04ac56__L04b57c:
.L04b57c:
        .dc.w   0x0900                        | +926  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +928  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +92a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +92c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +92e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +930  (dato / opcode no decodificado)
        .dc.w   0x1b58                        | +932  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +934  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +936  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +938  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +93a  (dato / opcode no decodificado)
        .dc.w   0x1b6c                        | +93c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +93e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +940  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +942  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +944  (dato / opcode no decodificado)
        .dc.w   0x1b80                        | +946  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +948  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +94a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +94c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +94e  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +950  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +952  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +954  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +956  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +958  (dato / opcode no decodificado)
        .dc.w   0x1ba8                        | +95a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +95c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +95e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +960  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +962  (dato / opcode no decodificado)
        .dc.w   0x1bbc                        | +964  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +966  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +968  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +96a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +96c  (dato / opcode no decodificado)
        .dc.w   0x1bd0                        | +96e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +970  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +972  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +974  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +976  (dato / opcode no decodificado)
        .dc.w   0x1be4                        | +978  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +97a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +97c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +97e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +980  (dato / opcode no decodificado)
        .dc.w   0x1bf4                        | +982  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +984  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +986  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +988  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +98a  (dato / opcode no decodificado)
        .dc.w   0x1c04                        | +98c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +98e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +990  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +992  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +994  (dato / opcode no decodificado)
        .dc.w   0x1c14                        | +996  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +998  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +99a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +99c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +99e  (dato / opcode no decodificado)
        .dc.w   0x1c20                        | +9a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9a2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9a4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9a6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +9a8  (dato / opcode no decodificado)
        .dc.w   0x1c14                        | +9aa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9ac  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9ae  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9b0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +9b2  (dato / opcode no decodificado)
        .dc.w   0x1c20                        | +9b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9b6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9b8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9ba  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +9bc  (dato / opcode no decodificado)
        .dc.w   0x1c14                        | +9be  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9c0  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +9c2  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b61a
HumanDeath_SpriteTbls_04ac56__L04b61a:
.L04b61a:
        .dc.w   0x0900                        | +9c4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9c8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9ca  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9cc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +9ce  (dato / opcode no decodificado)
        .dc.w   0x1b58                        | +9d0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9d2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9d4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9d6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +9d8  (dato / opcode no decodificado)
        .dc.w   0x1b6c                        | +9da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9dc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9de  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9e0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +9e2  (dato / opcode no decodificado)
        .dc.w   0x1b80                        | +9e4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9e6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +9e8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9ea  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +9ec  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +9ee  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9f0  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +9f2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9f4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +9f6  (dato / opcode no decodificado)
        .dc.w   0x1c2c                        | +9f8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +9fa  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +9fc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +9fe  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a00  (dato / opcode no decodificado)
        .dc.w   0x1c40                        | +a02  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a04  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +a06  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a08  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a0a  (dato / opcode no decodificado)
        .dc.w   0x1c54                        | +a0c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a0e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +a10  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a12  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a14  (dato / opcode no decodificado)
        .dc.w   0x1c68                        | +a16  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a18  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +a1a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a1c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a1e  (dato / opcode no decodificado)
        .dc.w   0x1b18                        | +a20  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a22  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a24  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a26  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a28  (dato / opcode no decodificado)
        .dc.w   0x1b28                        | +a2a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a2c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a2e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a30  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a32  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +a34  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a36  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a38  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a3a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a3c  (dato / opcode no decodificado)
        .dc.w   0x1b48                        | +a3e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a40  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a42  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a44  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a46  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +a48  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a4a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a4c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a4e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a50  (dato / opcode no decodificado)
        .dc.w   0x1b48                        | +a52  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a54  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a56  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a58  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a5a  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +a5c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a5e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +a60  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b6b8
HumanDeath_SpriteTbls_04ac56__L04b6b8:
.L04b6b8:
        .dc.w   0x0900                        | +a62  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a64  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a66  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +a68  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a6a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a6c  (dato / opcode no decodificado)
        .dc.w   0x1a90                        | +a6e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a70  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +a72  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a74  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a76  (dato / opcode no decodificado)
        .dc.w   0x1aa4                        | +a78  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a7a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +a7c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a7e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a80  (dato / opcode no decodificado)
        .dc.w   0x1ab8                        | +a82  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a84  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +a86  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a88  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a8a  (dato / opcode no decodificado)
        .dc.w   0x1acc                        | +a8c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a8e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a90  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a92  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a94  (dato / opcode no decodificado)
        .dc.w   0x1ae0                        | +a96  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +a98  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +a9a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +a9c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +a9e  (dato / opcode no decodificado)
        .dc.w   0x1af4                        | +aa0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +aa2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +aa4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +aa6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +aa8  (dato / opcode no decodificado)
        .dc.w   0x1b08                        | +aaa  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +aac  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +aae  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +ab0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +ab2  (dato / opcode no decodificado)
        .dc.w   0x1b18                        | +ab4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ab6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ab8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +aba  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +abc  (dato / opcode no decodificado)
        .dc.w   0x1b28                        | +abe  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ac0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ac2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +ac4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +ac6  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +ac8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +aca  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +acc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +ace  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +ad0  (dato / opcode no decodificado)
        .dc.w   0x1b48                        | +ad2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ad4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ad6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +ad8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +ada  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +adc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ade  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ae0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +ae2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +ae4  (dato / opcode no decodificado)
        .dc.w   0x1b48                        | +ae6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ae8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +aea  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +aec  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +aee  (dato / opcode no decodificado)
        .dc.w   0x1b38                        | +af0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +af2  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +af4  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b74c
HumanDeath_SpriteTbls_04ac56__L04b74c:
.L04b74c:
        .dc.w   0x0600                        | +af6  (dato / opcode no decodificado)
        .dc.w   0x103e                        | +af8  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +afa  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +afc  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +afe  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +b00  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +b02  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b04  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b06  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +b08  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b0a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b0c  (dato / opcode no decodificado)
        .dc.w   0x1d9a                        | +b0e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b10  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +b12  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b14  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b16  (dato / opcode no decodificado)
        .dc.w   0x1dae                        | +b18  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b1a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +b1c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b1e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b20  (dato / opcode no decodificado)
        .dc.w   0x1dc2                        | +b22  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b24  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +b26  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b28  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b2a  (dato / opcode no decodificado)
        .dc.w   0x1ddc                        | +b2c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b2e  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +b30  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +b32  (dato / opcode no decodificado)
        .dc.w   0xb798                        | +b34  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b78c
HumanDeath_SpriteTbls_04ac56__L04b78c:
.L04b78c:
        .dc.w   0x0600                        | +b36  (dato / opcode no decodificado)
        .dc.w   0x103e                        | +b38  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +b3a  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +b3c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +b3e  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +b40  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +b42  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b44  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b46  (dato / opcode no decodificado)
        .dc.w   0x1df6                        | +b48  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b4a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +b4c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b4e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b50  (dato / opcode no decodificado)
        .dc.w   0x1e10                        | +b52  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b54  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +b56  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b58  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b5a  (dato / opcode no decodificado)
        .dc.w   0x1e2a                        | +b5c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b5e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +b60  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b7b8
HumanDeath_SpriteTbls_04ac56__L04b7b8:
.L04b7b8:
        .dc.w   0x0001                        | +b62  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b64  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b66  (dato / opcode no decodificado)
        .dc.w   0x1e3e                        | +b68  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b6a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b6c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b6e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b70  (dato / opcode no decodificado)
        .dc.w   0x1e52                        | +b72  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b74  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b76  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b78  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b7a  (dato / opcode no decodificado)
        .dc.w   0x1e3e                        | +b7c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b7e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b80  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b82  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b84  (dato / opcode no decodificado)
        .dc.w   0x1e52                        | +b86  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b88  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +b8a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b8c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +b8e  (dato / opcode no decodificado)
        .dc.w   0x1e3e                        | +b90  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b92  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +b94  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b7ec
HumanDeath_SpriteTbls_04ac56__L04b7ec:
.L04b7ec:
        .dc.w   0x0900                        | +b96  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b98  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +b9a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +b9c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +b9e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +ba0  (dato / opcode no decodificado)
        .dc.w   0x3f3a                        | +ba2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ba4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +ba6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +ba8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +baa  (dato / opcode no decodificado)
        .dc.w   0x3fb8                        | +bac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bae  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +bb0  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +bb2  (dato / opcode no decodificado)
        .dc.w   0xb7f2                        | +bb4  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +bb6  (dato / opcode no decodificado)
        .dc.w   0x1044                        | +bb8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +bba  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +bbc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +bbe  (dato / opcode no decodificado)
        .dc.w   0x3f3a                        | +bc0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bc2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +bc4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +bc6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +bc8  (dato / opcode no decodificado)
        .dc.w   0x3fb8                        | +bca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bcc  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +bce  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +bd0  (dato / opcode no decodificado)
        .dc.w   0xb810                        | +bd2  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b82a
HumanDeath_SpriteTbls_04ac56__L04b82a:
.L04b82a:
        .dc.w   0x0800                        | +bd4  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +bd6  (dato / opcode no decodificado)
        .dc.w   0xa128                        | +bd8  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +bda  (dato / opcode no decodificado)
        .dc.w   0x1042                        | +bdc  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +bde  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +be0  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +be2  (dato / opcode no decodificado)
        .dc.w   0xa1ba                        | +be4  (dato / opcode no decodificado)
        .dc.w   0x0900                        | +be6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +be8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bea  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +bec  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +bee  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +bf0  (dato / opcode no decodificado)
        .dc.w   0x2458                        | +bf2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bf4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +bf6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +bf8  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +bfa  (dato / opcode no decodificado)
        .dc.w   0x2478                        | +bfc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +bfe  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c00  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c02  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c04  (dato / opcode no decodificado)
        .dc.w   0x2498                        | +c06  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c08  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c0a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c0c  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c0e  (dato / opcode no decodificado)
        .dc.w   0x24c0                        | +c10  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c12  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c14  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c16  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c18  (dato / opcode no decodificado)
        .dc.w   0x24f0                        | +c1a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c1c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c1e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c20  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c22  (dato / opcode no decodificado)
        .dc.w   0x2510                        | +c24  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c26  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c28  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c2a  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c2c  (dato / opcode no decodificado)
        .dc.w   0x2530                        | +c2e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c30  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c32  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c34  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c36  (dato / opcode no decodificado)
        .dc.w   0x2550                        | +c38  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c3a  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +c3c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +c3e  (dato / opcode no decodificado)
        .dc.w   0xb83c                        | +c40  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b898
HumanDeath_SpriteTbls_04ac56__L04b898:
.L04b898:
        .dc.w   0x0002                        | +c42  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c44  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c46  (dato / opcode no decodificado)
        .dc.w   0x2570                        | +c48  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c4a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c4c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c4e  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c50  (dato / opcode no decodificado)
        .dc.w   0x2590                        | +c52  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c54  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c56  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c58  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c5a  (dato / opcode no decodificado)
        .dc.w   0x25b0                        | +c5c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c5e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c60  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c62  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c64  (dato / opcode no decodificado)
        .dc.w   0x25d8                        | +c66  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c68  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c6a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c6c  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c6e  (dato / opcode no decodificado)
        .dc.w   0x2608                        | +c70  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c72  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c74  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c76  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c78  (dato / opcode no decodificado)
        .dc.w   0x2628                        | +c7a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c7c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c7e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c80  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c82  (dato / opcode no decodificado)
        .dc.w   0x2648                        | +c84  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c86  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c88  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c8a  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c8c  (dato / opcode no decodificado)
        .dc.w   0x2668                        | +c8e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c90  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c92  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c94  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +c96  (dato / opcode no decodificado)
        .dc.w   0x2688                        | +c98  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +c9a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +c9c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +c9e  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +ca0  (dato / opcode no decodificado)
        .dc.w   0x26a8                        | +ca2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ca4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +ca6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +ca8  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +caa  (dato / opcode no decodificado)
        .dc.w   0x26c8                        | +cac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +cae  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +cb0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +cb2  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +cb4  (dato / opcode no decodificado)
        .dc.w   0x26e8                        | +cb6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +cb8  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +cba  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +cbc  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +cbe  (dato / opcode no decodificado)
        .dc.w   0x2712                        | +cc0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +cc2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +cc4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +cc6  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +cc8  (dato / opcode no decodificado)
        .dc.w   0x273c                        | +cca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ccc  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +cce  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b926
HumanDeath_SpriteTbls_04ac56__L04b926:
.L04b926:
        .dc.w   0x0003                        | +cd0  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +cd2  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +cd4  (dato / opcode no decodificado)
        .dc.w   0x726e                        | +cd6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +cd8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +cda  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +cdc  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +cde  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +ce0  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +ce2  (dato / opcode no decodificado)
        .dc.w   0x727e                        | +ce4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ce6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ce8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +cea  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +cec  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +cee  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +cf0  (dato / opcode no decodificado)
        .dc.w   0x728e                        | +cf2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +cf4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +cf6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +cf8  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +cfa  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +cfc  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +cfe  (dato / opcode no decodificado)
        .dc.w   0x729e                        | +d00  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d02  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d04  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d06  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d08  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d0a  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d0c  (dato / opcode no decodificado)
        .dc.w   0x72ae                        | +d0e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d10  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d12  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d14  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d16  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d18  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d1a  (dato / opcode no decodificado)
        .dc.w   0x72be                        | +d1c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d1e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d20  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d22  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d24  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d26  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d28  (dato / opcode no decodificado)
        .dc.w   0x726e                        | +d2a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d2c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d2e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d30  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d32  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d34  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d36  (dato / opcode no decodificado)
        .dc.w   0x727e                        | +d38  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d3a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d3c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d3e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d40  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d42  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d44  (dato / opcode no decodificado)
        .dc.w   0x72ee                        | +d46  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d48  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d4a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d4c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d4e  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d50  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d52  (dato / opcode no decodificado)
        .dc.w   0x72fe                        | +d54  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d56  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d58  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d5a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d5c  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d5e  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d60  (dato / opcode no decodificado)
        .dc.w   0x730e                        | +d62  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d64  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d66  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d68  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d6a  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d6c  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d6e  (dato / opcode no decodificado)
        .dc.w   0x731e                        | +d70  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d72  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d74  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d76  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d78  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d7a  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d7c  (dato / opcode no decodificado)
        .dc.w   0x72ce                        | +d7e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d80  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d82  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d84  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +d86  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d88  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +d8a  (dato / opcode no decodificado)
        .dc.w   0x72de                        | +d8c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +d8e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d90  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +d92  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +d94  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +d96  (dato / opcode no decodificado)
        .dc.w   0xb926                        | +d98  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04b9f0
HumanDeath_SpriteTbls_04ac56__L04b9f0:
.L04b9f0:
        .dc.w   0x0002                        | +d9a  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +d9c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +d9e  (dato / opcode no decodificado)
        .dc.w   0xe97c                        | +da0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +da2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +da4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +da6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +da8  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +daa  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +dac  (dato / opcode no decodificado)
        .dc.w   0xe986                        | +dae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +db0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +db2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +db4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +db6  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +db8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +dba  (dato / opcode no decodificado)
        .dc.w   0xe99a                        | +dbc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +dbe  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +dc0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +dc2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +dc4  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +dc6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +dc8  (dato / opcode no decodificado)
        .dc.w   0xe9aa                        | +dca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +dcc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +dce  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +dd0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +dd2  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +dd4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +dd6  (dato / opcode no decodificado)
        .dc.w   0xe9ba                        | +dd8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +dda  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ddc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +dde  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +de0  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +de2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +de4  (dato / opcode no decodificado)
        .dc.w   0xe9ca                        | +de6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +de8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +dea  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +dec  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +dee  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +df0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +df2  (dato / opcode no decodificado)
        .dc.w   0xe9da                        | +df4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +df6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +df8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +dfa  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +dfc  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +dfe  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e00  (dato / opcode no decodificado)
        .dc.w   0xe9ea                        | +e02  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e04  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e06  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e08  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +e0a  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e0c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e0e  (dato / opcode no decodificado)
        .dc.w   0xea04                        | +e10  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e12  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e14  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e16  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +e18  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e1a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e1c  (dato / opcode no decodificado)
        .dc.w   0xea1c                        | +e1e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e20  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e22  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e24  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +e26  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e28  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e2a  (dato / opcode no decodificado)
        .dc.w   0xea38                        | +e2c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e2e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e30  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e32  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +e34  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e36  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e38  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +e3a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e3c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e3e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e40  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +e42  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e44  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e46  (dato / opcode no decodificado)
        .dc.w   0xea68                        | +e48  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e4a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e4c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e4e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +e50  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e52  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e54  (dato / opcode no decodificado)
        .dc.w   0xea7c                        | +e56  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e58  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e5a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e5c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +e5e  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e60  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e62  (dato / opcode no decodificado)
        .dc.w   0xea90                        | +e64  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e66  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e68  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e6a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +e6c  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e6e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e70  (dato / opcode no decodificado)
        .dc.w   0xeaa4                        | +e72  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e74  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e76  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e78  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +e7a  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04bad2
HumanDeath_SpriteTbls_04ac56__L04bad2:
.L04bad2:
        .dc.w   0x0001                        | +e7c  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e7e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e80  (dato / opcode no decodificado)
        .dc.w   0xb344                        | +e82  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e84  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e86  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e88  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +e8a  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e8c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e8e  (dato / opcode no decodificado)
        .dc.w   0xb38c                        | +e90  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +e92  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e94  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +e96  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +e98  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +e9a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +e9c  (dato / opcode no decodificado)
        .dc.w   0xb3e4                        | +e9e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ea0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ea2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ea4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ea6  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +ea8  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +eaa  (dato / opcode no decodificado)
        .dc.w   0xb448                        | +eac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +eae  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +eb0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +eb2  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +eb4  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +eb6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +eb8  (dato / opcode no decodificado)
        .dc.w   0xb4b6                        | +eba  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ebc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ebe  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ec0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ec2  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +ec4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +ec6  (dato / opcode no decodificado)
        .dc.w   0xb52e                        | +ec8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +eca  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ecc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ece  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ed0  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +ed2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +ed4  (dato / opcode no decodificado)
        .dc.w   0xb5a8                        | +ed6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ed8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +eda  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +edc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +ede  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +ee0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +ee2  (dato / opcode no decodificado)
        .dc.w   0xb612                        | +ee4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +ee6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +ee8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +eea  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +eec  (dato / opcode no decodificado)
        .global HumanDeath_SpriteTbls_04ac56__L04bb44
HumanDeath_SpriteTbls_04ac56__L04bb44:
.L04bb44:
        .dc.w   0x0100                        | +eee  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +ef0  (dato / opcode no decodificado)
        .dc.w   0xad04                        | +ef2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HumanDeath_SmokeSprite_04bb4a  @ $04BB4A  (68 B)
| ----------------------------------------------------------------------------
        .section .text.HumanDeath_SmokeSprite_04bb4a, "ax", @progbits
        .global HumanDeath_SmokeSprite_04bb4a
HumanDeath_SmokeSprite_04bb4a:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +004  (dato / opcode no decodificado)
        .dc.w   0x06e0                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x06f0                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0722                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x074a                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +032  (dato / opcode no decodificado)
        .dc.w   0x226e                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +036  (dato / opcode no decodificado)
        .dc.w   0x102e                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xb029                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x6500                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +042  (dato / opcode no decodificado)
