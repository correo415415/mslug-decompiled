| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $09773C..$099F3A  (9,252 B, 125 entradas, 96 huecos)
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
|  HiScore_LoadDefaults_09773c  @ $09773C  (134 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_LoadDefaults_09773c, "ax", @progbits
        .global HiScore_LoadDefaults_09773c
HiScore_LoadDefaults_09773c:
        lea     0x2f53b2.l,a0                   | +000
        lea     0x100002.l,a1                   | +006
        move.b  (a0)+,(a1)+                     | +00c
        move.b  (a0)+,(a1)+                     | +00e
        move.l  (a0)+,(a1)+                     | +010
        move.w  (a0)+,(a1)+                     | +012
        move.w  (a0)+,(a1)+                     | +014
        move.w  (a0)+,(a1)+                     | +016
        move.b  (a0)+,(a1)+                     | +018
        move.b  (a0)+,(a1)+                     | +01a
        move.l  (a0)+,(a1)+                     | +01c
        move.w  (a0)+,(a1)+                     | +01e
        move.w  (a0)+,(a1)+                     | +020
        move.w  (a0)+,(a1)+                     | +022
        move.b  (a0)+,(a1)+                     | +024
        move.b  (a0)+,(a1)+                     | +026
        move.l  (a0)+,(a1)+                     | +028
        move.w  (a0)+,(a1)+                     | +02a
        move.w  (a0)+,(a1)+                     | +02c
        move.w  (a0)+,(a1)+                     | +02e
        move.b  (a0)+,(a1)+                     | +030
        move.b  (a0)+,(a1)+                     | +032
        move.l  (a0)+,(a1)+                     | +034
        move.w  (a0)+,(a1)+                     | +036
        move.w  (a0)+,(a1)+                     | +038
        move.w  (a0)+,(a1)+                     | +03a
        move.b  (a0)+,(a1)+                     | +03c
        move.b  (a0)+,(a1)+                     | +03e
        move.l  (a0)+,(a1)+                     | +040
        move.w  (a0)+,(a1)+                     | +042
        move.w  (a0)+,(a1)+                     | +044
        move.w  (a0)+,(a1)+                     | +046
        move.b  (a0)+,(a1)+                     | +048
        move.b  (a0)+,(a1)+                     | +04a
        move.l  (a0)+,(a1)+                     | +04c
        move.w  (a0)+,(a1)+                     | +04e
        move.w  (a0)+,(a1)+                     | +050
        move.w  (a0)+,(a1)+                     | +052
        move.b  (a0)+,(a1)+                     | +054
        move.b  (a0)+,(a1)+                     | +056
        move.l  (a0)+,(a1)+                     | +058
        move.w  (a0)+,(a1)+                     | +05a
        move.w  (a0)+,(a1)+                     | +05c
        move.w  (a0)+,(a1)+                     | +05e
        move.b  (a0)+,(a1)+                     | +060
        move.b  (a0)+,(a1)+                     | +062
        move.l  (a0)+,(a1)+                     | +064
        move.w  (a0)+,(a1)+                     | +066
        move.w  (a0)+,(a1)+                     | +068
        move.w  (a0)+,(a1)+                     | +06a
        move.b  (a0)+,(a1)+                     | +06c
        move.b  (a0)+,(a1)+                     | +06e
        move.l  (a0)+,(a1)+                     | +070
        move.w  (a0)+,(a1)+                     | +072
        move.w  (a0)+,(a1)+                     | +074
        move.w  (a0)+,(a1)+                     | +076
        move.b  (a0)+,(a1)+                     | +078
        move.b  (a0)+,(a1)+                     | +07a
        move.l  (a0)+,(a1)+                     | +07c
        move.w  (a0)+,(a1)+                     | +07e
        move.w  (a0)+,(a1)+                     | +080
        move.w  (a0)+,(a1)+                     | +082
        rts                                     | +084

| ----------------------------------------------------------------------------
|  HiScore_Tpl_StaticLogo_0977c2  @ $0977C2  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_Tpl_StaticLogo_0977c2, "ax", @progbits
        .global HiScore_Tpl_StaticLogo_0977c2
HiScore_Tpl_StaticLogo_0977c2:
        lea     HiScore_Logo_Static_097962(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.w  #0x3c,0x80(a6)                  | +00a
        bra.w   HiScore_Tpl_Common_097816 | +010

| ----------------------------------------------------------------------------
|  HiScore_Tpl_Frame_0977d6  @ $0977D6  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_Tpl_Frame_0977d6, "ax", @progbits
        .global HiScore_Tpl_Frame_0977d6
HiScore_Tpl_Frame_0977d6:
        lea     HiScore_Logo_FadeIn_09794c(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.w  #0x3c,0x80(a6)                  | +00a
        bra.w   HiScore_Tpl_ClearFlags_0977fa | +010

| ----------------------------------------------------------------------------
|  HiScore_Tpl_Loader_0977ea  @ $0977EA  (96 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_Tpl_Loader_0977ea, "ax", @progbits
        .global HiScore_Tpl_Loader_0977ea
HiScore_Tpl_Loader_0977ea:
        lea     HiScore_Logo_FadeIn_09794c(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.w  #0x3c,0x80(a6)                  | +00a
        .global HiScore_Tpl_ClearFlags_0977fa
HiScore_Tpl_ClearFlags_0977fa:
        lea     0x100002.l,a1                   | +010
        moveq   #0,d1                           | +016
        move.b  d1,(a1)                         | +018
        move.b  d1,(a1)                         | +01a
        move.b  d1,(a1)                         | +01c
        move.b  d1,(a1)                         | +01e
        move.b  d1,(a1)                         | +020
        move.b  d1,(a1)                         | +022
        move.b  d1,(a1)                         | +024
        move.b  d1,(a1)                         | +026
        move.b  d1,(a1)                         | +028
        move.b  d1,(a1)                         | +02a
        .global HiScore_Tpl_Common_097816
HiScore_Tpl_Common_097816:
        move.b  #0xb,d0                         | +02c
        jsr     0x43568.l                       | +030
        moveq   #0,d0                           | +036
        moveq   #0,d1                           | +038
        jsr     0x437da.l                       | +03a
        clr.b   0x21(a6)                        | +040
        move.w  #0x0,0x70(a6)                   | +044
        movea.l #0x719b,a1                      | +04a
        lea     0x2f54a0.l,a2                   | +050
        move.w  #0x4300,d0                      | +056
        jsr     0x5dad8.l                       | +05a

| ----------------------------------------------------------------------------
|  HiScore_WaitLogo_097852  @ $097852  (42 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_WaitLogo_097852, "ax", @progbits
        .global HiScore_WaitLogo_097852
HiScore_WaitLogo_097852:
        tst.b   0x21(a6)                        | +000
        bne.w   .L09785c                        | +004
        rts                                     | +008
.L09785c:
        clr.b   0x20(a6)                        | +00a
        movea.l #0x70a6,a1                      | +00e
        move.l  a1,0x76(a6)                     | +014
        move.l  #0x100002,0x72(a6)              | +018
        cmpi.b  #0xff,0x21(a6)                  | +020
        bne.w   SetTaskHandler_097884           | +026

| ----------------------------------------------------------------------------
|  HiScore_HeaderDelay_09788c  @ $09788C  (24 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_HeaderDelay_09788c, "ax", @progbits
        .global HiScore_HeaderDelay_09788c
HiScore_HeaderDelay_09788c:
        addq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x10,0x70(a6)                  | +004
        blt.w   SetHandlerRts_0978aa            | +00a
        move.w  #0x0,0x70(a6)                   | +00e
        jsr     HiScore_DrawHeaders_0979f8(pc)  | +014

| ----------------------------------------------------------------------------
|  HiScore_DrawRowsStep_0978ac  @ $0978AC  (78 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_DrawRowsStep_0978ac, "ax", @progbits
        .global HiScore_DrawRowsStep_0978ac
HiScore_DrawRowsStep_0978ac:
        addq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x2,0x70(a6)                   | +004
        blt.w   .L0978f8                        | +00a
        movea.l 0x76(a6),a1                     | +00e
        movea.l 0x72(a6),a0                     | +012
        movem.l a0-a1,-(a7)                     | +016
        jsr     HiScore_DrawRow_097b74(pc)      | +01a
        movem.l (a7)+,a0-a1                     | +01e
        adda.l  #0x2,a1                         | +022
        adda.l  #0xc,a0                         | +028
        cmpa.l  #0x10007a,a0                    | +02e
        bne.w   .L0978ea                        | +034
        lea     HiScore_WaitExit_09792e(pc),a1  | +038
        move.l  a1,(a6)                         | +03c
.L0978ea:
        move.w  #0x0,0x70(a6)                   | +03e
        move.l  a0,0x72(a6)                     | +044
        move.l  a1,0x76(a6)                     | +048
.L0978f8:
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  HiScore_DrawAllRows_0978fa  @ $0978FA  (44 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_DrawAllRows_0978fa, "ax", @progbits
        .global HiScore_DrawAllRows_0978fa
HiScore_DrawAllRows_0978fa:
        jsr     HiScore_DrawHeaders_0979f8(pc)  | +000
        movea.l 0x76(a6),a1                     | +004
        movea.l 0x72(a6),a0                     | +008
        move.w  #0x9,d5                         | +00c
.L09790a:
        movem.l d5/a0-a1,-(a7)                  | +010
        jsr     HiScore_DrawRow_097b74(pc)      | +014
        movem.l (a7)+,d5/a0-a1                  | +018
        adda.l  #0x2,a1                         | +01c
        adda.l  #0xc,a0                         | +022
        dbra    d5,.L09790a                     | +028

| ----------------------------------------------------------------------------
|  HiScore_WaitExit_09792e  @ $09792E  (22 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_WaitExit_09792e, "ax", @progbits
        .global HiScore_WaitExit_09792e
HiScore_WaitExit_09792e:
        tst.b   0x20(a6)                        | +000
        bne.w   TaskHandler_09794a              | +004
        subq.w  #0x1,0x80(a6)                   | +008
        bne.w   TaskHandler_09794a              | +00c
        clr.b   0x106ed2.l                      | +010

| ----------------------------------------------------------------------------
|  HiScore_Logo_FadeIn_09794c  @ $09794C  (164 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_Logo_FadeIn_09794c, "ax", @progbits
        .global HiScore_Logo_FadeIn_09794c
HiScore_Logo_FadeIn_09794c:
        move.b  #0xff,0x21(a6)                  | +000
        move.b  #0x0,0x32(a6)                   | +006
        move.w  #0x0,0x70(a6)                   | +00c
        bra.w   .L097974                        | +012
        .global HiScore_Logo_Static_097962
HiScore_Logo_Static_097962:
        move.b  #0x7f,0x21(a6)                  | +016
        move.b  #0xff,0x32(a6)                  | +01c
        move.w  #0xff,0x70(a6)                  | +022
.L097974:
        move.w  #0x15f,d1                       | +028
        jsr     0x236e.l                        | +02c
        move.w  #0xa0,0x22(a6)                  | +032
        move.w  #0x180,0x24(a6)                 | +038
        move.b  #0x0,0x20(a6)                   | +03e
        jsr     0x5e9b6.l                       | +044
        andi.w  #0x1,d0                         | +04a
        movea.l #0x2f5504,a0                    | +04e
        lsl.w   #0x2,d0                         | +054
        movea.l (a0,d0.w),a0                    | +056
        cmpa.l  #0xffffffff,a0                  | +05a
        beq.w   .L0979b6                        | +060
        jsr     0x28cd4.l                       | +064
.L0979b6:
        lea     .L0979bc(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L0979bc:
        tst.b   0x20(a6)                        | +070
        bne.w   JsrAbsThunk_0979f0              | +074
        addi.b  #0x20,0x32(a6)                  | +078
        addi.w  #0x20,0x70(a6)                  | +07e
        cmpi.w  #0xff,0x70(a6)                  | +084
        bcs.w   JsrAbsThunk_0979f0              | +08a
        move.b  #0xff,0x32(a6)                  | +08e
        move.b  #0xff,0x20(a6)                  | +094
        movea.l 0xc(a6),a0                      | +09a
        move.b  0x21(a6),0x21(a0)               | +09e

| ----------------------------------------------------------------------------
|  HiScore_DrawHeaders_0979f8  @ $0979F8  (72 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_DrawHeaders_0979f8, "ax", @progbits
        .global HiScore_DrawHeaders_0979f8
HiScore_DrawHeaders_0979f8:
        movea.l #0x7083,a1                      | +000
        lea     0x2f547c.l,a2                   | +006
        move.w  #0x4000,d0                      | +00c
        moveq   #2,d1                           | +010
        moveq   #2,d2                           | +012
        jsr     0x5db1a.l                       | +014
        movea.l #0x71a3,a1                      | +01a
        lea     0x2f5486.l,a2                   | +020
        move.w  #0x4000,d0                      | +026
        moveq   #2,d1                           | +02a
        moveq   #2,d2                           | +02c
        jsr     0x5db1a.l                       | +02e
        movea.l #0x72c3,a1                      | +034
        lea     0x2f5490.l,a2                   | +03a
        move.w  #0x4000,d0                      | +040
        moveq   #2,d1                           | +044
        moveq   #2,d2                           | +046

| ----------------------------------------------------------------------------
|  HiScore_DrawHeaderA_097a48  @ $097A48  (6 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_DrawHeaderA_097a48, "ax", @progbits
        .global HiScore_DrawHeaderA_097a48
HiScore_DrawHeaderA_097a48:
        movea.l #0x7063,a1                      | +000

| ----------------------------------------------------------------------------
|  HiScore_DrawHeaderB_097a54  @ $097A54  (6 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_DrawHeaderB_097a54, "ax", @progbits
        .global HiScore_DrawHeaderB_097a54
HiScore_DrawHeaderB_097a54:
        movea.l #0x7323,a1                      | +000

| ----------------------------------------------------------------------------
|  HiScore_TryEnter_P1_097a60  @ $097A60  (18 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_TryEnter_P1_097a60, "ax", @progbits
        .global HiScore_TryEnter_P1_097a60
HiScore_TryEnter_P1_097a60:
        lea     0x106e94.l,a0                   | +000
        jsr     0x51aa4.l                       | +006
        moveq   #1,d1                           | +00c
        bra.w   HiScore_TryEnter_Common_097a80 | +00e

| ----------------------------------------------------------------------------
|  HiScore_TryEnter_P2_097a72  @ $097A72  (56 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_TryEnter_P2_097a72, "ax", @progbits
        .global HiScore_TryEnter_P2_097a72
HiScore_TryEnter_P2_097a72:
        lea     0x106e9c.l,a0                   | +000
        jsr     0x51aa4.l                       | +006
        moveq   #2,d1                           | +00c
        .global HiScore_TryEnter_Common_097a80
HiScore_TryEnter_Common_097a80:
        jsr     HiScore_InsertScore_097aaa(pc)  | +00e
        bcc.w   .L097aa4                        | +012
        movem.l d0-d1/a1,-(a7)                  | +016
        lea     NameEntry_InitStandalone_097d24(pc),a1 | +01a
        jsr     0x4ae.l                         | +01e
        movem.l (a7)+,d0-d1/a1                  | +024
        move.l  a1,0x80(a0)                     | +028
        move.w  d1,0x88(a0)                     | +02c
        rts                                     | +030
.L097aa4:
        clr.b   0x21(a6)                        | +032
        rts                                     | +036

| ----------------------------------------------------------------------------
|  HiScore_InsertScore_097aaa  @ $097AAA  (186 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_InsertScore_097aaa, "ax", @progbits
        .global HiScore_InsertScore_097aaa
HiScore_InsertScore_097aaa:
        movem.l d0-d1/a1,-(a7)                  | +000
        lea     0x100002.l,a2                   | +004
        moveq   #9,d2                           | +00a
.L097ab6:
        cmp.b   (a2),d1                         | +00c
        bne.w   .L097abe                        | +00e
        clr.b   (a6)                            | +012
.L097abe:
        adda.l  #0xc,a2                         | +014
        dbra    d2,.L097ab6                     | +01a
        moveq   #0,d3                           | +01e
        lea     0x100002.l,a2                   | +020
        move.w  #0x6c,d2                        | +026
.L097ad4:
        cmp.l   0x2(a2,d2.w),d0                 | +02a
        blt.w   .L097ae8                        | +02e
        addq.w  #0x1,d3                         | +032
        subi.w  #0xc,d2                         | +034
        bmi.w   .L097ae8                        | +038
        bra.b   .L097ad4                        | +03c
.L097ae8:
        tst.w   d3                              | +03e
        beq.w   HiScore_InsertScore_NoRank_097b6a | +040
        subq.w  #0x1,d3                         | +044
        beq.w   .L097b38                        | +046
        move.w  #0x6c,d4                        | +04a
        move.w  #0x60,d5                        | +04e
.L097afc:
        move.b  (a2,d5.w),(a2,d4.w)             | +052
        move.b  0x1(a2,d5.w),d6                 | +058
        cmp.l   0x2(a2,d5.w),d0                 | +05c
        beq.w   .L097b10                        | +060
        addq.b  #0x1,d6                         | +064
.L097b10:
        move.b  d6,0x1(a2,d4.w)                 | +066
        move.l  0x2(a2,d5.w),0x2(a2,d4.w)       | +06a
        move.w  0x6(a2,d5.w),0x6(a2,d4.w)       | +070
        move.w  0x8(a2,d5.w),0x8(a2,d4.w)       | +076
        move.w  0xa(a2,d5.w),0xa(a2,d4.w)       | +07c
        subi.w  #0xc,d4                         | +082
        subi.w  #0xc,d5                         | +086
        cmp.w   d5,d2                           | +08a
        bne.b   .L097afc                        | +08c
.L097b38:
        addi.w  #0xc,d2                         | +08e
        move.b  d1,(a2,d2.w)                    | +092
        move.l  d0,0x2(a2,d2.w)                 | +096
        move.w  d1,d0                           | +09a
        jsr     HiScore_RankOfPlayer_098176(pc) | +09c
        move.b  d1,0x1(a2,d2.w)                 | +0a0
        move.w  #0xb40,0x6(a2,d2.w)             | +0a4
        move.w  #0xb40,0x8(a2,d2.w)             | +0aa
        move.w  #0xb40,0xa(a2,d2.w)             | +0b0
        movem.l (a7)+,d0-d1/a1                  | +0b6

| ----------------------------------------------------------------------------
|  HiScore_InsertScore_NoRank_097b6a  @ $097B6A  (4 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_InsertScore_NoRank_097b6a, "ax", @progbits
        .global HiScore_InsertScore_NoRank_097b6a
HiScore_InsertScore_NoRank_097b6a:
        movem.l (a7)+,d0-d1/a1                  | +000

| ----------------------------------------------------------------------------
|  HiScore_DrawRow_097b74  @ $097B74  (134 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_DrawRow_097b74, "ax", @progbits
        .global HiScore_DrawRow_097b74
HiScore_DrawRow_097b74:
        moveq   #0,d0                           | +000
        move.b  (a0)+,d0                        | +002
        tst.b   d0                              | +004
        beq.w   .L097b9c                        | +006
        movem.l a0-a1,-(a7)                     | +00a
        move.l  d0,-(a7)                        | +00e
        lea     NameEntry_InitRow_097cc4(pc),a1 | +010
        jsr     0x4ae.l                         | +014
        move.l  (a7)+,d0                        | +01a
        move.w  d0,0x88(a0)                     | +01c
        movem.l (a7)+,a0-a1                     | +020
        addq.b  #0x1,0x20(a6)                   | +024
.L097b9c:
        move.b  (a0)+,d0                        | +028
        jsr     HiScore_DrawRank_097c02(pc)     | +02a
        adda.l  #0xc0,a1                        | +02e
        move.l  a1,-(a7)                        | +034
        move.l  (a0)+,d0                        | +036
        moveq   #4,d1                           | +038
        jsr     0x5d7be.l                       | +03a
        movea.l (a7)+,a1                        | +040
        adda.l  #0x240,a1                       | +042
        move.w  (a0)+,d0                        | +048
        ori.w   #0x4000,d0                      | +04a
        moveq   #2,d1                           | +04e
        moveq   #2,d2                           | +050
        move.l  a1,-(a7)                        | +052
        jsr     0x5da56.l                       | +054
        movea.l (a7)+,a1                        | +05a
        adda.l  #0x40,a1                        | +05c
        move.w  (a0)+,d0                        | +062
        ori.w   #0x4000,d0                      | +064
        moveq   #2,d1                           | +068
        moveq   #2,d2                           | +06a
        move.l  a1,-(a7)                        | +06c
        jsr     0x5da56.l                       | +06e
        movea.l (a7)+,a1                        | +074
        adda.l  #0x40,a1                        | +076
        move.w  (a0)+,d0                        | +07c
        ori.w   #0x4000,d0                      | +07e
        moveq   #2,d1                           | +082
        moveq   #2,d2                           | +084

| ----------------------------------------------------------------------------
|  HiScore_DrawRank_097c02  @ $097C02  (90 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_DrawRank_097c02, "ax", @progbits
        .global HiScore_DrawRank_097c02
HiScore_DrawRank_097c02:
        move.l  a1,-(a7)                        | +000
        andi.w  #0xff,d0                        | +002
        subq.w  #0x1,d0                         | +006
        bmi.w   .L097c16                        | +008
        cmpi.w  #0xa,d0                         | +00c
        ble.w   .L097c18                        | +010
.L097c16:
        moveq   #0,d0                           | +014
.L097c18:
        move.l  d0,-(a7)                        | +016
        lea     0x2f542a.l,a2                   | +018
        add.w   d0,d0                           | +01e
        move.w  (a2,d0.w),d0                    | +020
        ori.w   #0x4000,d0                      | +024
        moveq   #2,d1                           | +028
        moveq   #2,d2                           | +02a
        move.l  a1,-(a7)                        | +02c
        jsr     0x5da56.l                       | +02e
        movea.l (a7)+,a1                        | +034
        move.l  (a7)+,d0                        | +036
        adda.l  #0x41,a1                        | +038
        lea     0x2f5440.l,a2                   | +03e
        add.w   d0,d0                           | +044
        add.w   d0,d0                           | +046
        movea.l (a2,d0.w),a2                    | +048
        move.w  #0x4300,d0                      | +04c
        jsr     0x5dad8.l                       | +050
        movea.l (a7)+,a1                        | +056
        rts                                     | +058

| ----------------------------------------------------------------------------
|  HiScore_DrawNameChars_097c5c  @ $097C5C  (96 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_DrawNameChars_097c5c, "ax", @progbits
        .global HiScore_DrawNameChars_097c5c
HiScore_DrawNameChars_097c5c:
        tst.w   d0                              | +000
        beq.w   JsrAbsRts_097cc2                | +002
        move.l  d0,-(a7)                        | +006
        move.w  (a0),d0                         | +008
        ori.w   #0x4000,d0                      | +00a
        moveq   #2,d1                           | +00e
        moveq   #2,d2                           | +010
        move.l  a1,-(a7)                        | +012
        jsr     0x5da56.l                       | +014
        movea.l (a7)+,a1                        | +01a
        move.l  (a7)+,d0                        | +01c
        cmpi.w  #0x1,d0                         | +01e
        beq.w   JsrAbsRts_097cc2                | +022
        move.l  d0,-(a7)                        | +026
        adda.l  #0x40,a1                        | +028
        move.w  0x2(a0),d0                      | +02e
        ori.w   #0x4000,d0                      | +032
        moveq   #2,d1                           | +036
        moveq   #2,d2                           | +038
        move.l  a1,-(a7)                        | +03a
        jsr     0x5da56.l                       | +03c
        movea.l (a7)+,a1                        | +042
        move.l  (a7)+,d0                        | +044
        cmpi.w  #0x2,d0                         | +046
        beq.w   JsrAbsRts_097cc2                | +04a
        adda.l  #0x40,a1                        | +04e
        move.w  0x4(a0),d0                      | +054
        ori.w   #0x4000,d0                      | +058
        moveq   #2,d1                           | +05c
        moveq   #2,d2                           | +05e

| ----------------------------------------------------------------------------
|  NameEntry_InitRow_097cc4  @ $097CC4  (180 B)
| ----------------------------------------------------------------------------
        .section .text.NameEntry_InitRow_097cc4, "ax", @progbits
        .global NameEntry_InitRow_097cc4
NameEntry_InitRow_097cc4:
        move.b  #0xff,0x21(a6)                  | +000
        moveq   #0,d0                           | +006
        move.w  d0,0x7a(a6)                     | +008
        move.w  d0,0x7c(a6)                     | +00c
        move.w  d0,0x7e(a6)                     | +010
        move.w  d0,0x72(a6)                     | +014
        move.w  d0,0x70(a6)                     | +018
        move.b  d0,0x8c(a6)                     | +01c
        move.b  d0,0x8d(a6)                     | +020
        move.w  #0xb40,d1                       | +024
        move.w  d1,0x74(a6)                     | +028
        move.w  d1,0x76(a6)                     | +02c
        move.w  d1,0x78(a6)                     | +030
        move.w  0x88(a6),d0                     | +034
        jsr     HiScore_FindPlayerRow_0981ae(pc) | +038
        movea.l #0x70a6,a1                      | +03c
        subq.w  #0x1,d1                         | +042
        mulu.w  #0x2,d1                         | +044
        adda.l  d1,a1                           | +048
        move.l  a1,0x80(a6)                     | +04a
        move.l  #0x300,0x84(a6)                 | +04e
        lea     NameEntry_Run_097d78(pc),a1     | +056
        move.l  a1,(a6)                         | +05a
        bra.w   NameEntry_Run_097d78            | +05c
        .global NameEntry_InitStandalone_097d24
NameEntry_InitStandalone_097d24:
        clr.b   0x21(a6)                        | +060
        moveq   #0,d0                           | +064
        move.w  d0,0x7a(a6)                     | +066
        move.w  d0,0x7c(a6)                     | +06a
        move.w  d0,0x7e(a6)                     | +06e
        move.w  d0,0x72(a6)                     | +072
        move.w  d0,0x70(a6)                     | +076
        move.b  d0,0x8c(a6)                     | +07a
        move.b  d0,0x8d(a6)                     | +07e
        move.l  #0xc0,0x84(a6)                  | +082
        move.w  #0xb40,d1                       | +08a
        move.w  d1,0x74(a6)                     | +08e
        move.w  d1,0x76(a6)                     | +092
        move.w  d1,0x78(a6)                     | +096
        movea.l 0x80(a6),a1                     | +09a
        move.w  #0xb40,d0                       | +09e
        move.w  #0xe,d1                         | +0a2
        moveq   #2,d2                           | +0a6
        jsr     0x5da9c.l                       | +0a8
        lea     NameEntry_Run_097d78(pc),a1     | +0ae
        move.l  a1,(a6)                         | +0b2

| ----------------------------------------------------------------------------
|  NameEntry_Run_097d78  @ $097D78  (722 B)
| ----------------------------------------------------------------------------
        .section .text.NameEntry_Run_097d78, "ax", @progbits
        .global NameEntry_Run_097d78
NameEntry_Run_097d78:
        move.w  0x88(a6),d0                     | +000
        jsr     HiScore_RankOfPlayer_098176(pc) | +004
        movea.l 0x80(a6),a1                     | +008
        move.w  d1,d0                           | +00c
        jsr     HiScore_DrawRank_097c02(pc)     | +00e
        tst.b   0x21(a6)                        | +012
        beq.w   .L097dec                        | +016
        move.w  0x7a(a6),d0                     | +01a
        lsr.w   #0x4,d0                         | +01e
        andi.w  #0x3,d0                         | +020
        cmpi.w  #0x3,d0                         | +024
        beq.w   .L097dd2                        | +028
        move.w  0x88(a6),d0                     | +02c
        jsr     HiScore_FindPlayerRow_0981ae(pc) | +030
        lea     0x100002.l,a1                   | +034
        subq.w  #0x1,d1                         | +03a
        mulu.w  #0xc,d1                         | +03c
        move.l  0x2(a1,d1.w),d0                 | +040
        movea.l 0x80(a6),a1                     | +044
        adda.l  #0xc0,a1                        | +048
        moveq   #4,d1                           | +04e
        jsr     0x5d7be.l                       | +050
        bra.w   .L097dec                        | +056
.L097dd2:
        movea.l 0x80(a6),a1                     | +05a
        adda.l  #0xc0,a1                        | +05e
        move.w  #0xb40,d0                       | +064
        move.w  #0x10,d1                        | +068
        moveq   #2,d2                           | +06c
        jsr     0x5da9c.l                       | +06e
.L097dec:
        movea.l 0x80(a6),a1                     | +074
        adda.l  0x84(a6),a1                     | +078
        movea.l a6,a0                           | +07c
        adda.l  #0x74,a0                        | +07e
        move.w  0x70(a6),d0                     | +084
        jsr     HiScore_DrawNameChars_097c5c(pc) | +088
        movea.l 0x80(a6),a1                     | +08c
        adda.l  0x84(a6),a1                     | +090
        move.w  0x70(a6),d1                     | +094
        mulu.w  #0x40,d1                        | +098
        adda.l  d1,a1                           | +09c
        addq.w  #0x1,0x7e(a6)                   | +09e
        move.w  0x7e(a6),d0                     | +0a2
        btst    #0x4,d0                         | +0a6
        beq.w   .L097e3c                        | +0aa
        move.w  #0xb40,d0                       | +0ae
        ori.w   #0x4000,d0                      | +0b2
        moveq   #2,d1                           | +0b6
        moveq   #2,d2                           | +0b8
        jsr     0x5da56.l                       | +0ba
        bra.w   .L097e5a                        | +0c0
.L097e3c:
        lea     0x2f54b2.l,a0                   | +0c4
        move.w  0x72(a6),d0                     | +0ca
        add.w   d0,d0                           | +0ce
        move.w  (a0,d0.w),d0                    | +0d0
        ori.w   #0x4000,d0                      | +0d4
        moveq   #2,d1                           | +0d8
        moveq   #2,d2                           | +0da
        jsr     0x5da56.l                       | +0dc
.L097e5a:
        move.w  0x88(a6),d1                     | +0e2
        subq.w  #0x1,d1                         | +0e6
        add.w   d1,d1                           | +0e8
        add.w   d1,d1                           | +0ea
        lea     0x2f552c.l,a0                   | +0ec
        movea.l (a0,d1.w),a0                    | +0f2
        move.b  (a0),d0                         | +0f6
        andi.b  #0x3c,d0                        | +0f8
        bne.w   .L097e9e                        | +0fc
        lea     0x2f5534.l,a0                   | +100
        movea.l (a0,d1.w),a0                    | +106
        move.b  (a0),d0                         | +10a
        andi.b  #0xc,d0                         | +10c
        bne.w   .L097e9e                        | +110
        addq.w  #0x1,0x7c(a6)                   | +114
        cmpi.w  #0x12c,0x7c(a6)                 | +118
        bcc.w   .L097f96                        | +11e
        bra.w   .L097f72                        | +122
.L097e9e:
        move.w  #0x0,0x7c(a6)                   | +126
        move.w  #0x0,0x7e(a6)                   | +12c
        move.b  #0x0,0x8d(a6)                   | +132
        move.b  d0,0x8c(a6)                     | +138
        andi.b  #0xc,0x8c(a6)                   | +13c
        btst    #0x2,d0                         | +142
        beq.w   .L097ed0                        | +146
        subq.w  #0x1,0x72(a6)                   | +14a
        bpl.w   .L097ed0                        | +14e
        move.w  #0x28,0x72(a6)                  | +152
.L097ed0:
        btst    #0x3,d0                         | +158
        beq.w   .L097eea                        | +15c
        addq.w  #0x1,0x72(a6)                   | +160
        cmpi.w  #0x29,0x72(a6)                  | +164
        blt.w   .L097eea                        | +16a
        clr.w   0x72(a6)                        | +16e
.L097eea:
        btst    #0x4,d0                         | +172
        beq.w   .L097f2c                        | +176
        lea     0x2f54b2.l,a1                   | +17a
        move.w  0x72(a6),d1                     | +180
        move.w  0x70(a6),d2                     | +184
        add.w   d1,d1                           | +188
        add.w   d2,d2                           | +18a
        move.w  (a1,d1.w),d3                    | +18c
        cmpi.w  #0xce6,d3                       | +190
        bne.w   .L097f1a                        | +194
        move.w  #0xb40,0x74(a6,d2.w)            | +198
        bra.w   .L097fc0                        | +19e
.L097f1a:
        move.w  d3,0x74(a6,d2.w)                | +1a2
        addq.w  #0x1,0x70(a6)                   | +1a6
        cmpi.w  #0x3,0x70(a6)                   | +1aa
        beq.w   .L097fc0                        | +1b0
.L097f2c:
        btst    #0x5,d0                         | +1b4
        beq.w   .L097f72                        | +1b8
        movea.l 0x80(a6),a1                     | +1bc
        adda.l  0x84(a6),a1                     | +1c0
        move.w  0x70(a6),d1                     | +1c4
        mulu.w  #0x40,d1                        | +1c8
        adda.l  d1,a1                           | +1cc
        move.w  #0xb40,d0                       | +1ce
        ori.w   #0x4000,d0                      | +1d2
        moveq   #2,d1                           | +1d6
        moveq   #2,d2                           | +1d8
        jsr     0x5da56.l                       | +1da
        subq.w  #0x1,0x70(a6)                   | +1e0
        bpl.w   .L097f66                        | +1e4
        move.w  #0x0,0x70(a6)                   | +1e8
.L097f66:
        move.w  0x70(a6),d2                     | +1ee
        add.w   d2,d2                           | +1f2
        move.w  #0xb40,0x74(a6,d2.w)            | +1f4
.L097f72:
        addq.w  #0x1,0x7a(a6)                   | +1fa
        cmpi.w  #0x384,0x7a(a6)                 | +1fe
        bcc.w   .L097f96                        | +204
        tst.b   0x21(a6)                        | +208
        bne.w   .L097f94                        | +20c
        movea.l 0xc(a6),a0                      | +210
        tst.b   0x20(a0)                        | +214
        beq.w   .L097f96                        | +218
.L097f94:
        rts                                     | +21c
.L097f96:
        lea     0x2f54b2.l,a1                   | +21e
        move.w  0x72(a6),d1                     | +224
        move.w  0x70(a6),d2                     | +228
        add.w   d1,d1                           | +22c
        add.w   d2,d2                           | +22e
        move.w  (a1,d1.w),d3                    | +230
        cmpi.w  #0xce6,d3                       | +234
        bne.w   .L097fb8                        | +238
        move.w  #0xb40,d3                       | +23c
.L097fb8:
        move.w  d3,0x74(a6,d2.w)                | +240
        bra.w   .L097fc0                        | +244
.L097fc0:
        jsr     NameEntry_CensorName_098144(pc) | +248
        move.w  0x88(a6),d0                     | +24c
        jsr     HiScore_FindPlayerRow_0981ae(pc) | +250
        cmpi.w  #0xb,d1                         | +254
        beq.w   .L098002                        | +258
        lea     0x100002.l,a2                   | +25c
        subq.w  #0x1,d1                         | +262
        mulu.w  #0xc,d1                         | +264
        adda.l  d1,a2                           | +268
        move.w  0x88(a6),d0                     | +26a
        jsr     HiScore_RankOfPlayer_098176(pc) | +26e
        move.b  d1,0x1(a2)                      | +272
        clr.b   (a2)                            | +276
        move.w  0x74(a6),0x6(a2)                | +278
        move.w  0x76(a6),0x8(a2)                | +27e
        move.w  0x78(a6),0xa(a2)                | +284
.L098002:
        move.w  d1,0x8a(a6)                     | +28a
        move.w  d1,d0                           | +28e
        movea.l 0x80(a6),a1                     | +290
        jsr     HiScore_DrawRank_097c02(pc)     | +294
        movea.l 0x80(a6),a1                     | +298
        adda.l  0x84(a6),a1                     | +29c
        movea.l a6,a0                           | +2a0
        adda.l  #0x74,a0                        | +2a2
        jsr     HiScore_DrawNameChars_097c5c(pc) | +2a8
        tst.b   0x21(a6)                        | +2ac
        beq.w   .L098038                        | +2b0
        movea.l 0xc(a6),a0                      | +2b4
        subq.b  #0x1,0x20(a0)                   | +2b8
        bra.w   .L098044                        | +2bc
.L098038:
        movea.l 0xc(a6),a0                      | +2c0
        tst.b   0x20(a0)                        | +2c4
        beq.w   NameEntry_Finish_098052         | +2c8
.L098044:
        move.w  #0x0,0x7a(a6)                   | +2cc

| ----------------------------------------------------------------------------
|  NameEntry_Finish_098052  @ $098052  (24 B)
| ----------------------------------------------------------------------------
        .section .text.NameEntry_Finish_098052, "ax", @progbits
        .global NameEntry_Finish_098052
NameEntry_Finish_098052:
        tst.b   0x21(a6)                        | +000
        bne.w   .L098062                        | +004
        movea.l 0xc(a6),a0                      | +008
        clr.b   0x21(a0)                        | +00c
.L098062:
        jmp     0x518.l                         | +010
        rts                                     | +016

| ----------------------------------------------------------------------------
|  NameEntry_Blink_09806a  @ $09806A  (124 B)
| ----------------------------------------------------------------------------
        .section .text.NameEntry_Blink_09806a, "ax", @progbits
        .global NameEntry_Blink_09806a
NameEntry_Blink_09806a:
        tst.b   0x21(a6)                        | +000
        bne.w   .L09807c                        | +004
        movea.l 0xc(a6),a0                      | +008
        tst.b   0x20(a0)                        | +00c
        beq.b   NameEntry_Finish_098052         | +010
.L09807c:
        addq.w  #0x1,0x7a(a6)                   | +012
        move.w  0x7a(a6),d0                     | +016
        cmpi.w  #0x60,d0                        | +01a
        bge.b   NameEntry_Finish_098052         | +01e
        cmpi.w  #0x48,d0                        | +020
        bge.w   .L09809a                        | +024
        andi.w  #0x8,d0                         | +028
        bne.w   NameEntry_ClearRow_0980ec       | +02c
.L09809a:
        movea.l 0x80(a6),a1                     | +030
        move.w  0x8a(a6),d0                     | +034
        jsr     HiScore_DrawRank_097c02(pc)     | +038
        tst.b   0x21(a6)                        | +03c
        beq.w   .L0980d4                        | +040
        lea     0x100002.l,a1                   | +044
        move.w  0x8a(a6),d1                     | +04a
        subq.w  #0x1,d1                         | +04e
        mulu.w  #0xc,d1                         | +050
        move.l  0x2(a1,d1.w),d0                 | +054
        movea.l 0x80(a6),a1                     | +058
        adda.l  #0xc0,a1                        | +05c
        moveq   #4,d1                           | +062
        jsr     0x5d7be.l                       | +064
.L0980d4:
        movea.l 0x80(a6),a1                     | +06a
        adda.l  0x84(a6),a1                     | +06e
        movea.l a6,a0                           | +072
        adda.l  #0x74,a0                        | +074
        moveq   #3,d0                           | +07a

| ----------------------------------------------------------------------------
|  NameEntry_ClearRow_0980ec  @ $0980EC  (80 B)
| ----------------------------------------------------------------------------
        .section .text.NameEntry_ClearRow_0980ec, "ax", @progbits
        .global NameEntry_ClearRow_0980ec
NameEntry_ClearRow_0980ec:
        movea.l 0x80(a6),a1                     | +000
        move.w  #0xb40,d0                       | +004
        ori.w   #0x4000,d0                      | +008
        moveq   #6,d1                           | +00c
        moveq   #2,d2                           | +00e
        jsr     0x5da9c.l                       | +010
        tst.b   0x21(a6)                        | +016
        beq.w   .L098128                        | +01a
        movea.l 0x80(a6),a1                     | +01e
        adda.l  #0xc0,a1                        | +022
        move.w  #0xb40,d0                       | +028
        ori.w   #0x4000,d0                      | +02c
        move.w  #0x10,d1                        | +030
        moveq   #2,d2                           | +034
        jsr     0x5da9c.l                       | +036
.L098128:
        movea.l 0x80(a6),a1                     | +03c
        adda.l  0x84(a6),a1                     | +040
        move.w  #0xb40,d0                       | +044
        ori.w   #0x4000,d0                      | +048
        moveq   #6,d1                           | +04c
        moveq   #2,d2                           | +04e

| ----------------------------------------------------------------------------
|  NameEntry_CensorName_098144  @ $098144  (50 B)
| ----------------------------------------------------------------------------
        .section .text.NameEntry_CensorName_098144, "ax", @progbits
        .global NameEntry_CensorName_098144
NameEntry_CensorName_098144:
        cmpi.w  #0xba6,0x74(a6)                 | +000
        bne.w   .L098174                        | +006
        cmpi.w  #0xb8a,0x76(a6)                 | +00a
        bne.w   .L098174                        | +010
        cmpi.w  #0xca0,0x78(a6)                 | +014
        bne.w   .L098174                        | +01a
        move.w  #0xc8e,0x74(a6)                 | +01e
        move.w  #0xc80,0x76(a6)                 | +024
        move.w  #0xb42,0x78(a6)                 | +02a
.L098174:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  HiScore_RankOfPlayer_098176  @ $098176  (56 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_RankOfPlayer_098176, "ax", @progbits
        .global HiScore_RankOfPlayer_098176
HiScore_RankOfPlayer_098176:
        lea     0x100002.l,a1                   | +000
        moveq   #1,d1                           | +006
.L09817e:
        cmp.b   (a1),d0                         | +008
        beq.w   .L0981a4                        | +00a
        adda.l  #0xc,a1                         | +00e
        addq.w  #0x1,d1                         | +014
        cmpi.w  #0xb,d1                         | +016
        bne.b   .L09817e                        | +01a
        suba.l  #0xc,a1                         | +01c
        move.b  0x1(a1),d1                      | +022
        addq.b  #0x1,d1                         | +026
        andi.w  #0xff,d1                        | +028
        rts                                     | +02c
.L0981a4:
        move.b  0x1(a1),d1                      | +02e
        andi.w  #0xff,d1                        | +032
        rts                                     | +036

| ----------------------------------------------------------------------------
|  HiScore_FindPlayerRow_0981ae  @ $0981AE  (30 B)
| ----------------------------------------------------------------------------
        .section .text.HiScore_FindPlayerRow_0981ae, "ax", @progbits
        .global HiScore_FindPlayerRow_0981ae
HiScore_FindPlayerRow_0981ae:
        lea     0x100002.l,a1                   | +000
        moveq   #1,d1                           | +006
.L0981b6:
        cmp.b   (a1),d0                         | +008
        beq.w   .L0981ca                        | +00a
        adda.l  #0xc,a1                         | +00e
        addq.w  #0x1,d1                         | +014
        cmpi.w  #0xb,d1                         | +016
        bne.b   .L0981b6                        | +01a
.L0981ca:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_0981cc  @ $0981CC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_0981cc, "ax", @progbits
        .global Entity_CmpPrioWithSibling_0981cc
Entity_CmpPrioWithSibling_0981cc:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0981e2                    | +00c

| ----------------------------------------------------------------------------
|  MemCard_FileName_0981e8  @ $0981E8  (20 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_FileName_0981e8, "ax", @progbits
        .global MemCard_FileName_0981e8
MemCard_FileName_0981e8:
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

| ----------------------------------------------------------------------------
|  Fix_DrawGlyphList_09820e  @ $09820E  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_DrawGlyphList_09820e, "ax", @progbits
        .global Fix_DrawGlyphList_09820e
Fix_DrawGlyphList_09820e:
        move.w  (a0)+,d0                        | +000
        cmpi.w  #0xffff,d0                      | +002
        beq.w   .L098230                        | +006
        move.w  #0x2,d1                         | +00a
        move.w  #0x2,d2                         | +00e
        move.l  a1,-(a7)                        | +012
        jsr     0x5da56.l                       | +014
        movea.l (a7)+,a1                        | +01a
        adda.w  #0x40,a1                        | +01c
        bra.b   Fix_DrawGlyphList_09820e        | +020
.L098230:
        rts                                     | +022

| ----------------------------------------------------------------------------
|  Input_EdgePressedPlayers_098232  @ $098232  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Input_EdgePressedPlayers_098232, "ax", @progbits
        .global Input_EdgePressedPlayers_098232
Input_EdgePressedPlayers_098232:
        moveq   #0,d0                           | +000
        move.b  0x10e3a0.l,d7                   | +002
        subq.b  #0x1,d7                         | +008
        cmpi.b  #0x2,d7                         | +00a
        bcc.w   .L09824a                        | +00e
        or.b    0x10fd96.l,d0                   | +012
.L09824a:
        move.b  0x10e3a1.l,d7                   | +018
        subq.b  #0x1,d7                         | +01e
        cmpi.b  #0x2,d7                         | +020
        bcc.w   .L098260                        | +024
        or.b    0x10fd9c.l,d0                   | +028
.L098260:
        move.b  0x74(a6),d1                     | +02e
        move.b  d0,0x74(a6)                     | +032
        eor.b   d0,d1                           | +036
        and.b   d1,d0                           | +038
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  MemCard_Detect_09826e  @ $09826E  (14 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_Detect_09826e, "ax", @progbits
        .global MemCard_Detect_09826e
MemCard_Detect_09826e:
        move.b  0x380000.l,d0                   | +000
        andi.b  #0x30,d0                        | +006
        beq.w   SetC_098282                     | +00a

| ----------------------------------------------------------------------------
|  MemCard_LoadDialog_Init_098288  @ $098288  (120 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_LoadDialog_Init_098288, "ax", @progbits
        .global MemCard_LoadDialog_Init_098288
MemCard_LoadDialog_Init_098288:
        jsr     MemCard_BiosRead_098578(pc)     | +000
        bcs.w   MemCard_Dialog_ClearAndExit_0983b0 | +004
        movea.l 0x98(a6),a0                     | +008
        cmpa.l  #0x0,a0                         | +00c
        beq.w   .L0982a0                        | +012
        jsr     (a0)                            | +016
.L0982a0:
        movea.l #0x7000,a1                      | +018
        move.w  #0xff,d0                        | +01e
        move.w  #0x26,d1                        | +022
        move.w  #0x1b,d2                        | +026
        jsr     0x5da9c.l                       | +02a
        movea.l #0x7204,a1                      | +030
        lea     MemCard_Str_LoadTitle_098692(pc),a0 | +036
        bsr.w   Fix_DrawGlyphList_09820e        | +03a
        movea.l #0x724a,a1                      | +03e
        lea     MemCard_Str_Opt1_09869e(pc),a0 | +044
        bsr.w   Fix_DrawGlyphList_09820e        | +048
        movea.l #0x724e,a1                      | +04c
        lea     MemCard_Str_Opt2_0986a6(pc),a0 | +052
        bsr.w   Fix_DrawGlyphList_09820e        | +056
        movea.l #0x7116,a1                      | +05a
        lea     MemCard_Str_Prompt_0986ac(pc),a0 | +060
        bsr.w   Fix_DrawGlyphList_09820e        | +064
        move.w  #0x0,0x72(a6)                   | +068
        jsr     Input_EdgePressedPlayers_098232(pc) | +06e
        move.w  #0x12c,0x70(a6)                 | +072

| ----------------------------------------------------------------------------
|  MemCard_LoadDialog_Run_098308  @ $098308  (230 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_LoadDialog_Run_098308, "ax", @progbits
        .global MemCard_LoadDialog_Run_098308
MemCard_LoadDialog_Run_098308:
        lea     MemCard_CursorGlyphs_098700(pc),a0 | +000
        move.w  0x72(a6),d0                     | +004
        andi.w  #0x1,d0                         | +008
        add.w   d0,d0                           | +00c
        move.w  (a0,d0.w),d0                    | +00e
        movea.l #0x720a,a1                      | +012
        move.w  #0x2,d1                         | +018
        move.w  #0x2,d2                         | +01c
        jsr     0x5da56.l                       | +020
        lea     MemCard_CursorGlyphs_098700(pc),a0 | +026
        move.w  0x72(a6),d0                     | +02a
        addq.w  #0x1,d0                         | +02e
        andi.w  #0x1,d0                         | +030
        add.w   d0,d0                           | +034
        move.w  (a0,d0.w),d0                    | +036
        movea.l #0x720e,a1                      | +03a
        move.w  #0x2,d1                         | +040
        move.w  #0x2,d2                         | +044
        jsr     0x5da56.l                       | +048
        subq.w  #0x1,0x70(a6)                   | +04e
        beq.w   .L098388                        | +052
        bsr.w   Input_EdgePressedPlayers_098232 | +056
        btst    #0x4,d0                         | +05a
        bne.w   .L098388                        | +05e
        btst    #0x0,d0                         | +062
        beq.w   .L098378                        | +066
        move.w  #0x0,0x72(a6)                   | +06a
.L098378:
        btst    #0x1,d0                         | +070
        beq.w   .L098386                        | +074
        move.w  #0x1,0x72(a6)                   | +078
.L098386:
        rts                                     | +07e
.L098388:
        move.w  #0xff,d0                        | +080
        movea.l #0x7116,a1                      | +084
        move.w  #0x1a,d1                        | +08a
        move.w  #0x2,d2                         | +08e
        jsr     0x5da9c.l                       | +092
        tst.w   0x72(a6)                        | +098
        bne.w   MemCard_Dialog_ClearAndExit_0983b0 | +09c
        jsr     MemCard_BiosRead_098578(pc)     | +0a0
        bcc.w   .L0983da                        | +0a4
        .global MemCard_Dialog_ClearAndExit_0983b0
MemCard_Dialog_ClearAndExit_0983b0:
        lea     0x10e3a2.l,a0                   | +0a8
        clr.b   0x14(a0)                        | +0ae
        clr.b   0x15(a0)                        | +0b2
        clr.b   0x16(a0)                        | +0b6
        clr.b   0x17(a0)                        | +0ba
        clr.b   0x18(a0)                        | +0be
        clr.b   0x19(a0)                        | +0c2
        clr.b   0x1a(a0)                        | +0c6
        clr.b   0x1b(a0)                        | +0ca
        bra.w   MemCard_Dialog_Exit_0983fe | +0ce
.L0983da:
        movea.l #0x7152,a1                      | +0d2
        lea     MemCard_Str_LoadDone_0986c8(pc),a0 | +0d8
        bsr.w   Fix_DrawGlyphList_09820e        | +0dc
        move.w  #0x1e,0x70(a6)                  | +0e0

| ----------------------------------------------------------------------------
|  MemCard_Dialog_ExitDelay_0983f6  @ $0983F6  (22 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_Dialog_ExitDelay_0983f6, "ax", @progbits
        .global MemCard_Dialog_ExitDelay_0983f6
MemCard_Dialog_ExitDelay_0983f6:
        subq.w  #0x1,0x70(a6)                   | +000
        bne.w   .L09840a                        | +004
        .global MemCard_Dialog_Exit_0983fe
MemCard_Dialog_Exit_0983fe:
        clr.b   0x106ed2.l                      | +008
        jmp     0x518.l                         | +00e
.L09840a:
        rts                                     | +014

| ----------------------------------------------------------------------------
|  MemCard_SaveDialog_Init_09840c  @ $09840C  (110 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_SaveDialog_Init_09840c, "ax", @progbits
        .global MemCard_SaveDialog_Init_09840c
MemCard_SaveDialog_Init_09840c:
        move.b  0x380000.l,d0                   | +000
        andi.b  #0x30,d0                        | +006
        beq.w   .L09841e                        | +00a
        bra.w   MemCard_SaveDialog_Exit_09856a | +00e
.L09841e:
        movea.l #0x7000,a1                      | +012
        move.w  #0xff,d0                        | +018
        move.w  #0x26,d1                        | +01c
        move.w  #0x1b,d2                        | +020
        jsr     0x5da9c.l                       | +024
        movea.l #0x7204,a1                      | +02a
        lea     MemCard_Str_SaveTitle_0986de(pc),a0 | +030
        bsr.w   Fix_DrawGlyphList_09820e        | +034
        movea.l #0x724a,a1                      | +038
        lea     MemCard_Str_Opt1_09869e(pc),a0 | +03e
        bsr.w   Fix_DrawGlyphList_09820e        | +042
        movea.l #0x724e,a1                      | +046
        lea     MemCard_Str_Opt2_0986a6(pc),a0 | +04c
        bsr.w   Fix_DrawGlyphList_09820e        | +050
        movea.l #0x7116,a1                      | +054
        lea     MemCard_Str_Prompt_0986ac(pc),a0 | +05a
        bsr.w   Fix_DrawGlyphList_09820e        | +05e
        move.w  #0x1,0x72(a6)                   | +062
        move.w  #0x12c,0x70(a6)                 | +068

| ----------------------------------------------------------------------------
|  MemCard_SaveDialog_Run_098482  @ $098482  (246 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_SaveDialog_Run_098482, "ax", @progbits
        .global MemCard_SaveDialog_Run_098482
MemCard_SaveDialog_Run_098482:
        lea     MemCard_CursorGlyphs_098700(pc),a0 | +000
        move.w  0x72(a6),d0                     | +004
        add.w   d0,d0                           | +008
        move.w  (a0,d0.w),d0                    | +00a
        movea.l #0x720a,a1                      | +00e
        move.w  #0x2,d1                         | +014
        move.w  #0x2,d2                         | +018
        jsr     0x5da56.l                       | +01c
        lea     MemCard_CursorGlyphs_098700(pc),a0 | +022
        move.w  0x72(a6),d0                     | +026
        eori.w  #0x1,d0                         | +02a
        add.w   d0,d0                           | +02e
        move.w  (a0,d0.w),d0                    | +030
        movea.l #0x720e,a1                      | +034
        move.w  #0x2,d1                         | +03a
        move.w  #0x2,d2                         | +03e
        jsr     0x5da56.l                       | +042
        subq.w  #0x1,0x70(a6)                   | +048
        beq.w   .L0984fc                        | +04c
        bsr.w   Input_EdgePressedPlayers_098232 | +050
        btst    #0x4,d0                         | +054
        bne.w   .L0984fc                        | +058
        btst    #0x0,d0                         | +05c
        beq.w   .L0984ec                        | +060
        move.w  #0x0,0x72(a6)                   | +064
.L0984ec:
        btst    #0x1,d0                         | +06a
        beq.w   .L0984fa                        | +06e
        move.w  #0x1,0x72(a6)                   | +072
.L0984fa:
        rts                                     | +078
.L0984fc:
        move.w  #0xff,d0                        | +07a
        movea.l #0x7116,a1                      | +07e
        move.w  #0x1a,d1                        | +084
        move.w  #0x2,d2                         | +088
        jsr     0x5da9c.l                       | +08c
        tst.w   0x72(a6)                        | +092
        bne.w   MemCard_SaveDialog_Exit_09856a | +096
        move.b  #0x0,0x3a0005.l                 | +09a
        move.b  #0x0,0x3a0017.l                 | +0a2
        move.b  #0x0,0x3a0019.l                 | +0aa
        move.b  #0x0,0x380011.l                 | +0b2
        jsr     MemCard_BiosCreate_09866a(pc)   | +0ba
        jsr     MemCard_BiosWrite_0985ca(pc)    | +0be
        bcs.w   MemCard_SaveDialog_Exit_09856a | +0c2
        movea.l #0x7152,a1                      | +0c6
        lea     MemCard_Str_SaveDone_0986ea(pc),a0 | +0cc
        bsr.w   Fix_DrawGlyphList_09820e        | +0d0
        move.w  #0x1e,0x70(a6)                  | +0d4
        lea     .L098562(pc),a1                 | +0da
        move.l  a1,(a6)                         | +0de
.L098562:
        subq.w  #0x1,0x70(a6)                   | +0e0
        bne.w   .L098576                        | +0e4
        .global MemCard_SaveDialog_Exit_09856a
MemCard_SaveDialog_Exit_09856a:
        clr.b   0x106ed2.l                      | +0e8
        jmp     0x518.l                         | +0ee
.L098576:
        rts                                     | +0f4

| ----------------------------------------------------------------------------
|  MemCard_BiosRead_098578  @ $098578  (66 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_BiosRead_098578, "ax", @progbits
        .global MemCard_BiosRead_098578
MemCard_BiosRead_098578:
        movem.l d0-d7/a0-a6,-(a7)               | +000
        move.b  #0x2,0x10fdc4.l                 | +004
        move.w  #0x201,0x10fdce.l               | +00c
        move.b  #0x0,0x10fdd0.l                 | +014
        move.l  #0x10e3a2,0x10fdc8.l            | +01c
        move.w  #0x1c,0x10fdcc.l                | +026
        jsr     0xc00468.l                      | +02e
        tst.b   0x10fdc6.l                      | +034
        beq.w   MemCard_BiosRead_Ok_0985c0      | +03a
        movem.l (a7)+,d0-d7/a0-a6               | +03e

| ----------------------------------------------------------------------------
|  MemCard_BiosRead_Ok_0985c0  @ $0985C0  (4 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_BiosRead_Ok_0985c0, "ax", @progbits
        .global MemCard_BiosRead_Ok_0985c0
MemCard_BiosRead_Ok_0985c0:
        movem.l (a7)+,d0-d7/a0-a6               | +000

| ----------------------------------------------------------------------------
|  MemCard_BiosWrite_0985ca  @ $0985CA  (144 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_BiosWrite_0985ca, "ax", @progbits
        .global MemCard_BiosWrite_0985ca
MemCard_BiosWrite_0985ca:
        movem.l d0-d7/a0-a6,-(a7)               | +000
        move.b  #0x3,0x10fdc4.l                 | +004
        move.w  #0x201,0x10fdce.l               | +00c
        move.b  #0x0,0x10fdd0.l                 | +014
        lea     0x10e3a2.l,a0                   | +01c
        lea     MemCard_FileName_0981e8(pc),a1  | +022
        move.l  (a1)+,(a0)+                     | +026
        move.l  (a1)+,(a0)+                     | +028
        move.l  (a1)+,(a0)+                     | +02a
        move.l  (a1)+,(a0)+                     | +02c
        move.l  (a1)+,(a0)+                     | +02e
        move.l  #0x10e3a2,0x10fdc8.l            | +030
        move.w  #0x1c,0x10fdcc.l                | +03a
        jsr     0xc00468.l                      | +042
        tst.b   0x10fdc6.l                      | +048
        beq.w   MemCard_BiosWrite_Ok_098660     | +04e
        movea.l #0x7000,a1                      | +052
        move.w  #0xff,d0                        | +058
        move.w  #0x26,d1                        | +05c
        move.w  #0x1b,d2                        | +060
        jsr     0x5da9c.l                       | +064
        move.b  0x10fdc2.l,-(a7)                | +06a
        clr.b   0x10fdc2.l                      | +070
        jsr     0xc0046e.l                      | +076
        move.b  (a7)+,0x10fdc2.l                | +07c
        tst.b   0x10fdc6.l                      | +082
        beq.w   MemCard_BiosWrite_Ok_098660     | +088
        movem.l (a7)+,d0-d7/a0-a6               | +08c

| ----------------------------------------------------------------------------
|  MemCard_BiosWrite_Ok_098660  @ $098660  (4 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_BiosWrite_Ok_098660, "ax", @progbits
        .global MemCard_BiosWrite_Ok_098660
MemCard_BiosWrite_Ok_098660:
        movem.l (a7)+,d0-d7/a0-a6               | +000

| ----------------------------------------------------------------------------
|  MemCard_BiosCreate_09866a  @ $09866A  (40 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_BiosCreate_09866a, "ax", @progbits
        .global MemCard_BiosCreate_09866a
MemCard_BiosCreate_09866a:
        movem.l d0-d7/a0-a6,-(a7)               | +000
        move.b  #0x4,0x10fdc4.l                 | +004
        move.w  #0x201,0x10fdce.l               | +00c
        move.b  #0x0,0x10fdd0.l                 | +014
        jsr     0xc00468.l                      | +01c
        movem.l (a7)+,d0-d7/a0-a6               | +022
        rts                                     | +026

| ----------------------------------------------------------------------------
|  MemCard_Str_LoadTitle_098692  @ $098692  (114 B)
| ----------------------------------------------------------------------------
        .section .text.MemCard_Str_LoadTitle_098692, "ax", @progbits
        .global MemCard_Str_LoadTitle_098692
MemCard_Str_LoadTitle_098692:
        .dc.w   0x4c88                        | +000  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +004  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +006  (dato / opcode no decodificado)
        .dc.w   0x4c6e                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00a  (dato / opcode no decodificado)
        .global MemCard_Str_Opt1_09869e
MemCard_Str_Opt1_09869e:
        .dc.w   0x4ca2                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .global MemCard_Str_Opt2_0986a6
MemCard_Str_Opt2_0986a6:
        .dc.w   0x4c8c                        | +014  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .global MemCard_Str_Prompt_0986ac
MemCard_Str_Prompt_0986ac:
        .dc.w   0x4ba0                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x4baa                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x4c80                        | +020  (dato / opcode no decodificado)
        .dc.w   0x4b40                        | +022  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +024  (dato / opcode no decodificado)
        .dc.w   0x4b40                        | +026  (dato / opcode no decodificado)
        .dc.w   0x4b84                        | +028  (dato / opcode no decodificado)
        .dc.w   0x4baa                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x4ba8                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +030  (dato / opcode no decodificado)
        .dc.w   0x4c8c                        | +032  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +034  (dato / opcode no decodificado)
        .global MemCard_Str_LoadDone_0986c8
MemCard_Str_LoadDone_0986c8:
        .dc.w   0x4c68                        | +036  (dato / opcode no decodificado)
        .dc.w   0x4c88                        | +038  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x4b88                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x4b40                        | +040  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +042  (dato / opcode no decodificado)
        .dc.w   0x4c86                        | +044  (dato / opcode no decodificado)
        .dc.w   0x4b42                        | +046  (dato / opcode no decodificado)
        .dc.w   0x4c6c                        | +048  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04a  (dato / opcode no decodificado)
        .global MemCard_Str_SaveTitle_0986de
MemCard_Str_SaveTitle_0986de:
        .dc.w   0x4ba6                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x4bac                        | +050  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +052  (dato / opcode no decodificado)
        .dc.w   0x4c6e                        | +054  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +056  (dato / opcode no decodificado)
        .global MemCard_Str_SaveDone_0986ea
MemCard_Str_SaveDone_0986ea:
        .dc.w   0x4c68                        | +058  (dato / opcode no decodificado)
        .dc.w   0x4ba6                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x4b82                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x4bac                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x4b8a                        | +060  (dato / opcode no decodificado)
        .dc.w   0x4b40                        | +062  (dato / opcode no decodificado)
        .dc.w   0x4c8e                        | +064  (dato / opcode no decodificado)
        .dc.w   0x4c86                        | +066  (dato / opcode no decodificado)
        .dc.w   0x4b42                        | +068  (dato / opcode no decodificado)
        .dc.w   0x4c6c                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .global MemCard_CursorGlyphs_098700
MemCard_CursorGlyphs_098700:
        .dc.w   0x4b22                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x4b40                        | +070  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_098704  @ $098704  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_098704, "ax", @progbits
        .global Entity_CmpPrioWithSibling_098704
Entity_CmpPrioWithSibling_098704:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_09871a                    | +00c

| ----------------------------------------------------------------------------
|  LogoScene_Tpl_098720  @ $098720  (272 B)
| ----------------------------------------------------------------------------
        .section .text.LogoScene_Tpl_098720, "ax", @progbits
        .global LogoScene_Tpl_098720
LogoScene_Tpl_098720:
        move.w  #0x2c,d0                        | +000
        jsr     0x2352.l                        | +004
        move.w  #0xd,d0                         | +00a
        jsr     0x43568.l                       | +00e
        moveq   #0,d0                           | +014
        moveq   #0,d1                           | +016
        jsr     0x437da.l                       | +018
        move.w  #0xc8,0x82(a6)                  | +01e
        lea     LogoScene_Piece_Init_098838(pc),a1 | +024
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        move.w  #0x0,0x80(a0)                   | +034
        lea     LogoScene_Piece_Init_098838(pc),a1 | +03a
        jsr     0x4ae.l                         | +03e
        jsr     0x5dd02.l                       | +044
        move.w  #0x1,0x80(a0)                   | +04a
        lea     LogoScene_Piece_Init_098838(pc),a1 | +050
        jsr     0x4ae.l                         | +054
        jsr     0x5dd02.l                       | +05a
        move.w  #0x2,0x80(a0)                   | +060
        lea     LogoScene_Piece_Init_098838(pc),a1 | +066
        jsr     0x4ae.l                         | +06a
        jsr     0x5dd02.l                       | +070
        move.w  #0x3,0x80(a0)                   | +076
        lea     LogoScene_Piece_Init_098838(pc),a1 | +07c
        jsr     0x4ae.l                         | +080
        jsr     0x5dd02.l                       | +086
        move.w  #0x4,0x80(a0)                   | +08c
        lea     LogoScene_Piece_Init_098838(pc),a1 | +092
        jsr     0x4ae.l                         | +096
        jsr     0x5dd02.l                       | +09c
        move.w  #0x5,0x80(a0)                   | +0a2
        lea     LogoScene_Piece_Init_098838(pc),a1 | +0a8
        jsr     0x4ae.l                         | +0ac
        jsr     0x5dd02.l                       | +0b2
        move.w  #0x6,0x80(a0)                   | +0b8
        lea     LogoScene_Piece_Init_098838(pc),a1 | +0be
        jsr     0x4ae.l                         | +0c2
        jsr     0x5dd02.l                       | +0c8
        move.w  #0x7,0x80(a0)                   | +0ce
        lea     LogoScene_Piece_Init_098838(pc),a1 | +0d4
        jsr     0x4ae.l                         | +0d8
        jsr     0x5dd02.l                       | +0de
        move.w  #0x8,0x80(a0)                   | +0e4
        lea     LogoScene_Center_Init_0988b2(pc),a1 | +0ea
        jsr     0x4ae.l                         | +0ee
        jsr     0x5dd02.l                       | +0f4
        lea     .L098820(pc),a1                 | +0fa
        move.l  a1,(a6)                         | +0fe
.L098820:
        subq.w  #0x1,0x82(a6)                   | +100
        beq.w   .L09882a                        | +104
        rts                                     | +108
.L09882a:
        clr.b   0x106ed2.l                      | +10a

| ----------------------------------------------------------------------------
|  LogoScene_Piece_Init_098838  @ $098838  (70 B)
| ----------------------------------------------------------------------------
        .section .text.LogoScene_Piece_Init_098838, "ax", @progbits
        .global LogoScene_Piece_Init_098838
LogoScene_Piece_Init_098838:
        move.w  #0x11d,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  0x14(a6),d1                     | +00a
        jsr     0x2c66.l                        | +00e
        lea     0x2f553c.l,a0                   | +014
        move.w  0x80(a6),d0                     | +01a
        add.w   d0,d0                           | +01e
        add.w   d0,d0                           | +020
        move.w  (a0,d0.w),0x22(a6)              | +022
        move.w  #0x180,0x24(a6)                 | +028
        move.w  0x2(a0,d0.w),0x84(a6)           | +02e
        move.w  #0x0,0x82(a6)                   | +034
        lea     0x2f5576.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040

| ----------------------------------------------------------------------------
|  LogoScene_Piece_Run_098886  @ $098886  (44 B)
| ----------------------------------------------------------------------------
        .section .text.LogoScene_Piece_Run_098886, "ax", @progbits
        .global LogoScene_Piece_Run_098886
LogoScene_Piece_Run_098886:
        tst.w   0x84(a6)                        | +000
        beq.w   .L098896                        | +004
        subq.w  #0x1,0x84(a6)                   | +008
        bne.w   .L09889c                        | +00c
.L098896:
        jsr     0x28d70.l                       | +010
.L09889c:
        addq.w  #0x1,0x82(a6)                   | +016
        cmpi.w  #0xc,0x82(a6)                   | +01a
        blt.w   .L0988b0                        | +020
        jmp     0x518.l                         | +024
.L0988b0:
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  LogoScene_Center_Init_0988b2  @ $0988B2  (82 B)
| ----------------------------------------------------------------------------
        .section .text.LogoScene_Center_Init_0988b2, "ax", @progbits
        .global LogoScene_Center_Init_0988b2
LogoScene_Center_Init_0988b2:
        move.w  #0x11e,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  0x14(a6),d1                     | +00a
        jsr     0x2c66.l                        | +00e
        move.w  #0xa0,0x22(a6)                  | +014
        move.w  #0x180,0x24(a6)                 | +01a
        moveq   #0,d0                           | +020
        move.b  d0,0x20(a6)                     | +022
        move.b  d0,0x21(a6)                     | +026
        move.b  #0xff,0x32(a6)                  | +02a
        move.b  #0x2a,0x33(a6)                  | +030
        move.w  d0,0x86(a6)                     | +036
        move.w  #0xd,0x84(a6)                   | +03a
        move.w  #0x0,0x82(a6)                   | +040
        lea     0x2f5582.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c

| ----------------------------------------------------------------------------
|  LogoScene_Center_Run_09890c  @ $09890C  (118 B)
| ----------------------------------------------------------------------------
        .section .text.LogoScene_Center_Run_09890c, "ax", @progbits
        .global LogoScene_Center_Run_09890c
LogoScene_Center_Run_09890c:
        tst.w   0x84(a6)                        | +000
        beq.w   .L09892e                        | +004
        subq.w  #0x1,0x84(a6)                   | +008
        bne.w   JsrAbsRts_098988                | +00c
        move.w  0x14(a6),d1                     | +010
        move.w  #0x11f,d2                       | +014
        moveq   #1,d3                           | +018
        moveq   #1,d4                           | +01a
        jsr     0x2c30.l                        | +01c
.L09892e:
        jsr     0x28d70.l                       | +022
        cmpi.b  #0xff,0x33(a6)                  | +028
        bne.w   LogoScene_Center_PalStep_09898a | +02e
        addq.w  #0x1,0x82(a6)                   | +032
        cmpi.w  #0x10,0x82(a6)                  | +036
        blt.w   JsrAbsRts_098988                | +03c
        tst.b   0x20(a6)                        | +040
        bne.w   JsrAbsRts_098988                | +044
        move.b  #0xff,0x20(a6)                  | +048
        move.w  0x14(a6),d1                     | +04e
        move.w  #0x120,d2                       | +052
        moveq   #1,d3                           | +056
        moveq   #1,d4                           | +058
        jsr     0x2c30.l                        | +05a
        move.w  0x14(a6),d1                     | +060
        jsr     0x2c66.l                        | +064
        move.w  #0xff,d1                        | +06a
        move.w  #0x120,d2                       | +06e
        moveq   #1,d3                           | +072
        moveq   #1,d4                           | +074

| ----------------------------------------------------------------------------
|  LogoScene_Center_PalStep_09898a  @ $09898A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.LogoScene_Center_PalStep_09898a, "ax", @progbits
        .global LogoScene_Center_PalStep_09898a
LogoScene_Center_PalStep_09898a:
        lea     0x2f5560.l,a0                   | +000
        move.w  0x86(a6),d0                     | +006
        move.b  (a0,d0.w),0x33(a6)              | +00a
        addq.w  #0x1,0x86(a6)                   | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_0989a0  @ $0989A0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_0989a0, "ax", @progbits
        .global Entity_CmpPrioWithSibling_0989a0
Entity_CmpPrioWithSibling_0989a0:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0989b6                    | +00c

| ----------------------------------------------------------------------------
|  Mob_Tmpl174_Init_0989bc  @ $0989BC  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl174_Init_0989bc, "ax", @progbits
        .global Mob_Tmpl174_Init_0989bc
Mob_Tmpl174_Init_0989bc:
        move.w  #0x125,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x0,0x80(a6)                   | +00a
        jsr     0x8f002.l                       | +010
        bclr    #0x1,0x12(a6)                   | +016

| ----------------------------------------------------------------------------
|  Mob_Walk_0989e0  @ $0989E0  (160 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Walk_0989e0, "ax", @progbits
        .global Mob_Walk_0989e0
Mob_Walk_0989e0:
        move.b  #0x0,0x76(a6)                   | +000
        jsr     0x267e2.l                       | +006
        jsr     0x5e7c0.l                       | +00c
        move.w  #0x8000,d0                      | +012
        jsr     0x28134.l                       | +016
        andi.w  #0xffe3,0x38(a6)                | +01c
        ori.w   #0x4,0x38(a6)                   | +022
        .global Mob_Walk_Restart_098a08
Mob_Walk_Restart_098a08:
        move.w  #0x1,0x66(a6)                   | +028
        jsr     0x8f010.l                       | +02e
        bclr    #0x3,0x13(a6)                   | +034
        lea     0x2f55be.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        lea     .L098a2c(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L098a2c:
        jsr     0x27a92.l                       | +04c
        jsr     0x27eba.l                       | +052
        bcc.w   .L098a42                        | +058
        lea     Mob_Flee_098b66(pc),a1          | +05c
        move.l  a1,(a6)                         | +060
.L098a42:
        jsr     0x28d70.l                       | +062
        tst.b   0x76(a6)                        | +068
        beq.w   .L098a5a                        | +06c
        bclr    #0x3,0x13(a6)                   | +070
        bra.w   .L098a6a                        | +076
.L098a5a:
        jsr     0x2870a.l                       | +07a
        bcc.w   .L098a6a                        | +080
        lea     Mob_Hit_098a88(pc),a1           | +084
        move.l  a1,(a6)                         | +088
.L098a6a:
        movea.l #0xffffffff,a0                  | +08a
        lea     0x2f61ac.l,a0                   | +090
        jsr     0x5dd56.l                       | +096
        bcc.w   SetHandlerRts_098a86            | +09c

| ----------------------------------------------------------------------------
|  Mob_Hit_098a88  @ $098A88  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Hit_098a88, "ax", @progbits
        .global Mob_Hit_098a88
Mob_Hit_098a88:
        move.b  #0xff,0x76(a6)                  | +000
        lea     0x2f5642.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L098aa0(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L098aa0:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L098ab6                        | +024
        lea     Mob_Walk_Restart_098a08(pc),a1 | +028
        move.l  a1,(a6)                         | +02c
.L098ab6:
        movea.l #0xffffffff,a0                  | +02e
        lea     0x2f61ac.l,a0                   | +034
        jsr     0x5dd56.l                       | +03a
        bcc.w   SetHandlerRts_098ad2            | +040

| ----------------------------------------------------------------------------
|  Mob_Tmpl175_Init_098ad4  @ $098AD4  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl175_Init_098ad4, "ax", @progbits
        .global Mob_Tmpl175_Init_098ad4
Mob_Tmpl175_Init_098ad4:
        move.w  #0x125,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        jsr     0x8f002.l                       | +010
        move.w  #0x0,0x80(a6)                   | +016
        bclr    #0x1,0x12(a6)                   | +01c

| ----------------------------------------------------------------------------
|  Mob_Tmpl175_Idle_098afe  @ $098AFE  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl175_Idle_098afe, "ax", @progbits
        .global Mob_Tmpl175_Idle_098afe
Mob_Tmpl175_Idle_098afe:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x8000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x4,0x38(a6)                   | +016
        move.w  #0x1,0x66(a6)                   | +01c
        lea     0x2f5780.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        lea     .L098b32(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L098b32:
        jsr     0x2783a.l                       | +034
        jsr     0x28d70.l                       | +03a
        bcc.w   .L098b48                        | +040
        lea     Mob_Walk_0989e0(pc),a1          | +044
        move.l  a1,(a6)                         | +048
.L098b48:
        movea.l #0xffffffff,a0                  | +04a
        lea     0x2f61ac.l,a0                   | +050
        jsr     0x5dd56.l                       | +056
        bcc.w   SetHandlerRts_098b64            | +05c

| ----------------------------------------------------------------------------
|  Mob_Flee_098b66  @ $098B66  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Flee_098b66, "ax", @progbits
        .global Mob_Flee_098b66
Mob_Flee_098b66:
        jsr     0x267e2.l                       | +000
        move.w  #0xffe0,0x2a(a6)                | +006
        lea     .L098b78(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L098b78:
        jsr     0x27c8c.l                       | +012
        jsr     0x27eba.l                       | +018
        bcs.w   .L098b8e                        | +01e
        lea     Mob_DispatchBy80_098bb2(pc),a1  | +022
        move.l  a1,(a6)                         | +026
.L098b8e:
        jsr     0x28d70.l                       | +028
        movea.l #0xffffffff,a0                  | +02e
        lea     0x2f61ac.l,a0                   | +034
        jsr     0x5dd56.l                       | +03a
        bcc.w   SetHandlerRts_098bb0            | +040

| ----------------------------------------------------------------------------
|  Mob_DispatchBy80_098bb2  @ $098BB2  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_DispatchBy80_098bb2, "ax", @progbits
        .global Mob_DispatchBy80_098bb2
Mob_DispatchBy80_098bb2:
        move.w  0x80(a6),d0                     | +000
        lea     0x2f55a6.l,a0                   | +004
        add.w   d0,d0                           | +00a
        add.w   d0,d0                           | +00c
        movea.l (a0,d0.w),a0                    | +00e
        jmp     (a0)                            | +012

| ----------------------------------------------------------------------------
|  Mob_Tmpl176_Init_098bc6  @ $098BC6  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl176_Init_098bc6, "ax", @progbits
        .global Mob_Tmpl176_Init_098bc6
Mob_Tmpl176_Init_098bc6:
        move.w  #0x0,0x70(a6)                   | +000
        bra.w   .L098bdc                        | +006
        move.w  #0x1,0x70(a6)                   | +00a
        move.w  #0x10,0x72(a6)                  | +010
.L098bdc:
        move.w  #0x124,d1                       | +016
        jsr     0x236e.l                        | +01a
        move.w  #0x1,0x80(a6)                   | +020
        move.b  #0x0,0x76(a6)                   | +026
        bclr    #0x1,0x12(a6)                   | +02c

| ----------------------------------------------------------------------------
|  Mob_Tmpl176_Body_098c00  @ $098C00  (282 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl176_Body_098c00, "ax", @progbits
        .global Mob_Tmpl176_Body_098c00
Mob_Tmpl176_Body_098c00:
        jsr     0x267e2.l                       | +000
        jsr     0x5e7c0.l                       | +006
        jsr     0x8f002.l                       | +00c
        jsr     0x8f010.l                       | +012
        move.w  #0x8000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x4,0x38(a6)                   | +028
        bra.w   Mob_Patrol_098c5a | +02e
        jsr     0x8f010.l                       | +032
        bra.w   Mob_Patrol_098c5a | +038
        .global Mob_TurnAround_098c3c
Mob_TurnAround_098c3c:
        eori.b  #0x1,0x3a(a6)                   | +03c
        neg.w   0x28(a6)                        | +042
        move.w  #0x8,d0                         | +046
        btst    #0x0,0x3a(a6)                   | +04a
        bne.w   .L098c56                        | +050
        neg.w   d0                              | +054
.L098c56:
        add.w   d0,0x22(a6)                     | +056
        .global Mob_Patrol_098c5a
Mob_Patrol_098c5a:
        move.w  #0x1,0x66(a6)                   | +05a
        move.w  0x70(a6),d0                     | +060
        movea.l #0x2f558e,a0                    | +064
        lsl.w   #0x2,d0                         | +06a
        movea.l (a0,d0.w),a0                    | +06c
        cmpa.l  #0xffffffff,a0                  | +070
        beq.w   .L098c80                        | +076
        jsr     0x28cd4.l                       | +07a
.L098c80:
        bclr    #0x3,0x13(a6)                   | +080
        lea     .L098c8c(pc),a1                 | +086
        move.l  a1,(a6)                         | +08a
.L098c8c:
        jsr     0x27a92.l                       | +08c
        btst    #0x5,0x5a(a6)                   | +092
        beq.w   .L098ca2                        | +098
        lea     Mob_Hit2_098d22(pc),a1          | +09c
        move.l  a1,(a6)                         | +0a0
.L098ca2:
        jsr     0x27eba.l                       | +0a2
        bcc.w   .L098cb2                        | +0a8
        lea     Mob_Flee_098b66(pc),a1          | +0ac
        move.l  a1,(a6)                         | +0b0
.L098cb2:
        jsr     0x28d70.l                       | +0b2
        tst.b   0x76(a6)                        | +0b8
        beq.w   .L098cca                        | +0bc
        bclr    #0x3,0x13(a6)                   | +0c0
        bra.w   .L098d04                        | +0c6
.L098cca:
        jsr     0x2870a.l                       | +0ca
        bcc.w   .L098d04                        | +0d0
        tst.w   0x70(a6)                        | +0d4
        beq.w   .L098cfe                        | +0d8
        subq.w  #0x1,d0                         | +0dc
        move.b  0x3a(a6),d1                     | +0de
        andi.w  #0x1,d1                         | +0e2
        cmp.w   d1,d0                           | +0e6
        bne.w   .L098cfe                        | +0e8
        subq.w  #0x1,0x72(a6)                   | +0ec
        beq.w   .L098cfe                        | +0f0
        lea     Mob_Pause_098e4e(pc),a1         | +0f4
        move.l  a1,(a6)                         | +0f8
        bra.w   .L098d04                        | +0fa
.L098cfe:
        lea     Mob_Hit2_098d22(pc),a1          | +0fe
        move.l  a1,(a6)                         | +102
.L098d04:
        movea.l #0xffffffff,a0                  | +104
        lea     0x2f61ac.l,a0                   | +10a
        jsr     0x5dd56.l                       | +110
        bcc.w   SetHandlerRts_098d20            | +116

| ----------------------------------------------------------------------------
|  Mob_Hit2_098d22  @ $098D22  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Hit2_098d22, "ax", @progbits
        .global Mob_Hit2_098d22
Mob_Hit2_098d22:
        move.b  #0xff,0x76(a6)                  | +000
        tst.w   0x70(a6)                        | +006
        beq.w   .L098d46                        | +00a
        lea     Mob_Drop_098e9a(pc),a1          | +00e
        jsr     0x4ae.l                         | +012
        jsr     0x5dd02.l                       | +018
        move.w  #0x0,0x70(a6)                   | +01e
.L098d46:
        lea     0x2f5b50.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        lea     .L098d58(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L098d58:
        jsr     0x2783a.l                       | +036
        jsr     0x28d70.l                       | +03c
        bcc.w   .L098d6e                        | +042
        lea     Mob_TurnAround_098c3c(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L098d6e:
        movea.l #0xffffffff,a0                  | +04c
        lea     0x2f61ac.l,a0                   | +052
        jsr     0x5dd56.l                       | +058
        bcc.w   SetHandlerRts_098d8a            | +05e

| ----------------------------------------------------------------------------
|  Mob_Tmpl178_Init_098d8c  @ $098D8C  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl178_Init_098d8c, "ax", @progbits
        .global Mob_Tmpl178_Init_098d8c
Mob_Tmpl178_Init_098d8c:
        move.w  #0x0,0x70(a6)                   | +000
        bra.w   .L098da2                        | +006
        move.w  #0x1,0x70(a6)                   | +00a
        move.w  #0x3,0x72(a6)                   | +010
.L098da2:
        move.w  #0x124,d1                       | +016
        jsr     0x236e.l                        | +01a
        move.w  #0x1,0x80(a6)                   | +020
        move.b  #0x0,0x76(a6)                   | +026
        bclr    #0x1,0x12(a6)                   | +02c

| ----------------------------------------------------------------------------
|  Mob_Tmpl178_Body_098dc6  @ $098DC6  (128 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl178_Body_098dc6, "ax", @progbits
        .global Mob_Tmpl178_Body_098dc6
Mob_Tmpl178_Body_098dc6:
        jsr     0x267e2.l                       | +000
        jsr     0x5e7c0.l                       | +006
        jsr     0x8f002.l                       | +00c
        jsr     0x8f010.l                       | +012
        move.w  #0x8000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x4,0x38(a6)                   | +028
        move.w  0x70(a6),d0                     | +02e
        movea.l #0x2f5596,a0                    | +032
        lsl.w   #0x2,d0                         | +038
        movea.l (a0,d0.w),a0                    | +03a
        cmpa.l  #0xffffffff,a0                  | +03e
        beq.w   .L098e14                        | +044
        jsr     0x28cd4.l                       | +048
.L098e14:
        lea     .L098e1a(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L098e1a:
        jsr     0x2783a.l                       | +054
        jsr     0x28d70.l                       | +05a
        bcc.w   .L098e30                        | +060
        lea     Mob_Patrol_098c5a(pc),a1 | +064
        move.l  a1,(a6)                         | +068
.L098e30:
        movea.l #0xffffffff,a0                  | +06a
        lea     0x2f61ac.l,a0                   | +070
        jsr     0x5dd56.l                       | +076
        bcc.w   SetHandlerRts_098e4c            | +07c

| ----------------------------------------------------------------------------
|  Mob_Pause_098e4e  @ $098E4E  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Pause_098e4e, "ax", @progbits
        .global Mob_Pause_098e4e
Mob_Pause_098e4e:
        lea     0x2f5c80.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L098e60(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L098e60:
        jsr     0x2783a.l                       | +012
        bclr    #0x3,0x13(a6)                   | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L098e7c                        | +024
        lea     Mob_Patrol_098c5a(pc),a1 | +028
        move.l  a1,(a6)                         | +02c
.L098e7c:
        movea.l #0xffffffff,a0                  | +02e
        lea     0x2f61ac.l,a0                   | +034
        jsr     0x5dd56.l                       | +03a
        bcc.w   SetHandlerRts_098e98            | +040

| ----------------------------------------------------------------------------
|  Mob_Drop_098e9a  @ $098E9A  (158 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Drop_098e9a, "ax", @progbits
        .global Mob_Drop_098e9a
Mob_Drop_098e9a:
        move.w  #0x124,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x8000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x4,0x38(a6)                   | +01a
        jsr     0x267e2.l                       | +020
        move.w  #0x355,d0                       | +026
        jsr     0x5dca4.l                       | +02a
        move.w  d0,0x28(a6)                     | +030
        move.w  #0x690,0x2a(a6)                 | +034
        move.w  #0xffc8,0x2e(a6)                | +03a
        move.w  #0x0,0x2c(a6)                   | +040
        jsr     0x5e9b6.l                       | +046
        move.w  #0x7f,d0                        | +04c
        subi.w  #0x3f,d0                        | +050
        add.w   0x28(a6),d0                     | +054
        move.w  d0,0x28(a6)                     | +058
        btst    #0x0,0x3a(a6)                   | +05c
        beq.w   .L098f04                        | +062
        neg.w   0x28(a6)                        | +066
.L098f04:
        lea     0x2f5c9c.l,a0                   | +06a
        jsr     0x28cd4.l                       | +070
        lea     .L098f16(pc),a1                 | +076
        move.l  a1,(a6)                         | +07a
.L098f16:
        jsr     0x27d50.l                       | +07c
        jsr     0x28d70.l                       | +082
        movea.l #0xffffffff,a0                  | +088
        lea     0x2f61ac.l,a0                   | +08e
        jsr     0x5dd56.l                       | +094
        bcc.w   SetHandlerRts_098f3e            | +09a

| ----------------------------------------------------------------------------
|  Mob_Tmpl195_Init_098f40  @ $098F40  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl195_Init_098f40, "ax", @progbits
        .global Mob_Tmpl195_Init_098f40
Mob_Tmpl195_Init_098f40:
        move.w  #0x126,d1                       | +000
        jsr     0x236e.l                        | +004
        bra.w   .L098f66                        | +00a
        move.w  #0x127,d1                       | +00e
        jsr     0x236e.l                        | +012
        bra.w   .L098f66                        | +018
        move.w  #0x128,d1                       | +01c
        jsr     0x236e.l                        | +020
.L098f66:
        jsr     0x267e2.l                       | +026
        jsr     0x5e7c0.l                       | +02c
        jsr     0x8f002.l                       | +032
        move.w  #0x1,0x70(a6)                   | +038
        move.w  #0x8000,d0                      | +03e
        jsr     0x28134.l                       | +042
        andi.w  #0xffe3,0x38(a6)                | +048
        ori.w   #0x4,0x38(a6)                   | +04e
        bclr    #0x1,0x12(a6)                   | +054

| ----------------------------------------------------------------------------
|  Mob_Tmpl196_Init_098fa2  @ $098FA2  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl196_Init_098fa2, "ax", @progbits
        .global Mob_Tmpl196_Init_098fa2
Mob_Tmpl196_Init_098fa2:
        move.w  #0x126,d1                       | +000
        jsr     0x236e.l                        | +004
        bra.w   .L098fc8                        | +00a
        move.w  #0x127,d1                       | +00e
        jsr     0x236e.l                        | +012
        bra.w   .L098fc8                        | +018
        move.w  #0x128,d1                       | +01c
        jsr     0x236e.l                        | +020
.L098fc8:
        jsr     0x267e2.l                       | +026
        jsr     0x5e7c0.l                       | +02c
        jsr     0x8f002.l                       | +032
        move.w  #0x0,0x70(a6)                   | +038
        move.w  #0x8000,d0                      | +03e
        jsr     0x28134.l                       | +042
        andi.w  #0xffe3,0x38(a6)                | +048
        ori.w   #0x4,0x38(a6)                   | +04e
        bclr    #0x1,0x12(a6)                   | +054

| ----------------------------------------------------------------------------
|  Mob_Stand_099004  @ $099004  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Stand_099004, "ax", @progbits
        .global Mob_Stand_099004
Mob_Stand_099004:
        jsr     0x8f010.l                       | +000
        move.w  #0x1,0x66(a6)                   | +006
        move.w  0x70(a6),d0                     | +00c
        movea.l #0x2f55ae,a0                    | +010
        lsl.w   #0x2,d0                         | +016
        movea.l (a0,d0.w),a0                    | +018
        cmpa.l  #0xffffffff,a0                  | +01c
        beq.w   .L099030                        | +022
        jsr     0x28cd4.l                       | +026
.L099030:
        lea     .L099036(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L099036:
        jsr     0x27a92.l                       | +032
        jsr     0x27eba.l                       | +038
        bcc.w   .L09904c                        | +03e
        lea     Mob_Flee2_099070(pc),a1         | +042
        move.l  a1,(a6)                         | +046
.L09904c:
        jsr     0x28d70.l                       | +048
        movea.l #0xffffffff,a0                  | +04e
        lea     0x2f61ac.l,a0                   | +054
        jsr     0x5dd56.l                       | +05a
        bcc.w   SetHandlerRts_09906e            | +060

| ----------------------------------------------------------------------------
|  Mob_Flee2_099070  @ $099070  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Flee2_099070, "ax", @progbits
        .global Mob_Flee2_099070
Mob_Flee2_099070:
        jsr     0x267e2.l                       | +000
        move.w  #0xffe0,0x2a(a6)                | +006
        lea     .L099082(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L099082:
        jsr     0x27c8c.l                       | +012
        jsr     0x27eba.l                       | +018
        bcs.w   .L099098                        | +01e
        lea     Mob_Stand_099004(pc),a1         | +022
        move.l  a1,(a6)                         | +026
.L099098:
        jsr     0x28d70.l                       | +028
        movea.l #0xffffffff,a0                  | +02e
        lea     0x2f61ac.l,a0                   | +034
        jsr     0x5dd56.l                       | +03a
        bcc.w   SetHandlerRts_0990ba            | +040

| ----------------------------------------------------------------------------
|  Mob_Tmpl201_Init_0990bc  @ $0990BC  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl201_Init_0990bc, "ax", @progbits
        .global Mob_Tmpl201_Init_0990bc
Mob_Tmpl201_Init_0990bc:
        move.w  #0x126,d1                       | +000
        jsr     0x236e.l                        | +004
        bra.w   .L0990e2                        | +00a
        move.w  #0x127,d1                       | +00e
        jsr     0x236e.l                        | +012
        bra.w   .L0990e2                        | +018
        move.w  #0x128,d1                       | +01c
        jsr     0x236e.l                        | +020
.L0990e2:
        jsr     0x267e2.l                       | +026
        jsr     0x5e7c0.l                       | +02c
        jsr     0x8f002.l                       | +032
        move.w  #0x1,0x70(a6)                   | +038
        move.w  #0x8000,d0                      | +03e
        jsr     0x28134.l                       | +042
        andi.w  #0xffe3,0x38(a6)                | +048
        ori.w   #0x4,0x38(a6)                   | +04e
        bclr    #0x1,0x12(a6)                   | +054

| ----------------------------------------------------------------------------
|  Mob_Tmpl202_Init_09911e  @ $09911E  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl202_Init_09911e, "ax", @progbits
        .global Mob_Tmpl202_Init_09911e
Mob_Tmpl202_Init_09911e:
        move.w  #0x126,d1                       | +000
        jsr     0x236e.l                        | +004
        bra.w   .L099144                        | +00a
        move.w  #0x127,d1                       | +00e
        jsr     0x236e.l                        | +012
        bra.w   .L099144                        | +018
        move.w  #0x128,d1                       | +01c
        jsr     0x236e.l                        | +020
.L099144:
        jsr     0x267e2.l                       | +026
        jsr     0x5e7c0.l                       | +02c
        jsr     0x8f002.l                       | +032
        move.w  #0x0,0x70(a6)                   | +038
        move.w  #0x8000,d0                      | +03e
        jsr     0x28134.l                       | +042
        andi.w  #0xffe3,0x38(a6)                | +048
        ori.w   #0x4,0x38(a6)                   | +04e
        bclr    #0x1,0x12(a6)                   | +054

| ----------------------------------------------------------------------------
|  Mob_Appear_099180  @ $099180  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Appear_099180, "ax", @progbits
        .global Mob_Appear_099180
Mob_Appear_099180:
        move.w  0x70(a6),d0                     | +000
        movea.l #0x2f55b6,a0                    | +004
        lsl.w   #0x2,d0                         | +00a
        movea.l (a0,d0.w),a0                    | +00c
        cmpa.l  #0xffffffff,a0                  | +010
        beq.w   .L0991a0                        | +016
        jsr     0x28cd4.l                       | +01a
.L0991a0:
        lea     .L0991a6(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L0991a6:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        bcc.w   .L0991bc                        | +032
        lea     Mob_Stand_099004(pc),a1         | +036
        move.l  a1,(a6)                         | +03a
.L0991bc:
        movea.l #0xffffffff,a0                  | +03c
        lea     0x2f61ac.l,a0                   | +042
        jsr     0x5dd56.l                       | +048
        bcc.w   SetHandlerRts_0991d8            | +04e

| ----------------------------------------------------------------------------
|  Mob_Tmpl207_Init_0991da  @ $0991DA  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl207_Init_0991da, "ax", @progbits
        .global Mob_Tmpl207_Init_0991da
Mob_Tmpl207_Init_0991da:
        move.w  #0x125,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x5e7c0.l                       | +00a
        jsr     0x267e2.l                       | +010
        jsr     0x8f002.l                       | +016
        move.w  #0x0,d0                         | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x4,0x38(a6)                   | +02c
        bclr    #0x1,0x12(a6)                   | +032

| ----------------------------------------------------------------------------
|  Mob_Sit_09921a  @ $09921A  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Sit_09921a, "ax", @progbits
        .global Mob_Sit_09921a
Mob_Sit_09921a:
        move.w  #0x3c,0x74(a6)                  | +000
        move.w  #0x1,0x66(a6)                   | +006
        lea     0x2f5ed8.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L099238(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L099238:
        jsr     0x2783a.l                       | +01e
        jsr     0x28d70.l                       | +024
        tst.w   0x74(a6)                        | +02a
        beq.w   .L099254                        | +02e
        subq.w  #0x1,0x74(a6)                   | +032
        bne.w   .L099264                        | +036
.L099254:
        jsr     0x2870a.l                       | +03a
        bcc.w   .L099264                        | +040
        lea     Mob_Sit_Hop_099288(pc),a1       | +044
        move.l  a1,(a6)                         | +048
.L099264:
        bclr    #0x3,0x13(a6)                   | +04a
        movea.l #0xffffffff,a0                  | +050
        lea     0x2f61ac.l,a0                   | +056
        jsr     0x5dd56.l                       | +05c
        bcc.w   SetHandlerRts_099286            | +062

| ----------------------------------------------------------------------------
|  Mob_Sit_Hop_099288  @ $099288  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Sit_Hop_099288, "ax", @progbits
        .global Mob_Sit_Hop_099288
Mob_Sit_Hop_099288:
        move.w  #0x28,0x74(a6)                  | +000
        lea     0x2f5ef8.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L0992a0(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L0992a0:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        subq.w  #0x1,0x74(a6)                   | +024
        bne.w   .L0992ba                        | +028
        lea     Mob_Sit_09921a(pc),a1           | +02c
        move.l  a1,(a6)                         | +030
.L0992ba:
        movea.l #0xffffffff,a0                  | +032
        lea     0x2f61ac.l,a0                   | +038
        jsr     0x5dd56.l                       | +03e
        bcc.w   SetHandlerRts_0992d6            | +044

| ----------------------------------------------------------------------------
|  Mob_Tmpl208_Init_0992d8  @ $0992D8  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl208_Init_0992d8, "ax", @progbits
        .global Mob_Tmpl208_Init_0992d8
Mob_Tmpl208_Init_0992d8:
        move.w  #0x126,d1                       | +000
        jsr     0x236e.l                        | +004
        bra.w   .L0992fe                        | +00a
        move.w  #0x127,d1                       | +00e
        jsr     0x236e.l                        | +012
        bra.w   .L0992fe                        | +018
        move.w  #0x128,d1                       | +01c
        jsr     0x236e.l                        | +020
.L0992fe:
        jsr     0x267e2.l                       | +026
        jsr     0x5e7c0.l                       | +02c
        jsr     0x8f002.l                       | +032
        move.w  #0x8000,d0                      | +038
        jsr     0x28134.l                       | +03c
        andi.w  #0xffe3,0x38(a6)                | +042
        ori.w   #0x4,0x38(a6)                   | +048
        move.w  #0x1,0x66(a6)                   | +04e
        lea     0x2f5f18.l,a0                   | +054
        jsr     0x28cd4.l                       | +05a
        bclr    #0x1,0x12(a6)                   | +060

| ----------------------------------------------------------------------------
|  Mob_Tmpl209_Init_099346  @ $099346  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl209_Init_099346, "ax", @progbits
        .global Mob_Tmpl209_Init_099346
Mob_Tmpl209_Init_099346:
        move.w  #0x126,d1                       | +000
        jsr     0x236e.l                        | +004
        bra.w   Mob_Tmpl209_Reset_09936c | +00a
        move.w  #0x127,d1                       | +00e
        jsr     0x236e.l                        | +012
        bra.w   Mob_Tmpl209_Reset_09936c | +018
        move.w  #0x128,d1                       | +01c
        jsr     0x236e.l                        | +020
        .global Mob_Tmpl209_Reset_09936c
Mob_Tmpl209_Reset_09936c:
        lea     0x2f5f38.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        move.w  #0x8000,d0                      | +032
        jsr     0x28134.l                       | +036
        andi.w  #0xffe3,0x38(a6)                | +03c
        ori.w   #0x4,0x38(a6)                   | +042
        jsr     0x5e7c0.l                       | +048
        bclr    #0x1,0x12(a6)                   | +04e

| ----------------------------------------------------------------------------
|  Mob_Tmpl209_Body_0993a2  @ $0993A2  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl209_Body_0993a2, "ax", @progbits
        .global Mob_Tmpl209_Body_0993a2
Mob_Tmpl209_Body_0993a2:
        jsr     0x267e2.l                       | +000
        jsr     0x8f002.l                       | +006
        jsr     0x8f010.l                       | +00c
        move.w  #0x1,0x66(a6)                   | +012
        lea     .L0993c0(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L0993c0:
        jsr     0x27a92.l                       | +01e
        jsr     0x27eba.l                       | +024
        bcc.w   .L0993d6                        | +02a
        lea     Mob_Tmpl209_Reset_09936c(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L0993d6:
        jsr     0x28d70.l                       | +034
        movea.l #0xffffffff,a0                  | +03a
        lea     0x2f61ac.l,a0                   | +040
        jsr     0x5dd56.l                       | +046
        bcc.w   SetHandlerRts_0993f8            | +04c

| ----------------------------------------------------------------------------
|  Mob_Flee3_0993fa  @ $0993FA  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Flee3_0993fa, "ax", @progbits
        .global Mob_Flee3_0993fa
Mob_Flee3_0993fa:
        jsr     0x267e2.l                       | +000
        move.w  #0xffe0,0x2a(a6)                | +006
        lea     .L09940c(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L09940c:
        jsr     0x27c8c.l                       | +012
        jsr     0x27eba.l                       | +018
        bcs.w   .L099422                        | +01e
        lea     Mob_Tmpl209_Reset_09936c(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L099422:
        jsr     0x28d70.l                       | +028
        movea.l #0xffffffff,a0                  | +02e
        lea     0x2f61ac.l,a0                   | +034
        jsr     0x5dd56.l                       | +03a
        bcc.w   SetHandlerRts_099444            | +040

| ----------------------------------------------------------------------------
|  Mob_Tmpl210_Init_099446  @ $099446  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl210_Init_099446, "ax", @progbits
        .global Mob_Tmpl210_Init_099446
Mob_Tmpl210_Init_099446:
        move.w  #0x126,d1                       | +000
        jsr     0x236e.l                        | +004
        bra.w   .L09946c                        | +00a
        move.w  #0x127,d1                       | +00e
        jsr     0x236e.l                        | +012
        bra.w   .L09946c                        | +018
        move.w  #0x128,d1                       | +01c
        jsr     0x236e.l                        | +020
.L09946c:
        jsr     0x5e7c0.l                       | +026
        move.w  #0x8000,d0                      | +02c
        jsr     0x28134.l                       | +030
        andi.w  #0xffe3,0x38(a6)                | +036
        ori.w   #0x4,0x38(a6)                   | +03c
        lea     0x2f5f80.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        bclr    #0x1,0x12(a6)                   | +04e

| ----------------------------------------------------------------------------
|  Mob_Tmpl211_Init_0994a2  @ $0994A2  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl211_Init_0994a2, "ax", @progbits
        .global Mob_Tmpl211_Init_0994a2
Mob_Tmpl211_Init_0994a2:
        move.w  #0x126,d1                       | +000
        jsr     0x236e.l                        | +004
        bra.w   .L0994c8                        | +00a
        move.w  #0x127,d1                       | +00e
        jsr     0x236e.l                        | +012
        bra.w   .L0994c8                        | +018
        move.w  #0x128,d1                       | +01c
        jsr     0x236e.l                        | +020
.L0994c8:
        jsr     0x5e7c0.l                       | +026
        move.w  #0x8000,d0                      | +02c
        jsr     0x28134.l                       | +030
        andi.w  #0xffe3,0x38(a6)                | +036
        ori.w   #0x4,0x38(a6)                   | +03c
        lea     0x2f5fc8.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        bclr    #0x1,0x12(a6)                   | +04e

| ----------------------------------------------------------------------------
|  Mob_Tmpl220_Init_0994fe  @ $0994FE  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl220_Init_0994fe, "ax", @progbits
        .global Mob_Tmpl220_Init_0994fe
Mob_Tmpl220_Init_0994fe:
        move.w  #0x125,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        jsr     0x5e7c0.l                       | +010
        jsr     0x8f002.l                       | +016
        move.w  #0x8000,d0                      | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x4,0x38(a6)                   | +02c
        bclr    #0x1,0x12(a6)                   | +032

| ----------------------------------------------------------------------------
|  Mob_RunLeft_09953e  @ $09953E  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_RunLeft_09953e, "ax", @progbits
        .global Mob_RunLeft_09953e
Mob_RunLeft_09953e:
        move.w  #0x1,0x66(a6)                   | +000
        lea     0x2f6010.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L099556(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L099556:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        cmpi.w  #0xb0,0x22(a6)                  | +024
        bgt.w   .L099572                        | +02a
        lea     Mob_RunLeft_Stop_099590(pc),a1  | +02e
        move.l  a1,(a6)                         | +032
.L099572:
        movea.l #0xffffffff,a0                  | +034
        lea     0x2f61ac.l,a0                   | +03a
        jsr     0x5dd56.l                       | +040
        bcc.w   SetHandlerRts_09958e            | +046

| ----------------------------------------------------------------------------
|  Mob_RunLeft_Stop_099590  @ $099590  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_RunLeft_Stop_099590, "ax", @progbits
        .global Mob_RunLeft_Stop_099590
Mob_RunLeft_Stop_099590:
        lea     0x2f6022.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0995a2(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0995a2:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        movea.l #0xffffffff,a0                  | +01e
        lea     0x2f61ac.l,a0                   | +024
        jsr     0x5dd56.l                       | +02a
        bcc.w   SetHandlerRts_0995ca            | +030

| ----------------------------------------------------------------------------
|  Mob_Tmpl221_Init_0995cc  @ $0995CC  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl221_Init_0995cc, "ax", @progbits
        .global Mob_Tmpl221_Init_0995cc
Mob_Tmpl221_Init_0995cc:
        move.w  #0x127,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        jsr     0x5e7c0.l                       | +010
        jsr     0x8f002.l                       | +016
        clr.b   0x21(a6)                        | +01c
        move.w  #0x8000,d0                      | +020
        jsr     0x28134.l                       | +024
        andi.w  #0xffe3,0x38(a6)                | +02a
        ori.w   #0x4,0x38(a6)                   | +030
        bclr    #0x1,0x12(a6)                   | +036

| ----------------------------------------------------------------------------
|  Mob_RunLeft2_099610  @ $099610  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_RunLeft2_099610, "ax", @progbits
        .global Mob_RunLeft2_099610
Mob_RunLeft2_099610:
        move.w  #0x1,0x66(a6)                   | +000
        lea     0x2f6048.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L099628(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L099628:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        tst.b   0x21(a6)                        | +024
        bne.w   .L099652                        | +028
        cmpi.w  #0xb0,0x22(a6)                  | +02c
        bgt.w   .L099652                        | +032
        move.b  #0xff,0x21(a6)                  | +036
        lea     Mob_RunLeft2_Wait_099670(pc),a1 | +03c
        move.l  a1,(a6)                         | +040
.L099652:
        movea.l #0xffffffff,a0                  | +042
        lea     0x2f61ac.l,a0                   | +048
        jsr     0x5dd56.l                       | +04e
        bcc.w   SetHandlerRts_09966e            | +054

| ----------------------------------------------------------------------------
|  Mob_RunLeft2_Wait_099670  @ $099670  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_RunLeft2_Wait_099670, "ax", @progbits
        .global Mob_RunLeft2_Wait_099670
Mob_RunLeft2_Wait_099670:
        lea     0x2f6068.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L099682(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L099682:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L099698                        | +01e
        lea     Mob_RunLeft2_099610(pc),a1      | +022
        move.l  a1,(a6)                         | +026
.L099698:
        movea.l #0xffffffff,a0                  | +028
        lea     0x2f61ac.l,a0                   | +02e
        jsr     0x5dd56.l                       | +034
        bcc.w   SetHandlerRts_0996b4            | +03a

| ----------------------------------------------------------------------------
|  Mob_Tmpl222_Init_0996b6  @ $0996B6  (172 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_Tmpl222_Init_0996b6, "ax", @progbits
        .global Mob_Tmpl222_Init_0996b6
Mob_Tmpl222_Init_0996b6:
        move.w  #0x126,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2f607a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.w   .L099734                        | +016
        move.w  #0x127,d1                       | +01a
        jsr     0x236e.l                        | +01e
        lea     0x2f607a.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        bra.w   .L099734                        | +030
        move.w  #0x128,d1                       | +034
        jsr     0x236e.l                        | +038
        lea     0x2f607a.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        bra.w   .L099734                        | +04a
        move.w  #0x129,d1                       | +04e
        jsr     0x236e.l                        | +052
        lea     0x2f608c.l,a0                   | +058
        jsr     0x28cd4.l                       | +05e
        bra.w   .L099734                        | +064
        move.w  #0x129,d1                       | +068
        jsr     0x236e.l                        | +06c
        lea     0x2f609e.l,a0                   | +072
        jsr     0x28cd4.l                       | +078
.L099734:
        jsr     0x267e2.l                       | +07e
        jsr     0x5e7c0.l                       | +084
        jsr     0x8f002.l                       | +08a
        move.w  #0x8000,d0                      | +090
        jsr     0x28134.l                       | +094
        andi.w  #0xffe3,0x38(a6)                | +09a
        ori.w   #0x4,0x38(a6)                   | +0a0
        bclr    #0x1,0x12(a6)                   | +0a6

| ----------------------------------------------------------------------------
|  Mob_PhysLoop_09976a  @ $09976A  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Mob_PhysLoop_09976a, "ax", @progbits
        .global Mob_PhysLoop_09976a
Mob_PhysLoop_09976a:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        movea.l #0xffffffff,a0                  | +00c
        lea     0x2f61ac.l,a0                   | +012
        jsr     0x5dd56.l                       | +018
        bcc.w   SetHandlerRts_099792            | +01e

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_09979c  @ $09979C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_09979c, "ax", @progbits
        .global Entity_CmpPrioWithSibling_09979c
Entity_CmpPrioWithSibling_09979c:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0997b2                    | +00c

| ----------------------------------------------------------------------------
|  Trail_RingReset_0997b8  @ $0997B8  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_RingReset_0997b8, "ax", @progbits
        .global Trail_RingReset_0997b8
Trail_RingReset_0997b8:
        clr.w   0x10e47e.l                      | +000
        clr.w   0x10e480.l                      | +006
        clr.w   0x10e482.l                      | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Trail_RingAdvance_0997cc  @ $0997CC  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_RingAdvance_0997cc, "ax", @progbits
        .global Trail_RingAdvance_0997cc
Trail_RingAdvance_0997cc:
        move.w  0x10e480.l,0x10e47e.l           | +000
        move.w  0x10e482.l,0x10e480.l           | +00a
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Entity_TrailRecord_Alt_0997e2  @ $0997E2  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_TrailRecord_Alt_0997e2, "ax", @progbits
        .global Entity_TrailRecord_Alt_0997e2
Entity_TrailRecord_Alt_0997e2:
        lea     0x10e3be.l,a0                   | +000
        move.b  0x6c(a6),d7                     | +006
        lsr.b   #0x4,d7                         | +00a
        cmpi.b  #0xe,d7                         | +00c
        beq.w   .L09980e                        | +010
        move.b  0x6c(a6),d5                     | +014
        andi.b  #0xf,d5                         | +018
        ori.b   #0xe0,d5                        | +01c
        move.b  d5,0x6c(a6)                     | +020
        clr.w   d5                              | +024
        clr.w   d6                              | +026
        bra.w   Entity_TrailRecord_StoreSlot_099896 | +028
.L09980e:
        bra.w   Entity_TrailRecord_HasId_09985a | +02c

| ----------------------------------------------------------------------------
|  Trail_FindByKeyRange_0998ca  @ $0998CA  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_FindByKeyRange_0998ca, "ax", @progbits
        .global Trail_FindByKeyRange_0998ca
Trail_FindByKeyRange_0998ca:
        movem.l d7/a0,-(a7)                     | +000
        lea     0x10e3be.l,a0                   | +004
        move.w  0x10e480.l,d7                   | +00a
        .global Trail_FindByKeyRange_Loop_0998da
Trail_FindByKeyRange_Loop_0998da:
        cmp.w   0x10e47e.l,d7                   | +010
        bne.w   Trail_FindByKeyRange_Step_0998ee | +016
        movem.l (a7)+,d7/a0                     | +01a

| ----------------------------------------------------------------------------
|  Trail_FindByKeyRange_Step_0998ee  @ $0998EE  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_FindByKeyRange_Step_0998ee, "ax", @progbits
        .global Trail_FindByKeyRange_Step_0998ee
Trail_FindByKeyRange_Step_0998ee:
        subi.w  #0xc,d7                         | +000
        bcc.w   .L0998fa                        | +004
        addi.w  #0xc0,d7                        | +008
.L0998fa:
        cmp.w   0x4(a0,d7.w),d2                 | +00c
        bne.w   Trail_FindByKeyRange_Next_09993a | +010
        move.w  d1,d5                           | +014
        sub.w   0x2(a0,d7.w),d5                 | +016
        cmp.w   0x6(a0,d7.w),d5                 | +01a
        bcc.w   Trail_FindByKeyRange_Next_09993a | +01e
        move.b  0x6c(a6),d5                     | +022
        lsr.b   #0x4,d5                         | +026
        cmp.b   (a0,d7.w),d5                    | +028
        beq.w   Trail_FindByKeyRange_Next_09993a | +02c
        andi.b  #0xf0,0x6c(a6)                  | +030
        move.b  (a0,d7.w),d5                    | +036
        andi.b  #0xf,d5                         | +03a
        or.b    d5,0x6c(a6)                     | +03e
        movem.l (a7)+,d7/a0                     | +042

| ----------------------------------------------------------------------------
|  Trail_FindByKeyRange_Next_09993a  @ $09993A  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_FindByKeyRange_Next_09993a, "ax", @progbits
        .global Trail_FindByKeyRange_Next_09993a
Trail_FindByKeyRange_Next_09993a:
        bra.b   Trail_FindByKeyRange_Loop_0998da | +000

| ----------------------------------------------------------------------------
|  Trail_FindNearest_09993c  @ $09993C  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_FindNearest_09993c, "ax", @progbits
        .global Trail_FindNearest_09993c
Trail_FindNearest_09993c:
        movem.l d7/a0,-(a7)                     | +000
        lea     0x10e3be.l,a0                   | +004
        move.w  0x10e47e.l,d7                   | +00a
        .global Trail_FindNearest_Loop_09994c
Trail_FindNearest_Loop_09994c:
        cmp.w   0x10e480.l,d7                   | +010
        bne.w   .L099976                        | +016
        move.b  0x6c(a6),d7                     | +01a
        andi.b  #0xf,d7                         | +01e
        cmpi.b  #0xe,d7                         | +022
        bne.w   .L09996c                        | +026
        ori.b   #0xf,0x6c(a6)                   | +02a
.L09996c:
        move.b  #0xf,d6                         | +030
        movem.l (a7)+,d7/a0                     | +034
        rts                                     | +038
.L099976:
        move.w  0x4(a0,d7.w),d5                 | +03a
        cmp.w   d5,d2                           | +03e
        bgt.w   Trail_FindNearest_Next_0999ba   | +040
        subi.w  #0x14,d5                        | +044
        cmp.w   d5,d2                           | +048
        ble.w   Trail_FindNearest_Next_0999ba   | +04a
        move.w  d1,d5                           | +04e
        sub.w   0x2(a0,d7.w),d5                 | +050
        cmp.w   0x6(a0,d7.w),d5                 | +054
        bcc.w   Trail_FindNearest_Next_0999ba   | +058
        move.b  0x6c(a6),d6                     | +05c
        lsr.b   #0x4,d6                         | +060
        cmp.b   (a0,d7.w),d6                    | +062
        beq.w   Trail_FindNearest_Next_0999ba   | +066
        move.w  0x4(a0,d7.w),d5                 | +06a
        sub.w   d2,d5                           | +06e
        move.b  (a0,d7.w),d6                    | +070
        movem.l (a7)+,d7/a0                     | +074

| ----------------------------------------------------------------------------
|  Trail_FindNearest_Next_0999ba  @ $0999BA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_FindNearest_Next_0999ba, "ax", @progbits
        .global Trail_FindNearest_Next_0999ba
Trail_FindNearest_Next_0999ba:
        addi.w  #0xc,d7                         | +000
        cmpi.w  #0xc0,d7                        | +004
        bcs.w   .L0999c8                        | +008
        clr.w   d7                              | +00c
.L0999c8:
        bra.b   Trail_FindNearest_Loop_09994c | +00e

| ----------------------------------------------------------------------------
|  Trail_MergeIdNibble_0999ca  @ $0999CA  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_MergeIdNibble_0999ca, "ax", @progbits
        .global Trail_MergeIdNibble_0999ca
Trail_MergeIdNibble_0999ca:
        move.b  0x6c(a6),d7                     | +000
        andi.w  #0xf0,d7                        | +004
        andi.w  #0xf,d0                         | +008
        or.w    d7,d0                           | +00c

| ----------------------------------------------------------------------------
|  Trail_LookupById_0999f6  @ $0999F6  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_LookupById_0999f6, "ax", @progbits
        .global Trail_LookupById_0999f6
Trail_LookupById_0999f6:
        lsr.b   #0x4,d6                         | +000
        cmp.b   d6,d5                           | +002
        bne.w   .L099a00                        | +004
        trap    #0xf                            | +008
.L099a00:
        lea     0x10e3be.l,a0                   | +00a
        move.w  0x10e47e.l,d7                   | +010
        .global Trail_LookupById_Loop_099a0c
Trail_LookupById_Loop_099a0c:
        cmp.w   0x10e480.l,d7                   | +016
        bne.w   Trail_LookupById_Step_099a22    | +01c
        ori.b   #0xf,0x6c(a6)                   | +020

| ----------------------------------------------------------------------------
|  Trail_LookupById_Step_099a22  @ $099A22  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Trail_LookupById_Step_099a22, "ax", @progbits
        .global Trail_LookupById_Step_099a22
Trail_LookupById_Step_099a22:
        cmp.b   (a0,d7.w),d5                    | +000
        beq.w   .L099a3a                        | +004
        addi.w  #0xc,d7                         | +008
        cmpi.w  #0xc0,d7                        | +00c
        bcs.w   .L099a38                        | +010
        clr.w   d7                              | +014
.L099a38:
        bra.b   Trail_LookupById_Loop_099a0c | +016
.L099a3a:
        adda.w  d7,a0                           | +018
        move.w  0x8(a0),d2                      | +01a
        move.w  0xa(a0),d3                      | +01e

| ----------------------------------------------------------------------------
|  DebugCursor_Init_099a4a  @ $099A4A  (18 B)
| ----------------------------------------------------------------------------
        .section .text.DebugCursor_Init_099a4a, "ax", @progbits
        .global DebugCursor_Init_099a4a
DebugCursor_Init_099a4a:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.l  #0x2507fa,0x3c(a6)              | +00a

| ----------------------------------------------------------------------------
|  DebugCursor_Run_099a64  @ $099A64  (90 B)
| ----------------------------------------------------------------------------
        .section .text.DebugCursor_Run_099a64, "ax", @progbits
        .global DebugCursor_Run_099a64
DebugCursor_Run_099a64:
        move.b  0x10fd9c.l,d0                   | +000
        btst    #0x0,d0                         | +006
        beq.w   .L099a76                        | +00a
        addq.w  #0x8,0x2a(a6)                   | +00e
.L099a76:
        btst    #0x1,d0                         | +012
        beq.w   .L099a82                        | +016
        subq.w  #0x8,0x2a(a6)                   | +01a
.L099a82:
        btst    #0x2,d0                         | +01e
        beq.w   .L099a8e                        | +022
        subq.w  #0x8,0x28(a6)                   | +026
.L099a8e:
        btst    #0x3,d0                         | +02a
        beq.w   .L099a9a                        | +02e
        addq.w  #0x8,0x28(a6)                   | +032
.L099a9a:
        jsr     0x27cee.l                       | +036
        move.w  0x22(a6),d0                     | +03c
        move.w  0x24(a6),d1                     | +040
        subi.w  #0x10,d0                        | +044
        addq.w  #0x7,d1                         | +048
        move.w  #0x20,d2                        | +04a
        andi.w  #0x1ff,d0                       | +04e
        andi.w  #0x1ff,d1                       | +052
        jsr     Entity_TrailRecord_099812(pc)   | +056

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_099ac6  @ $099AC6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_099ac6, "ax", @progbits
        .global Entity_CmpPrioWithSibling_099ac6
Entity_CmpPrioWithSibling_099ac6:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_099adc                    | +00c

| ----------------------------------------------------------------------------
|  OptionSelect2_Tpl_099b06  @ $099B06  (120 B)
| ----------------------------------------------------------------------------
        .section .text.OptionSelect2_Tpl_099b06, "ax", @progbits
        .global OptionSelect2_Tpl_099b06
OptionSelect2_Tpl_099b06:
        movea.w #0x7258,a1                      | +000
        move.w  #0x2300,d0                      | +004
        lea     0x2f61b4.l,a2                   | +008
        jsr     0x5dad8.l                       | +00e
        movea.w #0x725a,a1                      | +014
        move.w  #0x2300,d0                      | +018
        lea     0x2f61ba.l,a2                   | +01c
        jsr     0x5dad8.l                       | +022
        clr.w   0x78(a6)                        | +028
        jsr     OptionSelect_DrawCursor_099ee4(pc) | +02c
        lea     .L099b3c(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L099b3c:
        jsr     Input_PressedEither_099ed2(pc)  | +036
        btst    #0x4,d0                         | +03a
        beq.w   .L099b64                        | +03e
        move.w  0x78(a6),d0                     | +042
        andi.w  #0x1,d0                         | +046
        move.b  d0,0x106ed5.l                   | +04a
        clr.b   0x106ed2.l                      | +050
        jmp     0x518.l                         | +056
        rts                                     | +05c
.L099b64:
        jsr     Input_PressedEither_099ed2(pc)  | +05e
        btst    #0x1,d0                         | +062
        beq.w   OptionSelect2_Left_099b84       | +066
        jsr     OptionSelect_EraseCursor_099f10(pc) | +06a
        addq.w  #0x1,0x78(a6)                   | +06e
        andi.w  #0x1,0x78(a6)                   | +072

| ----------------------------------------------------------------------------
|  OptionSelect2_Left_099b84  @ $099B84  (26 B)
| ----------------------------------------------------------------------------
        .section .text.OptionSelect2_Left_099b84, "ax", @progbits
        .global OptionSelect2_Left_099b84
OptionSelect2_Left_099b84:
        jsr     Input_PressedEither_099ed2(pc)  | +000
        btst    #0x0,d0                         | +004
        beq.w   OptionSelect2_Rts_099ba4        | +008
        jsr     OptionSelect_EraseCursor_099f10(pc) | +00c
        subq.w  #0x1,0x78(a6)                   | +010
        andi.w  #0x1,0x78(a6)                   | +014

| ----------------------------------------------------------------------------
|  OptionSelect2_Rts_099ba4  @ $099BA4  (2 B)
| ----------------------------------------------------------------------------
        .section .text.OptionSelect2_Rts_099ba4, "ax", @progbits
        .global OptionSelect2_Rts_099ba4
OptionSelect2_Rts_099ba4:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  OptionsMenu_Tpl_099ba6  @ $099BA6  (444 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_Tpl_099ba6, "ax", @progbits
        .global OptionsMenu_Tpl_099ba6
OptionsMenu_Tpl_099ba6:
        lea     0x59a86.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        cmpi.b  #0x2,0x10fd83.l                 | +00c
        bne.w   .L099bd2                        | +014
        lea     0x2f623c.l,a1                   | +018
        move.l  a1,0x80(a6)                     | +01e
        move.w  #0x3,0x7a(a6)                   | +022
        bra.w   .L099be2                        | +028
.L099bd2:
        lea     0x2f6236.l,a1                   | +02c
        move.l  a1,0x80(a6)                     | +032
        move.w  #0x2,0x7a(a6)                   | +036
.L099be2:
        movea.w #0x71c7,a1                      | +03c
        lea     0x2f61c2.l,a2                   | +040
        move.w  #0x4,d1                         | +046
        jsr     0x4784c.l                       | +04a
        movea.w #0x716b,a1                      | +050
        lea     0x2f61d8.l,a2                   | +054
        move.w  #0x4,d1                         | +05a
        jsr     0x4784c.l                       | +05e
        movea.w #0x716f,a1                      | +064
        lea     0x2f61de.l,a2                   | +068
        move.w  #0x4,d1                         | +06e
        jsr     0x4784c.l                       | +072
        cmpi.b  #0x2,0x10fd83.l                 | +078
        bne.w   .L099c3e                        | +080
        movea.w #0x7173,a1                      | +084
        lea     0x2f6210.l,a2                   | +088
        move.w  #0x4,d1                         | +08e
        jsr     0x477fc.l                       | +092
.L099c3e:
        movea.w #0x7177,a1                      | +098
        lea     0x2f61ea.l,a2                   | +09c
        move.w  #0x4,d1                         | +0a2
        jsr     0x4784c.l                       | +0a6
        clr.w   0x78(a6)                        | +0ac
        clr.w   d0                              | +0b0
        move.b  0x10fd8b.l,d0                   | +0b2
        andi.b  #0x7,d0                         | +0b8
        lea     0x2f6244.l,a1                   | +0bc
        move.b  (a1,d0.w),0x72(a6)              | +0c2
        move.b  0x10fd88.l,0x73(a6)             | +0c8
        cmpi.b  #0x1,0x73(a6)                   | +0d0
        bcc.w   .L099c86                        | +0d6
        move.b  #0x1,0x73(a6)                   | +0da
.L099c86:
        cmpi.b  #0x5,0x73(a6)                   | +0e0
        bls.w   .L099c96                        | +0e6
        move.b  #0x5,0x73(a6)                   | +0ea
.L099c96:
        move.b  0x10e486.l,0x74(a6)             | +0f0
        addq.b  #0x1,0x74(a6)                   | +0f8
        cmpi.b  #0x1,0x74(a6)                   | +0fc
        bcc.w   .L099cb2                        | +102
        move.b  #0x1,0x74(a6)                   | +106
.L099cb2:
        cmpi.b  #0x5,0x74(a6)                   | +10c
        bls.w   .L099cc2                        | +112
        move.b  #0x5,0x74(a6)                   | +116
.L099cc2:
        move.b  0x10fd92.l,0x76(a6)             | +11c
        cmpi.b  #0x1,0x76(a6)                   | +124
        beq.w   .L099cd8                        | +12a
        clr.b   0x76(a6)                        | +12e
.L099cd8:
        jsr     FixGlyphRun_Draw2F61F0_099FD2(pc) | +132
        jsr     FixGlyph16_DrawDigit72EF_099FF2(pc) | +136
        jsr     FixGlyphRun_DrawPad2P_09A086(pc) | +13a
        jsr     FixGlyph16_DrawCursorA_099F3A(pc) | +13e
        lea     .L099cee(pc),a1                 | +142
        move.l  a1,(a6)                         | +146
.L099cee:
        jsr     Input_PressedEither_099ed2(pc)  | +148
        btst    #0x4,d0                         | +14c
        beq.w   .L099d50                        | +150
        move.w  0x7a(a6),d0                     | +154
        cmp.w   0x78(a6),d0                     | +158
        bne.w   .L099d50                        | +15c
        lea     0x2f624c.l,a1                   | +160
        clr.w   d0                              | +166
        move.b  0x72(a6),d0                     | +168
        move.b  (a1,d0.w),0x10fd8b.l            | +16c
        move.b  0x73(a6),0x10fd88.l             | +174
        move.b  0x74(a6),d0                     | +17c
        subq.b  #0x1,d0                         | +180
        move.b  d0,0x10e486.l                   | +182
        cmpi.b  #0x2,0x10fd83.l                 | +188
        bne.w   .L099d42                        | +190
        move.b  0x76(a6),0x10fd92.l             | +194
.L099d42:
        clr.b   0x106ed2.l                      | +19c
        jmp     0x518.l                         | +1a2
        rts                                     | +1a8
.L099d50:
        andi.b  #0xc,d0                         | +1aa
        beq.w   OptionsMenu_NavDown_099d94      | +1ae
        cmpi.w  #0x0,0x78(a6)                   | +1b2
        bne.w   OptionsMenu_Row1_099d68         | +1b8

| ----------------------------------------------------------------------------
|  OptionsMenu_Row1_099d68  @ $099D68  (10 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_Row1_099d68, "ax", @progbits
        .global OptionsMenu_Row1_099d68
OptionsMenu_Row1_099d68:
        cmpi.w  #0x1,0x78(a6)                   | +000
        bne.w   OptionsMenu_Row2_099d78         | +006

| ----------------------------------------------------------------------------
|  OptionsMenu_Row2_099d78  @ $099D78  (22 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_Row2_099d78, "ax", @progbits
        .global OptionsMenu_Row2_099d78
OptionsMenu_Row2_099d78:
        cmpi.b  #0x2,0x10fd83.l                 | +000
        bne.w   OptionsMenu_NavDown_099d94      | +008
        cmpi.w  #0x2,0x78(a6)                   | +00c
        bne.w   OptionsMenu_NavDown_099d94      | +012

| ----------------------------------------------------------------------------
|  OptionsMenu_NavDown_099d94  @ $099D94  (36 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_NavDown_099d94, "ax", @progbits
        .global OptionsMenu_NavDown_099d94
OptionsMenu_NavDown_099d94:
        jsr     Input_PressedEither_099ed2(pc)  | +000
        btst    #0x1,d0                         | +004
        beq.w   OptionsMenu_NavUp_099dbe        | +008
        jsr     FixGlyph16_DrawCursorB_099F86(pc) | +00c
        addq.w  #0x1,0x78(a6)                   | +010
        move.w  0x78(a6),d0                     | +014
        cmp.w   0x7a(a6),d0                     | +018
        bls.w   JsrPcThunk_099db8               | +01c
        clr.w   0x78(a6)                        | +020

| ----------------------------------------------------------------------------
|  OptionsMenu_NavUp_099dbe  @ $099DBE  (30 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_NavUp_099dbe, "ax", @progbits
        .global OptionsMenu_NavUp_099dbe
OptionsMenu_NavUp_099dbe:
        jsr     Input_PressedEither_099ed2(pc)  | +000
        btst    #0x0,d0                         | +004
        beq.w   OptionsMenu_Rts_099de2          | +008
        jsr     FixGlyph16_DrawCursorB_099F86(pc) | +00c
        subq.w  #0x1,0x78(a6)                   | +010
        bcc.w   JsrPcThunk_099ddc               | +014
        move.w  0x7a(a6),0x78(a6)               | +018

| ----------------------------------------------------------------------------
|  OptionsMenu_Rts_099de2  @ $099DE2  (2 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_Rts_099de2, "ax", @progbits
        .global OptionsMenu_Rts_099de2
OptionsMenu_Rts_099de2:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  OptionsMenu_AdjDifficulty_099de4  @ $099DE4  (18 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_AdjDifficulty_099de4, "ax", @progbits
        .global OptionsMenu_AdjDifficulty_099de4
OptionsMenu_AdjDifficulty_099de4:
        btst    #0x2,d0                         | +000
        beq.w   OptionsMenu_AdjDifficulty_Right_099dfc | +004
        subq.b  #0x1,0x72(a6)                   | +008
        andi.b  #0x3,0x72(a6)                   | +00c

| ----------------------------------------------------------------------------
|  OptionsMenu_AdjDifficulty_Right_099dfc  @ $099DFC  (18 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_AdjDifficulty_Right_099dfc, "ax", @progbits
        .global OptionsMenu_AdjDifficulty_Right_099dfc
OptionsMenu_AdjDifficulty_Right_099dfc:
        btst    #0x3,d0                         | +000
        beq.w   JsrPcRts_099e12                 | +004
        addq.b  #0x1,0x72(a6)                   | +008
        andi.b  #0x3,0x72(a6)                   | +00c

| ----------------------------------------------------------------------------
|  OptionsMenu_AdjLives_099e14  @ $099E14  (28 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_AdjLives_099e14, "ax", @progbits
        .global OptionsMenu_AdjLives_099e14
OptionsMenu_AdjLives_099e14:
        btst    #0x2,d0                         | +000
        beq.w   OptionsMenu_AdjLives_Right_099e36 | +004
        subq.b  #0x1,0x73(a6)                   | +008
        cmpi.b  #0x1,0x73(a6)                   | +00c
        bcc.w   JsrPcThunk_099e30               | +012
        move.b  #0x1,0x73(a6)                   | +016

| ----------------------------------------------------------------------------
|  OptionsMenu_AdjLives_Right_099e36  @ $099E36  (28 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_AdjLives_Right_099e36, "ax", @progbits
        .global OptionsMenu_AdjLives_Right_099e36
OptionsMenu_AdjLives_Right_099e36:
        btst    #0x3,d0                         | +000
        beq.w   JsrPcRts_099e56                 | +004
        addq.b  #0x1,0x73(a6)                   | +008
        cmpi.b  #0x5,0x73(a6)                   | +00c
        bls.w   JsrPcThunk_099e52               | +012
        move.b  #0x5,0x73(a6)                   | +016

| ----------------------------------------------------------------------------
|  OptionsMenu_AdjCredits_099e58  @ $099E58  (28 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_AdjCredits_099e58, "ax", @progbits
        .global OptionsMenu_AdjCredits_099e58
OptionsMenu_AdjCredits_099e58:
        btst    #0x2,d0                         | +000
        beq.w   OptionsMenu_AdjCredits_Right_099e7a | +004
        subq.b  #0x1,0x74(a6)                   | +008
        cmpi.b  #0x1,0x74(a6)                   | +00c
        bcc.w   JsrPcThunk_099e74               | +012
        move.b  #0x1,0x74(a6)                   | +016

| ----------------------------------------------------------------------------
|  OptionsMenu_AdjCredits_Right_099e7a  @ $099E7A  (28 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_AdjCredits_Right_099e7a, "ax", @progbits
        .global OptionsMenu_AdjCredits_Right_099e7a
OptionsMenu_AdjCredits_Right_099e7a:
        btst    #0x3,d0                         | +000
        beq.w   JsrPcRts_099e9a                 | +004
        addq.b  #0x1,0x74(a6)                   | +008
        cmpi.b  #0x5,0x74(a6)                   | +00c
        bls.w   JsrPcThunk_099e96               | +012
        move.b  #0x5,0x74(a6)                   | +016

| ----------------------------------------------------------------------------
|  OptionsMenu_Adj2PMode_099e9c  @ $099E9C  (28 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_Adj2PMode_099e9c, "ax", @progbits
        .global OptionsMenu_Adj2PMode_099e9c
OptionsMenu_Adj2PMode_099e9c:
        cmpi.b  #0x2,0x10fd83.l                 | +000
        beq.w   .L099eaa                        | +008
        rts                                     | +00c
.L099eaa:
        btst    #0x2,d0                         | +00e
        beq.w   OptionsMenu_Adj2PMode_Right_099ebe | +012
        move.b  #0x0,0x76(a6)                   | +016

| ----------------------------------------------------------------------------
|  OptionsMenu_Adj2PMode_Right_099ebe  @ $099EBE  (14 B)
| ----------------------------------------------------------------------------
        .section .text.OptionsMenu_Adj2PMode_Right_099ebe, "ax", @progbits
        .global OptionsMenu_Adj2PMode_Right_099ebe
OptionsMenu_Adj2PMode_Right_099ebe:
        btst    #0x3,d0                         | +000
        beq.w   JsrPcRts_099ed0                 | +004
        move.b  #0x1,0x76(a6)                   | +008

| ----------------------------------------------------------------------------
|  Input_PressedEither_099ed2  @ $099ED2  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Input_PressedEither_099ed2, "ax", @progbits
        .global Input_PressedEither_099ed2
Input_PressedEither_099ed2:
        move.b  0x10e215.l,d0                   | +000
        bne.w   .L099ee2                        | +006
        move.b  0x10e21b.l,d0                   | +00a
.L099ee2:
        rts                                     | +010

| ----------------------------------------------------------------------------
|  OptionSelect_DrawCursor_099ee4  @ $099EE4  (44 B)
| ----------------------------------------------------------------------------
        .section .text.OptionSelect_DrawCursor_099ee4, "ax", @progbits
        .global OptionSelect_DrawCursor_099ee4
OptionSelect_DrawCursor_099ee4:
        lea     0x2f6232.l,a1                   | +000
        move.w  0x78(a6),d2                     | +006
        lsl.w   #0x1,d2                         | +00a
        move.w  (a1,d2.w),d0                    | +00c
        move.w  #0x237b,d1                      | +010
        movem.w d0-d1,0x3c0000.l                | +014
        addi.w  #0x20,d0                        | +01c
        addq.w  #0x1,d1                         | +020
        movem.w d0-d1,0x3c0000.l                | +022
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  OptionSelect_EraseCursor_099f10  @ $099F10  (42 B)
| ----------------------------------------------------------------------------
        .section .text.OptionSelect_EraseCursor_099f10, "ax", @progbits
        .global OptionSelect_EraseCursor_099f10
OptionSelect_EraseCursor_099f10:
        lea     0x2f6232.l,a1                   | +000
        move.w  0x78(a6),d2                     | +006
        lsl.w   #0x1,d2                         | +00a
        move.w  (a1,d2.w),d0                    | +00c
        move.w  #0x2320,d1                      | +010
        movem.w d0-d1,0x3c0000.l                | +014
        addi.w  #0x20,d0                        | +01c
        movem.w d0-d1,0x3c0000.l                | +020
        rts                                     | +028
