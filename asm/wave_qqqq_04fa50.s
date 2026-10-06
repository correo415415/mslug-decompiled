| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $04FA50..$051914  (7,396 B, 96 entradas, 36 huecos)
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
|  TaskHandler_04fa58  @ $04FA58  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04fa58, "ax", @progbits
        .global TaskHandler_04fa58
TaskHandler_04fa58:
        move.w  #0x11,d1                        | +000
        move.w  #0xc1,d2                        | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c

| ----------------------------------------------------------------------------
|  Sub_0004FA70  @ $04FA70  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FA70, "ax", @progbits
        .global Sub_0004FA70
Sub_0004FA70:
        move.w  0x70(a6),d0                     | +000
        neg.w   d0                              | +004
        cmp.w   0x22(a6),d0                     | +006
        ble.w   ClearC_04fa84                   | +00a

| ----------------------------------------------------------------------------
|  Sub_0004FA8A  @ $04FA8A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FA8A, "ax", @progbits
        .global Sub_0004FA8A
Sub_0004FA8A:
        cmpi.b  #0x10,0x21(a6)                  | +000
        bne.w   TaskHandler_04faa0              | +006
        lea     0x29557e.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_04faa0  @ $04FAA0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04faa0, "ax", @progbits
        .global TaskHandler_04faa0
TaskHandler_04faa0:
        cmpi.b  #0x11,0x21(a6)                  | +000
        bne.w   TaskHandler_04fab6              | +006
        lea     0x295592.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_04fab6  @ $04FAB6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04fab6, "ax", @progbits
        .global TaskHandler_04fab6
TaskHandler_04fab6:
        cmpi.b  #0x12,0x21(a6)                  | +000
        bne.w   TaskHandler_04facc              | +006
        lea     0x2955a6.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_04facc  @ $04FACC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04facc, "ax", @progbits
        .global TaskHandler_04facc
TaskHandler_04facc:
        cmpi.b  #0x1,0x21(a6)                   | +000
        bne.w   TaskHandler_04fae2              | +006
        lea     0x2955ba.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_04fae2  @ $04FAE2  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04fae2, "ax", @progbits
        .global TaskHandler_04fae2
TaskHandler_04fae2:
        cmpi.b  #0x2,0x21(a6)                   | +000
        bne.w   JsrPcRts_04faf6                 | +006
        lea     0x2955ce.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  PcThunkTarget_04faf8  @ $04FAF8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_04faf8, "ax", @progbits
        .global PcThunkTarget_04faf8
PcThunkTarget_04faf8:
        cmpi.b  #0x10,0x21(a6)                  | +000
        bne.w   TaskHandler_04fb0e              | +006
        lea     0x295646.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_04fb0e  @ $04FB0E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04fb0e, "ax", @progbits
        .global TaskHandler_04fb0e
TaskHandler_04fb0e:
        cmpi.b  #0x1,0x21(a6)                   | +000
        bne.w   TaskHandler_04fb24              | +006
        lea     0x29565a.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_04fb24  @ $04FB24  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04fb24, "ax", @progbits
        .global TaskHandler_04fb24
TaskHandler_04fb24:
        cmpi.b  #0x11,0x21(a6)                  | +000
        bne.w   Stub_0004FB3A                   | +006
        lea     0x29566e.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Sub_0004FB3C  @ $04FB3C  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FB3C, "ax", @progbits
        .global Sub_0004FB3C
Sub_0004FB3C:
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
|  TaskHandler_04fb74  @ $04FB74  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04fb74, "ax", @progbits
        .global TaskHandler_04fb74
TaskHandler_04fb74:
        lea     0x29575e.l,a2                   | +000
        jsr     Sprite_InvokeBlit8Params(pc)    | +006
        lea     0x295772.l,a2                   | +00a

| ----------------------------------------------------------------------------
|  Sub_0004FB8A  @ $04FB8A  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FB8A, "ax", @progbits
        .global Sub_0004FB8A
Sub_0004FB8A:
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
|  Sub_0004FBE0  @ $04FBE0  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FBE0, "ax", @progbits
        .global Sub_0004FBE0
Sub_0004FBE0:
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
|  Sub_0004FC36  @ $04FC36  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FC36, "ax", @progbits
        .global Sub_0004FC36
Sub_0004FC36:
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
|  Sub_0004FC8C  @ $04FC8C  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FC8C, "ax", @progbits
        .global Sub_0004FC8C
Sub_0004FC8C:
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
|  Sub_0004FCD8  @ $04FCD8  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FCD8, "ax", @progbits
        .global Sub_0004FCD8
Sub_0004FCD8:
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
|  Sub_0004FD2C  @ $04FD2C  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FD2C, "ax", @progbits
        .global Sub_0004FD2C
Sub_0004FD2C:
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
|  Sub_0004FD8A  @ $04FD8A  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FD8A, "ax", @progbits
        .global Sub_0004FD8A
Sub_0004FD8A:
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
|  Sub_0004FDDE  @ $04FDDE  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FDDE, "ax", @progbits
        .global Sub_0004FDDE
Sub_0004FDDE:
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
|  Sub_0004FE32  @ $04FE32  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FE32, "ax", @progbits
        .global Sub_0004FE32
Sub_0004FE32:
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
|  Sub_0004FE86  @ $04FE86  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FE86, "ax", @progbits
        .global Sub_0004FE86
