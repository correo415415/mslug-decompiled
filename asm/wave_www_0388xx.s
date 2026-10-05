| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $0388F0..$03A60A  (7,432 B, 125 entradas, 4 huecos)
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
|  Sub_000388F0  @ $0388F0  (306 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000388F0, "ax", @progbits
        .global Sub_000388F0
Sub_000388F0:
        bset    #0x1,0x8c(a6)                   | +000
        bclr    #0x1,0x8c(a6)                   | +006
        move.l  #0x32598,0x60(a6)               | +00c
        jsr     0x267e6.l                       | +014
        clr.w   0x28(a6)                        | +01a
        clr.w   0x2c(a6)                        | +01e
        jsr     0x2abcc.l                       | +022
        bcs.w   .L038944                        | +028
        move.b  #0x30,0x70(a6)                  | +02c
        lea     0x279d1a.l,a0                   | +032
        move.l  -0x4(a0),0x74(a6)               | +038
        move.b  #0xff,0x21(a6)                  | +03e
        lea     0x279d1a.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        bra.w   .L038968                        | +050
.L038944:
        move.b  #0x30,0x70(a6)                  | +054
        lea     0x279d1a.l,a0                   | +05a
        move.l  -0x4(a0),0x74(a6)               | +060
        move.b  #0xff,0x21(a6)                  | +066
        lea     0x279d1a.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
.L038968:
        lea     Sub_00032734(pc),a0             | +078
        move.l  a0,0x48(a6)                     | +07c
        lea     .L038976(pc),a1                 | +080
        move.l  a1,(a6)                         | +084
.L038976:
        bset    #0x6,0x8d(a6)                   | +086
        jsr     Player_FrameCommon_032ff2(pc)   | +08c
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +090
        jsr     0x27a92.l                       | +094
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +09a
        bcc.w   .L038998                        | +09e
        lea     Player_CrouchIdleC_03820e(pc),a1 | +0a2
        move.l  a1,(a6)                         | +0a6
.L038998:
        btst    #0x2,0x8c(a6)                   | +0a8
        bne.w   .L038a02                        | +0ae
        btst    #0x0,0x3a(a6)                   | +0b2
        bne.w   .L0389be                        | +0b8
        jsr     Input_RightThunk_032e42(pc)     | +0bc
        bcc.w   .L0389ba                        | +0c0
        lea     Player_CrawlLeft_Alt_0386e2(pc),a1 | +0c4
        move.l  a1,(a6)                         | +0c8
.L0389ba:
        bra.w   .L0389cc                        | +0ca
.L0389be:
        jsr     JmpAbsThunk_032e3c(pc)          | +0ce
        bcc.w   .L0389cc                        | +0d2
        lea     Player_CrawlRight_Alt_038688(pc),a1 | +0d6
        move.l  a1,(a6)                         | +0da
.L0389cc:
        jsr     Player_ActionSelect_0330d0(pc)  | +0dc
        bcc.w   .L038a02                        | +0e0
        cmpi.b  #0xff,d1                        | +0e4
        bne.w   .L0389e6                        | +0e8
        lea     Sub_00038A28(pc),a1             | +0ec
        move.l  a1,(a6)                         | +0f0
        bra.w   .L038a02                        | +0f2
.L0389e6:
        cmpi.b  #0x3,d1                         | +0f6
        bne.w   .L0389f8                        | +0fa
        lea     Sub_000388F0(pc),a1             | +0fe
        move.l  a1,(a6)                         | +102
        bra.w   .L038a02                        | +104
.L0389f8:
        lea     Player_CrouchShoot_03873c(pc),a1 | +108
        move.l  a1,(a6)                         | +10c
        bra.w   .L038a02                        | +10e
.L038a02:
        jsr     0x5cef8.l                       | +112
        bcs.w   .L038a12                        | +118
        lea     Player_CrouchExit_037ec2(pc),a1 | +11c
        move.l  a1,(a6)                         | +120
.L038a12:
        jsr     0x27eba.l                       | +122
        bcc.w   JsrPcThunk_038a22               | +128
        lea     Player_KnockbackDelay_036fc2(pc),a1 | +12c
        move.l  a1,(a6)                         | +130

| ----------------------------------------------------------------------------
|  Sub_00038A28  @ $038A28  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00038A28, "ax", @progbits
        .global Sub_00038A28
Sub_00038A28:
        bset    #0x1,0x8c(a6)                   | +000
        bclr    #0x1,0x8c(a6)                   | +006
        lea     Sub_00032638(pc),a0             | +00c
        move.l  a0,0x4c(a6)                     | +010
        jsr     0x283ca.l                       | +014
        clr.w   0x28(a6)                        | +01a
        clr.w   0x2c(a6)                        | +01e
        clr.w   0x2a(a6)                        | +022
        clr.w   0x2e(a6)                        | +026
        move.l  #0x32500,0x60(a6)               | +02a
        jsr     0x2abcc.l                       | +032
        bcs.w   .L038a8c                        | +038
        move.b  #0x0,0x70(a6)                   | +03c
        lea     0x279eea.l,a0                   | +042
        move.l  -0x4(a0),0x74(a6)               | +048
        move.b  #0xff,0x21(a6)                  | +04e
        lea     0x279eea.l,a0                   | +054
        jsr     0x28cd4.l                       | +05a
        bra.w   .L038ab0                        | +060
.L038a8c:
        move.b  #0x0,0x70(a6)                   | +064
        lea     0x279eea.l,a0                   | +06a
        move.l  -0x4(a0),0x74(a6)               | +070
        move.b  #0xff,0x21(a6)                  | +076
        lea     0x279eea.l,a0                   | +07c
        jsr     0x28cd4.l                       | +082
.L038ab0:
        lea     Sub_000327DC(pc),a0             | +088  -> $0327DC (hueco futuro, defsym forward)
        move.l  a0,0x48(a6)                     | +08c
        lea     .L038abe(pc),a1                 | +090
        move.l  a1,(a6)                         | +094
.L038abe:
        bset    #0x6,0x8d(a6)                   | +096
        jsr     Player_FrameCommon_032ff2(pc)   | +09c
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +0a0
        jsr     0x27a92.l                       | +0a4
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0aa
        bcc.w   JsrPcThunk_038ae0               | +0ae
        lea     Player_CrouchIdleC_03820e(pc),a1 | +0b2
        move.l  a1,(a6)                         | +0b6

| ----------------------------------------------------------------------------
|  Sub_00038AE6  @ $038AE6  (248 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00038AE6, "ax", @progbits
        .global Sub_00038AE6
Sub_00038AE6:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bset    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        move.b  #0x33,0x70(a6)                  | +01e
        lea     0x27970c.l,a0                   | +024
        move.l  -0x4(a0),0x74(a6)               | +02a
        move.b  #0xff,0x21(a6)                  | +030
        lea     0x27970c.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        lea     Sub_00032734(pc),a0             | +042
        move.l  a0,0x48(a6)                     | +046
        move.l  #0x32500,0x60(a6)               | +04a
        jsr     0x267e6.l                       | +052
        clr.w   0x28(a6)                        | +058
        lea     .L038b48(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L038b48:
        bset    #0x6,0x8d(a6)                   | +062
        jsr     Player_FrameCommon_032ff2(pc)   | +068
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +06c
        jsr     0x27a92.l                       | +070
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +076
        bcc.w   .L038b6a                        | +07a
        lea     Player_CrouchIdleB_0381a2(pc),a1 | +07e
        move.l  a1,(a6)                         | +082
.L038b6a:
        btst    #0x2,0x8c(a6)                   | +084
        bne.w   .L038b7a                        | +08a
        move.b  #0x1,0x85(a6)                   | +08e
.L038b7a:
        jsr     0x5cef8.l                       | +094
        bcs.w   .L038b8a                        | +09a
        lea     Player_CrouchExit_037ec2(pc),a1 | +09e
        move.l  a1,(a6)                         | +0a2
.L038b8a:
        jsr     Player_ActionSelect_0330d0(pc)  | +0a4
        bcc.w   .L038bc0                        | +0a8
        cmpi.b  #0xff,d1                        | +0ac
        bne.w   .L038ba4                        | +0b0
        lea     Sub_00038A28(pc),a1             | +0b4
        move.l  a1,(a6)                         | +0b8
        bra.w   .L038bc0                        | +0ba
.L038ba4:
        cmpi.b  #0x3,d1                         | +0be
        bne.w   .L038bb6                        | +0c2
        lea     Sub_000388F0(pc),a1             | +0c6
        move.l  a1,(a6)                         | +0ca
        bra.w   .L038bc0                        | +0cc
.L038bb6:
        lea     Player_CrouchShoot_03873c(pc),a1 | +0d0
        move.l  a1,(a6)                         | +0d4
        bra.w   .L038bc0                        | +0d6
.L038bc0:
        jsr     Input_FireByMode_033034(pc)     | +0da
        bcc.w   .L038bce                        | +0de
        lea     Player_JumpStart_036914(pc),a1  | +0e2
        move.l  a1,(a6)                         | +0e6
.L038bce:
        jsr     0x27eba.l                       | +0e8
        bcc.w   JsrPcThunk_038bde               | +0ee
        lea     Player_KnockbackDelay_036fc2(pc),a1 | +0f2
        move.l  a1,(a6)                         | +0f6

| ----------------------------------------------------------------------------
|  Sub_00038BE4  @ $038BE4  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00038BE4, "ax", @progbits
        .global Sub_00038BE4
Sub_00038BE4:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x2,0x6d(a0)                   | +004
        bne.w   .L038bfa                        | +00a
        move.w  #0x176,d1                       | +00e
        bra.w   .L038bfe                        | +012
.L038bfa:
        move.w  #0x177,d1                       | +016
.L038bfe:
        jsr     0x236e.l                        | +01a
        move.w  #0xfef0,d0                      | +020
        jsr     0x5dca4.l                       | +024
        move.w  d0,0x28(a6)                     | +02a
        move.w  #0x50c,0x2a(a6)                 | +02e
        move.w  #0xffc9,0x2e(a6)                | +034
        move.w  #0x0,0x2c(a6)                   | +03a
        lea     0x27a28e.l,a0                   | +040
        jsr     0x28cd4.l                       | +046
        andi.w  #0xffe3,0x38(a6)                | +04c
        ori.w   #0x18,0x38(a6)                  | +052
        lea     .L038c42(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L038c42:
        jsr     0x27bc8.l                       | +05e
        bcc.w   .L038c52                        | +064
        lea     TaskHandler_038c70(pc),a1       | +068
        move.l  a1,(a6)                         | +06c
.L038c52:
        jsr     0x28d70.l                       | +06e
        movea.l #0xffffffff,a0                  | +074
        jsr     0x5dd56.l                       | +07a
        bcc.w   .L038c6e                        | +080
        lea     TaskHandler_038cee(pc),a1       | +084
        move.l  a1,(a6)                         | +088
.L038c6e:
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  TaskHandler_038c70  @ $038C70  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038c70, "ax", @progbits
        .global TaskHandler_038c70
TaskHandler_038c70:
        jsr     0x77d88.l                       | +000
        move.w  0x22(a6),0x22(a0)               | +006
        move.w  0x24(a6),0x24(a0)               | +00c
        move.w  #0xff67,d0                      | +012
        jsr     0x5dca4.l                       | +016
        move.w  d0,0x28(a6)                     | +01c
        move.w  #0x3fc,0x2a(a6)                 | +020
        move.w  #0xffbc,0x2e(a6)                | +026
        move.w  #0x0,0x2c(a6)                   | +02c
        move.b  #0x28,0x45(a6)                  | +032
        move.b  #0x28,0x44(a6)                  | +038
        lea     .L038cb4(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L038cb4:
        eori.b  #0x8,0x12(a6)                   | +044
        andi.b  #0xfe,0x46(a6)                  | +04a
        jsr     0x27bc8.l                       | +050
        bcc.w   .L038cd0                        | +056
        lea     TaskHandler_038cee(pc),a1       | +05a
        move.l  a1,(a6)                         | +05e
.L038cd0:
        jsr     0x28d70.l                       | +060
        movea.l #0xffffffff,a0                  | +066
        jsr     0x5dd56.l                       | +06c
        bcc.w   .L038cec                        | +072
        lea     TaskHandler_038cee(pc),a1       | +076
        move.l  a1,(a6)                         | +07a
.L038cec:
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  TaskHandler_038cee  @ $038CEE  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038cee, "ax", @progbits
        .global TaskHandler_038cee
TaskHandler_038cee:
        jmp     0x518.l                         | +000

| ----------------------------------------------------------------------------
|  TaskHandler_038cf4  @ $038CF4  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038cf4, "ax", @progbits
        .global TaskHandler_038cf4
TaskHandler_038cf4:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Sub_00038CF6  @ $038CF6  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00038CF6, "ax", @progbits
        .global Sub_00038CF6
Sub_00038CF6:
        move.w  #0x18d,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0xffff.w,a0                     | +00a
        move.l  a0,0x48(a6)                     | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_038d08  @ $038D08  (128 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038d08, "ax", @progbits
        .global TaskHandler_038d08
TaskHandler_038d08:
        lea     .L038d0e(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L038d0e:
        jsr     0x5e4ee.l                       | +006
        subi.w  #0x20,0x38(a6)                  | +00c
        clr.l   d0                              | +012
        jsr     Sub_00038E9A(pc)                | +014
        movea.l #0x28f8d4,a0                    | +018
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a0                    | +020
        cmpa.l  #0xffffffff,a0                  | +024
        beq.w   .L038d3c                        | +02a
        jsr     0x28cd4.l                       | +02e
.L038d3c:
        jsr     0x28d70.l                       | +034
        jsr     0x28758.l                       | +03a
        bcc.w   .L038d52                        | +040
        lea     TaskHandler_038e4a(pc),a1       | +044
        move.l  a1,(a6)                         | +048
.L038d52:
        movea.l 0xc(a6),a0                      | +04a
        btst    #0x2,0x8c(a0)                   | +04e
        beq.w   .L038d66                        | +054
        lea     TaskHandler_038d88(pc),a1       | +058
        move.l  a1,(a6)                         | +05c
.L038d66:
        cmpi.b  #0x4,0x70(a0)                   | +05e
        beq.w   .L038d76                        | +064
        lea     TaskHandler_038e10(pc),a1       | +068
        move.l  a1,(a6)                         | +06c
.L038d76:
        btst    #0x0,0x13(a0)                   | +06e
        beq.w   .L038d86                        | +074
        lea     TaskHandler_038e4a(pc),a1       | +078
        move.l  a1,(a6)                         | +07c
.L038d86:
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_038d88  @ $038D88  (136 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038d88, "ax", @progbits
        .global TaskHandler_038d88
TaskHandler_038d88:
        clr.l   d0                              | +000
        jsr     Sub_00038E9A(pc)                | +002
        movea.l #0x28fdbc,a0                    | +006
        lsl.w   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a0                    | +00e
        cmpa.l  #0xffffffff,a0                  | +012
        beq.w   .L038daa                        | +018
        jsr     0x28cd4.l                       | +01c
.L038daa:
        lea     .L038db0(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L038db0:
        jsr     0x5e4ee.l                       | +028
        subq.w  #0x1,0x38(a6)                   | +02e
        jsr     0x28d70.l                       | +032
        bcc.w   .L038dca                        | +038
        lea     TaskHandler_038d08(pc),a1       | +03c
        move.l  a1,(a6)                         | +040
.L038dca:
        jsr     0x28758.l                       | +042
        bcc.w   .L038dda                        | +048
        lea     TaskHandler_038e4a(pc),a1       | +04c
        move.l  a1,(a6)                         | +050
.L038dda:
        movea.l 0xc(a6),a0                      | +052
        cmpi.b  #0x4,0x70(a0)                   | +056
        bne.w   .L038dfe                        | +05c
        lea     TaskHandler_038e10(pc),a1       | +060
        move.l  a1,(a6)                         | +064
        cmpi.b  #0x26,0x70(a0)                  | +066
        bne.w   .L038dfe                        | +06c
        lea     TaskHandler_038e4a(pc),a1       | +070
        move.l  a1,(a6)                         | +074
.L038dfe:
        btst    #0x0,0x13(a0)                   | +076
        beq.w   .L038e0e                        | +07c
        lea     TaskHandler_038e4a(pc),a1       | +080
        move.l  a1,(a6)                         | +084
.L038e0e:
        rts                                     | +086

| ----------------------------------------------------------------------------
|  TaskHandler_038e10  @ $038E10  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038e10, "ax", @progbits
        .global TaskHandler_038e10
TaskHandler_038e10:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x3a(a0),d0                     | +004
        eori.b  #0x1,d0                         | +008
        move.b  d0,0x3a(a6)                     | +00c
        lea     0x28fe2c.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L038e32(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L038e32:
        jsr     0x5e4ee.l                       | +022
        jsr     0x28d70.l                       | +028
        bcc.w   .L038e48                        | +02e
        jmp     0x518.l                         | +032
.L038e48:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  TaskHandler_038e4a  @ $038E4A  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038e4a, "ax", @progbits
        .global TaskHandler_038e4a
TaskHandler_038e4a:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x3a(a0),0x3a(a6)               | +004
        move.b  #0x0,0x71(a6)                   | +00a
        lea     Data_038ec8(pc),a0              | +010
        jsr     0x28cd4.l                       | +014
        lea     .L038e6a(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L038e6a:
        jsr     0x2783a.l                       | +020
        cmpi.b  #0x0,0x71(a6)                   | +026
        beq.w   .L038e88                        | +02c
        move.b  0x106f28.l,d0                   | +030
        btst    #0x0,d0                         | +036
        bne.w   .L038e8e                        | +03a
.L038e88:
        jsr     0x28d70.l                       | +03e
.L038e8e:
        bcc.w   .L038e98                        | +044
        jmp     0x518.l                         | +048
.L038e98:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Sub_00038E9A  @ $038E9A  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00038E9A, "ax", @progbits
        .global Sub_00038E9A
Sub_00038E9A:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x28(a0),d1                     | +004
        addi.w  #0x400,d1                       | +008
        move.w  #0x400,d0                       | +00c
        addi.w  #0x400,d0                       | +010
        move.w  d0,d2                           | +014
        asr.w   #0x5,d2                         | +016
        add.w   d2,d1                           | +018
        add.w   d2,d0                           | +01a
        sub.w   d1,d0                           | +01c
        lsr.w   #0x8,d0                         | +01e
        cmpi.w  #0xa,d0                         | +020
        bcs.w   .L038ec6                        | +024
        andi.w  #0x7,d0                         | +028
.L038ec6:
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  Data_038ec8  @ $038EC8  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Data_038ec8, "ax", @progbits
        .global Data_038ec8
Data_038ec8:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf596                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +012  (dato / opcode no decodificado)
        .dc.w   0xf5ba                        | +014  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0501                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0071                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +020  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf5de                        | +026  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +032  (dato / opcode no decodificado)
        .dc.w   0xf602                        | +034  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +040  (dato / opcode no decodificado)
        .dc.w   0xf626                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_038f12  @ $038F12  (52 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038f12, "ax", @progbits
        .global TaskHandler_038f12
TaskHandler_038f12:
        move.b  d0,d3                           | +000
        lea     TaskHandler_039010(pc),a1       | +002
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.b  0x98(a6),0x98(a0)               | +012
        lea     TaskHandler_039050(pc),a1       | +018
        jsr     0x4ae.l                         | +01c
        jsr     0x5dd02.l                       | +022
        move.b  0x98(a6),0x98(a0)               | +028
        jmp     0x518.l                         | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_038f46  @ $038F46  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038f46, "ax", @progbits
        .global TaskHandler_038f46
TaskHandler_038f46:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_038f48  @ $038F48  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038f48, "ax", @progbits
        .global TaskHandler_038f48
TaskHandler_038f48:
        lea     TaskHandler_039030(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  0x98(a6),0x98(a0)               | +010
        lea     TaskHandler_039070(pc),a1       | +016
        jsr     0x4ae.l                         | +01a
        jsr     0x5dd02.l                       | +020
        move.b  0x98(a6),0x98(a0)               | +026
        jmp     0x518.l                         | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_038f7a  @ $038F7A  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038f7a, "ax", @progbits
        .global TaskHandler_038f7a
TaskHandler_038f7a:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Data_038f7c  @ $038F7C  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Data_038f7c, "ax", @progbits
        .global Data_038f7c
Data_038f7c:
        .dc.w   0x0101                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +020  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_038f9e  @ $038F9E  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038f9e, "ax", @progbits
        .global TaskHandler_038f9e
TaskHandler_038f9e:
        .dc.w   0xffe0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_038fa8  @ $038FA8  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038fa8, "ax", @progbits
        .global TaskHandler_038fa8
TaskHandler_038fa8:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +004  (dato / opcode no decodificado)
        .dc.w   0x19a4                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +00a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_038fb4  @ $038FB4  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038fb4, "ax", @progbits
        .global TaskHandler_038fb4
TaskHandler_038fb4:
        .dc.w   0xfff4                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +002  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_038fbc  @ $038FBC  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_038fbc, "ax", @progbits
        .global TaskHandler_038fbc
TaskHandler_038fbc:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0x8f7c                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffe                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffe                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffe                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_039010  @ $039010  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039010, "ax", @progbits
        .global TaskHandler_039010
TaskHandler_039010:
        lea     TaskHandler_038fbc(pc),a0       | +000
        move.l  a0,0x48(a6)                     | +004
        lea     0x100440.l,a0                   | +008
        lea     TaskHandler_038fb4(pc),a1       | +00e
        bclr    #0x0,0x3a(a6)                   | +012
        move.w  #0x2,d1                         | +018
        bra.w   TaskHandler_039070__L039090     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_039030  @ $039030  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039030, "ax", @progbits
        .global TaskHandler_039030
TaskHandler_039030:
        lea     TaskHandler_038fbc(pc),a0       | +000
        move.l  a0,0x48(a6)                     | +004
        lea     0x100440.l,a0                   | +008
        lea     TaskHandler_038fb4(pc),a1       | +00e
        bset    #0x0,0x3a(a6)                   | +012
        move.w  #0x1,d1                         | +018
        bra.w   TaskHandler_039070__L039090     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_039050  @ $039050  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039050, "ax", @progbits
        .global TaskHandler_039050
TaskHandler_039050:
        lea     TaskHandler_038fbc(pc),a0       | +000
        move.l  a0,0x48(a6)                     | +004
        lea     0x1004e0.l,a0                   | +008
        lea     TaskHandler_038fb4(pc),a1       | +00e
        bclr    #0x0,0x3a(a6)                   | +012
        move.w  #0x2,d1                         | +018
        bra.w   TaskHandler_039070__L039090     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_039070  @ $039070  (112 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039070, "ax", @progbits
        .global TaskHandler_039070
TaskHandler_039070:
        lea     TaskHandler_038fbc(pc),a0       | +000
        move.l  a0,0x48(a6)                     | +004
        lea     0x1004e0.l,a0                   | +008
        lea     TaskHandler_038fb4(pc),a1       | +00e
        bset    #0x0,0x3a(a6)                   | +012
        move.w  #0x1,d1                         | +018
        bra.w   TaskHandler_039070__L039090     | +01c
        .global TaskHandler_039070__L039090
TaskHandler_039070__L039090:
        move.l  a0,0x70(a6)                     | +020
        move.l  a1,0x74(a6)                     | +024
        jsr     0x236e.l                        | +028
        lea     TaskHandler_038fa8(pc),a0       | +02e
        jsr     0x28cd4.l                       | +032
        jsr     0x267e2.l                       | +038
        cmpi.b  #0x1,0x98(a6)                   | +03e
        bne.w   .L0390d6                        | +044
        jsr     0x5e7c0.l                       | +048
        bcs.w   .L0390d6                        | +04e
        addq.w  #0x3,0x24(a6)                   | +052
        move.w  #0xfc00,0x2a(a6)                | +056
        lea     TaskHandler_0390e0(pc),a1       | +05c
        move.l  a1,(a6)                         | +060
        bra.w   TaskHandler_0390e0              | +062
.L0390d6:
        lea     TaskHandler_0390f0(pc),a1       | +066
        move.l  a1,(a6)                         | +06a
        bra.w   TaskHandler_0390f0              | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_0390e0  @ $0390E0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0390e0, "ax", @progbits
        .global TaskHandler_0390e0
TaskHandler_0390e0:
        jsr     0x27bc8.l                       | +000
        bcc.w   TaskHandler_0390f0              | +006
        lea     TaskHandler_0390f0(pc),a1       | +00a
        move.l  a1,(a6)                         | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_0390f0  @ $0390F0  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0390f0, "ax", @progbits
        .global TaskHandler_0390f0
TaskHandler_0390f0:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        movea.l #0xffffffff,a0                  | +00c
        lea     TaskHandler_038f9e(pc),a0       | +012
        jsr     0x5dd5c.l                       | +016
        bcc.w   .L039118                        | +01c
        jmp     0x518.l                         | +020
        rts                                     | +026
.L039118:
        movea.l 0x70(a6),a0                     | +028
        move.b  0x3a(a6),d0                     | +02c
        move.b  0x3a(a0),d1                     | +030
        eor.b   d1,d0                           | +034
        btst    #0x0,d0                         | +036
        bne.w   .L039146                        | +03a
        movea.l 0x74(a6),a1                     | +03e
        jsr     0x5e260.l                       | +042
        bcs.w   .L039146                        | +048
        movea.l 0x70(a6),a0                     | +04c
        bset    #0x0,0x88(a0)                   | +050
.L039146:
        rts                                     | +056

| ----------------------------------------------------------------------------
|  Sub_00039148  @ $039148  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00039148, "ax", @progbits
        .global Sub_00039148
Sub_00039148:
        move.w  #0x185,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x27a182.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x0,0x5c(a6)                   | +016
        lea     .L03916a(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L03916a:
        jsr     0x5e4ca.l                       | +022
        move.w  d1,0x22(a6)                     | +028
        move.w  d2,0x24(a6)                     | +02c
        andi.w  #0xffe3,d0                      | +030
        or.w    0x5c(a6),d0                     | +034
        move.w  d0,0x38(a6)                     | +038
        jsr     0x28d70.l                       | +03c
        bcc.w   .L039194                        | +042
        lea     TaskHandler_0391aa(pc),a1       | +046
        move.l  a1,(a6)                         | +04a
.L039194:
        movea.l 0xc(a6),a0                      | +04c
        btst    #0x1,0x8d(a0)                   | +050
        bne.w   .L0391a8                        | +056
        lea     TaskHandler_0391aa(pc),a1       | +05a
        move.l  a1,(a6)                         | +05e
.L0391a8:
        rts                                     | +060

| ----------------------------------------------------------------------------
|  TaskHandler_0391aa  @ $0391AA  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0391aa, "ax", @progbits
        .global TaskHandler_0391aa
TaskHandler_0391aa:
        lea     0x27a228.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.w  #0x0,0x5c(a6)                   | +00c
        lea     TaskHandler_0391aa__L0391c2(pc),a1 | +012
        move.l  a1,(a6)                         | +016
        .global TaskHandler_0391aa__L0391c2
TaskHandler_0391aa__L0391c2:
        jsr     0x5e4ca.l                       | +018
        move.w  d1,0x22(a6)                     | +01e
        move.w  d2,0x24(a6)                     | +022
        andi.w  #0xffe3,d0                      | +026
        or.w    0x5c(a6),d0                     | +02a
        move.w  d0,0x38(a6)                     | +02e
        jsr     0x28d70.l                       | +032
        bcc.w   .L0391ec                        | +038
        jmp     0x518.l                         | +03c
.L0391ec:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Sub_000391EE  @ $0391EE  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000391EE, "ax", @progbits
        .global Sub_000391EE
Sub_000391EE:
        move.w  #0x185,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x27a0fe.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x1c,0x5c(a6)                  | +016
        lea     .L039210(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L039210:
        jmp     TaskHandler_0391aa__L0391c2(pc) | +022

| ----------------------------------------------------------------------------
|  Sub_00039214  @ $039214  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00039214, "ax", @progbits
        .global Sub_00039214
Sub_00039214:
        move.w  #0x1df,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x279f9a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L039230(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L039230:
        jmp     TaskHandler_0391aa__L0391c2(pc) | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_039234  @ $039234  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039234, "ax", @progbits
        .global TaskHandler_039234
TaskHandler_039234:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   .L03924a                        | +00c
        andi.b  #0xee,ccr                       | +010
        rts                                     | +014
.L03924a:
        ori.b   #0x11,ccr                       | +016
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Data_039250  @ $039250  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039250, "ax", @progbits
        .global Data_039250
Data_039250:
        .dc.w   0x030b                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +00c  (dato / opcode no decodificado)
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
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_0392a4  @ $0392A4  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0392a4, "ax", @progbits
        .global TaskHandler_0392a4
TaskHandler_0392a4:
        .dc.w   0x030a                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0032                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_0392f8  @ $0392F8  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0392f8, "ax", @progbits
        .global TaskHandler_0392f8
TaskHandler_0392f8:
        move.w  0x16(a6),0x14(a6)               | +000
        bset    #0x6,0x6b(a6)                   | +006
        move.b  #0xff,0x32(a6)                  | +00c
        move.b  #0xff,0x33(a6)                  | +012
        jsr     0x5e98a.l                       | +018
        clr.b   0x87(a1)                        | +01e
        clr.b   0x8c(a6)                        | +022
        clr.w   0x72(a6)                        | +026
        clr.w   0x36(a6)                        | +02a
        clr.b   0x91(a6)                        | +02e
        bclr    #0x3,0x13(a6)                   | +032
        lea     0xffff.w,a0                     | +038
        move.l  a0,0x48(a6)                     | +03c
        rts                                     | +040

| ----------------------------------------------------------------------------
|  Sub_0003933A  @ $03933A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0003933A, "ax", @progbits
        .global Sub_0003933A
Sub_0003933A:
        movea.l 0xc(a6),a0                      | +000
        moveq   #0,d0                           | +004
        move.b  0x71(a0),d0                     | +006
        cmpa.l  #0x1004e0,a0                    | +00a
        bne.w   .L039350                        | +010
        addq.w  #0x5,d0                         | +014
.L039350:
        lsl.w   #0x2,d0                         | +016
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Sub_00039354  @ $039354  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00039354, "ax", @progbits
        .global Sub_00039354
Sub_00039354:
        jsr     0x28d70.l                       | +000
        bcc.w   .L03936c                        | +006
        movea.l 0xc(a6),a0                      | +00a
        clr.b   0x21(a0)                        | +00e
        ori.b   #0x11,ccr                       | +012
        rts                                     | +016
.L03936c:
        movea.l 0xc(a6),a0                      | +018
        move.b  #0xff,0x21(a0)                  | +01c
        andi.b  #0xee,ccr                       | +022
        rts                                     | +026

| ----------------------------------------------------------------------------
|  Sub_0003937C  @ $03937C  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0003937C, "ax", @progbits
        .global Sub_0003937C
Sub_0003937C:
        jsr     0x5e4b2.l                       | +000
        move.w  0x7c(a0),d1                     | +006
        move.w  0x7e(a0),d2                     | +00a
        move.b  0x3a(a0),d3                     | +00e
        btst    #0x0,d3                         | +012
        bne.w   .L03939e                        | +016
        add.w   d1,0x22(a6)                     | +01a
        bra.w   .L0393a2                        | +01e
.L03939e:
        sub.w   d1,0x22(a6)                     | +022
.L0393a2:
        add.w   d2,0x24(a6)                     | +026
        move.b  d3,0x3a(a6)                     | +02a
        btst    #0x7,0x5a(a0)                   | +02e
        beq.w   .L0393ba                        | +034
        bset    #0x7,0x5a(a6)                   | +038
.L0393ba:
        andi.w  #0xfffc,0x38(a6)                | +03e
        move.b  0x59(a0),0x59(a6)               | +044
        jsr     0x32d00.l                       | +04a
        rts                                     | +050

| ----------------------------------------------------------------------------
|  TaskHandler_0393ce  @ $0393CE  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0393ce, "ax", @progbits
        .global TaskHandler_0393ce
TaskHandler_0393ce:
        jmp     0x5cf04.l                       | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0393d4  @ $0393D4  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0393d4, "ax", @progbits
        .global TaskHandler_0393d4
TaskHandler_0393d4:
        jmp     0x5cf10.l                       | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0393da  @ $0393DA  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0393da, "ax", @progbits
        .global TaskHandler_0393da
TaskHandler_0393da:
        jmp     TaskHandler_0393de(pc)          | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0393de  @ $0393DE  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0393de, "ax", @progbits
        .global TaskHandler_0393de
TaskHandler_0393de:
        jsr     0x5cf84.l                       | +000
        bcc.w   .L0393ea                        | +006
        rts                                     | +00a
.L0393ea:
        jsr     0x5ceec.l                       | +00c
        bcc.w   .L0393fa                        | +012
        jmp     0x5cf9c.l                       | +016
.L0393fa:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_0393fc  @ $0393FC  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0393fc, "ax", @progbits
        .global TaskHandler_0393fc
TaskHandler_0393fc:
        jsr     Sub_00039410(pc)                | +000
        bcc.w   .L03940a                        | +004
        ori.b   #0x11,ccr                       | +008
        rts                                     | +00c
.L03940a:
        andi.b  #0xee,ccr                       | +00e
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Sub_00039410  @ $039410  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00039410, "ax", @progbits
        .global Sub_00039410
Sub_00039410:
        jmp     0x5cef8.l                       | +000

| ----------------------------------------------------------------------------
|  PcThunkTarget_039416  @ $039416  (24 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_039416, "ax", @progbits
        .global PcThunkTarget_039416
PcThunkTarget_039416:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x8c(a6),d0                     | +004
        andi.b  #0x4,d0                         | +008
        andi.b  #0xfb,0x8c(a0)                  | +00c
        or.b    d0,0x8c(a0)                     | +012
        rts                                     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_03942e  @ $03942E  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03942e, "ax", @progbits
        .global TaskHandler_03942e
TaskHandler_03942e:
        jsr     0x283ca.l                       | +000
        jsr     0x283d8.l                       | +006
        btst    #0x2,0x5a(a6)                   | +00c
        beq.w   .L03944e                        | +012
        move.w  #0x10fb,d0                      | +016
        jsr     0x2352.l                        | +01a
.L03944e:
        rts                                     | +020

| ----------------------------------------------------------------------------
|  TaskHandler_039450  @ $039450  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039450, "ax", @progbits
        .global TaskHandler_039450
TaskHandler_039450:
        jsr     0x5e9b6.l                       | +000
        andi.b  #0xa,d0                         | +006
        move.b  d0,0x47(a6)                     | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_039460  @ $039460  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039460, "ax", @progbits
        .global TaskHandler_039460
TaskHandler_039460:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_03947c(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_03947c__L0394a4            | +018

| ----------------------------------------------------------------------------
|  Data_03947c  @ $03947C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03947c, "ax", @progbits
        .global Data_03947c
Data_03947c:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaf9a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xaf9a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +026  (dato / opcode no decodificado)
        .global Data_03947c__L0394a4
Data_03947c__L0394a4:
        bra.w   TaskTpl_0394A8__L03960e         | +028

| ----------------------------------------------------------------------------
|  TaskTpl_0394A8  @ $0394A8  (1024 B)
| ----------------------------------------------------------------------------
        .section .text.TaskTpl_0394A8, "ax", @progbits
        .global TaskTpl_0394A8
TaskTpl_0394A8:
        move.w  #0x176,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x190,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x191,d1                       | +014
        jsr     0x236e.l                        | +018
        move.w  #0x1b,0x1c(a6)                  | +01e
        jsr     0x138fe.l                       | +024
        jsr     TaskHandler_0392f8(pc)          | +02a
        bclr    #0x4,0x12(a6)                   | +02e
        ori.w   #0x2,0x38(a6)                   | +034
        bra.w   .L039520                        | +03a
        move.w  #0x177,d1                       | +03e
        jsr     0x236e.l                        | +042
        move.w  #0x190,d1                       | +048
        jsr     0x236e.l                        | +04c
        move.w  #0x192,d1                       | +052
        jsr     0x236e.l                        | +056
        move.w  #0x1c,0x1c(a6)                  | +05c
        jsr     0x138fe.l                       | +062
        jsr     TaskHandler_0392f8(pc)          | +068
        bset    #0x4,0x12(a6)                   | +06c
        ori.w   #0x0,0x38(a6)                   | +072
.L039520:
        lea     0x27aaea.l,a0                   | +078
        jsr     0x28cd4.l                       | +07e
        lea     .L039532(pc),a1                 | +084
        move.l  a1,(a6)                         | +088
.L039532:
        jsr     Sub_0003937C(pc)                | +08a
        movea.l 0xc(a6),a0                      | +08e
        move.b  0x70(a0),d0                     | +092
        cmpi.b  #0x10,d0                        | +096
        beq.w   .L039644                        | +09a
        cmpi.b  #0x21,d0                        | +09e
        beq.w   .L03961c                        | +0a2
        cmpi.b  #0x11,d0                        | +0a6
        beq.w   .L0396d8                        | +0aa
        cmpi.b  #0x22,d0                        | +0ae
        beq.w   .L0396b0                        | +0b2
        cmpi.b  #0x12,d0                        | +0b6
        beq.w   .L03976c                        | +0ba
        cmpi.b  #0x23,d0                        | +0be
        beq.w   .L039744                        | +0c2
        cmpi.b  #0x25,d0                        | +0c6
        beq.w   .L03984a                        | +0ca
        cmpi.b  #0x27,d0                        | +0ce
        beq.w   .L039842                        | +0d2
        cmpi.b  #0x26,d0                        | +0d6
        beq.w   .L0397e4                        | +0da
        cmpi.b  #0x28,d0                        | +0de
        beq.w   .L0397dc                        | +0e2
        cmpi.b  #0x30,d0                        | +0e6
        beq.w   .L0395ee                        | +0ea
        cmpi.b  #0x31,d0                        | +0ee
        beq.w   .L0395ee                        | +0f2
        cmpi.b  #0x1,d0                         | +0f6
        beq.w   .L0395ee                        | +0fa
        cmpi.b  #0x32,d0                        | +0fe
        beq.w   .L0395ee                        | +102
        cmpi.b  #0x2,d0                         | +106
        beq.w   .L0395ee                        | +10a
        cmpi.b  #0x3,d0                         | +10e
        beq.w   .L0395ee                        | +112
        cmpi.b  #0x20,d0                        | +116
        beq.w   .L0395ee                        | +11a
        cmpi.b  #0x4,d0                         | +11e
        beq.w   .L0395ee                        | +122
        cmpi.b  #0x0,d0                         | +126
        beq.w   .L0395ee                        | +12a
        cmpi.b  #0x34,d0                        | +12e
        bcs.w   .L0395ea                        | +132
        nop                                     | +136
        nop                                     | +138
        cmpi.b  #0x34,d0                        | +13a
        nop                                     | +13e
        trap    #0xf                            | +140
.L0395ea:
        bra.w   .L0395ee                        | +142
.L0395ee:
        movea.l 0xc(a6),a0                      | +146
        btst    #0x5,0x69(a0)                   | +14a
        beq.w   TaskTpl_0394A8__L03960e         | +150
        movea.l 0x74(a0),a1                     | +154
        jsr     (a1)                            | +158
        movea.l 0xc(a6),a0                      | +15a
        move.w  0x72(a0),d0                     | +15e
        move.w  d0,0x72(a6)                     | +162
        .global TaskTpl_0394A8__L03960e
TaskTpl_0394A8__L03960e:
        clr.w   0x72(a6)                        | +166
        jsr     Sub_00039354(pc)                | +16a
        jsr     PcThunkTarget_039416(pc)        | +16e
        rts                                     | +172
.L03961c:
        movea.l 0xc(a6),a0                      | +174
        btst    #0x5,0x69(a0)                   | +178
        beq.w   .L039640                        | +17e
        movea.l 0x74(a0),a1                     | +182
        jsr     (a1)                            | +186
        movea.l 0xc(a6),a0                      | +188
        move.w  0x72(a0),d0                     | +18c
        move.w  d0,0x72(a6)                     | +190
        bra.w   .L0396a6                        | +194
.L039640:
        bra.w   .L0396a6                        | +198
.L039644:
        movea.l 0xc(a6),a0                      | +19c
        btst    #0x2,0x8c(a6)                   | +1a0
        beq.w   .L039656                        | +1a6
        bra.w   .L039676                        | +1aa
.L039656:
        btst    #0x5,0x69(a0)                   | +1ae
        beq.w   .L039676                        | +1b4
        movea.l 0x74(a0),a1                     | +1b8
        jsr     (a1)                            | +1bc
        movea.l 0xc(a6),a0                      | +1be
        move.w  0x72(a0),d0                     | +1c2
        move.w  d0,0x72(a6)                     | +1c6
        bra.w   .L0396a6                        | +1ca
.L039676:
        movea.l 0x74(a0),a1                     | +1ce
        cmpa.l  #0x39e54,a1                     | +1d2
        beq.b   .L03968c                        | +1d8
        cmpa.l  #0x39f56,a1                     | +1da
        bne.w   .L0396a6                        | +1e0
.L03968c:
        move.w  0x72(a0),d0                     | +1e4
        cmp.w   0x72(a6),d0                     | +1e8
        beq.w   .L0396a6                        | +1ec
        move.w  d0,0x72(a6)                     | +1f0
        movea.l 0x74(a0),a1                     | +1f4
        jsr     (a1)                            | +1f8
        bra.w   .L0396a6                        | +1fa
.L0396a6:
        jsr     Sub_00039354(pc)                | +1fe
        jsr     PcThunkTarget_039416(pc)        | +202
        rts                                     | +206
.L0396b0:
        movea.l 0xc(a6),a0                      | +208
        btst    #0x5,0x69(a0)                   | +20c
        beq.w   .L0396d4                        | +212
        movea.l 0x74(a0),a1                     | +216
        jsr     (a1)                            | +21a
        movea.l 0xc(a6),a0                      | +21c
        move.w  0x72(a0),d0                     | +220
        move.w  d0,0x72(a6)                     | +224
        bra.w   .L03973a                        | +228
.L0396d4:
        bra.w   .L03973a                        | +22c
.L0396d8:
        movea.l 0xc(a6),a0                      | +230
        btst    #0x2,0x8c(a6)                   | +234
        beq.w   .L0396ea                        | +23a
        bra.w   .L03970a                        | +23e
.L0396ea:
        btst    #0x5,0x69(a0)                   | +242
        beq.w   .L03970a                        | +248
        movea.l 0x74(a0),a1                     | +24c
        jsr     (a1)                            | +250
        movea.l 0xc(a6),a0                      | +252
        move.w  0x72(a0),d0                     | +256
        move.w  d0,0x72(a6)                     | +25a
        bra.w   .L03973a                        | +25e
.L03970a:
        movea.l 0x74(a0),a1                     | +262
        cmpa.l  #0x3a058,a1                     | +266
        beq.b   .L039720                        | +26c
        cmpa.l  #0x3a108,a1                     | +26e
        bne.w   .L03973a                        | +274
.L039720:
        move.w  0x72(a0),d0                     | +278
        cmp.w   0x72(a6),d0                     | +27c
        beq.w   .L03973a                        | +280
        move.w  d0,0x72(a6)                     | +284
        movea.l 0x74(a0),a1                     | +288
        jsr     (a1)                            | +28c
        bra.w   .L03973a                        | +28e
.L03973a:
        jsr     Sub_00039354(pc)                | +292
        jsr     PcThunkTarget_039416(pc)        | +296
        rts                                     | +29a
.L039744:
        movea.l 0xc(a6),a0                      | +29c
        btst    #0x5,0x69(a0)                   | +2a0
        beq.w   .L039768                        | +2a6
        movea.l 0x74(a0),a1                     | +2aa
        jsr     (a1)                            | +2ae
        movea.l 0xc(a6),a0                      | +2b0
        move.w  0x72(a0),d0                     | +2b4
        move.w  d0,0x72(a6)                     | +2b8
        bra.w   .L0397d2                        | +2bc
.L039768:
        bra.w   .L0397d2                        | +2c0
.L03976c:
        movea.l 0xc(a6),a0                      | +2c4
        btst    #0x2,0x8c(a6)                   | +2c8
        beq.w   .L03977e                        | +2ce
        bra.w   .L0397a2                        | +2d2
.L03977e:
        movea.l 0xc(a6),a0                      | +2d6
        btst    #0x5,0x69(a0)                   | +2da
        beq.w   .L0397a2                        | +2e0
        movea.l 0x74(a0),a1                     | +2e4
        jsr     (a1)                            | +2e8
        movea.l 0xc(a6),a0                      | +2ea
        move.w  0x72(a0),d0                     | +2ee
        move.w  d0,0x72(a6)                     | +2f2
        bra.w   .L0397d2                        | +2f6
.L0397a2:
        movea.l 0x74(a0),a1                     | +2fa
        cmpa.l  #0x3a1b8,a1                     | +2fe
        beq.b   .L0397b8                        | +304
        cmpa.l  #0x3a268,a1                     | +306
        bne.w   .L0397d2                        | +30c
.L0397b8:
        move.w  0x72(a0),d0                     | +310
        cmp.w   0x72(a6),d0                     | +314
        beq.w   .L0397d2                        | +318
        move.w  d0,0x72(a6)                     | +31c
        movea.l 0x74(a0),a1                     | +320
        jsr     (a1)                            | +324
        bra.w   .L0397d2                        | +326
.L0397d2:
        jsr     Sub_00039354(pc)                | +32a
        jsr     PcThunkTarget_039416(pc)        | +32e
        rts                                     | +332
.L0397dc:
        movea.l 0xc(a6),a0                      | +334
        bra.w   .L0397e4                        | +338
.L0397e4:
        movea.l 0xc(a6),a0                      | +33c
        btst    #0x5,0x69(a0)                   | +340
        beq.w   .L039808                        | +346
        movea.l 0x74(a0),a1                     | +34a
        jsr     (a1)                            | +34e
        movea.l 0xc(a6),a0                      | +350
        move.w  0x72(a0),d0                     | +354
        move.w  d0,0x72(a6)                     | +358
        bra.w   .L039838                        | +35c
.L039808:
        movea.l 0x74(a0),a1                     | +360
        cmpa.l  #0x3a656,a1                     | +364
        bne.w   .L039838                        | +36a
        move.w  0x72(a0),d0                     | +36e
        cmp.w   0x72(a6),d0                     | +372
        beq.w   .L039838                        | +376
        movem.w d0,-(a7)                        | +37a
        movea.l 0x74(a0),a1                     | +37e
        jsr     (a1)                            | +382
        movem.w (a7)+,d0                        | +384
        move.w  d0,0x72(a6)                     | +388
        bra.w   .L039838                        | +38c
.L039838:
        jsr     Sub_00039354(pc)                | +390
        jsr     PcThunkTarget_039416(pc)        | +394
        rts                                     | +398
.L039842:
        movea.l 0xc(a6),a0                      | +39a
        bra.w   .L03984a                        | +39e
.L03984a:
        movea.l 0xc(a6),a0                      | +3a2
        btst    #0x5,0x69(a0)                   | +3a6
        beq.w   .L03986e                        | +3ac
        movea.l 0x74(a0),a1                     | +3b0
        jsr     (a1)                            | +3b4
        movea.l 0xc(a6),a0                      | +3b6
        move.w  0x72(a0),d0                     | +3ba
        move.w  d0,0x72(a6)                     | +3be
        bra.w   .L03989e                        | +3c2
.L03986e:
        movea.l 0x74(a0),a1                     | +3c6
        cmpa.l  #0x3a60a,a1                     | +3ca
        bne.w   .L03989e                        | +3d0
        move.w  0x72(a0),d0                     | +3d4
        cmp.w   0x72(a6),d0                     | +3d8
        beq.w   .L03989e                        | +3dc
        movem.w d0,-(a7)                        | +3e0
        movea.l 0x74(a0),a1                     | +3e4
        jsr     (a1)                            | +3e8
        movem.w (a7)+,d0                        | +3ea
        move.w  d0,0x72(a6)                     | +3ee
        bra.w   .L03989e                        | +3f2
.L03989e:
        jsr     Sub_00039354(pc)                | +3f6
        jsr     PcThunkTarget_039416(pc)        | +3fa
        rts                                     | +3fe

| ----------------------------------------------------------------------------
|  TaskHandler_0398a8  @ $0398A8  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0398a8, "ax", @progbits
        .global TaskHandler_0398a8
TaskHandler_0398a8:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_0398ca(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_0398ca__L0398f2            | +01e

| ----------------------------------------------------------------------------
|  Data_0398ca  @ $0398CA  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_0398ca, "ax", @progbits
        .global Data_0398ca
Data_0398ca:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xa62e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xda6c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xda6c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xda6c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xda6c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xa62e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xda6c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xda6c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xda6c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xda6c                        | +026  (dato / opcode no decodificado)
        .global Data_0398ca__L0398f2
Data_0398ca__L0398f2:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_0398f4  @ $0398F4  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0398f4, "ax", @progbits
        .global TaskHandler_0398f4
TaskHandler_0398f4:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_039916(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_039916__L03993e            | +01e

| ----------------------------------------------------------------------------
|  Data_039916  @ $039916  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039916, "ax", @progbits
        .global Data_039916
Data_039916:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xa904                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd42                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdd42                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xdd42                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xdd42                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xa904                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xdd42                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xdd42                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xdd42                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xdd42                        | +026  (dato / opcode no decodificado)
        .global Data_039916__L03993e
Data_039916__L03993e:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039940  @ $039940  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039940, "ax", @progbits
        .global TaskHandler_039940
TaskHandler_039940:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_039962(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_039962__L03998a            | +01e

| ----------------------------------------------------------------------------
|  Data_039962  @ $039962  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039962, "ax", @progbits
        .global Data_039962
Data_039962:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaf9a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xaf9a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe588                        | +026  (dato / opcode no decodificado)
        .global Data_039962__L03998a
Data_039962__L03998a:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03998c  @ $03998C  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03998c, "ax", @progbits
        .global TaskHandler_03998c
TaskHandler_03998c:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_0399ae(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_0399ae__L0399d6            | +01e

| ----------------------------------------------------------------------------
|  Data_0399ae  @ $0399AE  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_0399ae, "ax", @progbits
        .global Data_0399ae
Data_0399ae:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe684                        | +026  (dato / opcode no decodificado)
        .global Data_0399ae__L0399d6
Data_0399ae__L0399d6:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_0399d8  @ $0399D8  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0399d8, "ax", @progbits
        .global TaskHandler_0399d8
TaskHandler_0399d8:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_0399fa(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_0399fa__L039a22            | +01e

| ----------------------------------------------------------------------------
|  Data_0399fa  @ $0399FA  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_0399fa, "ax", @progbits
        .global Data_0399fa
Data_0399fa:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb7d6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb7d6                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +026  (dato / opcode no decodificado)
        .global Data_0399fa__L039a22
Data_0399fa__L039a22:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039a24  @ $039A24  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039a24, "ax", @progbits
        .global TaskHandler_039a24
TaskHandler_039a24:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_039a46(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_039a46__L039a6e            | +01e

| ----------------------------------------------------------------------------
|  Data_039a46  @ $039A46  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039a46, "ax", @progbits
        .global Data_039a46
Data_039a46:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb7d6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb7d6                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xedfe                        | +026  (dato / opcode no decodificado)
        .global Data_039a46__L039a6e
Data_039a46__L039a6e:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039a70  @ $039A70  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039a70, "ax", @progbits
        .global TaskHandler_039a70
TaskHandler_039a70:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_039a8c(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_039a8c__L039ab4            | +018

| ----------------------------------------------------------------------------
|  Data_039a8c  @ $039A8C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039a8c, "ax", @progbits
        .global Data_039a8c
Data_039a8c:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbc22                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf32c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbc22                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf32c                        | +026  (dato / opcode no decodificado)
        .global Data_039a8c__L039ab4
Data_039a8c__L039ab4:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039ab6  @ $039AB6  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039ab6, "ax", @progbits
        .global TaskHandler_039ab6
TaskHandler_039ab6:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_039ad8(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_039ad8__L039b00            | +01e

| ----------------------------------------------------------------------------
|  Data_039ad8  @ $039AD8  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039ad8, "ax", @progbits
        .global Data_039ad8
Data_039ad8:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaec2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0a46                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x162c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x1fe6                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe460                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xaec2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0a46                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x162c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x1fe6                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe460                        | +026  (dato / opcode no decodificado)
        .global Data_039ad8__L039b00
Data_039ad8__L039b00:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039b02  @ $039B02  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039b02, "ax", @progbits
        .global TaskHandler_039b02
TaskHandler_039b02:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_039b24(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_039b24__L039b4c            | +01e

| ----------------------------------------------------------------------------
|  Data_039b24  @ $039B24  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039b24, "ax", @progbits
        .global Data_039b24
Data_039b24:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaef8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0a7c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1662                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x201c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe4aa                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xaef8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0a7c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1662                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x201c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe4aa                        | +026  (dato / opcode no decodificado)
        .global Data_039b24__L039b4c
Data_039b24__L039b4c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039b4e  @ $039B4E  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039b4e, "ax", @progbits
        .global TaskHandler_039b4e
TaskHandler_039b4e:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_039b6a(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_039b6a__L039b92            | +018

| ----------------------------------------------------------------------------
|  Data_039b6a  @ $039B6A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039b6a, "ax", @progbits
        .global Data_039b6a
Data_039b6a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbb20                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf1c4                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbb20                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf1c4                        | +026  (dato / opcode no decodificado)
        .global Data_039b6a__L039b92
Data_039b6a__L039b92:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039b94  @ $039B94  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039b94, "ax", @progbits
        .global TaskHandler_039b94
TaskHandler_039b94:
        movea.l 0xc(a6),a0                      | +000
        btst    #0x2,0x78(a0)                   | +004
        bne.w   Data_039bbe__L039bf0            | +00a
        bclr    #0x2,0x8c(a6)                   | +00e
        jsr     Sub_0003933A(pc)                | +014
        lea     Data_039bbe(pc),a0              | +018
        movea.l (a0,d0.w),a0                    | +01c
        jsr     0x28cd4.l                       | +020
        bra.w   Data_039bbe__L039be6            | +026

| ----------------------------------------------------------------------------
|  Data_039bbe  @ $039BBE  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039bbe, "ax", @progbits
        .global Data_039bbe
Data_039bbe:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaef8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0a7c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1662                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x201c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe4aa                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xaef8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0a7c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1662                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x201c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe4aa                        | +026  (dato / opcode no decodificado)
        .global Data_039bbe__L039be6
Data_039bbe__L039be6:
        bclr    #0x2,0x8c(a6)                   | +028
        bra.w   Data_039c64__L039c8c            | +02e
        .global Data_039bbe__L039bf0
Data_039bbe__L039bf0:
        bclr    #0x2,0x8c(a6)                   | +032
        jsr     0x2abcc.l                       | +038
        bcs.w   Data_039c1c__L039c48            | +03e
        bclr    #0x2,0x8c(a6)                   | +042
        jsr     Sub_0003933A(pc)                | +048
        lea     Data_039c1c(pc),a0              | +04c
        movea.l (a0,d0.w),a0                    | +050
        jsr     0x28cd4.l                       | +054
        bra.w   Data_039c1c__L039c44            | +05a

| ----------------------------------------------------------------------------
|  Data_039c1c  @ $039C1C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039c1c, "ax", @progbits
        .global Data_039c1c
Data_039c1c:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbc22                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf3dc                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbc22                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf3dc                        | +026  (dato / opcode no decodificado)
        .global Data_039c1c__L039c44
Data_039c1c__L039c44:
        bra.w   Data_039c64__L039c8c            | +028
        .global Data_039c1c__L039c48
Data_039c1c__L039c48:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     Sub_0003933A(pc)                | +032
        lea     Data_039c64(pc),a0              | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   Data_039c64__L039c8c            | +044

| ----------------------------------------------------------------------------
|  Data_039c64  @ $039C64  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039c64, "ax", @progbits
        .global Data_039c64
Data_039c64:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbd2a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4b6                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbd2a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4b6                        | +026  (dato / opcode no decodificado)
        .global Data_039c64__L039c8c
Data_039c64__L039c8c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039c8e  @ $039C8E  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039c8e, "ax", @progbits
        .global TaskHandler_039c8e
TaskHandler_039c8e:
        movea.l 0xc(a6),a0                      | +000
        btst    #0x2,0x78(a0)                   | +004
        beq.w   Data_039cbe__L039cea            | +00a
        bclr    #0x2,0x8c(a6)                   | +00e
        bclr    #0x2,0x8c(a6)                   | +014
        jsr     Sub_0003933A(pc)                | +01a
        lea     Data_039cbe(pc),a0              | +01e
        movea.l (a0,d0.w),a0                    | +022
        jsr     0x28cd4.l                       | +026
        bra.w   Data_039cbe__L039ce6            | +02c

| ----------------------------------------------------------------------------
|  Data_039cbe  @ $039CBE  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039cbe, "ax", @progbits
        .global Data_039cbe
Data_039cbe:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaec2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0a46                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x162c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x1fe6                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe460                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xaec2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0a46                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x162c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x1fe6                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe460                        | +026  (dato / opcode no decodificado)
        .global Data_039cbe__L039ce6
Data_039cbe__L039ce6:
        bra.w   Data_039d58__L039d80            | +028
        .global Data_039cbe__L039cea
Data_039cbe__L039cea:
        jsr     0x2abcc.l                       | +02c
        bcs.w   Data_039d10__L039d3c            | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     Sub_0003933A(pc)                | +03c
        lea     Data_039d10(pc),a0              | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   Data_039d10__L039d38            | +04e

| ----------------------------------------------------------------------------
|  Data_039d10  @ $039D10  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039d10, "ax", @progbits
        .global Data_039d10
Data_039d10:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbb20                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf24e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbb20                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf24e                        | +026  (dato / opcode no decodificado)
        .global Data_039d10__L039d38
Data_039d10__L039d38:
        bra.w   Data_039d58__L039d80            | +028
        .global Data_039d10__L039d3c
Data_039d10__L039d3c:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     Sub_0003933A(pc)                | +032
        lea     Data_039d58(pc),a0              | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   Data_039d58__L039d80            | +044

| ----------------------------------------------------------------------------
|  Data_039d58  @ $039D58  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039d58, "ax", @progbits
        .global Data_039d58
Data_039d58:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbd24                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4aa                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbd24                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4aa                        | +026  (dato / opcode no decodificado)
        .global Data_039d58__L039d80
Data_039d58__L039d80:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039d82  @ $039D82  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039d82, "ax", @progbits
        .global TaskHandler_039d82
TaskHandler_039d82:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_039d9e(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_039d9e__L039dc6            | +018

| ----------------------------------------------------------------------------
|  Data_039d9e  @ $039D9E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039d9e, "ax", @progbits
        .global Data_039d9e
Data_039d9e:
        .dc.w   0x0028                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0dbc                        | +026  (dato / opcode no decodificado)
        .global Data_039d9e__L039dc6
Data_039d9e__L039dc6:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039dc8  @ $039DC8  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039dc8, "ax", @progbits
        .global TaskHandler_039dc8
TaskHandler_039dc8:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_039de4(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_039de4__L039e0c            | +018

| ----------------------------------------------------------------------------
|  Data_039de4  @ $039DE4  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039de4, "ax", @progbits
        .global Data_039de4
Data_039de4:
        .dc.w   0x0028                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0e5c                        | +026  (dato / opcode no decodificado)
        .global Data_039de4__L039e0c
Data_039de4__L039e0c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039e0e  @ $039E0E  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039e0e, "ax", @progbits
        .global TaskHandler_039e0e
TaskHandler_039e0e:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_039e2a(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_039e2a__L039e52            | +018

| ----------------------------------------------------------------------------
|  Data_039e2a  @ $039E2A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039e2a, "ax", @progbits
        .global Data_039e2a
Data_039e2a:
        .dc.w   0x0028                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0efc                        | +026  (dato / opcode no decodificado)
        .global Data_039e2a__L039e52
Data_039e2a__L039e52:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039e54  @ $039E54  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039e54, "ax", @progbits
        .global TaskHandler_039e54
TaskHandler_039e54:
        bclr    #0x2,0x8c(a6)                   | +000
        movea.l 0xc(a6),a0                      | +006
        cmpi.w  #0x1,0x72(a0)                   | +00a
        bne.w   .L039e6c                        | +010
        jmp     TaskHandler_039f56(pc)          | +014
.L039e6c:
        cmpi.b  #0x41,0x79(a0)                  | +018
        bne.w   Data_039e92__L039ebe            | +01e
        bclr    #0x2,0x8c(a6)                   | +022
        jsr     Sub_0003933A(pc)                | +028
        lea     Data_039e92(pc),a0              | +02c
        movea.l (a0,d0.w),a0                    | +030
        jsr     0x28cd4.l                       | +034
        bra.w   Data_039e92__L039eba            | +03a

| ----------------------------------------------------------------------------
|  Data_039e92  @ $039E92  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039e92, "ax", @progbits
        .global Data_039e92
Data_039e92:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xae2c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe3ca                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe3ca                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe3ca                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe3ca                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xae2c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe3ca                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe3ca                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe3ca                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe3ca                        | +026  (dato / opcode no decodificado)
        .global Data_039e92__L039eba
Data_039e92__L039eba:
        bra.w   Data_039f2c__L039f54            | +028
        .global Data_039e92__L039ebe
Data_039e92__L039ebe:
        jsr     0x2abcc.l                       | +02c
        bcs.w   Data_039ee4__L039f10            | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     Sub_0003933A(pc)                | +03c
        lea     Data_039ee4(pc),a0              | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   Data_039ee4__L039f0c            | +04e

| ----------------------------------------------------------------------------
|  Data_039ee4  @ $039EE4  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039ee4, "ax", @progbits
        .global Data_039ee4
Data_039ee4:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaaea                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdfd8                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdfd8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xdfd8                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xdfd8                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xaaea                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xdfd8                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xdfd8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xdfd8                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xdfd8                        | +026  (dato / opcode no decodificado)
        .global Data_039ee4__L039f0c
Data_039ee4__L039f0c:
        bra.w   Data_039f2c__L039f54            | +028
        .global Data_039ee4__L039f10
Data_039ee4__L039f10:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     Sub_0003933A(pc)                | +032
        lea     Data_039f2c(pc),a0              | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   Data_039f2c__L039f54            | +044

| ----------------------------------------------------------------------------
|  Data_039f2c  @ $039F2C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039f2c, "ax", @progbits
        .global Data_039f2c
Data_039f2c:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb42c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb42c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +026  (dato / opcode no decodificado)
        .global Data_039f2c__L039f54
Data_039f2c__L039f54:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_039f56  @ $039F56  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_039f56, "ax", @progbits
        .global TaskHandler_039f56
TaskHandler_039f56:
        bclr    #0x2,0x8c(a6)                   | +000
        movea.l 0xc(a6),a0                      | +006
        cmpi.w  #0x0,0x72(a0)                   | +00a
        bne.w   .L039f6e                        | +010
        jmp     TaskHandler_039e54(pc)          | +014
.L039f6e:
        cmpi.b  #0x14,0x79(a0)                  | +018
        bne.w   Data_039f94__L039fc0            | +01e
        bclr    #0x2,0x8c(a6)                   | +022
        jsr     Sub_0003933A(pc)                | +028
        lea     Data_039f94(pc),a0              | +02c
        movea.l (a0,d0.w),a0                    | +030
        jsr     0x28cd4.l                       | +034
        bra.w   Data_039f94__L039fbc            | +03a

| ----------------------------------------------------------------------------
|  Data_039f94  @ $039F94  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039f94, "ax", @progbits
        .global Data_039f94
Data_039f94:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xadfa                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe398                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe398                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe398                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe398                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xadfa                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe398                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe398                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe398                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe398                        | +026  (dato / opcode no decodificado)
        .global Data_039f94__L039fbc
Data_039f94__L039fbc:
        bra.w   Data_03a02e__L03a056            | +028
        .global Data_039f94__L039fc0
Data_039f94__L039fc0:
        jsr     0x2abcc.l                       | +02c
        bcs.w   Data_039fe6__L03a012            | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     Sub_0003933A(pc)                | +03c
        lea     Data_039fe6(pc),a0              | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   Data_039fe6__L03a00e            | +04e

| ----------------------------------------------------------------------------
|  Data_039fe6  @ $039FE6  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_039fe6, "ax", @progbits
        .global Data_039fe6
Data_039fe6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xad6a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe308                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe308                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe308                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe308                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xad6a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe308                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe308                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe308                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe308                        | +026  (dato / opcode no decodificado)
        .global Data_039fe6__L03a00e
Data_039fe6__L03a00e:
        bra.w   Data_03a02e__L03a056            | +028
        .global Data_039fe6__L03a012
Data_039fe6__L03a012:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     Sub_0003933A(pc)                | +032
        lea     Data_03a02e(pc),a0              | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   Data_03a02e__L03a056            | +044

| ----------------------------------------------------------------------------
|  Data_03a02e  @ $03A02E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a02e, "ax", @progbits
        .global Data_03a02e
Data_03a02e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb53a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb53a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +026  (dato / opcode no decodificado)
        .global Data_03a02e__L03a056
Data_03a02e__L03a056:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a058  @ $03A058  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a058, "ax", @progbits
        .global TaskHandler_03a058
TaskHandler_03a058:
        bclr    #0x2,0x8c(a6)                   | +000
        movea.l 0xc(a6),a0                      | +006
        cmpi.w  #0x1,0x72(a0)                   | +00a
        bne.w   .L03a070                        | +010
        jmp     TaskHandler_03a108(pc)          | +014
.L03a070:
        cmpi.b  #0x41,0x79(a0)                  | +018
        bne.w   Data_03a096__L03a0c2            | +01e
        bclr    #0x2,0x8c(a6)                   | +022
        jsr     Sub_0003933A(pc)                | +028
        lea     Data_03a096(pc),a0              | +02c
        movea.l (a0,d0.w),a0                    | +030
        jsr     0x28cd4.l                       | +034
        bra.w   Data_03a096__L03a0be            | +03a

| ----------------------------------------------------------------------------
|  Data_03a096  @ $03A096  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a096, "ax", @progbits
        .global Data_03a096
Data_03a096:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xae90                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xae90                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +026  (dato / opcode no decodificado)
        .global Data_03a096__L03a0be
Data_03a096__L03a0be:
        bra.w   Data_03a0de__L03a106            | +028
        .global Data_03a096__L03a0c2
Data_03a096__L03a0c2:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     Sub_0003933A(pc)                | +032
        lea     Data_03a0de(pc),a0              | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   Data_03a0de__L03a106            | +044

| ----------------------------------------------------------------------------
|  Data_03a0de  @ $03A0DE  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a0de, "ax", @progbits
        .global Data_03a0de
Data_03a0de:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb368                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe990                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe990                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe990                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe990                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb368                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe990                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe990                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe990                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe990                        | +026  (dato / opcode no decodificado)
        .global Data_03a0de__L03a106
Data_03a0de__L03a106:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a108  @ $03A108  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a108, "ax", @progbits
        .global TaskHandler_03a108
TaskHandler_03a108:
        bclr    #0x2,0x8c(a6)                   | +000
        movea.l 0xc(a6),a0                      | +006
        cmpi.w  #0x0,0x72(a0)                   | +00a
        bne.w   .L03a120                        | +010
        jmp     TaskHandler_03a058(pc)          | +014
.L03a120:
        cmpi.b  #0x14,0x79(a0)                  | +018
        bne.w   Data_03a146__L03a172            | +01e
        bclr    #0x2,0x8c(a6)                   | +022
        jsr     Sub_0003933A(pc)                | +028
        lea     Data_03a146(pc),a0              | +02c
        movea.l (a0,d0.w),a0                    | +030
        jsr     0x28cd4.l                       | +034
        bra.w   Data_03a146__L03a16e            | +03a

| ----------------------------------------------------------------------------
|  Data_03a146  @ $03A146  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a146, "ax", @progbits
        .global Data_03a146
Data_03a146:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xae5e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xae5e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +026  (dato / opcode no decodificado)
        .global Data_03a146__L03a16e
Data_03a146__L03a16e:
        bra.w   Data_03a18e__L03a1b6            | +028
        .global Data_03a146__L03a172
Data_03a146__L03a172:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     Sub_0003933A(pc)                | +032
        lea     Data_03a18e(pc),a0              | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   Data_03a18e__L03a1b6            | +044

| ----------------------------------------------------------------------------
|  Data_03a18e  @ $03A18E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a18e, "ax", @progbits
        .global Data_03a18e
Data_03a18e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb3d2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe9fa                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe9fa                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe9fa                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe9fa                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb3d2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe9fa                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe9fa                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe9fa                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe9fa                        | +026  (dato / opcode no decodificado)
        .global Data_03a18e__L03a1b6
Data_03a18e__L03a1b6:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a1b8  @ $03A1B8  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a1b8, "ax", @progbits
        .global TaskHandler_03a1b8
TaskHandler_03a1b8:
        bclr    #0x2,0x8c(a6)                   | +000
        movea.l 0xc(a6),a0                      | +006
        cmpi.w  #0x1,0x72(a0)                   | +00a
        bne.w   .L03a1d0                        | +010
        jmp     TaskHandler_03a268(pc)          | +014
.L03a1d0:
        cmpi.b  #0x41,0x79(a0)                  | +018
        bne.w   Data_03a1f6__L03a222            | +01e
        bclr    #0x2,0x8c(a6)                   | +022
        jsr     Sub_0003933A(pc)                | +028
        lea     Data_03a1f6(pc),a0              | +02c
        movea.l (a0,d0.w),a0                    | +030
        jsr     0x28cd4.l                       | +034
        bra.w   Data_03a1f6__L03a21e            | +03a

| ----------------------------------------------------------------------------
|  Data_03a1f6  @ $03A1F6  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a1f6, "ax", @progbits
        .global Data_03a1f6
Data_03a1f6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xae90                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xae90                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe42e                        | +026  (dato / opcode no decodificado)
        .global Data_03a1f6__L03a21e
Data_03a1f6__L03a21e:
        bra.w   Data_03a23e__L03a266            | +028
        .global Data_03a1f6__L03a222
Data_03a1f6__L03a222:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     Sub_0003933A(pc)                | +032
        lea     Data_03a23e(pc),a0              | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   Data_03a23e__L03a266            | +044

| ----------------------------------------------------------------------------
|  Data_03a23e  @ $03A23E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a23e, "ax", @progbits
        .global Data_03a23e
Data_03a23e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb42c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb42c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xea54                        | +026  (dato / opcode no decodificado)
        .global Data_03a23e__L03a266
Data_03a23e__L03a266:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a268  @ $03A268  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a268, "ax", @progbits
        .global TaskHandler_03a268
TaskHandler_03a268:
        bclr    #0x2,0x8c(a6)                   | +000
        movea.l 0xc(a6),a0                      | +006
        cmpi.w  #0x0,0x72(a0)                   | +00a
        bne.w   .L03a280                        | +010
        jmp     TaskHandler_03a1b8(pc)          | +014
.L03a280:
        cmpi.b  #0x14,0x79(a0)                  | +018
        bne.w   Data_03a2a6__L03a2d2            | +01e
        bclr    #0x2,0x8c(a6)                   | +022
        jsr     Sub_0003933A(pc)                | +028
        lea     Data_03a2a6(pc),a0              | +02c
        movea.l (a0,d0.w),a0                    | +030
        jsr     0x28cd4.l                       | +034
        bra.w   Data_03a2a6__L03a2ce            | +03a

| ----------------------------------------------------------------------------
|  Data_03a2a6  @ $03A2A6  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a2a6, "ax", @progbits
        .global Data_03a2a6
Data_03a2a6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xae5e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xae5e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe3fc                        | +026  (dato / opcode no decodificado)
        .global Data_03a2a6__L03a2ce
Data_03a2a6__L03a2ce:
        bra.w   Data_03a2ee__L03a316            | +028
        .global Data_03a2a6__L03a2d2
Data_03a2a6__L03a2d2:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     Sub_0003933A(pc)                | +032
        lea     Data_03a2ee(pc),a0              | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   Data_03a2ee__L03a316            | +044

| ----------------------------------------------------------------------------
|  Data_03a2ee  @ $03A2EE  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a2ee, "ax", @progbits
        .global Data_03a2ee
Data_03a2ee:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb53a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb53a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xeb62                        | +026  (dato / opcode no decodificado)
        .global Data_03a2ee__L03a316
Data_03a2ee__L03a316:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a318  @ $03A318  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a318, "ax", @progbits
        .global TaskHandler_03a318
TaskHandler_03a318:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_03a334(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_03a334__L03a35c            | +018

| ----------------------------------------------------------------------------
|  Data_03a334  @ $03A334  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a334, "ax", @progbits
        .global Data_03a334
Data_03a334:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbd2a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4b0                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbd2a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4b0                        | +026  (dato / opcode no decodificado)
        .global Data_03a334__L03a35c
Data_03a334__L03a35c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a35e  @ $03A35E  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a35e, "ax", @progbits
        .global TaskHandler_03a35e
TaskHandler_03a35e:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_03a37a(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_03a37a__L03a3a2            | +018

| ----------------------------------------------------------------------------
|  Data_03a37a  @ $03A37A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a37a, "ax", @progbits
        .global Data_03a37a
Data_03a37a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbd24                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4a4                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbd24                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4a4                        | +026  (dato / opcode no decodificado)
        .global Data_03a37a__L03a3a2
Data_03a37a__L03a3a2:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a3a4  @ $03A3A4  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a3a4, "ax", @progbits
        .global TaskHandler_03a3a4
TaskHandler_03a3a4:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_03a3c6(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_03a3c6__L03a3ee            | +01e

| ----------------------------------------------------------------------------
|  Data_03a3c6  @ $03A3C6  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a3c6, "ax", @progbits
        .global Data_03a3c6
Data_03a3c6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaec2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0a46                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x162c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x1fe6                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe460                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xaec2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0a46                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x162c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x1fe6                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe460                        | +026  (dato / opcode no decodificado)
        .global Data_03a3c6__L03a3ee
Data_03a3c6__L03a3ee:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a3f0  @ $03A3F0  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a3f0, "ax", @progbits
        .global TaskHandler_03a3f0
TaskHandler_03a3f0:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_03a412(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_03a412__L03a43a            | +01e

| ----------------------------------------------------------------------------
|  Data_03a412  @ $03A412  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a412, "ax", @progbits
        .global Data_03a412
Data_03a412:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaef8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0a7c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1662                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x201c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe4aa                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xaef8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0a7c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1662                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x201c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe4aa                        | +026  (dato / opcode no decodificado)
        .global Data_03a412__L03a43a
Data_03a412__L03a43a:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a43c  @ $03A43C  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a43c, "ax", @progbits
        .global TaskHandler_03a43c
TaskHandler_03a43c:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_03a458(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_03a458__L03a480            | +018

| ----------------------------------------------------------------------------
|  Data_03a458  @ $03A458  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a458, "ax", @progbits
        .global Data_03a458
Data_03a458:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbd2a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4b6                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbd2a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0bb2                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1798                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x21aa                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4b6                        | +026  (dato / opcode no decodificado)
        .global Data_03a458__L03a480
Data_03a458__L03a480:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a482  @ $03A482  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a482, "ax", @progbits
        .global TaskHandler_03a482
TaskHandler_03a482:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     Sub_0003933A(pc)                | +006
        lea     Data_03a49e(pc),a0              | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   Data_03a49e__L03a4c6            | +018

| ----------------------------------------------------------------------------
|  Data_03a49e  @ $03A49E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a49e, "ax", @progbits
        .global Data_03a49e
Data_03a49e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbd24                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4aa                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbd24                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0b1e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1704                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x20be                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4aa                        | +026  (dato / opcode no decodificado)
        .global Data_03a49e__L03a4c6
Data_03a49e__L03a4c6:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a4c8  @ $03A4C8  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a4c8, "ax", @progbits
        .global TaskHandler_03a4c8
TaskHandler_03a4c8:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_03a4ea(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_03a4ea__L03a512            | +01e

| ----------------------------------------------------------------------------
|  Data_03a4ea  @ $03A4EA  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a4ea, "ax", @progbits
        .global Data_03a4ea
Data_03a4ea:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb6a8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xecd0                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xecd0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xecd0                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xecd0                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb6a8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xecd0                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xecd0                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xecd0                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xecd0                        | +026  (dato / opcode no decodificado)
        .global Data_03a4ea__L03a512
Data_03a4ea__L03a512:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a514  @ $03A514  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a514, "ax", @progbits
        .global TaskHandler_03a514
TaskHandler_03a514:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     Sub_0003933A(pc)                | +00c
        lea     Data_03a536(pc),a0              | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   Data_03a536__L03a55e            | +01e

| ----------------------------------------------------------------------------
|  Data_03a536  @ $03A536  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a536, "ax", @progbits
        .global Data_03a536
Data_03a536:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb6d6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xecfe                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xecfe                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xecfe                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xecfe                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb6d6                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xecfe                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xecfe                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xecfe                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xecfe                        | +026  (dato / opcode no decodificado)
        .global Data_03a536__L03a55e
Data_03a536__L03a55e:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_03a560  @ $03A560  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_03a560, "ax", @progbits
        .global TaskHandler_03a560
TaskHandler_03a560:
        addq.b  #0x1,0x30(a6)                   | +000
        btst    #0x0,0x30(a6)                   | +004
        bne.w   Data_03a58a__L03a5b6            | +00a
        bclr    #0x2,0x8c(a6)                   | +00e
        jsr     Sub_0003933A(pc)                | +014
        lea     Data_03a58a(pc),a0              | +018
        movea.l (a0,d0.w),a0                    | +01c
        jsr     0x28cd4.l                       | +020
        bra.w   Data_03a58a__L03a5b2            | +026

| ----------------------------------------------------------------------------
|  Data_03a58a  @ $03A58A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a58a, "ax", @progbits
        .global Data_03a58a
Data_03a58a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc3f2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfa34                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfa34                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfa34                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfa34                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc3f2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xfa34                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xfa34                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xfa34                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfa34                        | +026  (dato / opcode no decodificado)
        .global Data_03a58a__L03a5b2
Data_03a58a__L03a5b2:
        bra.w   Data_03a5d2__L03a5fa            | +028
        .global Data_03a58a__L03a5b6
Data_03a58a__L03a5b6:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     Sub_0003933A(pc)                | +032
        lea     Data_03a5d2(pc),a0              | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   Data_03a5d2__L03a5fa            | +044

| ----------------------------------------------------------------------------
|  Data_03a5d2  @ $03A5D2  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Data_03a5d2, "ax", @progbits
        .global Data_03a5d2
Data_03a5d2:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc4de                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfb20                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfb20                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfb20                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfb20                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc4de                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xfb20                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xfb20                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xfb20                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfb20                        | +026  (dato / opcode no decodificado)
        .global Data_03a5d2__L03a5fa
Data_03a5d2__L03a5fa:
        lea     TaskHandler_0392a4(pc),a0       | +028
        move.l  a0,0x4c(a6)                     | +02c
        jsr     0x283ca.l                       | +030
        rts                                     | +036
