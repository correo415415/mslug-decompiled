| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave OOO — Bichos (Bobber/Leaper/Runner/Nest/Swarmer), física de grunt y rings de zonas/posiciones/targets
|  Región: $08E4E4..$08F6D2  (3,980 B, 84 entradas, 54 huecos)
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
|  Bobber_Tmpl182_08e4e6  @ $08E4E6  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Bobber_Tmpl182_08e4e6, "ax", @progbits
        .global Bobber_Tmpl182_08e4e6
Bobber_Tmpl182_08e4e6:
        move.w  #0xc000,0x38(a6)                | +000
        jsr     Phys_FacingFromParam_08f002(pc) | +006
        jsr     Phys_VelXFromParam_08f010(pc)   | +00a
        move.w  #0x12a,d1                       | +00e
        jsr     0x236e.l                        | +012

| ----------------------------------------------------------------------------
|  Bobber_Descend_08e4fe  @ $08E4FE  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Bobber_Descend_08e4fe, "ax", @progbits
        .global Bobber_Descend_08e4fe
Bobber_Descend_08e4fe:
        move.w  #0x4,0x2e(a6)                   | +000
        move.w  #0x6,0x70(a6)                   | +006
        move.w  #0x7,d0                         | +00c
        jsr     0x5ea1c.l                       | +010
        add.w   d0,0x70(a6)                     | +016
        lea     0x2f4036.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L08e52a(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L08e52a:
        jsr     0x2783a.l                       | +02c
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +032
        jsr     0x28d70.l                       | +036
        cmpi.w  #0x1d0,0x24(a6)                 | +03c
        blt.w   Bobber_DescendEnd_08e54c        | +042

| ----------------------------------------------------------------------------
|  Bobber_DescendEnd_08e54c  @ $08E54C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Bobber_DescendEnd_08e54c, "ax", @progbits
        .global Bobber_DescendEnd_08e54c
Bobber_DescendEnd_08e54c:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bhi.w   JsrPcThunk_08e560               | +00a
        lea     Bobber_Ascend_08e566(pc),a1     | +00e
        move.l  a1,(a6)                         | +012

| ----------------------------------------------------------------------------
|  Bobber_Ascend_08e566  @ $08E566  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Bobber_Ascend_08e566, "ax", @progbits
        .global Bobber_Ascend_08e566
Bobber_Ascend_08e566:
        move.w  #0xfffc,0x2e(a6)                | +000
        move.w  #0x6,0x70(a6)                   | +006
        move.w  #0x7,d0                         | +00c
        jsr     0x5ea1c.l                       | +010
        add.w   d0,0x70(a6)                     | +016
        lea     0x2f4036.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L08e592(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L08e592:
        jsr     0x2783a.l                       | +02c
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +032
        jsr     0x28d70.l                       | +036
        cmpi.w  #0x160,0x24(a6)                 | +03c
        bge.w   Bobber_AscendEnd_08e5b4         | +042

| ----------------------------------------------------------------------------
|  Bobber_AscendEnd_08e5b4  @ $08E5B4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Bobber_AscendEnd_08e5b4, "ax", @progbits
        .global Bobber_AscendEnd_08e5b4
Bobber_AscendEnd_08e5b4:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bhi.w   JsrPcThunk_08e5c8               | +00a
        lea     Bobber_Descend_08e4fe(pc),a1    | +00e
        move.l  a1,(a6)                         | +012

| ----------------------------------------------------------------------------
|  Leaper_Tmpl183_08e5ce  @ $08E5CE  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Leaper_Tmpl183_08e5ce, "ax", @progbits
        .global Leaper_Tmpl183_08e5ce
Leaper_Tmpl183_08e5ce:
        jsr     Prio_Set8018_08f108(pc)         | +000
        jsr     Phys_FacingFromParam_08f002(pc) | +004
        jsr     Phys_VelXFromParam_08f010(pc)   | +008
        move.w  #0xfd00,0x2a(a6)                | +00c
        move.w  #0x10,0x2e(a6)                  | +012
        move.w  #0x124,d1                       | +018
        jsr     0x236e.l                        | +01c
        lea     0x2f4072.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        jsr     0x27cee.l                       | +02e
        jsr     0x28d70.l                       | +034
        move.w  #0xf,d0                         | +03a
        jsr     0x5ea1c.l                       | +03e
        addi.w  #0xc,d0                         | +044
        move.w  d0,0x70(a6)                     | +048

| ----------------------------------------------------------------------------
|  Leaper_Jump_08e622  @ $08E622  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Leaper_Jump_08e622, "ax", @progbits
        .global Leaper_Jump_08e622
Leaper_Jump_08e622:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L08e636                        | +00a
        lea     Leaper_Hop_08e65c(pc),a1        | +00e
        move.l  a1,(a6)                         | +012
.L08e636:
        cmpi.w  #0x40,0x2a(a6)                  | +014
        blt.w   .L08e646                        | +01a
        lea     Leaper_Hop_08e65c(pc),a1        | +01e
        move.l  a1,(a6)                         | +022
.L08e646:
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +024
        jsr     0x2783a.l                       | +028
        jsr     Phys_GroundKill_08efb0(pc)      | +02e

| ----------------------------------------------------------------------------
|  Leaper_Hop_08e65c  @ $08E65C  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Leaper_Hop_08e65c, "ax", @progbits
        .global Leaper_Hop_08e65c
Leaper_Hop_08e65c:
        move.w  #0x3f,d0                        | +000
        jsr     0x5ea1c.l                       | +004
        addi.w  #0x40,d0                        | +00a
        neg.w   d0                              | +00e
        move.w  d0,0x2a(a6)                     | +010
        move.w  #0x4,0x2e(a6)                   | +014
        move.w  #0xb,0x70(a6)                   | +01a
        lea     0x2f407e.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L08e68e(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L08e68e:
        subq.w  #0x1,0x70(a6)                   | +032
        cmpi.w  #0x0,0x70(a6)                   | +036
        bgt.w   .L08e6a2                        | +03c
        lea     Leaper_Fall_08e6b8(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L08e6a2:
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +046
        jsr     0x2783a.l                       | +04a
        jsr     Phys_GroundKill_08efb0(pc)      | +050

| ----------------------------------------------------------------------------
|  Leaper_Fall_08e6b8  @ $08E6B8  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Leaper_Fall_08e6b8, "ax", @progbits
        .global Leaper_Fall_08e6b8
Leaper_Fall_08e6b8:
        lea     0x2f408a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e6ca(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e6ca:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +012
        jsr     0x2783a.l                       | +016
        jsr     Phys_GroundKill_08efb0(pc)      | +01c

| ----------------------------------------------------------------------------
|  Runner_Tmpl184_08e6e0  @ $08E6E0  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Runner_Tmpl184_08e6e0, "ax", @progbits
        .global Runner_Tmpl184_08e6e0
Runner_Tmpl184_08e6e0:
        jsr     Prio_Set8018_08f108(pc)         | +000
        jsr     Phys_FacingFromParam_08f002(pc) | +004
        jsr     Phys_VelXFromParam_08f010(pc)   | +008
        move.w  #0x128,d1                       | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f4096.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        jsr     0x27cee.l                       | +022
        jsr     0x28d70.l                       | +028

| ----------------------------------------------------------------------------
|  Runner_Run_08e716  @ $08E716  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Runner_Run_08e716, "ax", @progbits
        .global Runner_Run_08e716
Runner_Run_08e716:
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +000
        jsr     0x2783a.l                       | +004
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +00a
        bcc.w   JsrAbsThunk_08e730              | +00e
        jmp     0x518.l                         | +012

| ----------------------------------------------------------------------------
|  Rts_08e72e  @ $08E72E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08e72e, "ax", @progbits
        .global Rts_08e72e
Rts_08e72e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Nest_Spawn3_08e738  @ $08E738  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_Spawn3_08e738, "ax", @progbits
        .global Nest_Spawn3_08e738
Nest_Spawn3_08e738:
        move.b  #0x3,0x9a(a6)                   | +000
        clr.w   0x5c(a6)                        | +006
        bra.w   Nest_Tmpl185_08e746__L08e750    | +00a

| ----------------------------------------------------------------------------
|  Nest_Tmpl185_08e746  @ $08E746  (132 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_Tmpl185_08e746, "ax", @progbits
        .global Nest_Tmpl185_08e746
Nest_Tmpl185_08e746:
        jsr     Prio_Set8018_08f108(pc)         | +000
        move.w  #0x1,0x5c(a6)                   | +004
        .global Nest_Tmpl185_08e746__L08e750
Nest_Tmpl185_08e746__L08e750:
        jsr     Phys_FacingFromParam_08f002(pc) | +00a
        move.w  #0x125,d1                       | +00e
        jsr     0x236e.l                        | +012
        lea     0x2f41f2.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     0x2f414a.l,a0                   | +024
        move.l  a0,0x48(a6)                     | +02a
        bclr    #0x3,0x13(a6)                   | +02e
        move.w  #0x64,0x66(a6)                  | +034
.L08e780:
        subq.b  #0x1,0x9a(a6)                   | +03a
        cmpi.b  #0x0,0x9a(a6)                   | +03e
        blt.w   .L08e7a0                        | +044
        lea     Swarmer_Init_08eb56(pc),a1      | +048
        jsr     0x4ae.l                         | +04c
        jsr     0x5dd02.l                       | +052
        bra.b   .L08e780                        | +058
.L08e7a0:
        lea     .L08e7a6(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L08e7a6:
        jsr     0x2783a.l                       | +060
        jsr     0x28d70.l                       | +066
        jsr     0x2870a.l                       | +06c
        bcc.w   .L08e7c2                        | +072
        lea     Nest_Stage1_Hit_08e7de(pc),a1   | +076
        move.l  a1,(a6)                         | +07a
.L08e7c2:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +07c
        bcc.w   JsrPcThunk_08e7d8               | +080

| ----------------------------------------------------------------------------
|  Nest_Stage1_Hit_08e7de  @ $08E7DE  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_Stage1_Hit_08e7de, "ax", @progbits
        .global Nest_Stage1_Hit_08e7de
Nest_Stage1_Hit_08e7de:
        lea     0x2f4202.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e7f0(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e7f0:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L08e806                        | +01e
        lea     Nest_Stage2_08e822(pc),a1       | +022
        move.l  a1,(a6)                         | +026
.L08e806:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +028
        bcc.w   JsrPcThunk_08e81c               | +02c

| ----------------------------------------------------------------------------
|  Nest_Stage2_08e822  @ $08E822  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_Stage2_08e822, "ax", @progbits
        .global Nest_Stage2_08e822
Nest_Stage2_08e822:
        bclr    #0x3,0x13(a6)                   | +000
        move.w  #0x64,0x66(a6)                  | +006
        lea     .L08e834(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e834:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L08e850                        | +024
        lea     Nest_Stage2_Hit_08e86c(pc),a1   | +028
        move.l  a1,(a6)                         | +02c
.L08e850:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +02e
        bcc.w   JsrPcThunk_08e866               | +032

| ----------------------------------------------------------------------------
|  Nest_Stage2_Hit_08e86c  @ $08E86C  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_Stage2_Hit_08e86c, "ax", @progbits
        .global Nest_Stage2_Hit_08e86c
Nest_Stage2_Hit_08e86c:
        lea     0x2f4284.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e87e(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e87e:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L08e894                        | +01e
        lea     Nest_Stage3_08e8b0(pc),a1       | +022
        move.l  a1,(a6)                         | +026
.L08e894:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +028
        bcc.w   JsrPcThunk_08e8aa               | +02c

| ----------------------------------------------------------------------------
|  Nest_Stage3_08e8b0  @ $08E8B0  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_Stage3_08e8b0, "ax", @progbits
        .global Nest_Stage3_08e8b0
Nest_Stage3_08e8b0:
        bclr    #0x3,0x13(a6)                   | +000
        move.w  #0x64,0x66(a6)                  | +006
        lea     .L08e8c2(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e8c2:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L08e8de                        | +024
        lea     Nest_Stage3_Hit_08e8fa(pc),a1   | +028
        move.l  a1,(a6)                         | +02c
.L08e8de:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +02e
        bcc.w   JsrPcThunk_08e8f4               | +032

| ----------------------------------------------------------------------------
|  Nest_Stage3_Hit_08e8fa  @ $08E8FA  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_Stage3_Hit_08e8fa, "ax", @progbits
        .global Nest_Stage3_Hit_08e8fa
Nest_Stage3_Hit_08e8fa:
        lea     0x2f4306.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e90c(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e90c:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L08e922                        | +01e
        lea     Nest_Stage4_08e93e(pc),a1       | +022
        move.l  a1,(a6)                         | +026
.L08e922:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +028
        bcc.w   JsrPcThunk_08e938               | +02c

| ----------------------------------------------------------------------------
|  Nest_Stage4_08e93e  @ $08E93E  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_Stage4_08e93e, "ax", @progbits
        .global Nest_Stage4_08e93e
Nest_Stage4_08e93e:
        bclr    #0x3,0x13(a6)                   | +000
        move.w  #0x64,0x66(a6)                  | +006
        lea     .L08e950(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e950:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L08e96c                        | +024
        lea     Nest_Destroyed_08e988(pc),a1    | +028
        move.l  a1,(a6)                         | +02c
.L08e96c:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +02e
        bcc.w   JsrPcThunk_08e982               | +032

| ----------------------------------------------------------------------------
|  Nest_Destroyed_08e988  @ $08E988  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_Destroyed_08e988, "ax", @progbits
        .global Nest_Destroyed_08e988
Nest_Destroyed_08e988:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        lea     0x2f4388.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        lea     .L08e9a2(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L08e9a2:
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        bcc.w   .L08e9b8                        | +026
        lea     Nest_DestroyedWait_08e9d4(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L08e9b8:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +030
        bcc.w   JsrPcThunk_08e9ce               | +034

| ----------------------------------------------------------------------------
|  Nest_DestroyedWait_08e9d4  @ $08E9D4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_DestroyedWait_08e9d4, "ax", @progbits
        .global Nest_DestroyedWait_08e9d4
Nest_DestroyedWait_08e9d4:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +00c
        bcc.w   JsrPcThunk_08e9f6               | +010

| ----------------------------------------------------------------------------
|  Nest_FlyOff_08e9fc  @ $08E9FC  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_FlyOff_08e9fc, "ax", @progbits
        .global Nest_FlyOff_08e9fc
Nest_FlyOff_08e9fc:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        move.w  #0xff80,0x2a(a6)                | +008
        move.w  #0xffc0,0x2e(a6)                | +00e
        lea     Nest_FlyOff_Run_08ea16(pc),a1   | +014
        move.l  a1,(a6)                         | +018

| ----------------------------------------------------------------------------
|  Nest_FlyOff_Run_08ea16  @ $08EA16  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_FlyOff_Run_08ea16, "ax", @progbits
        .global Nest_FlyOff_Run_08ea16
Nest_FlyOff_Run_08ea16:
        jsr     0x27c8c.l                       | +000
        bcc.w   .L08ea2c                        | +006
        jsr     0x5b6.l                         | +00a
        jmp     0x518.l                         | +010
.L08ea2c:
        jsr     0x2783a.l                       | +016
        jsr     0x28d70.l                       | +01c
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +022
        bcc.w   Stub_0008EA4E                   | +026

| ----------------------------------------------------------------------------
|  Nest_DieIfParentGone_08ea50  @ $08EA50  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Nest_DieIfParentGone_08ea50, "ax", @progbits
        .global Nest_DieIfParentGone_08ea50
Nest_DieIfParentGone_08ea50:
        cmpi.w  #0x0,0x5c(a6)                   | +000
        beq.w   .L08ea5c                        | +006
        rts                                     | +00a
.L08ea5c:
        movea.l 0xc(a6),a1                      | +00c
        cmpi.l  #0xffffffff,0x48(a1)            | +010
        bne.w   SetHandlerRts_08ea72            | +018

| ----------------------------------------------------------------------------
|  Nest2_Tmpl187_08ea74  @ $08EA74  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Nest2_Tmpl187_08ea74, "ax", @progbits
        .global Nest2_Tmpl187_08ea74
Nest2_Tmpl187_08ea74:
        jsr     Prio_Set8018_08f108(pc)         | +000
        lea     0x2f419e.l,a0                   | +004
        move.l  a0,0x48(a6)                     | +00a
        bclr    #0x3,0x13(a6)                   | +00e
        move.w  #0x64,0x66(a6)                  | +014
        clr.b   0x20(a6)                        | +01a
.L08ea92:
        subq.b  #0x1,0x98(a6)                   | +01e
        cmpi.b  #0x0,0x98(a6)                   | +022
        blt.w   .L08eab2                        | +028
        lea     Swarmer_Init_08eb56(pc),a1      | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        bra.b   .L08ea92                        | +03c
.L08eab2:
        lea     Nest2_Tmpl187_08ea74__L08eab8(pc),a1 | +03e
        move.l  a1,(a6)                         | +042
        .global Nest2_Tmpl187_08ea74__L08eab8
Nest2_Tmpl187_08ea74__L08eab8:
        jsr     0x2783a.l                       | +044
        jsr     0x2870a.l                       | +04a
        bcc.w   .L08eace                        | +050
        lea     Nest2_Hit_08eaee(pc),a1         | +054
        move.l  a1,(a6)                         | +058
.L08eace:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +05a
        bcc.w   Stub_0008EAEC                   | +05e
        lea     0xffff.w,a0                     | +062
        move.l  a0,0x48(a6)                     | +066

| ----------------------------------------------------------------------------
|  Nest2_Hit_08eaee  @ $08EAEE  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Nest2_Hit_08eaee, "ax", @progbits
        .global Nest2_Hit_08eaee
Nest2_Hit_08eaee:
        move.b  #0x1,0x20(a6)                   | +000
        move.w  #0x14,0x70(a6)                  | +006

| ----------------------------------------------------------------------------
|  Nest2_HitWait_08eb02  @ $08EB02  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Nest2_HitWait_08eb02, "ax", @progbits
        .global Nest2_HitWait_08eb02
Nest2_HitWait_08eb02:
        clr.b   0x20(a6)                        | +000
        lea     .L08eb0c(pc),a1                 | +004
        move.l  a1,(a6)                         | +008
.L08eb0c:
        jsr     0x2783a.l                       | +00a
        subq.w  #0x1,0x70(a6)                   | +010
        cmpi.w  #0x0,0x70(a6)                   | +014
        bgt.w   .L08eb36                        | +01a
        bclr    #0x3,0x13(a6)                   | +01e
        move.w  #0x64,0x66(a6)                  | +024
        clr.b   0x20(a6)                        | +02a
        lea     Nest2_Tmpl187_08ea74__L08eab8(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L08eb36:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +034
        bcc.w   Stub_0008EB54                   | +038
        lea     0xffff.w,a0                     | +03c
        move.l  a0,0x48(a6)                     | +040

| ----------------------------------------------------------------------------
|  Swarmer_Init_08eb56  @ $08EB56  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Swarmer_Init_08eb56, "ax", @progbits
        .global Swarmer_Init_08eb56
Swarmer_Init_08eb56:
        move.w  #0x125,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2f440a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010

| ----------------------------------------------------------------------------
|  Swarmer_AimParent_08eb6c  @ $08EB6C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Swarmer_AimParent_08eb6c, "ax", @progbits
        .global Swarmer_AimParent_08eb6c
Swarmer_AimParent_08eb6c:
        movea.l 0xc(a6),a1                      | +000
        move.w  0x22(a1),d0                     | +004
        subi.w  #0xc,d0                         | +008
        move.w  0x24(a1),d1                     | +00c
        subi.w  #0x18,d1                        | +010
        sub.w   0x22(a6),d0                     | +014
        sub.w   0x24(a6),d1                     | +018
        jsr     0x5e018.l                       | +01c
        move.w  d0,0x34(a6)                     | +022
        bra.w   Swarmer_AimRandom_08eb96__L08eba4 | +026

| ----------------------------------------------------------------------------
|  Swarmer_AimRandom_08eb96  @ $08EB96  (180 B)
| ----------------------------------------------------------------------------
        .section .text.Swarmer_AimRandom_08eb96, "ax", @progbits
        .global Swarmer_AimRandom_08eb96
Swarmer_AimRandom_08eb96:
        move.w  #0xff,d0                        | +000
        jsr     0x5ea1c.l                       | +004
        move.w  d0,0x34(a6)                     | +00a
        .global Swarmer_AimRandom_08eb96__L08eba4
Swarmer_AimRandom_08eb96__L08eba4:
        move.w  #0xff,d0                        | +00e
        jsr     0x5ea1c.l                       | +012
        move.w  d0,d1                           | +018
        addi.w  #0x180,d1                       | +01a
        move.w  0x34(a6),d0                     | +01e
        jsr     0x13c0e.l                       | +022
        move.w  d1,0x28(a6)                     | +028
        move.w  d2,0x2a(a6)                     | +02c
        cmpi.w  #0x0,0x28(a6)                   | +030
        bge.w   .L08ebd8                        | +036
        clr.b   0x3a(a6)                        | +03a
        bra.w   .L08ebde                        | +03e
.L08ebd8:
        move.b  #0x1,0x3a(a6)                   | +042
.L08ebde:
        move.w  #0x3,d0                         | +048
        jsr     0x5ea1c.l                       | +04c
        addi.w  #0x1,d0                         | +052
        move.w  d0,0x70(a6)                     | +056
        lea     .L08ebf6(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L08ebf6:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +060
        jsr     0x2783a.l                       | +064
        subq.w  #0x1,0x70(a6)                   | +06a
        cmpi.w  #0x0,0x70(a6)                   | +06e
        bgt.w   .L08ec30                        | +074
        move.w  #0x3,d0                         | +078
        jsr     0x5ea1c.l                       | +07c
        cmpi.w  #0x0,d0                         | +082
        bne.w   .L08ec2a                        | +086
        lea     Swarmer_AimParent_08eb6c(pc),a1 | +08a
        move.l  a1,(a6)                         | +08e
        bra.w   .L08ec30                        | +090
.L08ec2a:
        lea     Swarmer_AimRandom_08eb96(pc),a1 | +094
        move.l  a1,(a6)                         | +098
.L08ec30:
        movea.l 0xc(a6),a1                      | +09a
        cmpi.b  #0x0,0x20(a1)                   | +09e
        beq.w   .L08ec44                        | +0a4
        lea     Swarmer_Flee_08ec50(pc),a1      | +0a8
        move.l  a1,(a6)                         | +0ac
.L08ec44:
        jsr     0x28d70.l                       | +0ae

| ----------------------------------------------------------------------------
|  Swarmer_Flee_08ec50  @ $08EC50  (132 B)
| ----------------------------------------------------------------------------
        .section .text.Swarmer_Flee_08ec50, "ax", @progbits
        .global Swarmer_Flee_08ec50
Swarmer_Flee_08ec50:
        movea.l 0xc(a6),a1                      | +000
        move.w  0x22(a6),d0                     | +004
        addi.w  #0xc,d0                         | +008
        move.w  0x24(a6),d1                     | +00c
        addi.w  #0x18,d1                        | +010
        sub.w   0x22(a1),d0                     | +014
        sub.w   0x24(a1),d1                     | +018
        jsr     0x5e018.l                       | +01c
        move.w  d0,0x34(a6)                     | +022
        move.w  #0x300,d1                       | +026
        move.w  0x34(a6),d0                     | +02a
        jsr     0x13c0e.l                       | +02e
        move.w  d1,0x28(a6)                     | +034
        move.w  d2,0x2a(a6)                     | +038
        cmpi.w  #0x0,0x28(a6)                   | +03c
        bge.w   .L08ec9e                        | +042
        clr.b   0x3a(a6)                        | +046
        bra.w   .L08eca4                        | +04a
.L08ec9e:
        move.b  #0x1,0x3a(a6)                   | +04e
.L08eca4:
        move.w  #0x4,0x70(a6)                   | +054
        lea     .L08ecb0(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L08ecb0:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +060
        jsr     0x2783a.l                       | +064
        subq.w  #0x1,0x70(a6)                   | +06a
        cmpi.w  #0x0,0x70(a6)                   | +06e
        bgt.w   .L08ecce                        | +074
        lea     Swarmer_AimParent_08eb6c(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L08ecce:
        jsr     0x28d70.l                       | +07e

| ----------------------------------------------------------------------------
|  Swarmer_Tmpl186_08ecda  @ $08ECDA  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Swarmer_Tmpl186_08ecda, "ax", @progbits
        .global Swarmer_Tmpl186_08ecda
Swarmer_Tmpl186_08ecda:
        jsr     Prio_Set8018_08f108(pc)         | +000
        jsr     Phys_FacingFromParam_08f002(pc) | +004
        move.w  #0x125,d1                       | +008
        jsr     0x236e.l                        | +00c
        lea     0x2f440a.l,a0                   | +012
        jsr     0x28cd4.l                       | +018

| ----------------------------------------------------------------------------
|  Swarmer_Wander_08ecf8  @ $08ECF8  (132 B)
| ----------------------------------------------------------------------------
        .section .text.Swarmer_Wander_08ecf8, "ax", @progbits
        .global Swarmer_Wander_08ecf8
Swarmer_Wander_08ecf8:
        move.w  #0xff,d0                        | +000
        jsr     0x5ea1c.l                       | +004
        move.w  d0,0x34(a6)                     | +00a
        move.w  #0xff,d0                        | +00e
        jsr     0x5ea1c.l                       | +012
        move.w  d0,d1                           | +018
        addi.w  #0x100,d1                       | +01a
        move.w  0x34(a6),d0                     | +01e
        jsr     0x13c0e.l                       | +022
        move.w  d1,0x28(a6)                     | +028
        move.w  d2,0x2a(a6)                     | +02c
        move.w  #0x3,d0                         | +030
        jsr     0x5ea1c.l                       | +034
        addi.w  #0x2,d0                         | +03a
        move.w  d0,0x70(a6)                     | +03e
        cmpi.w  #0x0,0x28(a6)                   | +042
        bge.w   .L08ed4c                        | +048
        clr.b   0x3a(a6)                        | +04c
        bra.w   .L08ed52                        | +050
.L08ed4c:
        move.b  #0x1,0x3a(a6)                   | +054
.L08ed52:
        lea     .L08ed58(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L08ed58:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +060
        jsr     0x2783a.l                       | +064
        subq.w  #0x1,0x70(a6)                   | +06a
        cmpi.w  #0x0,0x70(a6)                   | +06e
        bgt.w   .L08ed76                        | +074
        lea     Swarmer_Wander_08ecf8(pc),a1    | +078
        move.l  a1,(a6)                         | +07c
.L08ed76:
        jsr     0x28d70.l                       | +07e

| ----------------------------------------------------------------------------
|  Static_Tmpl188_08ed82  @ $08ED82  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Static_Tmpl188_08ed82, "ax", @progbits
        .global Static_Tmpl188_08ed82
Static_Tmpl188_08ed82:
        move.w  #0x0,0x38(a6)                   | +000
        jsr     Phys_FacingFromParam_08f002(pc) | +006
        move.w  #0x12a,d1                       | +00a
        jsr     0x236e.l                        | +00e
        lea     0x2f4426.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L08eda8(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L08eda8:
        jsr     0x2783a.l                       | +026
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +02c
        bcc.w   JsrAbsThunk_08edbe              | +030
        jmp     0x518.l                         | +034

| ----------------------------------------------------------------------------
|  Rts_08edbc  @ $08EDBC  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08edbc, "ax", @progbits
        .global Rts_08edbc
Rts_08edbc:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  CamProp_Tmpl189_08edc6  @ $08EDC6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.CamProp_Tmpl189_08edc6, "ax", @progbits
        .global CamProp_Tmpl189_08edc6
CamProp_Tmpl189_08edc6:
        lea     0x2f4442.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   CamProp_Tmpl190_08edd6__L08ede2 | +00c

| ----------------------------------------------------------------------------
|  CamProp_Tmpl190_08edd6  @ $08EDD6  (54 B)
| ----------------------------------------------------------------------------
        .section .text.CamProp_Tmpl190_08edd6, "ax", @progbits
        .global CamProp_Tmpl190_08edd6
CamProp_Tmpl190_08edd6:
        lea     0x2f44ae.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global CamProp_Tmpl190_08edd6__L08ede2
CamProp_Tmpl190_08edd6__L08ede2:
        move.w  #0x0,0x38(a6)                   | +00c
        move.w  #0xf4,d1                        | +012
        jsr     0x236e.l                        | +016
        lea     .L08edf8(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L08edf8:
        jsr     0x4407a.l                       | +022
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +028
        bcc.w   JsrAbsThunk_08ee0e              | +02c
        jmp     0x518.l                         | +030

| ----------------------------------------------------------------------------
|  Rts_08ee0c  @ $08EE0C  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08ee0c, "ax", @progbits
        .global Rts_08ee0c
Rts_08ee0c:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Lob_Tmpl191_08ee16  @ $08EE16  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Lob_Tmpl191_08ee16, "ax", @progbits
        .global Lob_Tmpl191_08ee16
Lob_Tmpl191_08ee16:
        jsr     Phys_FacingFromParam_08f002(pc) | +000
        jsr     Phys_VelXFromParam_08f010(pc)   | +004
        clr.w   d0                              | +008
        move.b  0x99(a6),d0                     | +00a
        lsl.w   #0x5,d0                         | +00e
        neg.w   d0                              | +010
        move.w  d0,0x2a(a6)                     | +012
        move.w  #0x0,0x38(a6)                   | +016
        move.w  #0xf4,d1                        | +01c
        jsr     0x236e.l                        | +020
        lea     0x2f44ca.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L08ee4e(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L08ee4e:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +038
        cmpi.w  #0x190,0x24(a6)                 | +03c
        bgt.w   .L08ee6e                        | +042
        lea     0x2f44d6.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        lea     Lob_Landed_08ee7a(pc),a1        | +052
        move.l  a1,(a6)                         | +056
.L08ee6e:
        jsr     Phys_GroundKill_08efb0(pc)      | +058

| ----------------------------------------------------------------------------
|  Lob_Landed_08ee7a  @ $08EE7A  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Lob_Landed_08ee7a, "ax", @progbits
        .global Lob_Landed_08ee7a
Lob_Landed_08ee7a:
        jsr     0x4407a.l                       | +000
        jsr     Phys_GroundKill_08efb0(pc)      | +006
        jsr     0x28d70.l                       | +00a
        bcc.w   .L08ee94                        | +010
        jmp     0x518.l                         | +014
.L08ee94:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Shard_V0_08ee96  @ $08EE96  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Shard_V0_08ee96, "ax", @progbits
        .global Shard_V0_08ee96
Shard_V0_08ee96:
        lea     0x2f40c6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.b  #0x0,0x74(a6)                   | +00c
        bra.w   Shard_V2_08eec2__L08eed4        | +012

| ----------------------------------------------------------------------------
|  Shard_V1_08eeac  @ $08EEAC  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Shard_V1_08eeac, "ax", @progbits
        .global Shard_V1_08eeac
Shard_V1_08eeac:
        lea     0x2f40f2.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.b  #0x1,0x74(a6)                   | +00c
        bra.w   Shard_V2_08eec2__L08eed4        | +012

| ----------------------------------------------------------------------------
|  Shard_V2_08eec2  @ $08EEC2  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Shard_V2_08eec2, "ax", @progbits
        .global Shard_V2_08eec2
Shard_V2_08eec2:
        lea     0x2f411e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.b  #0x2,0x74(a6)                   | +00c
        .global Shard_V2_08eec2__L08eed4
Shard_V2_08eec2__L08eed4:
        move.w  #0x1f,d0                        | +012
        jsr     0x5ea1c.l                       | +016
        neg.w   d0                              | +01c
        move.w  d0,0x2a(a6)                     | +01e
        move.w  #0x1f,d0                        | +022
        jsr     0x5ea1c.l                       | +026
        subi.w  #0x30,d0                        | +02c
        move.w  d0,0x2e(a6)                     | +030
        jsr     Phys_FacingFromParam_08f002(pc) | +034
        move.w  #0xad,d1                        | +038
        jsr     0x236e.l                        | +03c
        movea.l 0xc(a6),a1                      | +042
        movea.l 0xc(a1),a2                      | +046
        move.w  0x24(a2),0x5c(a6)               | +04a
        lea     .L08ef18(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L08ef18:
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +056
        move.w  0x5c(a6),d0                     | +05a
        cmp.w   0x24(a6),d0                     | +05e
        blt.w   .L08ef2e                        | +062
        lea     Shard_Land_08ef40(pc),a1        | +066
        move.l  a1,(a6)                         | +06a
.L08ef2e:
        jsr     0x2783a.l                       | +06c
        jsr     Phys_GroundKill_08efb0(pc)      | +072

| ----------------------------------------------------------------------------
|  Shard_Land_08ef40  @ $08EF40  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Shard_Land_08ef40, "ax", @progbits
        .global Shard_Land_08ef40
Shard_Land_08ef40:
        cmpi.b  #0x0,0x74(a6)                   | +000
        bne.w   .L08ef5a                        | +006
        lea     0x2f40d2.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.w   .L08ef8e                        | +016
.L08ef5a:
        cmpi.b  #0x0,0x74(a6)                   | +01a
        bne.w   .L08ef74                        | +020
        lea     0x2f40fe.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        bra.w   .L08ef8e                        | +030
.L08ef74:
        cmpi.b  #0x0,0x74(a6)                   | +034
        bne.w   .L08ef8e                        | +03a
        lea     0x2f412a.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        bra.w   .L08ef8e                        | +04a
.L08ef8e:
        lea     .L08ef94(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L08ef94:
        jsr     0x2783a.l                       | +054
        jsr     Phys_GroundKill_08efb0(pc)      | +05a
        jsr     0x28d70.l                       | +05e
        bcc.w   .L08efae                        | +064
        jmp     0x518.l                         | +068
.L08efae:
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  Phys_GroundKill_08efb0  @ $08EFB0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Phys_GroundKill_08efb0, "ax", @progbits
        .global Phys_GroundKill_08efb0
Phys_GroundKill_08efb0:
        movea.l #0xffffffff,a0                  | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   Jsr5B6Rts_08efcc                | +00c

| ----------------------------------------------------------------------------
|  Phys_PlayerNearX_08efce  @ $08EFCE  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Phys_PlayerNearX_08efce, "ax", @progbits
        .global Phys_PlayerNearX_08efce
Phys_PlayerNearX_08efce:
        lea     0x100440.l,a1                   | +000
        move.w  0x22(a6),d1                     | +006
        sub.w   0x22(a1),d1                     | +00a
        cmp.w   d0,d1                           | +00e
        blt.w   SetXN_08effc                    | +010
        lea     0x1004e0.l,a1                   | +014
        move.w  0x22(a6),d1                     | +01a
        sub.w   0x22(a1),d1                     | +01e
        cmp.w   d0,d1                           | +022
        blt.w   SetXN_08effc                    | +024

| ----------------------------------------------------------------------------
|  Phys_FacingFromParam_08f002  @ $08F002  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Phys_FacingFromParam_08f002, "ax", @progbits
        .global Phys_FacingFromParam_08f002
Phys_FacingFromParam_08f002:
        andi.b  #0x1,0x98(a6)                   | +000
        move.b  0x98(a6),0x3a(a6)               | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Phys_VelXFromParam_08f010  @ $08F010  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Phys_VelXFromParam_08f010, "ax", @progbits
        .global Phys_VelXFromParam_08f010
Phys_VelXFromParam_08f010:
        clr.w   d0                              | +000
        move.b  0x99(a6),d0                     | +002
        lsl.w   #0x5,d0                         | +006
        move.w  d0,0x28(a6)                     | +008
        cmpi.b  #0x0,0x3a(a6)                   | +00c
        bne.w   .L08f02a                        | +012
        neg.w   0x28(a6)                        | +016
.L08f02a:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Phys_ScrollTarget_08f02c  @ $08F02C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Phys_ScrollTarget_08f02c, "ax", @progbits
        .global Phys_ScrollTarget_08f02c
Phys_ScrollTarget_08f02c:
        clr.w   d0                              | +000
        move.b  0x99(a6),d0                     | +002
        move.w  0x106f50.l,0x72(a6)             | +006
        add.w   d0,0x72(a6)                     | +00e
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Phys_ScrollReached_08f040  @ $08F040  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Phys_ScrollReached_08f040, "ax", @progbits
        .global Phys_ScrollReached_08f040
Phys_ScrollReached_08f040:
        cmpi.b  #0x0,0x99(a6)                   | +000
        bne.w   Phys_ScrollReached_Cmp_08f050   | +006

| ----------------------------------------------------------------------------
|  Phys_ScrollReached_Cmp_08f050  @ $08F050  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Phys_ScrollReached_Cmp_08f050, "ax", @progbits
        .global Phys_ScrollReached_Cmp_08f050
Phys_ScrollReached_Cmp_08f050:
        move.w  0x106f50.l,d0                   | +000
        cmp.w   0x72(a6),d0                     | +006
        bcs.w   ClearXN_08f068                  | +00a

| ----------------------------------------------------------------------------
|  Phys_ScrollReached_Yes_08f064  @ $08F064  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Phys_ScrollReached_Yes_08f064, "ax", @progbits
        .global Phys_ScrollReached_Yes_08f064
Phys_ScrollReached_Yes_08f064:
        bra.w   Rts_08f06e                      | +000

| ----------------------------------------------------------------------------
|  Rts_08f06e  @ $08F06E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08f06e, "ax", @progbits
        .global Rts_08f06e
Rts_08f06e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Snd_ByParam9A_Base_08f070  @ $08F070  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Snd_ByParam9A_Base_08f070, "ax", @progbits
        .global Snd_ByParam9A_Base_08f070
Snd_ByParam9A_Base_08f070:
        clr.w   d0                              | +000
        move.b  0x9a(a6),d0                     | +002
        addi.w  #0xfa,d0                        | +006
        move.w  d0,d1                           | +00a

| ----------------------------------------------------------------------------
|  Snd_ByParam9A_A_08f084  @ $08F084  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Snd_ByParam9A_A_08f084, "ax", @progbits
        .global Snd_ByParam9A_A_08f084
Snd_ByParam9A_A_08f084:
        cmpi.b  #0x0,0x9a(a6)                   | +000
        bne.w   .L08f098                        | +006
        move.w  #0x14a,d1                       | +00a
        jsr     0x236e.l                        | +00e
.L08f098:
        cmpi.b  #0x1,0x9a(a6)                   | +014
        bne.w   .L08f0ac                        | +01a
        move.w  #0xf6,d1                        | +01e
        jsr     0x236e.l                        | +022
.L08f0ac:
        cmpi.b  #0x1,0x9a(a6)                   | +028
        bne.w   .L08f0c4                        | +02e
        move.w  #0x13d,d1                       | +032
        jsr     0x236e.l                        | +036
        bra.w   JsrAbsRts_08f0ce                | +03c
.L08f0c4:
        move.w  #0x14a,d1                       | +040

| ----------------------------------------------------------------------------
|  Snd_ByParam9A_B_08f0d0  @ $08F0D0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Snd_ByParam9A_B_08f0d0, "ax", @progbits
        .global Snd_ByParam9A_B_08f0d0
Snd_ByParam9A_B_08f0d0:
        cmpi.b  #0x0,0x9a(a6)                   | +000
        bne.w   .L08f0e4                        | +006
        move.w  #0x149,d1                       | +00a
        jsr     0x236e.l                        | +00e
.L08f0e4:
        cmpi.b  #0x1,0x9a(a6)                   | +014
        bne.w   .L08f0fc                        | +01a
        move.w  #0xf7,d1                        | +01e
        jsr     0x236e.l                        | +022
        bra.w   JsrAbsRts_08f106                | +028
.L08f0fc:
        move.w  #0x149,d1                       | +02c

| ----------------------------------------------------------------------------
|  Prio_Set8018_08f108  @ $08F108  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Prio_Set8018_08f108, "ax", @progbits
        .global Prio_Set8018_08f108
Prio_Set8018_08f108:
        move.w  #0x8000,0x38(a6)                | +000
        andi.w  #0xffe3,0x38(a6)                | +006
        ori.w   #0x18,0x38(a6)                  | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_08f11c  @ $08F11C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_08f11c, "ax", @progbits
        .global Entity_CmpPrioWithSibling_08f11c
Entity_CmpPrioWithSibling_08f11c:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_08f132                    | +00c

| ----------------------------------------------------------------------------
|  Ring_Reset_08f138  @ $08F138  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Ring_Reset_08f138, "ax", @progbits
        .global Ring_Reset_08f138
Ring_Reset_08f138:
        clr.w   (a0)                            | +000
        clr.w   0x2(a0)                         | +002
        clr.w   0x4(a0)                         | +006
        clr.w   0x6(a0)                         | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Ring_Compact_08f148  @ $08F148  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Ring_Compact_08f148, "ax", @progbits
        .global Ring_Compact_08f148
Ring_Compact_08f148:
        move.w  0x4(a0),(a0)                    | +000
        move.w  0x2(a0),0x4(a0)                 | +004
        clr.w   0x6(a0)                         | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Rings_InitAll_08f158  @ $08F158  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Rings_InitAll_08f158, "ax", @progbits
        .global Rings_InitAll_08f158
Rings_InitAll_08f158:
        lea     0x10e2f2.l,a0                   | +000
        bsr.b   Ring_Reset_08f138               | +006
        lea     0x10e33a.l,a0                   | +008
        bsr.b   Ring_Reset_08f138               | +00e
        lea     0x10e362.l,a0                   | +010
        bsr.b   Ring_Reset_08f138               | +016
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Rings_CompactAll_08f172  @ $08F172  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Rings_CompactAll_08f172, "ax", @progbits
        .global Rings_CompactAll_08f172
Rings_CompactAll_08f172:
        lea     0x10e2f2.l,a0                   | +000
        bsr.b   Ring_Compact_08f148             | +006
        lea     0x10e33a.l,a0                   | +008
        bsr.b   Ring_Compact_08f148             | +00e
        lea     0x10e362.l,a0                   | +010
        bsr.b   Ring_Compact_08f148             | +016
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Zone_Tmpl152_08f18c  @ $08F18C  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Zone_Tmpl152_08f18c, "ax", @progbits
        .global Zone_Tmpl152_08f18c
Zone_Tmpl152_08f18c:
        move.w  0x22(a6),d0                     | +000
        add.w   0x106f50.l,d0                   | +004
        move.w  d0,0x22(a6)                     | +00a
        clr.w   d1                              | +00e
        move.b  0x98(a6),d1                     | +010
        lsl.w   #0x4,d1                         | +014
        add.w   d1,d0                           | +016
        move.w  d0,0x70(a6)                     | +018
        move.w  0x24(a6),d0                     | +01c
        subi.w  #0x200,d0                       | +020
        neg.w   d0                              | +024
        add.w   0x106f54.l,d0                   | +026
        move.w  d0,0x24(a6)                     | +02c
        clr.w   d1                              | +030
        move.b  0x99(a6),d1                     | +032
        lsl.w   #0x4,d1                         | +036
        add.w   d1,d0                           | +038
        move.w  d0,0x72(a6)                     | +03a
        lea     .L08f1d0(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L08f1d0:
        move.w  0x70(a6),d0                     | +044
        cmp.w   0x106f50.l,d0                   | +048
        bcc.w   ZoneRing_Push_08f1ec            | +04e

| ----------------------------------------------------------------------------
|  ZoneRing_Push_08f1ec  @ $08F1EC  (72 B)
| ----------------------------------------------------------------------------
        .section .text.ZoneRing_Push_08f1ec, "ax", @progbits
        .global ZoneRing_Push_08f1ec
ZoneRing_Push_08f1ec:
        lea     0x10e2f2.l,a0                   | +000
        lea     0x8(a0),a1                      | +006
        adda.w  0x2(a0),a1                      | +00a
        move.w  0x22(a6),(a1)                   | +00e
        move.w  0x24(a6),0x2(a1)                | +012
        move.w  0x70(a6),0x4(a1)                | +018
        move.w  0x72(a6),0x6(a1)                | +01e
        cmpi.w  #0x4,0x6(a0)                    | +024
        bcc.w   .L08f232                        | +02a
        move.w  0x2(a0),d7                      | +02e
        addq.w  #0x8,d7                         | +032
        cmpi.w  #0x40,d7                        | +034
        bcs.w   .L08f22a                        | +038
        clr.w   d7                              | +03c
.L08f22a:
        move.w  d7,0x2(a0)                      | +03e
        addq.w  #0x1,0x6(a0)                    | +042
.L08f232:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  ZoneRing_PushOffset_08f234  @ $08F234  (108 B)
| ----------------------------------------------------------------------------
        .section .text.ZoneRing_PushOffset_08f234, "ax", @progbits
        .global ZoneRing_PushOffset_08f234
ZoneRing_PushOffset_08f234:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        add.w   0x106f50.l,d0                   | +008
        subi.w  #0x200,d1                       | +00e
        neg.w   d1                              | +012
        add.w   0x106f54.l,d1                   | +014
        move.w  d0,d2                           | +01a
        move.w  d1,d3                           | +01c
        add.w   (a0),d0                         | +01e
        add.w   0x2(a0),d2                      | +020
        add.w   0x4(a0),d1                      | +024
        add.w   0x6(a0),d3                      | +028
        lea     0x10e2f2.l,a0                   | +02c
        lea     0x8(a0),a1                      | +032
        adda.w  0x2(a0),a1                      | +036
        move.w  d0,(a1)                         | +03a
        move.w  d1,0x2(a1)                      | +03c
        move.w  d2,0x4(a1)                      | +040
        move.w  d3,0x6(a1)                      | +044
        cmpi.w  #0x4,0x6(a0)                    | +048
        bcc.w   .L08f29e                        | +04e
        move.w  0x2(a0),d7                      | +052
        addq.w  #0x8,d7                         | +056
        cmpi.w  #0x40,d7                        | +058
        bcs.w   .L08f296                        | +05c
        clr.w   d7                              | +060
.L08f296:
        move.w  d7,0x2(a0)                      | +062
        addq.w  #0x1,0x6(a0)                    | +066
.L08f29e:
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  ZoneRing_HitTest_08f2a0  @ $08F2A0  (78 B)
| ----------------------------------------------------------------------------
        .section .text.ZoneRing_HitTest_08f2a0, "ax", @progbits
        .global ZoneRing_HitTest_08f2a0
ZoneRing_HitTest_08f2a0:
        move.w  0x22(a6),d0                     | +000
        add.w   0x106f50.l,d0                   | +004
        move.w  0x24(a6),d1                     | +00a
        subi.w  #0x200,d1                       | +00e
        neg.w   d1                              | +012
        add.w   0x106f54.l,d1                   | +014
        lea     0x10e2f2.l,a0                   | +01a
        lea     0x8(a0),a1                      | +020
        move.w  (a0),d7                         | +024
        .global ZoneRing_HitTest_08f2a0__L08f2c6
ZoneRing_HitTest_08f2a0__L08f2c6:
        cmp.w   0x4(a0),d7                      | +026
        beq.w   ClearC_08f302                   | +02a
        cmp.w   (a1,d7.w),d0                    | +02e
        bcs.w   ZoneRing_HitTest_Next_08f2f4    | +032
        cmp.w   0x4(a1,d7.w),d0                 | +036
        bhi.w   ZoneRing_HitTest_Next_08f2f4    | +03a
        cmp.w   0x2(a1,d7.w),d1                 | +03e
        bcs.w   ZoneRing_HitTest_Next_08f2f4    | +042
        cmp.w   0x6(a1,d7.w),d1                 | +046
        bhi.w   ZoneRing_HitTest_Next_08f2f4    | +04a

| ----------------------------------------------------------------------------
|  ZoneRing_HitTest_Next_08f2f4  @ $08F2F4  (14 B)
| ----------------------------------------------------------------------------
        .section .text.ZoneRing_HitTest_Next_08f2f4, "ax", @progbits
        .global ZoneRing_HitTest_Next_08f2f4
ZoneRing_HitTest_Next_08f2f4:
        addq.w  #0x8,d7                         | +000
        cmpi.w  #0x40,d7                        | +002
        bcs.w   .L08f300                        | +006
        clr.w   d7                              | +00a
.L08f300:
        bra.b   ZoneRing_HitTest_08f2a0__L08f2c6 | +00c

| ----------------------------------------------------------------------------
|  PosRing_FindNear_08f344  @ $08F344  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PosRing_FindNear_08f344, "ax", @progbits
        .global PosRing_FindNear_08f344
PosRing_FindNear_08f344:
        lea     0x10e33a.l,a0                   | +000
        lea     0x8(a0),a1                      | +006
        move.w  (a0),d7                         | +00a
        .global PosRing_FindNear_08f344__L08f350
PosRing_FindNear_08f344__L08f350:
        cmp.w   0x4(a0),d7                      | +00c
        beq.w   ClearC_08f3a0                   | +010
        move.w  (a1,d7.w),d0                    | +014
        move.w  0x2(a1,d7.w),d1                 | +018
        sub.w   0x22(a6),d0                     | +01c
        smi.b   d2                              | +020
        sub.w   0x24(a6),d1                     | +022
        addi.w  #0x40,d0                        | +026
        addi.w  #0x30,d1                        | +02a
        cmpi.w  #0x80,d0                        | +02e
        bcc.w   PosRing_FindNear_Next_08f392    | +032
        cmpi.w  #0x60,d1                        | +036
        bcc.w   PosRing_FindNear_Next_08f392    | +03a
        btst    #0x0,0x3a(a6)                   | +03e
        seq.b   d0                              | +044
        eor.b   d2,d0                           | +046

| ----------------------------------------------------------------------------
|  PosRing_FindNear_Next_08f392  @ $08F392  (14 B)
| ----------------------------------------------------------------------------
        .section .text.PosRing_FindNear_Next_08f392, "ax", @progbits
        .global PosRing_FindNear_Next_08f392
PosRing_FindNear_Next_08f392:
        addq.w  #0x4,d7                         | +000
        cmpi.w  #0x20,d7                        | +002
        bcs.w   .L08f39e                        | +006
        clr.w   d7                              | +00a
.L08f39e:
        bra.b   PosRing_FindNear_08f344__L08f350 | +00c

| ----------------------------------------------------------------------------
|  TargetRing_NewId_08f3a6  @ $08F3A6  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_NewId_08f3a6, "ax", @progbits
        .global TargetRing_NewId_08f3a6
TargetRing_NewId_08f3a6:
        move.b  0x10e2f0.l,d0                   | +000
        addq.b  #0x1,d0                         | +006
        andi.b  #0x3f,d0                        | +008
        addi.b  #0x40,d0                        | +00c
        move.b  d0,0x10e2f0.l                   | +010
        rts                                     | +016

| ----------------------------------------------------------------------------
|  TargetRing_Register_08f3be  @ $08F3BE  (178 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_Register_08f3be, "ax", @progbits
        .global TargetRing_Register_08f3be
TargetRing_Register_08f3be:
        lea     0x10e362.l,a0                   | +000
        lea     0x8(a0),a1                      | +006
        move.b  #0xfe,d3                        | +00a
        move.w  (a0),d7                         | +00e
.L08f3ce:
        cmp.w   0x4(a0),d7                      | +010
        beq.w   .L08f3f8                        | +014
        cmp.b   0x4(a1,d7.w),d2                 | +018
        bne.w   .L08f3ea                        | +01c
        move.b  0x5(a1,d7.w),d4                 | +020
        bmi.w   .L08f3ea                        | +024
        move.b  #0xff,d3                        | +028
.L08f3ea:
        addq.w  #0x6,d7                         | +02c
        cmpi.w  #0x30,d7                        | +02e
        bcs.w   .L08f3f6                        | +032
        clr.w   d7                              | +036
.L08f3f6:
        bra.b   .L08f3ce                        | +038
.L08f3f8:
        move.w  0x22(a6),d5                     | +03a
        subi.w  #0x10,d5                        | +03e
        subi.w  #0x130,d5                       | +042
        bcc.w   .L08f46e                        | +046
.L08f408:
        cmp.w   0x2(a0),d7                      | +04a
        beq.w   .L08f43c                        | +04e
        cmp.b   0x4(a1,d7.w),d2                 | +052
        bne.w   .L08f42e                        | +056
        move.b  0x5(a1,d7.w),d5                 | +05a
        bpl.w   .L08f42c                        | +05e
        move.w  d0,(a1,d7.w)                    | +062
        move.w  d1,0x2(a1,d7.w)                 | +066
        move.b  d4,0x5(a1,d7.w)                 | +06a
.L08f42c:
        rts                                     | +06e
.L08f42e:
        addq.w  #0x6,d7                         | +070
        cmpi.w  #0x30,d7                        | +072
        bcs.w   .L08f43a                        | +076
        clr.w   d7                              | +07a
.L08f43a:
        bra.b   .L08f408                        | +07c
.L08f43c:
        adda.w  d7,a1                           | +07e
        move.w  d0,(a1)                         | +080
        move.w  d1,0x2(a1)                      | +082
        move.b  d2,0x4(a1)                      | +086
        move.b  d3,0x5(a1)                      | +08a
        cmpi.w  #0x4,0x6(a0)                    | +08e
        bcc.w   .L08f46e                        | +094
        move.w  0x2(a0),d7                      | +098
        addq.w  #0x6,d7                         | +09c
        cmpi.w  #0x30,d7                        | +09e
        bcs.w   .L08f466                        | +0a2
        clr.w   d7                              | +0a6
.L08f466:
        move.w  d7,0x2(a0)                      | +0a8
        addq.w  #0x1,0x6(a0)                    | +0ac
.L08f46e:
        rts                                     | +0b0

| ----------------------------------------------------------------------------
|  TargetRing_FindPending_08f470  @ $08F470  (108 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_FindPending_08f470, "ax", @progbits
        .global TargetRing_FindPending_08f470
TargetRing_FindPending_08f470:
        lea     0x10e362.l,a0                   | +000
        lea     0x8(a0),a1                      | +006
        move.w  (a0),d7                         | +00a
        cmpa.l  #0x100440,a6                    | +00c
        beq.w   .L08f4bc                        | +012
        cmpa.l  #0x1004e0,a6                    | +016
        beq.w   .L08f4bc                        | +01c
.L08f490:
        cmp.w   0x4(a0),d7                      | +020
        beq.w   .L08f4b8                        | +024
        tst.b   0x4(a1,d7.w)                    | +028
        bpl.w   .L08f4aa                        | +02c
        cmpi.b  #0xfe,0x5(a1,d7.w)              | +030
        beq.w   TargetRing_InRange_08f4e2       | +036
.L08f4aa:
        addq.w  #0x6,d7                         | +03a
        cmpi.w  #0x30,d7                        | +03c
        bcs.w   .L08f4b6                        | +040
        clr.w   d7                              | +044
.L08f4b6:
        bra.b   .L08f490                        | +046
.L08f4b8:
        bra.w   ClearC_08f4dc                   | +048
.L08f4bc:
        cmp.w   0x4(a0),d7                      | +04c
        beq.w   ClearC_08f4dc                   | +050
        cmpi.b  #0xfe,0x5(a1,d7.w)              | +054
        beq.w   TargetRing_InRange_08f4e2       | +05a
        addq.w  #0x6,d7                         | +05e
        cmpi.w  #0x30,d7                        | +060
        bcs.w   .L08f4da                        | +064
        clr.w   d7                              | +068
.L08f4da:
        bra.b   .L08f4bc                        | +06a

| ----------------------------------------------------------------------------
|  TargetRing_InRange_08f4e2  @ $08F4E2  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_InRange_08f4e2, "ax", @progbits
        .global TargetRing_InRange_08f4e2
TargetRing_InRange_08f4e2:
        lea     (a1,d7.w),a2                    | +000
        move.w  (a2),d0                         | +004
        move.w  0x2(a2),d1                      | +006
        moveq   #-1,d2                          | +00a
        move.b  0x4(a2),d2                      | +00c
        move.w  0x22(a6),d4                     | +010
        sub.w   d0,d4                           | +014
        addi.w  #0x18,d4                        | +016
        cmpi.w  #0x30,d4                        | +01a
        bcc.w   SetC_08f51a                     | +01e
        move.w  0x24(a6),d4                     | +022
        sub.w   d1,d4                           | +026
        addi.w  #0x20,d4                        | +028
        cmpi.w  #0x40,d4                        | +02c
        bcc.w   SetC_08f51a                     | +030
        andi.w  #0xff,d2                        | +034

| ----------------------------------------------------------------------------
|  TargetRing_ClaimById_08f520  @ $08F520  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_ClaimById_08f520, "ax", @progbits
        .global TargetRing_ClaimById_08f520
TargetRing_ClaimById_08f520:
        lea     0x10e362.l,a0                   | +000
        lea     0x8(a0),a1                      | +006
        move.w  (a0),d7                         | +00a
        move.b  #0x0,d3                         | +00c
        cmpa.l  #0x100440,a6                    | +010
        beq.w   .L08f54c                        | +016
        move.b  #0x1,d3                         | +01a
        cmpa.l  #0x1004e0,a6                    | +01e
        beq.w   .L08f54c                        | +024
        move.b  #0x2,d3                         | +028
.L08f54c:
        cmp.w   0x4(a0),d7                      | +02c
        beq.w   ClearC_08f574                   | +030
        cmp.b   0x4(a1,d7.w),d2                 | +034
        bne.w   .L08f566                        | +038
        cmpi.b  #0xfe,0x5(a1,d7.w)              | +03c
        beq.w   TargetRing_ClaimById_Mark_08f57a | +042
.L08f566:
        addq.w  #0x6,d7                         | +046
        cmpi.w  #0x30,d7                        | +048
        bcs.w   .L08f572                        | +04c
        clr.w   d7                              | +050
.L08f572:
        bra.b   .L08f54c                        | +052

| ----------------------------------------------------------------------------
|  TargetRing_ClaimById_Mark_08f57a  @ $08F57A  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_ClaimById_Mark_08f57a, "ax", @progbits
        .global TargetRing_ClaimById_Mark_08f57a
TargetRing_ClaimById_Mark_08f57a:
        move.b  #0xff,0x5(a1,d7.w)              | +000
        move.w  0x4(a0),d7                      | +006
        .global TargetRing_ClaimById_Mark_08f57a__L08f584
TargetRing_ClaimById_Mark_08f57a__L08f584:
        cmp.w   0x2(a0),d7                      | +00a
        beq.w   TargetRing_ClaimById_Append_08f5ac | +00e
        cmp.b   0x4(a1,d7.w),d2                 | +012
        bne.w   TargetRing_ClaimById_Next_08f59e | +016
        move.b  d3,0x5(a1,d7.w)                 | +01a

| ----------------------------------------------------------------------------
|  TargetRing_ClaimById_Next_08f59e  @ $08F59E  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_ClaimById_Next_08f59e, "ax", @progbits
        .global TargetRing_ClaimById_Next_08f59e
TargetRing_ClaimById_Next_08f59e:
        addq.w  #0x6,d7                         | +000
        cmpi.w  #0x30,d7                        | +002
        bcs.w   .L08f5aa                        | +006
        clr.w   d7                              | +00a
.L08f5aa:
        bra.b   TargetRing_ClaimById_Mark_08f57a__L08f584 | +00c

| ----------------------------------------------------------------------------
|  TargetRing_ClaimById_Append_08f5ac  @ $08F5AC  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_ClaimById_Append_08f5ac, "ax", @progbits
        .global TargetRing_ClaimById_Append_08f5ac
TargetRing_ClaimById_Append_08f5ac:
        move.b  d2,0x4(a1,d7.w)                 | +000
        move.b  d3,0x5(a1,d7.w)                 | +004
        cmpi.w  #0x4,0x6(a0)                    | +008
        bcc.w   SetC_08f5d6                     | +00e
        move.w  0x2(a0),d7                      | +012
        addq.w  #0x6,d7                         | +016
        cmpi.w  #0x30,d7                        | +018
        bcs.w   .L08f5ce                        | +01c
        clr.w   d7                              | +020
.L08f5ce:
        move.w  d7,0x2(a0)                      | +022
        addq.w  #0x1,0x6(a0)                    | +026

| ----------------------------------------------------------------------------
|  TargetRing_ClaimByKey_08f5dc  @ $08F5DC  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_ClaimByKey_08f5dc, "ax", @progbits
        .global TargetRing_ClaimByKey_08f5dc
TargetRing_ClaimByKey_08f5dc:
        lea     0x10e362.l,a0                   | +000
        lea     0x8(a0),a1                      | +006
        move.w  (a0),d7                         | +00a
.L08f5e8:
        cmp.w   0x4(a0),d7                      | +00c
        beq.w   ClearC_08f60e                   | +010
        cmp.b   0x4(a1,d7.w),d0                 | +014
        bne.w   .L08f600                        | +018
        tst.b   0x5(a1,d7.w)                    | +01c
        bpl.w   TargetRing_ClaimByKey_Found_08f614 | +020
.L08f600:
        addq.w  #0x6,d7                         | +024
        cmpi.w  #0x30,d7                        | +026
        bcs.w   .L08f60c                        | +02a
        clr.w   d7                              | +02e
.L08f60c:
        bra.b   .L08f5e8                        | +030

| ----------------------------------------------------------------------------
|  TargetRing_ClaimByKey_Found_08f614  @ $08F614  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_ClaimByKey_Found_08f614, "ax", @progbits
        .global TargetRing_ClaimByKey_Found_08f614
TargetRing_ClaimByKey_Found_08f614:
        move.b  d0,d2                           | +000
        move.w  (a1,d7.w),d0                    | +002
        move.w  0x2(a1,d7.w),d1                 | +006
        move.b  #0x0,d3                         | +00a
        cmpa.l  #0x100440,a6                    | +00e
        beq.w   .L08f63e                        | +014
        move.b  #0x1,d3                         | +018
        cmpa.l  #0x1004e0,a6                    | +01c
        beq.w   .L08f63e                        | +022
        move.b  #0x2,d3                         | +026
.L08f63e:
        move.w  0x4(a0),d7                      | +02a
        .global TargetRing_ClaimByKey_Found_08f614__L08f642
TargetRing_ClaimByKey_Found_08f614__L08f642:
        cmp.w   0x2(a0),d7                      | +02e
        beq.w   TargetRing_ClaimByKey_Append_08f66a | +032
        cmp.b   0x4(a1,d7.w),d2                 | +036
        bne.w   TargetRing_ClaimByKey_Next_08f65c | +03a
        move.b  d3,0x5(a1,d7.w)                 | +03e

| ----------------------------------------------------------------------------
|  TargetRing_ClaimByKey_Next_08f65c  @ $08F65C  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_ClaimByKey_Next_08f65c, "ax", @progbits
        .global TargetRing_ClaimByKey_Next_08f65c
TargetRing_ClaimByKey_Next_08f65c:
        addq.w  #0x6,d7                         | +000
        cmpi.w  #0x30,d7                        | +002
        bcs.w   .L08f668                        | +006
        clr.w   d7                              | +00a
.L08f668:
        bra.b   TargetRing_ClaimByKey_Found_08f614__L08f642 | +00c

| ----------------------------------------------------------------------------
|  TargetRing_ClaimByKey_Append_08f66a  @ $08F66A  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TargetRing_ClaimByKey_Append_08f66a, "ax", @progbits
        .global TargetRing_ClaimByKey_Append_08f66a
TargetRing_ClaimByKey_Append_08f66a:
        move.b  d2,0x4(a1,d7.w)                 | +000
        move.b  #0xff,0x5(a1,d7.w)              | +004
        cmpi.w  #0x4,0x6(a0)                    | +00a
        bcc.w   SetC_08f696                     | +010
        move.w  0x2(a0),d7                      | +014
        addq.w  #0x6,d7                         | +018
        cmpi.w  #0x30,d7                        | +01a
        bcs.w   .L08f68e                        | +01e
        clr.w   d7                              | +022
.L08f68e:
        move.w  d7,0x2(a0)                      | +024
        addq.w  #0x1,0x6(a0)                    | +028

| ----------------------------------------------------------------------------
|  Turret8_SndByState_08f69c  @ $08F69C  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_SndByState_08f69c, "ax", @progbits
        .global Turret8_SndByState_08f69c
Turret8_SndByState_08f69c:
        move.w  #0x134,d1                       | +000
        tst.b   d4                              | +004
        beq.w   .L08f6b4                        | +006
        move.w  #0x152,d1                       | +00a
        subq.b  #0x1,d4                         | +00e
        beq.w   .L08f6b4                        | +010
        move.w  #0xe,d1                         | +014
.L08f6b4:
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_08f6b6  @ $08F6B6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_08f6b6, "ax", @progbits
        .global Entity_CmpPrioWithSibling_08f6b6
Entity_CmpPrioWithSibling_08f6b6:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_08f6cc                    | +00c