Sub_0004FE86:
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
|  Sub_0004FEDA  @ $04FEDA  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FEDA, "ax", @progbits
        .global Sub_0004FEDA
Sub_0004FEDA:
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
|  Sub_0004FF2E  @ $04FF2E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FF2E, "ax", @progbits
        .global Sub_0004FF2E
Sub_0004FF2E:
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
|  Sub_0004FF82  @ $04FF82  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FF82, "ax", @progbits
        .global Sub_0004FF82
Sub_0004FF82:
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
|  Sub_0004FFD6  @ $04FFD6  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004FFD6, "ax", @progbits
        .global Sub_0004FFD6
Sub_0004FFD6:
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
|  Sub_0005002A  @ $05002A  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0005002A, "ax", @progbits
        .global Sub_0005002A
Sub_0005002A:
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
|  Sub_0005007E  @ $05007E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0005007E, "ax", @progbits
        .global Sub_0005007E
Sub_0005007E:
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
|  Sub_000500D2  @ $0500D2  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000500D2, "ax", @progbits
        .global Sub_000500D2
Sub_000500D2:
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
|  Sub_00050126  @ $050126  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00050126, "ax", @progbits
        .global Sub_00050126
Sub_00050126:
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
|  Sub_0005017A  @ $05017A  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0005017A, "ax", @progbits
        .global Sub_0005017A
Sub_0005017A:
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
|  Sub_000501D8  @ $0501D8  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000501D8, "ax", @progbits
        .global Sub_000501D8
Sub_000501D8:
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
|  TaskHandler_050250  @ $050250  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050250, "ax", @progbits
        .global TaskHandler_050250
TaskHandler_050250:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_050266                    | +00c

| ----------------------------------------------------------------------------
|  Sub_0005029C  @ $05029C  (196 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0005029C, "ax", @progbits
        .global Sub_0005029C
Sub_0005029C:
        move.l  #0x27bc8,-(a7)                  | +000
        bra.w   .L0502ac                        | +006
        .global Sub_0005029C__L0502a6
Sub_0005029C__L0502a6:
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
|  TaskHandler_050360  @ $050360  (138 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050360, "ax", @progbits
        .global TaskHandler_050360
TaskHandler_050360:
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
        jsr     Sub_0005029C(pc)                | +028
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
|  Data_0503ea  @ $0503EA  (166 B)
| ----------------------------------------------------------------------------
        .section .text.Data_0503ea, "ax", @progbits
        .global Data_0503ea
Data_0503ea:
        .dc.w   0xff00                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +002  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +006  (dato / opcode no decodificado)
        .global Data_0503ea__L0503f2
Data_0503ea__L0503f2:
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
        lea     Data_0503ea(pc),a0              | +036
        jsr     0x5e086.l                       | +03a
        scc.b   0x79(a6)                        | +040
        bcs.w   .L050436                        | +044
        move.l  a0,0x72(a6)                     | +048
.L050436:
        bra.w   .L050462                        | +04c
.L05043a:
        lea     Data_0503ea(pc),a0              | +050
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
|  TaskHandler_050490  @ $050490  (90 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050490, "ax", @progbits
        .global TaskHandler_050490
TaskHandler_050490:
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
|  TaskHandler_0504f0  @ $0504F0  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0504f0, "ax", @progbits
        .global TaskHandler_0504f0
TaskHandler_0504f0:
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
|  TaskHandler_050520  @ $050520  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050520, "ax", @progbits
        .global TaskHandler_050520
TaskHandler_050520:
        move.w  0x28(a6),d0                     | +000
        bmi.w   .L050536                        | +004
        cmpi.w  #0x110,0x22(a6)                 | +008
        bpl.w   SetC_050546                     | +00e
        bra.w   ClearC_050540                   | +012
.L050536:
        cmpi.w  #0x30,0x22(a6)                  | +016
        bmi.w   SetC_050546                     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_05054c  @ $05054C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05054c, "ax", @progbits
        .global TaskHandler_05054c
TaskHandler_05054c:
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
|  TaskHandler_0505a0  @ $0505A0  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0505a0, "ax", @progbits
        .global TaskHandler_0505a0
TaskHandler_0505a0:
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
|  TaskHandler_0505f0  @ $0505F0  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0505f0, "ax", @progbits
        .global TaskHandler_0505f0
TaskHandler_0505f0:
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
|  TaskHandler_05063e  @ $05063E  (100 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05063e, "ax", @progbits
        .global TaskHandler_05063e
TaskHandler_05063e:
        move.l  0x72(a6),d0                     | +000
        bmi.w   .L0506a0                        | +004
        move.l  a6,-(a7)                        | +008
        lea     0x100800.l,a6                   | +00a
        lea     Data_051208__L0512b4(pc),a1     | +010
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
|  TaskHandler_0506a2  @ $0506A2  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0506a2, "ax", @progbits
        .global TaskHandler_0506a2
TaskHandler_0506a2:
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
|  TaskHandler_0506f6  @ $0506F6  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0506f6, "ax", @progbits
        .global TaskHandler_0506f6
TaskHandler_0506f6:
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
|  Data_050726  @ $050726  (228 B)
| ----------------------------------------------------------------------------
        .section .text.Data_050726, "ax", @progbits
        .global Data_050726
Data_050726:
        .dc.w   0xffe0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +002  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +006  (dato / opcode no decodificado)
        .global Data_050726__L05072e
Data_050726__L05072e:
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
        lea     TaskHandler_050e40(pc),a1       | +0c2
        move.l  a1,(a6)                         | +0c6
        .global Data_050726__L0507ee
Data_050726__L0507ee:
.L0507ee:
        move.w  0x66(a6),0x84(a6)               | +0c8
        lea     Data_050726(pc),a0              | +0ce
        jsr     0x5dd56.l                       | +0d2
        bcc.w   .L050808                        | +0d8
        jmp     0x518.l                         | +0dc
.L050808:
        rts                                     | +0e2

| ----------------------------------------------------------------------------
|  TaskHandler_05080a  @ $05080A  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05080a, "ax", @progbits
        .global TaskHandler_05080a
TaskHandler_05080a:
        lea     TaskHandler_05100e(pc),a1       | +000

| ----------------------------------------------------------------------------
|  TaskHandler_050816  @ $050816  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050816, "ax", @progbits
        .global TaskHandler_050816
TaskHandler_050816:
        lea     TaskHandler_05100e__L05101e(pc),a1 | +000

| ----------------------------------------------------------------------------
|  TaskHandler_050822  @ $050822  (90 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050822, "ax", @progbits
        .global TaskHandler_050822
TaskHandler_050822:
        move.w  #0x10f4,d0                      | +000
        jsr     0x2352.l                        | +004
        move.l  a6,-(a7)                        | +00a
        lea     0x100800.l,a6                   | +00c
        lea     Data_05148c__L051530(pc),a1     | +012
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
|  TaskHandler_05087c  @ $05087C  (90 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05087c, "ax", @progbits
        .global TaskHandler_05087c
TaskHandler_05087c:
        move.w  #0x10f4,d0                      | +000
        jsr     0x2352.l                        | +004
        move.l  a6,-(a7)                        | +00a
        lea     0x100800.l,a6                   | +00c
        lea     Data_05148c__L051530(pc),a1     | +012
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
|  TaskHandler_0508d6  @ $0508D6  (152 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0508d6, "ax", @progbits
        .global TaskHandler_0508d6
TaskHandler_0508d6:
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
        lea     TaskHandler_050f7e__L050fa6(pc),a1 | +046
        jsr     0x4ae.l                         | +04a
        lea     TaskHandler_050f7e__L050fba(pc),a1 | +050
        jsr     0x4ae.l                         | +054
        lea     TaskHandler_050f7e(pc),a1       | +05a
        jsr     0x4ae.l                         | +05e
        lea     TaskHandler_050f7e__L050f92(pc),a1 | +064
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
        bsr.w   Data_0503ea__L0503f2            | +094

| ----------------------------------------------------------------------------
|  TaskHandler_050976  @ $050976  (96 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050976, "ax", @progbits
        .global TaskHandler_050976
TaskHandler_050976:
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
        bsr.w   Data_0503ea__L0503f2            | +038
        bsr.w   TaskHandler_050360              | +03c
        jsr     0x28d70.l                       | +040
        move.w  0x22(a6),d0                     | +046
        subi.w  #0x30,d0                        | +04a
        cmpi.w  #0xe0,d0                        | +04e
        bcc.w   .L0509d2                        | +052
        lea     TaskHandler_0509de(pc),a1       | +056
        move.l  a1,(a6)                         | +05a
.L0509d2:
        bra.w   Data_050726__L05072e            | +05c

| ----------------------------------------------------------------------------
|  Data_0509d6  @ $0509D6  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Data_0509d6, "ax", @progbits
        .global Data_0509d6
Data_0509d6:
        .dc.w   0x113c                        | +000  (dato / opcode no decodificado)
        .dc.w   0x113d                        | +002  (dato / opcode no decodificado)
        .dc.w   0x113e                        | +004  (dato / opcode no decodificado)
        .dc.w   0x113c                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_0509de  @ $0509DE  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0509de, "ax", @progbits
        .global TaskHandler_0509de
TaskHandler_0509de:
        lea     0x296bd8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.w  #0x5a,0x30(a6)                  | +00c
        clr.w   0x28(a6)                        | +012
        lea     .L0509fa(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0509fa:
        move.b  #0x1e,0x45(a6)                  | +01c
        bsr.w   Data_0503ea__L0503f2            | +022
        bsr.w   TaskHandler_050360              | +026
        jsr     0x28d70.l                       | +02a
        subq.w  #0x1,0x30(a6)                   | +030
        bne.w   .L050a26                        | +034
        lea     0x2963c4.l,a0                   | +038
        move.l  a0,0x48(a6)                     | +03e
        lea     TaskHandler_050a2a(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L050a26:
        bra.w   Data_050726__L05072e            | +048

| ----------------------------------------------------------------------------
|  TaskHandler_050a2a  @ $050A2A  (248 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050a2a, "ax", @progbits
        .global TaskHandler_050a2a
TaskHandler_050a2a:
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
        bsr.w   Data_0503ea__L0503f2            | +04c
        bsr.w   TaskHandler_050360              | +050
        jsr     0x28d70.l                       | +054
        move.l  0x72(a6),d0                     | +05a
        bpl.w   .L050a92                        | +05e
        lea     TaskHandler_050c18(pc),a1       | +062
        move.l  a1,(a6)                         | +066
.L050a92:
        tst.b   0x77(a6)                        | +068
        bne.w   .L050aa0                        | +06c
        lea     TaskHandler_050b22(pc),a1       | +070
        move.l  a1,(a6)                         | +074
.L050aa0:
        jsr     TaskHandler_050490(pc)          | +076
        bcc.w   .L050aae                        | +07a
        lea     TaskHandler_050bd2(pc),a1       | +07e
        move.l  a1,(a6)                         | +082
.L050aae:
        jsr     TaskHandler_0504f0(pc)          | +084
        bcc.w   .L050abc                        | +088
        lea     TaskHandler_050bc2(pc),a1       | +08c
        move.l  a1,(a6)                         | +090
.L050abc:
        jsr     TaskHandler_0505f0(pc)          | +092
        bcc.w   .L050aca                        | +096
        lea     TaskHandler_050dfc(pc),a1       | +09a
        move.l  a1,(a6)                         | +09e
.L050aca:
        jsr     TaskHandler_0505a0(pc)          | +0a0
        bcc.w   .L050ad8                        | +0a4
        lea     TaskHandler_050b6a(pc),a1       | +0a8
        move.l  a1,(a6)                         | +0ac
.L050ad8:
        jsr     TaskHandler_050520(pc)          | +0ae
        bcc.w   .L050ae6                        | +0b2
        lea     TaskHandler_050b5e(pc),a1       | +0b6
        move.l  a1,(a6)                         | +0ba
.L050ae6:
        jsr     TaskHandler_05054c(pc)          | +0bc
        bcc.w   .L050af4                        | +0c0
        lea     TaskHandler_050d1a(pc),a1       | +0c4
        move.l  a1,(a6)                         | +0c8
.L050af4:
        jsr     TaskHandler_0506a2(pc)          | +0ca
        bcc.w   .L050b02                        | +0ce
        lea     TaskHandler_050dbe(pc),a1       | +0d2
        move.l  a1,(a6)                         | +0d6
.L050b02:
        jsr     TaskHandler_0506f6(pc)          | +0d8
        bcc.w   .L050b10                        | +0dc
        lea     TaskHandler_050b5e(pc),a1       | +0e0
        move.l  a1,(a6)                         | +0e4
.L050b10:
        tst.b   0x70(a6)                        | +0e6
        beq.w   .L050b1e                        | +0ea
        lea     TaskHandler_050c88(pc),a1       | +0ee
        move.l  a1,(a6)                         | +0f2
.L050b1e:
        bra.w   Data_050726__L05072e            | +0f4

| ----------------------------------------------------------------------------
|  TaskHandler_050b22  @ $050B22  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050b22, "ax", @progbits
        .global TaskHandler_050b22
TaskHandler_050b22:
        clr.b   0x86(a6)                        | +000
        lea     0x296954.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        clr.w   0x28(a6)                        | +010
        lea     .L050b3c(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L050b3c:
        bsr.w   Data_0503ea__L0503f2            | +01a
        bsr.w   TaskHandler_050360              | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L050b5a                        | +028
        bchg    #0x0,0x3a(a6)                   | +02c
        lea     TaskHandler_050a2a(pc),a1       | +032
        move.l  a1,(a6)                         | +036
.L050b5a:
        bra.w   Data_050726__L05072e            | +038

| ----------------------------------------------------------------------------
|  TaskHandler_050b5e  @ $050B5E  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050b5e, "ax", @progbits
        .global TaskHandler_050b5e
TaskHandler_050b5e:
        addq.b  #0x1,0x7a(a6)                   | +000
        not.b   0x78(a6)                        | +004
        bra.w   TaskHandler_050a2a              | +008

| ----------------------------------------------------------------------------
|  TaskHandler_050b6a  @ $050B6A  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050b6a, "ax", @progbits
        .global TaskHandler_050b6a
TaskHandler_050b6a:
        tst.b   0x86(a6)                        | +000
        bne.w   .L050b90                        | +004
        move.b  #0x1e,0x86(a6)                  | +008
        jsr     0x5e9b6.l                       | +00e
        andi.w  #0x6,d0                         | +014
        lea     Data_0509d6(pc),a0              | +018
        move.w  (a0,d0.w),d0                    | +01c
        jsr     0x2352.l                        | +020
.L050b90:
        lea     0x296b76.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        clr.w   0x28(a6)                        | +032
        lea     .L050ba6(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L050ba6:
        bsr.w   Data_0503ea__L0503f2            | +03c
        bsr.w   TaskHandler_050360              | +040
        jsr     0x28d70.l                       | +044
        bcc.w   .L050bbe                        | +04a
        lea     TaskHandler_050a2a(pc),a1       | +04e
        move.l  a1,(a6)                         | +052
.L050bbe:
        bra.w   Data_050726__L05072e            | +054

| ----------------------------------------------------------------------------
|  TaskHandler_050bc2  @ $050BC2  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050bc2, "ax", @progbits
        .global TaskHandler_050bc2
TaskHandler_050bc2:
        lea     0x29658a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   TaskHandler_050bd2__L050bde     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_050bd2  @ $050BD2  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050bd2, "ax", @progbits
        .global TaskHandler_050bd2
TaskHandler_050bd2:
        lea     0x29668c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global TaskHandler_050bd2__L050bde
TaskHandler_050bd2__L050bde:
.L050bde:
        clr.b   0x86(a6)                        | +00c
        clr.w   0x28(a6)                        | +010
        lea     .L050bec(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L050bec:
        bsr.w   Data_0503ea__L0503f2            | +01a
        bsr.w   TaskHandler_050360              | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L050c14                        | +028
        lea     0x296efc.l,a0                   | +02c
        jsr     0x799de.l                       | +032
        move.b  d0,0x7b(a6)                     | +038
        lea     TaskHandler_050a2a(pc),a1       | +03c
        move.l  a1,(a6)                         | +040
.L050c14:
        bra.w   Data_050726__L05072e            | +042

| ----------------------------------------------------------------------------
|  TaskHandler_050c18  @ $050C18  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050c18, "ax", @progbits
        .global TaskHandler_050c18
TaskHandler_050c18:
        clr.w   0x28(a6)                        | +000
        lea     0x296bd8.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L050c2e(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L050c2e:
        bsr.w   Data_0503ea__L0503f2            | +016
        bsr.w   TaskHandler_050360              | +01a
        jsr     0x28d70.l                       | +01e
        move.l  0x72(a6),d0                     | +024
        bmi.w   .L050c4a                        | +028
        lea     TaskHandler_050a2a(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L050c4a:
        bra.w   Data_050726__L05072e            | +032

| ----------------------------------------------------------------------------
|  TaskHandler_050c4e  @ $050C4E  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050c4e, "ax", @progbits
        .global TaskHandler_050c4e
TaskHandler_050c4e:
        clr.b   0x86(a6)                        | +000
        lea     0x296a04.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        clr.b   0x7a(a6)                        | +010
        clr.w   0x28(a6)                        | +014
        lea     .L050c6c(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L050c6c:
        bsr.w   Data_0503ea__L0503f2            | +01e
        bsr.w   TaskHandler_050360              | +022
        jsr     0x28d70.l                       | +026
        bcc.w   .L050c84                        | +02c
        lea     TaskHandler_050a2a(pc),a1       | +030
        move.l  a1,(a6)                         | +034
.L050c84:
        bra.w   Data_050726__L05072e            | +036

| ----------------------------------------------------------------------------
|  TaskHandler_050c88  @ $050C88  (146 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050c88, "ax", @progbits
        .global TaskHandler_050c88
TaskHandler_050c88:
        move.w  0x24(a6),d1                     | +000
        jsr     0x4400e.l                       | +004
        cmpi.w  #0x8f,d1                        | +00a
        ble.w   .L050cb4                        | +00e
        move.l  0x72(a6),d0                     | +012
        bmi.w   .L050cb4                        | +016
        movea.l d0,a0                           | +01a
        move.w  0x24(a0),d0                     | +01c
        cmp.w   0x24(a6),d0                     | +020
        bcs.w   TaskHandler_050d1a              | +024
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
        lea     TaskHandler_050d90(pc),a1       | +064
        move.l  a1,(a6)                         | +068
        bra.w   TaskHandler_050d90              | +06a
.L050cf6:
        lea     0x2969bc.l,a0                   | +06e
        jsr     0x28cd4.l                       | +074
        clr.w   0x28(a6)                        | +07a
        clr.w   0x2a(a6)                        | +07e
        move.w  #0xffc0,0x2e(a6)                | +082
        lea     TaskHandler_050d90(pc),a1       | +088
        move.l  a1,(a6)                         | +08c
        bra.w   TaskHandler_050d90              | +08e

| ----------------------------------------------------------------------------
|  TaskHandler_050d1a  @ $050D1A  (118 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050d1a, "ax", @progbits
        .global TaskHandler_050d1a
TaskHandler_050d1a:
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
        lea     TaskHandler_050d90(pc),a1       | +070
        move.l  a1,(a6)                         | +074

| ----------------------------------------------------------------------------
|  TaskHandler_050d90  @ $050D90  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050d90, "ax", @progbits
        .global TaskHandler_050d90
TaskHandler_050d90:
        bsr.w   Data_0503ea__L0503f2            | +000
        bsr.w   TaskHandler_050360              | +004
        jsr     0x28d70.l                       | +008
        tst.b   0x70(a6)                        | +00e
        bne.w   .L050dac                        | +012
        lea     TaskHandler_050c4e(pc),a1       | +016
        move.l  a1,(a6)                         | +01a
.L050dac:
        jsr     TaskHandler_0505f0(pc)          | +01c
        bcc.w   .L050dba                        | +020
        lea     TaskHandler_050dfc(pc),a1       | +024
        move.l  a1,(a6)                         | +028
.L050dba:
        bra.w   Data_050726__L05072e            | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_050dbe  @ $050DBE  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050dbe, "ax", @progbits
        .global TaskHandler_050dbe
TaskHandler_050dbe:
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
        lea     TaskHandler_050d90(pc),a1       | +036
        move.l  a1,(a6)                         | +03a
        bra.b   TaskHandler_050d90              | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_050dfc  @ $050DFC  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050dfc, "ax", @progbits
        .global TaskHandler_050dfc
TaskHandler_050dfc:
        clr.b   0x86(a6)                        | +000
        lea     0x296c82.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        jsr     0x283ca.l                       | +010
        lea     .L050e18(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L050e18:
        bsr.w   Data_0503ea__L0503f2            | +01c
        bsr.w   TaskHandler_050360              | +020
        jsr     0x28d70.l                       | +024
        bcc.w   .L050e30                        | +02a
        lea     TaskHandler_050a2a(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L050e30:
        tst.b   0x70(a6)                        | +034
        bne.w   .L050e3c                        | +038
        clr.w   0x28(a6)                        | +03c
.L050e3c:
        bra.w   Data_050726__L05072e            | +040

| ----------------------------------------------------------------------------
|  TaskHandler_050e40  @ $050E40  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050e40, "ax", @progbits
        .global TaskHandler_050e40
TaskHandler_050e40:
        jsr     0x4aad2.l                       | +000
        move.w  #0x1140,d0                      | +006
        jsr     0x2352.l                        | +00a
        move.w  0x14(a6),d1                     | +010
        move.w  #0x18e,d2                       | +014
        move.w  #0x1,d3                         | +018
        move.w  #0x1,d4                         | +01c
        jsr     0x2c30.l                        | +020
        tst.b   0x70(a6)                        | +026
        bne.w   TaskHandler_050e84__L050ea2     | +02a
        lea     0x296a4c.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        clr.w   0x28(a6)                        | +03a
        lea     TaskHandler_050e84(pc),a1       | +03e
        move.l  a1,(a6)                         | +042

| ----------------------------------------------------------------------------
|  TaskHandler_050e84  @ $050E84  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050e84, "ax", @progbits
        .global TaskHandler_050e84
TaskHandler_050e84:
        move.b  #0x2,0x45(a6)                   | +000
        bsr.w   TaskHandler_050360              | +006
        jsr     0x28d70.l                       | +00a
        bcc.w   .L050e9e                        | +010
        lea     TaskHandler_050f56(pc),a1       | +014
        move.l  a1,(a6)                         | +018
.L050e9e:
        bra.w   Data_050726__L0507ee            | +01a
        .global TaskHandler_050e84__L050ea2
TaskHandler_050e84__L050ea2:
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
        jsr     TaskHandler_050360(pc)          | +056
        jsr     0x28d70.l                       | +05a
        tst.b   0x70(a6)                        | +060
        bne.w   .L050ef2                        | +064
        lea     TaskHandler_050ef6(pc),a1       | +068
        move.l  a1,(a6)                         | +06c
.L050ef2:
        bra.w   Data_050726__L0507ee            | +06e

| ----------------------------------------------------------------------------
|  TaskHandler_050ef6  @ $050EF6  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050ef6, "ax", @progbits
        .global TaskHandler_050ef6
TaskHandler_050ef6:
        lea     0x296adc.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x28(a6)                        | +00c
        lea     TaskHandler_050e84(pc),a1       | +010
        move.l  a1,(a6)                         | +014
        bra.w   TaskHandler_050e84              | +016

| ----------------------------------------------------------------------------
|  TaskHandler_050f10  @ $050F10  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050f10, "ax", @progbits
        .global TaskHandler_050f10
TaskHandler_050f10:
        lea     0x296b24.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x28(a6)                        | +00c
        lea     .L050f26(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L050f26:
        move.b  #0x2,0x45(a6)                   | +016
        bsr.w   TaskHandler_050360              | +01c
        jsr     0x28d70.l                       | +020
        bcc.w   .L050f52                        | +026
        move.b  #0xf,0x45(a6)                   | +02a
        move.b  0x45(a6),0x59(a6)               | +030
        bclr    #0x3,0x13(a6)                   | +036
        lea     TaskHandler_050a2a(pc),a1       | +03c
        move.l  a1,(a6)                         | +040
.L050f52:
        bra.w   Data_050726__L0507ee            | +042

| ----------------------------------------------------------------------------
|  TaskHandler_050f56  @ $050F56  (40 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050f56, "ax", @progbits
        .global TaskHandler_050f56
TaskHandler_050f56:
        move.b  #0x1e,0x59(a6)                  | +000
        lea     .L050f62(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L050f62:
        bsr.w   TaskHandler_050360              | +00c
        jsr     0x28d70.l                       | +010
        tst.b   0x59(a6)                        | +016
        bne.w   .L050f7a                        | +01a
        jmp     0x518.l                         | +01e
.L050f7a:
        bra.w   Data_050726__L0507ee            | +024

| ----------------------------------------------------------------------------
|  TaskHandler_050f7e  @ $050F7E  (144 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_050f7e, "ax", @progbits
        .global TaskHandler_050f7e
TaskHandler_050f7e:
        lea     0x29643a.l,a0                   | +000
        move.l  a0,0x48(a6)                     | +006
        move.w  #0x7e,0x5c(a6)                  | +00a
        bra.w   .L050fca                        | +010
        .global TaskHandler_050f7e__L050f92
TaskHandler_050f7e__L050f92:
.L050f92:
        lea     0x29648e.l,a0                   | +014
        move.l  a0,0x48(a6)                     | +01a
        move.w  #0x7f,0x5c(a6)                  | +01e
        bra.w   .L050fca                        | +024
        .global TaskHandler_050f7e__L050fa6
TaskHandler_050f7e__L050fa6:
.L050fa6:
        lea     0x2964e2.l,a0                   | +028
        move.l  a0,0x48(a6)                     | +02e
        move.w  #0x7c,0x5c(a6)                  | +032
        bra.w   .L050fca                        | +038
        .global TaskHandler_050f7e__L050fba
TaskHandler_050f7e__L050fba:
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
|  TaskHandler_05100e  @ $05100E  (90 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05100e, "ax", @progbits
        .global TaskHandler_05100e
TaskHandler_05100e:
        lea     0x29663a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L05102a                        | +00c
        .global TaskHandler_05100e__L05101e
TaskHandler_05100e__L05101e:
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
|  Data_051068  @ $051068  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Data_051068, "ax", @progbits
        .global Data_051068
Data_051068:
        .dc.w   0x0016                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +004  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_051070  @ $051070  (408 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_051070, "ax", @progbits
        .global TaskHandler_051070
TaskHandler_051070:
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
        .global TaskHandler_051070__L051114
TaskHandler_051070__L051114:
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
|  Data_051208  @ $051208  (386 B)
| ----------------------------------------------------------------------------
        .section .text.Data_051208, "ax", @progbits
        .global Data_051208
Data_051208:
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
        .global Data_051208__L0512b4
Data_051208__L0512b4:
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
        lea     Data_051208(pc),a0              | +0fe
        jsr     0x28cd4.l                       | +102
        lea     TaskHandler_051070(pc),a0       | +108
        move.l  a0,0x4c(a6)                     | +10c
        jsr     0x283ca.l                       | +110
        lea     .L051324(pc),a1                 | +116
        move.l  a1,(a6)                         | +11a
.L051324:
        jsr     Sub_0005029C__L0502a6(pc)       | +11c
        bcc.w   .L051332                        | +120
        lea     TaskHandler_051392(pc),a1       | +124
        move.l  a1,(a6)                         | +128
.L051332:
        moveq   #0,d0                           | +12a
        move.b  0x106f28.l,d0                   | +12c
        andi.w  #0x3,d0                         | +132
        lsl.w   #0x1,d0                         | +136
        lea     Data_051068(pc),a0              | +138
        move.w  (a0,d0.w),d1                    | +13c
        move.w  (a6,d1.w),0x14(a6)              | +140
        jsr     0x28d70.l                       | +146
        btst    #0x1,0x13(a6)                   | +14c
        beq.w   .L051364                        | +152
        lea     TaskHandler_051392__L0513da(pc),a1 | +156
        move.l  a1,(a6)                         | +15a
.L051364:
        jsr     0x2870a.l                       | +15c
        bcc.w   .L051374                        | +162
        lea     TaskHandler_051392__L0513b6(pc),a1 | +166
        move.l  a1,(a6)                         | +16a
.L051374:
        movea.l #0xffffffff,a0                  | +16c
        lea     0x55cc8.l,a0                    | +172
        jsr     0x5dd56.l                       | +178
        bcc.w   SetHandlerRts_051390            | +17e

| ----------------------------------------------------------------------------
|  TaskHandler_051392  @ $051392  (184 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_051392, "ax", @progbits
        .global TaskHandler_051392
TaskHandler_051392:
        move.w  #0x2000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x10,0x38(a6)                  | +010
        move.w  #0x1027,d0                      | +016
        jsr     0x2352.l                        | +01a
        bra.w   .L0513fa                        | +020
        .global TaskHandler_051392__L0513b6
TaskHandler_051392__L0513b6:
.L0513b6:
        move.w  #0x8000,d0                      | +024
        jsr     0x28134.l                       | +028
        andi.w  #0xffe3,0x38(a6)                | +02e
        ori.w   #0x4,0x38(a6)                   | +034
        move.w  #0x1027,d0                      | +03a
        jsr     0x2352.l                        | +03e
        bra.w   .L0513fa                        | +044
        .global TaskHandler_051392__L0513da
TaskHandler_051392__L0513da:
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
        lea     TaskHandler_051070__L051114(pc),a0 | +09e
        move.l  a0,0x4c(a6)                     | +0a2
        jsr     0x283ca.l                       | +0a6
        jsr     0x283ca.l                       | +0ac
        jsr     0x283d8.l                       | +0b2

| ----------------------------------------------------------------------------
|  TaskHandler_051452  @ $051452  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_051452, "ax", @progbits
        .global TaskHandler_051452
TaskHandler_051452:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L05146e                        | +00c
        jsr     0x5b6.l                         | +010
        jmp     0x518.l                         | +016
.L05146e:
        lea     0xffff.w,a0                     | +01c
        move.l  a0,0x4c(a6)                     | +020

| ----------------------------------------------------------------------------
|  Data_05148c  @ $05148C  (368 B)
| ----------------------------------------------------------------------------
        .section .text.Data_05148c, "ax", @progbits
        .global Data_05148c
Data_05148c:
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
        .global Data_05148c__L051530
Data_05148c__L051530:
.L051530:
        .dc.w   0x323c                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x018f                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x236e                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x41f9                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x67d6                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x8cd4                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x41f9                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x7000                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x99de                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x3200                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x302e                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0034                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x3c0e                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x3d41                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x3d42                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x41fa                        | +0da  (dato / opcode no decodificado)
        .dc.w   0xff24                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x2d48                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x004c                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x83ca                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x43fa                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x2c89                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x4eba                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0xed2a                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x8d70                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x83d8                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x6500                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +100  (dato / opcode no decodificado)
        .dc.w   0x302e                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0640                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0c40                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0160                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x6400                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0058                        | +110  (dato / opcode no decodificado)
        .dc.w   0x302e                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +114  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +116  (dato / opcode no decodificado)
        .dc.w   0x00f0                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0c40                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0120                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x6400                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0048                        | +120  (dato / opcode no decodificado)
        .dc.w   0x4e75                        | +122  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +126  (dato / opcode no decodificado)
        .dc.w   0x236e                        | +128  (dato / opcode no decodificado)
        .dc.w   0x302e                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x6600                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +130  (dato / opcode no decodificado)
        .dc.w   0x41f9                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +134  (dato / opcode no decodificado)
        .dc.w   0x6d26                        | +136  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x8cd4                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x6000                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +140  (dato / opcode no decodificado)
        .dc.w   0x41f9                        | +142  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +144  (dato / opcode no decodificado)
        .dc.w   0x6d78                        | +146  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x8cd4                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x426e                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +150  (dato / opcode no decodificado)
        .dc.w   0x426e                        | +152  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +154  (dato / opcode no decodificado)
        .dc.w   0x43fa                        | +156  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +158  (dato / opcode no decodificado)
        .dc.w   0x2c89                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x8d70                        | +160  (dato / opcode no decodificado)
        .dc.w   0x6500                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +164  (dato / opcode no decodificado)
        .dc.w   0x609a                        | +166  (dato / opcode no decodificado)
        .dc.w   0x4ef9                        | +168  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x0518                        | +16c  (dato / opcode no decodificado)
        rts                                     | +16e

| ----------------------------------------------------------------------------
|  TaskHandler_0515fc  @ $0515FC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0515fc, "ax", @progbits
        .global TaskHandler_0515fc
TaskHandler_0515fc:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_051612                    | +00c

| ----------------------------------------------------------------------------
|  Data_051618  @ $051618  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Data_051618, "ax", @progbits
        .global Data_051618
Data_051618:
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
|  Sub_0005164A  @ $05164A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0005164A, "ax", @progbits
        .global Sub_0005164A
Sub_0005164A:
        lea     0x2970f8.l,a0                   | +000
        jsr     0x43562.l                       | +006
        moveq   #0,d0                           | +00c
        moveq   #0,d1                           | +00e
        jmp     0x437da.l                       | +010

| ----------------------------------------------------------------------------
|  TaskHandler_051660  @ $051660  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_051660, "ax", @progbits
        .global TaskHandler_051660
TaskHandler_051660:
        jsr     0x9826e.l                       | +000
        bcs.w   .L051674                        | +006
        clr.b   0x106ed2.l                      | +00a
        bra.w   JsrAbsRts_051694                | +010
.L051674:
        move.b  #0x1,0x106ed2.l                 | +014
        jsr     Sub_0005164A(pc)                | +01c
        move.b  #0xff,0x10e3b6.l                | +020
        lea     0x9840c.l,a1                    | +028

| ----------------------------------------------------------------------------
|  TaskHandler_051696  @ $051696  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_051696, "ax", @progbits
        .global TaskHandler_051696
TaskHandler_051696:
        move.b  0x10e3b7.l,d0                   | +000
        move.b  d0,0x106ed0.l                   | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_0516a4  @ $0516A4  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0516a4, "ax", @progbits
        .global TaskHandler_0516a4
TaskHandler_0516a4:
        tst.b   0x10e3b6.l                      | +000
        bne.w   SetC_0516b4                     | +006

| ----------------------------------------------------------------------------
|  TaskHandler_0516ba  @ $0516BA  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0516ba, "ax", @progbits
        .global TaskHandler_0516ba
TaskHandler_0516ba:
        lea     0x10e3a2.l,a4                   | +000
        move.b  0x106ed0.l,0x15(a4)             | +006
        move.b  0x10fdb6.l,0x16(a4)             | +00e
        move.b  0x10fdb7.l,0x17(a4)             | +016
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_0516da  @ $0516DA  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0516da, "ax", @progbits
        .global TaskHandler_0516da
TaskHandler_0516da:
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
|  TaskHandler_05174c  @ $05174C  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05174c, "ax", @progbits
        .global TaskHandler_05174c
TaskHandler_05174c:
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
|  TaskHandler_05178e  @ $05178E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05178e, "ax", @progbits
        .global TaskHandler_05178e
TaskHandler_05178e:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0517a4                    | +00c

| ----------------------------------------------------------------------------
|  Sub_000517AA  @ $0517AA  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000517AA, "ax", @progbits
        .global Sub_000517AA
Sub_000517AA:
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
|  ThunkTarget_05180c  @ $05180C  (8 B)
| ----------------------------------------------------------------------------
        .section .text.ThunkTarget_05180c, "ax", @progbits
        .global ThunkTarget_05180c
ThunkTarget_05180c:
        clr.l   (a1)                            | +000
        clr.l   0x4(a1)                         | +002
        rts                                     | +006

| ----------------------------------------------------------------------------
|  TaskHandler_051814  @ $051814  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_051814, "ax", @progbits
        .global TaskHandler_051814
TaskHandler_051814:
        move.l  a1,-(a7)                        | +000
        movea.l a1,a0                           | +002
        lea     0x1081b6.l,a1                   | +004
        move.l  d0,(a1)                         | +00a
        bsr.w   Sub_00051828                    | +00c
        movea.l (a7)+,a1                        | +010
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Sub_00051828  @ $051828  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00051828, "ax", @progbits
        .global Sub_00051828
Sub_00051828:
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
|  Sub_00051862  @ $051862  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00051862, "ax", @progbits
        .global Sub_00051862
Sub_00051862:
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
|  StateJumpTable_05188C  @ $05188C  (136 B)
| ----------------------------------------------------------------------------
        .section .text.StateJumpTable_05188C, "ax", @progbits
        .global StateJumpTable_05188C
StateJumpTable_05188C:
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
