| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $000400..$002F30  (3,938 B, 76 entradas, 26 huecos)
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
|  RtsStub_0400  @ $000400  (2 B)
| ----------------------------------------------------------------------------
        .section .text.RtsStub_0400, "ax", @progbits
        .global RtsStub_0400
RtsStub_0400:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_000402  @ $000402  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_000402, "ax", @progbits
        .global TaskHandler_000402
TaskHandler_000402:
        jsr     0x2783a.l                       | +000

| ----------------------------------------------------------------------------
|  Task_AllocFail_0506  @ $000506  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Task_AllocFail_0506, "ax", @progbits
        .global Task_AllocFail_0506
Task_AllocFail_0506:
        lea     0x1009e0.l,a0                   | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Task_InstallHandler_0000050E  @ $00050E  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Task_InstallHandler_0000050E, "ax", @progbits
        .global Task_InstallHandler_0000050E
Task_InstallHandler_0000050E:
        bsr.b   Task_AllocFromFreeList__L0004c6 | +000
        bset    #0x0,0x12(a0)                   | +002
        rts                                     | +008

| ----------------------------------------------------------------------------
|  EmptyEntity_Init_00076A  @ $00076A  (8 B)
| ----------------------------------------------------------------------------
        .section .text.EmptyEntity_Init_00076A, "ax", @progbits
        .global EmptyEntity_Init_00076A
EmptyEntity_Init_00076A:
        lea     0x1009e0.l,a0                   | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  TaskHandler_0007b2  @ $0007B2  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0007b2, "ax", @progbits
        .global TaskHandler_0007b2
TaskHandler_0007b2:
        movea.l 0xc(a6),a0                      | +000
        cmpi.l  #0x52a,(a0)                     | +004
        beq.w   SetXN_0007c6                    | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_00082e  @ $00082E  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_00082e, "ax", @progbits
        .global TaskHandler_00082e
TaskHandler_00082e:
        jmp     SoftReset_085E(pc)              | +000

| ----------------------------------------------------------------------------
|  TaskHandler_00085c  @ $00085C  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_00085c, "ax", @progbits
        .global TaskHandler_00085c
TaskHandler_00085c:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0008a4  @ $0008A4  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0008a4, "ax", @progbits
        .global TaskHandler_0008a4
TaskHandler_0008a4:
        move.w  #0x1234,d0                      | +000
        move.w  #0xa,d1                         | +004
        trap    #0xf                            | +008
        move.w  #0x1234,d0                      | +00a
        move.w  #0xb,d1                         | +00e
        trap    #0xf                            | +012
        move.w  #0x1234,d0                      | +014
        move.w  #0xc,d1                         | +018
        trap    #0xf                            | +01c
        move.w  #0x1234,d0                      | +01e
        move.w  #0xd,d1                         | +022
        trap    #0xf                            | +026
        move.w  #0x1234,d0                      | +028
        move.w  #0xe,d1                         | +02c
        trap    #0xf                            | +030

| ----------------------------------------------------------------------------
|  TaskHandler_0012f4  @ $0012F4  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0012f4, "ax", @progbits
        .global TaskHandler_0012f4
TaskHandler_0012f4:
        jsr     0xc004c2.l                      | +000
        jsr     0x52712.l                       | +006
        lea     0x8f91a.l,a1                    | +00c
        jsr     0x4ae.l                         | +012
        move.b  #0xff,0x106ece.l                | +018
        move.b  #0xff,0x106ecf.l                | +020
        jsr     PcThunkTarget_001af8(pc)        | +028

| ----------------------------------------------------------------------------
|  TaskHandler_001332  @ $001332  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001332, "ax", @progbits
        .global TaskHandler_001332
TaskHandler_001332:
        bsr.w   Sub_00001DB8                    | +000
        tst.b   0x106ed2.l                      | +004
        beq.w   .L001344                        | +00a
        bra.w   JsrPcThunk_00134e               | +00e
.L001344:
        jsr     0x5b6.l                         | +012
        bra.w   SchedulerDispatch_LoopB_000FE0  | +018

| ----------------------------------------------------------------------------
|  TaskHandler_001354  @ $001354  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001354, "ax", @progbits
        .global TaskHandler_001354
TaskHandler_001354:
        jsr     0x13d2a.l                       | +000
        bra.w   SchedulerDispatch_LoopB_000FE0  | +006

| ----------------------------------------------------------------------------
|  TaskHandler_00135e  @ $00135E  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_00135e, "ax", @progbits
        .global TaskHandler_00135e
TaskHandler_00135e:
        jsr     0x13d20.l                       | +000
        bra.w   SchedulerDispatch_LoopB_000FE0  | +006

| ----------------------------------------------------------------------------
|  TaskHandler_001368  @ $001368  (94 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001368, "ax", @progbits
        .global TaskHandler_001368
TaskHandler_001368:
        jsr     0xc004c2.l                      | +000
        jsr     0x52712.l                       | +006
        jsr     0x212e.l                        | +00c
        move.b  #0x9,0x106ece.l                 | +012
        move.b  #0xff,0x106ecf.l                | +01a
        clr.b   0x106ed2.l                      | +022
        lea     0x10fd84.l,a3                   | +028
        move.b  0xa(a3),d0                      | +02e
        tst.b   d0                              | +032
        beq.w   .L0013c2                        | +034
        lea     0x100300.l,a0                   | +038
        move.l  #0x8c008,(a0)                   | +03e
        jsr     0x5fe.l                         | +044
        move.w  #0x1,0x106e92.l                 | +04a
        move.b  #0xff,0x106ed2.l                | +052
.L0013c2:
        bra.w   SchedulerDispatch_LoopB_000FE0  | +05a

| ----------------------------------------------------------------------------
|  TaskHandler_0013c6  @ $0013C6  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0013c6, "ax", @progbits
        .global TaskHandler_0013c6
TaskHandler_0013c6:
        bsr.w   Sub_00001DB8                    | +000
        tst.b   0x106ed2.l                      | +004
        beq.w   .L00140c                        | +00a
        jsr     0x138e6.l                       | +00e
        bcc.w   .L00140a                        | +014
        cmpi.b  #0x1,0x10fdb6.l                 | +018
        bne.w   .L0013f4                        | +020
        jsr     0x5d09a.l                       | +024
        bcs.w   .L00140c                        | +02a
.L0013f4:
        cmpi.b  #0x1,0x10fdb7.l                 | +02e
        bne.w   .L00140a                        | +036
        jsr     0x5d0ac.l                       | +03a
        bcs.w   .L00140c                        | +040
.L00140a:
        rts                                     | +044
.L00140c:
        bsr.w   TaskList_ChangeAndRunEight_001CD4 | +046
        bra.w   SchedulerDispatch_LoopB_000FE0  | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_001414  @ $001414  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001414, "ax", @progbits
        .global TaskHandler_001414
TaskHandler_001414:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_001416  @ $001416  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001416, "ax", @progbits
        .global TaskHandler_001416
TaskHandler_001416:
        jsr     0xc004c2.l                      | +000
        jsr     0x46ac6.l                       | +006
        clr.b   0x106ed0.l                      | +00c
        jsr     0x981fc.l                       | +012
        jsr     0x5162c.l                       | +018
        bra.w   SchedulerDispatch_LoopB_000FE0  | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_001438  @ $001438  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001438, "ax", @progbits
        .global TaskHandler_001438
TaskHandler_001438:
        bsr.w   Sub_00001DB8                    | +000
        tst.b   0x106ed2.l                      | +004
        bne.w   .L001450                        | +00a
        jsr     0x44236.l                       | +00e
        bra.w   SchedulerDispatch_LoopB_000FE0  | +014
.L001450:
        rts                                     | +018

| ----------------------------------------------------------------------------
|  TaskHandler_001452  @ $001452  (138 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001452, "ax", @progbits
        .global TaskHandler_001452
TaskHandler_001452:
        move.w  0x10007c.l,d0                   | +000
        addq.w  #0x1,d0                         | +006
        move.w  d0,0x10007c.l                   | +008
        jsr     0x5e998.l                       | +00e
        jsr     0x51696.l                       | +014
        move.b  0x106ed0.l,d0                   | +01a
        cmpi.b  #0x6,d0                         | +020
        bcs.w   .L001482                        | +024
        move.b  #0x5,0x106ed0.l                 | +028
.L001482:
        move.b  0x106ed0.l,d0                   | +030
        cmpi.b  #0x10,d0                        | +036
        bcs.w   .L001498                        | +03a
        clr.b   d0                              | +03e
        move.b  d0,0x106ed0.l                   | +040
.L001498:
        andi.w  #0xff,d0                        | +046
        lea     BootDispatchTable_000B92__L000c0a(pc),a0 | +04a
        lsl.w   #0x3,d0                         | +04e
        movea.l (a0,d0.w),a0                    | +050
        move.l  a0,0x70(a6)                     | +054
        bra.w   Sub_00000FC6                    | +058
        move.b  #0x0,0x106ed0.l                 | +05c
        jsr     0xc004c2.l                      | +064
        bsr.w   Sub_000014F4                    | +06a
        bra.w   SchedulerDispatch_LoopB_000FE0  | +06e
        jsr     0x5ce26.l                       | +072
        bcc.w   .L0014d2                        | +078
        bsr.w   Sub_000014DC                    | +07c
.L0014d2:
        jsr     0x5ce14.l                       | +080
        bcs.b   .L001482                        | +086
        rts                                     | +088

| ----------------------------------------------------------------------------
|  Sub_000014DC  @ $0014DC  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000014DC, "ax", @progbits
        .global Sub_000014DC
Sub_000014DC:
        move.b  0x106ed0.l,d0                   | +000
        addq.b  #0x1,d0                         | +006
        cmpi.b  #0x10,d0                        | +008
        bcs.w   .L0014ee                        | +00c
        clr.b   d0                              | +010
.L0014ee:
        move.b  d0,0x106ed0.l                   | +012

| ----------------------------------------------------------------------------
|  Sub_000014F4  @ $0014F4  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000014F4, "ax", @progbits
        .global Sub_000014F4
Sub_000014F4:
        moveq   #0,d0                           | +000
        move.b  0x106ed0.l,d0                   | +002
        lea     BootDispatchTable_000B92__L000c0a(pc),a0 | +008
        lsl.w   #0x3,d0                         | +00c
        move.l  0x4(a0,d0.w),d1                 | +00e
        bmi.b   Sub_000014DC                    | +012
        movea.l d1,a0                           | +014
        moveq   #0,d3                           | +016
        move.b  (a0),d3                         | +018
        jmp     0x5eae4.l                       | +01a

| ----------------------------------------------------------------------------
|  Data_001514  @ $001514  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Data_001514, "ax", @progbits
        .global Data_001514
Data_001514:
        .dc.b   0x0a                          | +000  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +001  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +002  '.'  (dato, rango --data)
        .dc.b   0x0d                          | +003  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +004  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +005  '.'  (dato, rango --data)
        .dc.b   0x10                          | +006  '.'  (dato, rango --data)
        .dc.b   0x11                          | +007  '.'  (dato, rango --data)
        .dc.b   0x12                          | +008  '.'  (dato, rango --data)
        .dc.b   0x13                          | +009  '.'  (dato, rango --data)
        .dc.b   0x18                          | +00a  '.'  (dato, rango --data)
        .dc.b   0x15                          | +00b  '.'  (dato, rango --data)
        .dc.b   0x1b                          | +00c  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +00d  '.'  (dato, rango --data)
        .dc.b   0x04                          | +00e  '.'  (dato, rango --data)
        .dc.b   0x1c                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x00                          | +011  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  TaskHandler_001526  @ $001526  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001526, "ax", @progbits
        .global TaskHandler_001526
TaskHandler_001526:
        jsr     0xc004c2.l                      | +000
        jsr     0x46ac6.l                       | +006
        move.b  #0x1,0x106ed2.l                 | +00c
        jsr     0x51660.l                       | +014
        bra.w   SchedulerDispatch_LoopB_000FE0  | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_001544  @ $001544  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001544, "ax", @progbits
        .global TaskHandler_001544
TaskHandler_001544:
        bsr.w   Sub_00001DB8                    | +000
        tst.b   0x106ed2.l                      | +004
        beq.w   SchedulerDispatch_LoopB_000FE0  | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_001554  @ $001554  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001554, "ax", @progbits
        .global TaskHandler_001554
TaskHandler_001554:
        addi.b  #0x1,0x106ed0.l                 | +000
        jsr     0x516ba.l                       | +008
        jmp     SchedulerDispatch_LoopB_000FE0(pc) | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_001566  @ $001566  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001566, "ax", @progbits
        .global TaskHandler_001566
TaskHandler_001566:
        jsr     0x44036.l                       | +000
        jsr     PcThunkTarget_001E1C(pc)        | +006
        jsr     0x52712.l                       | +00a
        bsr.w   Sub_00001DA4                    | +010
        jsr     0xc004c2.l                      | +014
        jsr     0x46ac6.l                       | +01a
        lea     0x7a456.l,a1                    | +020
        jsr     0x4ae.l                         | +026
        move.b  #0xff,0x106ed2.l                | +02c
        bra.w   SchedulerDispatch_LoopB_000FE0  | +034

| ----------------------------------------------------------------------------
|  TaskHandler_00159e  @ $00159E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_00159e, "ax", @progbits
        .global TaskHandler_00159e
TaskHandler_00159e:
        bsr.w   Sub_00001DB8                    | +000
        jsr     0x5d288.l                       | +004
        bcs.w   .L0015ae                        | +00a
        rts                                     | +00e
.L0015ae:
        bsr.w   TaskList_ChangeAndRunEight_001CD4 | +010
        bra.w   SchedulerDispatch_LoopB_000FE0  | +014

| ----------------------------------------------------------------------------
|  TaskHandler_0015b6  @ $0015B6  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0015b6, "ax", @progbits
        .global TaskHandler_0015b6
TaskHandler_0015b6:
        jsr     0xc004c2.l                      | +000
        jsr     0x46ac6.l                       | +006
        bsr.w   PcThunkTarget_001E1C            | +00c
        jsr     0x52712.l                       | +010
        bsr.w   Sub_00001DA4                    | +016
        clr.b   0x20(a6)                        | +01a
        move.b  #0xff,0x106ed2.l                | +01e
        move.b  #0xd,0x106ece.l                 | +026
        lea     0x598fc.l,a1                    | +02e
        jsr     0x4ae.l                         | +034
        bra.w   SchedulerDispatch_LoopB_000FE0  | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_0015f4  @ $0015F4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0015f4, "ax", @progbits
        .global TaskHandler_0015f4
TaskHandler_0015f4:
        move.b  #0x2,0x45(a6)                   | +000
        tst.b   0x20(a6)                        | +006
        bne.w   SchedulerDispatch_LoopB_000FE0  | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_001604  @ $001604  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001604, "ax", @progbits
        .global TaskHandler_001604
TaskHandler_001604:
        move.b  #0x2,0x45(a6)                   | +000
        jsr     0x5174c.l                       | +006
        bcc.w   .L00161c                        | +00c
        lea     BootDispatchTable_000B92__L000d62(pc),a0 | +010
        bra.w   .L001630                        | +014
.L00161c:
        lea     BootDispatchTable_000B92__L000d8a(pc),a0 | +018
        cmpi.b  #0x0,0x10fd83.l                 | +01c
        bne.w   .L001630                        | +024
        lea     BootDispatchTable_000B92__L000d76(pc),a0 | +028
.L001630:
        move.l  a0,0x70(a6)                     | +02c
        bsr.w   Sub_00001DA4                    | +030
        bra.w   Sub_00000FC6                    | +034

| ----------------------------------------------------------------------------
|  TaskHandler_00163c  @ $00163C  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_00163c, "ax", @progbits
        .global TaskHandler_00163c
TaskHandler_00163c:
        move.b  #0xb,0x106ece.l                 | +000
        lea     0x100260.l,a0                   | +008
        move.l  #0x8ce64,(a0)                   | +00e
        jsr     0x5fe.l                         | +014
        bra.w   .L001692                        | +01a
        move.b  #0xc,0x106ece.l                 | +01e
        lea     0x100260.l,a0                   | +026
        move.l  #0x8d0fa,(a0)                   | +02c
        jsr     0x5fe.l                         | +032
        bra.w   .L001692                        | +038
        move.b  #0xc,0x106ece.l                 | +03c
        lea     0x100260.l,a0                   | +044
        move.l  #0x8d0a8,(a0)                   | +04a
        jsr     0x5fe.l                         | +050
.L001692:
        jsr     0xc004c2.l                      | +056
        jsr     0x46ac6.l                       | +05c
        jsr     0x52712.l                       | +062
        move.b  #0xff,0x106ecf.l                | +068
        move.b  #0xff,0x106ed2.l                | +070
        bsr.w   PcThunkTarget_001DCC            | +078
        jsr     0x46ac6.l                       | +07c
        bra.w   SchedulerDispatch_LoopB_000FE0  | +082

| ----------------------------------------------------------------------------
|  TaskHandler_0016c2  @ $0016C2  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0016c2, "ax", @progbits
        .global TaskHandler_0016c2
TaskHandler_0016c2:
        move.b  #0x2,0x45(a6)                   | +000
        move.b  #0x2,0x44(a6)                   | +006
        bra.w   Attract_PostStart_Cleanup_001AB6__L001adc | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_0016d2  @ $0016D2  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0016d2, "ax", @progbits
        .global TaskHandler_0016d2
TaskHandler_0016d2:
        jsr     0xc004c2.l                      | +000
        bsr.w   PcThunkTarget_001E1C            | +006
        bsr.w   Sub_00001DA4                    | +00a
        move.b  #0xff,0x106ed2.l                | +00e
        move.b  #0xff,0x106ece.l                | +016
        lea     0x8c956.l,a1                    | +01e
        jsr     0x4ae.l                         | +024
        bra.w   SchedulerDispatch_LoopB_000FE0  | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_001700  @ $001700  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001700, "ax", @progbits
        .global TaskHandler_001700
TaskHandler_001700:
        jsr     0xc004c2.l                      | +000
        bsr.w   Sub_00001DB8                    | +006
        jsr     0x52712.l                       | +00a
        lea     0x99ba6.l,a1                    | +010
        jsr     0x4ae.l                         | +016
        move.b  #0xff,0x106ed2.l                | +01c
        jsr     0x46ac6.l                       | +024
        bra.w   SchedulerDispatch_LoopB_000FE0  | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_00172e  @ $00172E  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_00172e, "ax", @progbits
        .global TaskHandler_00172e
TaskHandler_00172e:
        tst.b   0x106ed2.l                      | +000
        bne.w   .L001742                        | +006
        jsr     0x5b6.l                         | +00a
        bra.w   SchedulerDispatch_LoopB_000FE0  | +010
.L001742:
        rts                                     | +014

| ----------------------------------------------------------------------------
|  PcThunkTarget_001af8  @ $001AF8  (28 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_001af8, "ax", @progbits
        .global PcThunkTarget_001af8
PcThunkTarget_001af8:
        move.b  0x10fdaf.l,d0                   | +000
        cmpi.b  #0x2,d0                         | +006
        bne.w   SetHandlerRts_001b1a            | +00a
        bsr.w   TaskList_ChangeAndRunEight_001CD4 | +00e
        move.w  #0xffff,d0                      | +012
        jsr     0x24fec.l                       | +016

| ----------------------------------------------------------------------------
|  Sub_00001B1C  @ $001B1C  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00001B1C, "ax", @progbits
        .global Sub_00001B1C
Sub_00001B1C:
        move.w  #0x0,-(a7)                      | +000
        moveq   #0,d0                           | +004
        jsr     0x5e3a2.l                       | +006
        bcc.w   .L001b2e                        | +00c
        addq.w  #0x1,(a7)                       | +010
.L001b2e:
        moveq   #1,d0                           | +012
        jsr     0x5e3a2.l                       | +014
        bcc.w   .L001b3c                        | +01a
        addq.w  #0x1,(a7)                       | +01e
.L001b3c:
        move.w  (a7)+,d0                        | +020
        rts                                     | +022

| ----------------------------------------------------------------------------
|  TaskHandler_001b40  @ $001B40  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001b40, "ax", @progbits
        .global TaskHandler_001b40
TaskHandler_001b40:
        jsr     Sub_00001B1C(pc)                | +000
        move.b  d0,0x106ed1.l                   | +004
        rts                                     | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_001b4c  @ $001B4C  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001b4c, "ax", @progbits
        .global TaskHandler_001b4c
TaskHandler_001b4c:
        jsr     0x79970.l                       | +000
        move.w  d0,0x106e92.l                   | +006
        move.b  #0xff,d1                        | +00c
        jsr     0x47482.l                       | +010
        move.w  #0x5a,0x30(a6)                  | +016

| ----------------------------------------------------------------------------
|  TaskHandler_001b70  @ $001B70  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001b70, "ax", @progbits
        .global TaskHandler_001b70
TaskHandler_001b70:
        subq.w  #0x1,0x30(a6)                   | +000
        bne.w   SetHandlerRts_001b7e            | +004

| ----------------------------------------------------------------------------
|  TaskHandler_001b80  @ $001B80  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001b80, "ax", @progbits
        .global TaskHandler_001b80
TaskHandler_001b80:
        jsr     TaskHandler_001cac(pc)          | +000
        bcs.w   Handler_TimerAndReplace_001BCA__L001c32 | +004
        tst.b   0x106ed3.l                      | +008
        beq.w   Handler_TimerAndReplace_001BCA__L001c32 | +00e
        jsr     0x2f85a.l                       | +012
        bcs.w   Handler_TimerAndReplace_001BCA__L001c32 | +018
        tst.b   0x10e39c.l                      | +01c
        bne.w   Handler_TimerAndReplace_001BCA__L001c32 | +022
        move.b  #0xff,d1                        | +026
        move.b  0x106f28.l,d0                   | +02a
        cmpi.w  #0xa,0x106e92.l                 | +030
        bgt.w   Handler_TimerAndReplace_001BCA__L001bde | +038
        move.b  d0,d2                           | +03c
        andi.b  #0x10,d2                        | +03e
        cmpi.b  #0x10,d2                        | +042
        bne.w   Handler_TimerAndReplace_001BCA__L001bde | +046

| ----------------------------------------------------------------------------
|  Sub_00001C34  @ $001C34  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00001C34, "ax", @progbits
        .global Sub_00001C34
Sub_00001C34:
        tst.b   0x21(a6)                        | +000
        bne.w   SetHandlerRts_001c42            | +004

| ----------------------------------------------------------------------------
|  TaskHandler_001C44  @ $001C44  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001C44, "ax", @progbits
        .global TaskHandler_001C44
TaskHandler_001C44:
        cmpi.b  #0x2,0x10fdaf.l                 | +000
        beq.w   .L001c56                        | +008
        jmp     0x518.l                         | +00c
.L001c56:
        cmpi.b  #0x6,0x106ed0.l                 | +012
        bcs.w   .L001c68                        | +01a
        jmp     0x518.l                         | +01e
.L001c68:
        move.w  #0xf,0x34(a6)                   | +024
        lea     .L001c74(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L001c74:
        subi.w  #0x1,0x34(a6)                   | +030
        bne.w   .L001c86                        | +036
        lea     0x7a970.l,a1                    | +03a
        move.l  a1,(a6)                         | +040
.L001c86:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  PcThunkTarget_001C88  @ $001C88  (36 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_001C88, "ax", @progbits
        .global PcThunkTarget_001C88
PcThunkTarget_001C88:
        move.w  #0x4b0,0x22(a6)                 | +000
        lea     .L001c94(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L001c94:
        subi.w  #0x1,0x22(a6)                   | +00c
        bne.w   .L001caa                        | +012
        clr.b   0x106ed2.l                      | +016
        jmp     0x518.l                         | +01c
.L001caa:
        rts                                     | +022

| ----------------------------------------------------------------------------
|  TaskHandler_001cac  @ $001CAC  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001cac, "ax", @progbits
        .global TaskHandler_001cac
TaskHandler_001cac:
        lea     0x10fdb6.l,a0                   | +000
        move.b  (a0),d0                         | +006
        move.b  0x1(a0),d2                      | +008
        cmpi.b  #0x1,d0                         | +00c
        beq.w   ClearXN_001cce                  | +010
        cmpi.b  #0x1,d2                         | +014
        beq.w   ClearXN_001cce                  | +018

| ----------------------------------------------------------------------------
|  PcThunkTarget_001D3C  @ $001D3C  (96 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_001D3C, "ax", @progbits
        .global PcThunkTarget_001D3C
PcThunkTarget_001D3C:
        lea     0x100260.l,a0                   | +000
        jsr     0x626.l                         | +006
        lea     0x100800.l,a0                   | +00c
        jsr     0x626.l                         | +012
        lea     0x1008a0.l,a0                   | +018
        jsr     0x626.l                         | +01e
        lea     0x100440.l,a0                   | +024
        jsr     0x626.l                         | +02a
        lea     0x1004e0.l,a0                   | +030
        jsr     0x626.l                         | +036
        lea     0x100580.l,a0                   | +03c
        jsr     0x626.l                         | +042
        lea     0x100300.l,a0                   | +048
        jsr     0x5da.l                         | +04e
        lea     0x1003a0.l,a0                   | +054
        jsr     0x5da.l                         | +05a

| ----------------------------------------------------------------------------
|  Sub_00001DA4  @ $001DA4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00001DA4, "ax", @progbits
        .global Sub_00001DA4
Sub_00001DA4:
        lea     ScriptSlotPairTable_0009B4__L0009fa(pc),a0 | +000
        jsr     0x2aec.l                        | +004
        lea     ScriptSlotPairTable_0009B4(pc),a0 | +00a
        jmp     0x2b58.l                        | +00e

| ----------------------------------------------------------------------------
|  Sub_00001DB8  @ $001DB8  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00001DB8, "ax", @progbits
        .global Sub_00001DB8
Sub_00001DB8:
        move.b  #0x5,0x1081be.l                 | +000
        jsr     0x52050.l                       | +008
        jmp     0x5223a.l                       | +00e

| ----------------------------------------------------------------------------
|  PcThunkTarget_001DCC  @ $001DCC  (54 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_001DCC, "ax", @progbits
        .global PcThunkTarget_001DCC
PcThunkTarget_001DCC:
        lea     0x1008a0.l,a0                   | +000
        move.l  #0x5ea96,(a0)                   | +006
        jsr     0x5fe.l                         | +00c
        lea     0x100800.l,a0                   | +012
        move.l  #0x442e6,(a0)                   | +018
        jsr     0x5fe.l                         | +01e
        btst    #0x0,0x100001.l                 | +024
        beq.w   JsrAbsRts_001e08                | +02c
        lea     0x5efca.l,a1                    | +030

| ----------------------------------------------------------------------------
|  Sub_00001E0A  @ $001E0A  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00001E0A, "ax", @progbits
        .global Sub_00001E0A
Sub_00001E0A:
        move.b  #0x1,0x106ecc.l                 | +000
        move.b  #0x1,0x106ecd.l                 | +008
        rts                                     | +010

| ----------------------------------------------------------------------------
|  PcThunkTarget_001E1C  @ $001E1C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_001E1C, "ax", @progbits
        .global PcThunkTarget_001E1C
PcThunkTarget_001E1C:
        clr.w   d0                              | +000
        clr.w   d1                              | +002
        clr.w   d2                              | +004
        jmp     0x1390e.l                       | +006

| ----------------------------------------------------------------------------
|  TaskHandler_001e28  @ $001E28  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001e28, "ax", @progbits
        .global TaskHandler_001e28
TaskHandler_001e28:
        lea     0x1001c0.l,a0                   | +000
        tst.b   0x45(a0)                        | +006
        bne.w   ClearC_001e46                   | +00a
        tst.b   0x10e39c.l                      | +00e
        bne.w   ClearC_001e46                   | +014

| ----------------------------------------------------------------------------
|  TaskHandler_001e4c  @ $001E4C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001e4c, "ax", @progbits
        .global TaskHandler_001e4c
TaskHandler_001e4c:
        move.b  #0xff,0x106ed6.l                | +000
        rts                                     | +008

| ----------------------------------------------------------------------------
|  TaskHandler_001e56  @ $001E56  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001e56, "ax", @progbits
        .global TaskHandler_001e56
TaskHandler_001e56:
        move.b  d0,0x106eda.l                   | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  TaskHandler_001efe  @ $001EFE  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001efe, "ax", @progbits
        .global TaskHandler_001efe
TaskHandler_001efe:
        move.w  0x106ee4.l,d0                   | +000
        move.w  d0,-(a7)                        | +006
.L001f06:
        move.b  0x106f26.l,d0                   | +008
        cmp.b   0x106f27.l,d0                   | +00e
        beq.w   .L001f40                        | +014
        addq.b  #0x1,d0                         | +018
        cmpi.b  #0x8,d0                         | +01a
        bcs.w   .L001f22                        | +01e
        sub.b   d0,d0                           | +022
.L001f22:
        move.b  d0,0x106f26.l                   | +024
        lea     0x106ee6.l,a0                   | +02a
        andi.w  #0xff,d0                        | +030
        lsl.w   #0x3,d0                         | +034
        movea.l 0x4(a0,d0.w),a6                 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     (a0)                            | +03e
        bra.b   .L001f06                        | +040
.L001f40:
        move.w  (a7)+,d0                        | +042
        move.w  d0,0x106ee4.l                   | +044
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  Fn_00001F4A  @ $001F4A  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Fn_00001F4A, "ax", @progbits
        .global Fn_00001F4A
Fn_00001F4A:
        move.b  0x106f27.l,d0                   | +000
        addq.b  #0x1,d0                         | +006
        cmpi.b  #0x8,d0                         | +008
        bcs.w   .L001f5c                        | +00c
        sub.b   d0,d0                           | +010
.L001f5c:
        cmp.b   0x106f26.l,d0                   | +012
        beq.w   .L001f82                        | +018
        move.b  d0,d1                           | +01c
        andi.w  #0xff,d0                        | +01e
        lsl.w   #0x3,d0                         | +022
        lea     0x106ee6.l,a1                   | +024
        move.l  a6,0x4(a1,d0.w)                 | +02a
        move.l  a0,(a1,d0.w)                    | +02e
        move.b  d1,0x106f27.l                   | +032
.L001f82:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  TaskHandler_001f84  @ $001F84  (276 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_001f84, "ax", @progbits
        .global TaskHandler_001f84
TaskHandler_001f84:
        move.b  #0x0,0x106f2a.l                 | +000
        move.b  #0x3,0x106f2b.l                 | +008
        clr.b   0x12(a6)                        | +010
        clr.b   0x13(a6)                        | +014
        clr.b   0x5b(a6)                        | +018
        clr.b   0x5a(a6)                        | +01c
        clr.b   0x106f26.l                      | +020
        clr.b   0x106f27.l                      | +026
        clr.b   0x106eda.l                      | +02c
        clr.b   0x106edb.l                      | +032
        clr.b   0x106edc.l                      | +038
        clr.b   0x106edd.l                      | +03e
        clr.b   0x106ed8.l                      | +044
        clr.b   0x10e48b.l                      | +04a
        bset    #0x7,0x10fd80.l                 | +050
        lea     .L001fe2(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L001fe2:
        addq.w  #0x1,0x106ee0.l                 | +05e
        clr.b   0x106edd.l                      | +064
        cmpi.b  #0x0,0x106ede.l                 | +06a
        beq.w   .L002004                        | +072
        cmpi.b  #0x1,0x106ed9.l                 | +076
        bls.b   .L001fe2                        | +07e
.L002004:
        tst.b   0x106ed8.l                      | +080
        beq.b   .L001fe2                        | +086
        clr.b   0x106ed9.l                      | +088
        jsr     0x5cc1a.l                       | +08e
        jsr     0x13c3c.l                       | +094
        bcc.w   .L002030                        | +09a
        clr.w   0x106ee2.l                      | +09e
        clr.b   0x106ed8.l                      | +0a4
        bra.b   .L001fe2                        | +0aa
.L002030:
        jsr     0x2242.l                        | +0ac
        move.b  d0,0x106f29.l                   | +0b2
        addq.b  #0x1,0x106f28.l                 | +0b8
        move.b  0x10e48b.l,d0                   | +0be
        move.b  d0,0x10e48a.l                   | +0c4
        clr.b   0x10e48b.l                      | +0ca
        jsr     0x1b40.l                        | +0d0
        move.w  0x106e8a.l,d0                   | +0d6
        move.w  d0,0x106e88.l                   | +0dc
        clr.w   0x106e8a.l                      | +0e2
        clr.w   0x106e8c.l                      | +0e8
        move.b  #0x1,0x106ede.l                 | +0ee
        move.l  a6,-(a7)                        | +0f6
        jsr     0x5a9ba.l                       | +0f8
        move.w  0x106ee0.l,d0                   | +0fe
        clr.w   0x106ee0.l                      | +104
        clr.w   0x106ee2.l                      | +10a
        movea.l (a7)+,a6                        | +110
        rts                                     | +112

| ----------------------------------------------------------------------------
|  TaskHandler_002098  @ $002098  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_002098, "ax", @progbits
        .global TaskHandler_002098
TaskHandler_002098:
        clr.b   0x12(a6)                        | +000
        clr.b   0x13(a6)                        | +004
        clr.b   0x5b(a6)                        | +008
        clr.b   0x5a(a6)                        | +00c
        move.l  a6,-(a7)                        | +010
        move.b  #0xff,0x106edc.l                | +012
        jsr     0x2c86.l                        | +01a
        move.b  d0,0x3a0001.l                   | +020
        move.b  #0xff,0x106edb.l                | +026
        clr.b   0x106edc.l                      | +02e
        jsr     0x5b232.l                       | +034
        move.b  d0,0x3a0001.l                   | +03a
        movea.l (a7)+,a6                        | +040

| ----------------------------------------------------------------------------
|  TaskHandler_0021a6  @ $0021A6  (124 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0021a6, "ax", @progbits
        .global TaskHandler_0021a6
TaskHandler_0021a6:
        cmpi.b  #0x20,d0                        | +000
        bcc.w   .L0021b0                        | +004
        rts                                     | +008
.L0021b0:
        move.w  d0,d1                           | +00a
        clr.b   d1                              | +00c
        cmpi.w  #0x100,d1                       | +00e
        beq.w   .L0021fe                        | +012
        cmpi.w  #0x300,d1                       | +016
        beq.w   .L0021fe                        | +01a
        cmpi.w  #0x200,d1                       | +01e
        beq.w   .L0021fe                        | +022
        cmpi.w  #0x1000,d1                      | +026
        bcc.w   .L0021de                        | +02a
        tst.w   d1                              | +02e
        beq.w   .L0021de                        | +030
        bra.w   .L002200                        | +034
.L0021de:
        tst.b   0x10fd82.l                      | +038
        beq.w   .L002200                        | +03e
        tst.b   0xd00046.l                      | +042
        beq.w   .L002200                        | +048
        cmpi.b  #0x1,0x10fdaf.l                 | +04c
        bne.w   .L002200                        | +054
.L0021fe:
        rts                                     | +058
.L002200:
        tst.b   0x1081aa.l                      | +05a
        beq.w   .L002214                        | +060
        cmpi.w  #0x1000,d0                      | +064
        bcs.w   .L002214                        | +068
        rts                                     | +06c
.L002214:
        move.w  d0,-(a7)                        | +06e
        lsr.w   #0x8,d0                         | +070
        bsr.w   InputQueue_InitAndPushOp4_00212E__L002152 | +072
        move.w  (a7)+,d0                        | +076
        bra.w   InputQueue_InitAndPushOp4_00212E__L002152 | +078

| ----------------------------------------------------------------------------
|  TaskHandler_002222  @ $002222  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_002222, "ax", @progbits
        .global TaskHandler_002222
TaskHandler_002222:
        move.w  d0,-(a7)                        | +000
        moveq   #12,d0                          | +002
        bsr.w   InputQueue_InitAndPushOp4_00212E__L002152 | +004
        move.w  (a7)+,d0                        | +008
        bra.w   ClampD0ToRange                  | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_002230  @ $002230  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_002230, "ax", @progbits
        .global TaskHandler_002230
TaskHandler_002230:
        move.b  #0xff,0x1081aa.l                | +000
        rts                                     | +008

| ----------------------------------------------------------------------------
|  TaskHandler_002242  @ $002242  (40 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_002242, "ax", @progbits
        .global TaskHandler_002242
TaskHandler_002242:
        move.b  0x1081a8.l,d0                   | +000
        cmp.b   0x1081a9.l,d0                   | +006
        beq.w   .L002264                        | +00c
        move.b  d0,0x1081a9.l                   | +010
        bpl.w   .L002264                        | +016
        subi.b  #0x80,d0                        | +01a
        lsr.b   #0x1,d0                         | +01e
        rts                                     | +020
.L002264:
        move.b  #0xff,d0                        | +022
        rts                                     | +026

| ----------------------------------------------------------------------------
|  TaskHandler_00226a  @ $00226A  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_00226a, "ax", @progbits
        .global TaskHandler_00226a
TaskHandler_00226a:
        tst.b   0x10007a.l                      | +000
        beq.w   .L002286                        | +006
        clr.b   0x10007a.l                      | +00a
        move.b  #0xf,0x320000.l                 | +010
        bra.w   .L0022bc                        | +018
.L002286:
        move.b  0x320000.l,d0                   | +01c
        move.b  d0,0x1081a8.l                   | +022
        move.w  0x1081a4.l,d1                   | +028
        cmp.w   0x1081a6.l,d1                   | +02e
        beq.w   .L0022bc                        | +034
        lea     0x108184.l,a0                   | +038
        move.b  (a0,d1.w),0x320000.l            | +03e
        addq.w  #0x1,d1                         | +046
        andi.w  #0x1f,d1                        | +048
        move.w  d1,0x1081a4.l                   | +04c
.L0022bc:
        rts                                     | +052

| ----------------------------------------------------------------------------
|  TaskHandler_0022c6  @ $0022C6  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0022c6, "ax", @progbits
        .global TaskHandler_0022c6
TaskHandler_0022c6:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Sub_000022C8  @ $0022C8  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000022C8, "ax", @progbits
        .global Sub_000022C8
Sub_000022C8:
        moveq   #6,d0                           | +000
        bra.w   InputQueue_InitAndPushOp4_00212E__L002152 | +002

| ----------------------------------------------------------------------------
|  TaskHandler_0022ce  @ $0022CE  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0022ce, "ax", @progbits
        .global TaskHandler_0022ce
TaskHandler_0022ce:
        moveq   #5,d0                           | +000
        bra.w   InputQueue_InitAndPushOp4_00212E__L002152 | +002

| ----------------------------------------------------------------------------
|  TaskHandler_0022d4  @ $0022D4  (52 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0022d4, "ax", @progbits
        .global TaskHandler_0022d4
TaskHandler_0022d4:
        addi.b  #0x20,d0                        | +000
        scs.b   d2                              | +004
        or.b    d2,d0                           | +006
        subi.b  #0x20,d1                        | +008
        scc.b   d2                              | +00c
        and.b   d2,d1                           | +00e
        addi.b  #0x20,d1                        | +010
        move.b  d1,-(a7)                        | +014
        move.b  d0,-(a7)                        | +016
        move.b  #0x7,d0                         | +018
        bsr.w   InputQueue_InitAndPushOp4_00212E__L002152 | +01c
        move.b  (a7)+,d0                        | +020
        bsr.w   InputQueue_InitAndPushOp4_00212E__L002152 | +022
        move.b  #0x8,d0                         | +026
        bsr.w   InputQueue_InitAndPushOp4_00212E__L002152 | +02a
        move.b  (a7)+,d0                        | +02e
        bra.w   InputQueue_InitAndPushOp4_00212E__L002152 | +030

| ----------------------------------------------------------------------------
|  Sub_00002308  @ $002308  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00002308, "ax", @progbits
        .global Sub_00002308
Sub_00002308:
        move.b  d0,-(a7)                        | +000
        move.b  #0xa,d0                         | +002
        bra.w   .L002318                        | +006
        move.b  d0,-(a7)                        | +00a
        move.b  #0xb,d0                         | +00c
.L002318:
        bsr.w   InputQueue_InitAndPushOp4_00212E__L002152 | +010
        move.b  (a7)+,d0                        | +014
        addi.b  #0x20,d0                        | +016
        scs.b   d1                              | +01a
        or.w    d1,d0                           | +01c
        bra.w   InputQueue_InitAndPushOp4_00212E__L002152 | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_00232a  @ $00232A  (40 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_00232a, "ax", @progbits
        .global TaskHandler_00232a
TaskHandler_00232a:
        move.b  d0,-(a7)                        | +000
        move.b  d1,-(a7)                        | +002
        move.b  #0x9,d0                         | +004
        bsr.w   InputQueue_InitAndPushOp4_00212E__L002152 | +008
        move.b  (a7)+,d0                        | +00c
        addi.b  #0x20,d0                        | +00e
        scs.b   d1                              | +012
        or.w    d1,d0                           | +014
        bsr.w   InputQueue_InitAndPushOp4_00212E__L002152 | +016
        move.b  #0x0,d0                         | +01a
        bsr.w   InputQueue_InitAndPushOp4_00212E__L002152 | +01e
        move.b  (a7)+,d0                        | +022
        bra.w   InputQueue_InitAndPushOp4_00212E__L002152 | +024

| ----------------------------------------------------------------------------
|  TaskHandler_0029f2  @ $0029F2  (138 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0029f2, "ax", @progbits
        .global TaskHandler_0029f2
TaskHandler_0029f2:
        cmpi.w  #0x3,0x1e(a6)                   | +000
        bge.w   .L002a7a                        | +006
        tst.w   0x1082c2.l                      | +00a
        beq.b   Rts_shared_29A6                 | +010
        andi.l  #0xffff,d1                      | +012
        lsl.l   #0x6,d1                         | +018
        lea     0x14e00.l,a1                    | +01a
        add.l   a1,d1                           | +020
        subq.w  #0x1,0x1082c2.l                 | +022
        move.w  0x1082c4.l,d3                   | +028
        addq.w  #0x1,0x1082c4.l                 | +02e
        cmpi.w  #0x30,0x1082c4.l                | +034
        blt.w   .L002a38                        | +03c
        clr.w   0x1082c4.l                      | +040
.L002a38:
        lea     0x1081c2.l,a2                   | +046
        moveq   #0,d2                           | +04c
        move.b  (a2,d3.w),d2                    | +04e
        addi.w  #0x94,d2                        | +052
        move.w  d2,d4                           | +056
        lsl.w   #0x5,d2                         | +058
        lea     0x1082c8.l,a1                   | +05a
        move.l  d1,0x2(a1,d2.w)                 | +060
        addq.w  #0x1,0x6(a1,d2.w)               | +064
        move.b  #0x4,(a1,d2.w)                  | +068
        ori.b   #0x1,(a1,d2.w)                  | +06e
        move.w  d4,d2                           | +074
        move.w  d2,0x14(a6)                     | +076
        move.w  0x1e(a6),d1                     | +07a
        add.w   d1,d1                           | +07e
        move.w  d2,0x16(a6,d1.w)                | +080
        addq.w  #0x1,0x1e(a6)                   | +084
.L002a7a:
        rts                                     | +088

| ----------------------------------------------------------------------------
|  TaskHandler_002a7c  @ $002A7C  (220 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_002a7c, "ax", @progbits
        .global TaskHandler_002a7c
TaskHandler_002a7c:
        move.w  (a0),d1                         | +000
        cmpi.w  #0xffff,d1                      | +002
        beq.w   .L002ad8                        | +006
        andi.l  #0xff,d1                        | +00a
        cmpi.w  #0x74,d1                        | +010
        bgt.w   .L002ae8                        | +014
        cmpi.w  #0x10,d1                        | +018
        blt.w   .L002ae8                        | +01c
        lsl.w   #0x5,d1                         | +020
        lea     0x1082c8.l,a1                   | +022
        move.w  0x2(a0),d2                      | +028
        cmpi.w  #0xffff,d2                      | +02c
        beq.w   .L002ada                        | +030
        andi.l  #0xffff,d2                      | +034
        lsl.l   #0x6,d2                         | +03a
        lea     0x14e00.l,a2                    | +03c
        add.l   a2,d2                           | +042
        move.l  d2,0x2(a1,d1.w)                 | +044
        move.w  #0x1,0x6(a1,d1.w)               | +048
        move.b  (a0),(a1,d1.w)                  | +04e
        ori.b   #0x1,(a1,d1.w)                  | +052
        addq.l  #0x4,a0                         | +058
        bra.b   TaskHandler_002a7c              | +05a
.L002ad8:
        rts                                     | +05c
.L002ada:
        clr.w   0x6(a1,d1.w)                    | +05e
        move.b  #0x40,(a1,d1.w)                 | +062
        addq.l  #0x4,a0                         | +068
        bra.b   TaskHandler_002a7c              | +06a
        .global TaskHandler_002a7c__L002ae8
TaskHandler_002a7c__L002ae8:
.L002ae8:
        trap    #0xf                            | +06c
        bra.b   .L002ae8                        | +06e
.L002aec:
        move.w  (a0),d1                         | +070
        cmpi.w  #0xffff,d1                      | +072
        beq.w   .L002b48                        | +076
        andi.l  #0xff,d1                        | +07a
        cmpi.w  #0x20,d1                        | +080
        bgt.b   .L002ae8                        | +084
        cmpi.w  #0x0,d1                         | +086
        blt.b   .L002ae8                        | +08a
        addi.w  #0x74,d1                        | +08c
        lsl.w   #0x5,d1                         | +090
        lea     0x1082c8.l,a1                   | +092
        move.w  0x2(a0),d2                      | +098
        cmpi.w  #0xffff,d2                      | +09c
        beq.w   .L002b4a                        | +0a0
        andi.l  #0xffff,d2                      | +0a4
        lsl.l   #0x6,d2                         | +0aa
        lea     0x14e00.l,a2                    | +0ac
        add.l   a2,d2                           | +0b2
        move.l  d2,0x2(a1,d1.w)                 | +0b4
        move.w  #0x1,0x6(a1,d1.w)               | +0b8
        move.b  (a0),(a1,d1.w)                  | +0be
        ori.b   #0x1,(a1,d1.w)                  | +0c2
        addq.l  #0x4,a0                         | +0c8
        bra.b   .L002aec                        | +0ca
.L002b48:
        rts                                     | +0cc
.L002b4a:
        clr.w   0x6(a1,d1.w)                    | +0ce
        move.b  #0x40,(a1,d1.w)                 | +0d2
        addq.l  #0x4,a0                         | +0d8
        bra.b   .L002aec                        | +0da

| ----------------------------------------------------------------------------
|  Sub_00002B58  @ $002B58  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00002B58, "ax", @progbits
        .global Sub_00002B58
Sub_00002B58:
        move.w  (a0),d1                         | +000
        cmpi.w  #0xffff,d1                      | +002
        beq.w   .L002bb4                        | +006
        andi.l  #0xff,d1                        | +00a
        cmpi.w  #0xff,d1                        | +010
        beq.w   .L002b78                        | +014
        cmpi.w  #0x74,d1                        | +018
        bgt.w   TaskHandler_002a7c__L002ae8     | +01c
.L002b78:
        lsl.w   #0x5,d1                         | +020
        lea     0x1082c8.l,a1                   | +022
        move.w  0x2(a0),d2                      | +028
        cmpi.w  #0xffff,d2                      | +02c
        beq.w   .L002bb6                        | +030
        andi.l  #0xffff,d2                      | +034
        lsl.l   #0x6,d2                         | +03a
        lea     0x1ce00.l,a2                    | +03c
        add.l   a2,d2                           | +042
        move.l  d2,0x2(a1,d1.w)                 | +044
        move.w  #0x1,0x6(a1,d1.w)               | +048
        move.b  (a0),(a1,d1.w)                  | +04e
        ori.b   #0x1,(a1,d1.w)                  | +052
        addq.l  #0x4,a0                         | +058
        bra.b   Sub_00002B58                    | +05a
.L002bb4:
        rts                                     | +05c
.L002bb6:
        clr.w   0x6(a1,d1.w)                    | +05e
        move.b  #0x40,(a1,d1.w)                 | +062
        addq.l  #0x4,a0                         | +068
        bra.b   Sub_00002B58                    | +06a

| ----------------------------------------------------------------------------
|  Sub_00002BC4  @ $002BC4  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00002BC4, "ax", @progbits
        .global Sub_00002BC4
Sub_00002BC4:
        movem.l d1-d3/a1-a2,-(a7)               | +000
        move.w  d1,d2                           | +004
        lsl.w   #0x5,d1                         | +006
        lea     0x1082c8.l,a1                   | +008
        subq.w  #0x1,0x6(a1,d1.w)               | +00e
        bne.w   .L002c20                        | +012
        move.b  #0x80,(a1,d1.w)                 | +016
        ori.b   #0x8,(a1,d1.w)                  | +01c
        clr.l   0x2(a1,d1.w)                    | +022
        clr.w   0x8(a1,d1.w)                    | +026
        move.w  0x1082c6.l,d1                   | +02a
        lea     0x1081c2.l,a1                   | +030
        subi.w  #0x94,d2                        | +036
        move.b  d2,(a1,d1.w)                    | +03a
        addq.w  #0x1,0x1082c6.l                 | +03e
        cmpi.w  #0x30,0x1082c6.l                | +044
        blt.w   .L002c1a                        | +04c
        clr.w   0x1082c6.l                      | +050
.L002c1a:
        addq.w  #0x1,0x1082c2.l                 | +056
.L002c20:
        movem.l (a7)+,d1-d3/a1-a2               | +05c
        rts                                     | +060

| ----------------------------------------------------------------------------
|  TaskHandler_002c66  @ $002C66  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_002c66, "ax", @progbits
        .global TaskHandler_002c66
TaskHandler_002c66:
        lea     0x1082c8.l,a1                   | +000
        lsl.w   #0x5,d1                         | +006
        ori.b   #0x8,(a1,d1.w)                  | +008
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_002c76  @ $002C76  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_002c76, "ax", @progbits
        .global TaskHandler_002c76
TaskHandler_002c76:
        lea     0x1082c8.l,a1                   | +000
        lsl.w   #0x5,d1                         | +006
        andi.b  #0xf7,(a1,d1.w)                 | +008
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_002c86  @ $002C86  (682 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_002c86, "ax", @progbits
        .global TaskHandler_002c86
TaskHandler_002c86:
        cmpi.b  #0xff,0x10a2d2.l                | +000
        beq.w   .L002c9e                        | +008
        lea     0x1387e.l,a0                    | +00c
        jsr     0x1f4a.l                        | +012
.L002c9e:
        moveq   #0,d4                           | +018
        clr.b   0x10a2cd.l                      | +01a
        clr.b   0x10a2ce.l                      | +020
        move.b  0x10a2c8.l,d0                   | +026
        bne.w   .L002d88                        | +02c
        move.b  0x10a2c9.l,d0                   | +030
        beq.w   .L002ce0                        | +036
        cmpi.b  #0x80,d0                        | +03a
        beq.w   .L002ce0                        | +03e
        moveq   #0,d1                           | +042
        move.b  d1,0x10a2ca.l                   | +044
        move.b  d1,0x10a2cb.l                   | +04a
        move.b  d1,0x10a2cc.l                   | +050
        bra.w   .L002d88                        | +056
.L002ce0:
        lea     0x1082c8.l,a1                   | +05a
        moveq   #0,d1                           | +060
        move.w  #0xc3,d0                        | +062
.L002cec:
        btst    #0x7,(a1,d1.w)                  | +066
        bne.w   .L002d52                        | +06c
        btst    #0x1,(a1,d1.w)                  | +070
        beq.w   .L002d0e                        | +076
        jsr     0x13408.l                       | +07a
        move.b  #0xff,0x10a2ce.l                | +080
.L002d0e:
        btst    #0x0,(a1,d1.w)                  | +088
        beq.w   .L002d32                        | +08e
        movea.l 0x2(a1,d1.w),a2                 | +092
        lea     0x10a2d4.l,a3                   | +096
        adda.l  d1,a3                           | +09c
        jsr     0x133b0.l                       | +09e
        move.b  #0xff,0x10a2ce.l                | +0a4
.L002d32:
        btst    #0x6,(a1,d1.w)                  | +0ac
        beq.w   .L002d52                        | +0b2
        lea     0x10a2d4.l,a3                   | +0b6
        adda.l  d1,a3                           | +0bc
        jsr     0x133e6.l                       | +0be
        move.b  #0xff,0x10a2ce.l                | +0c4
.L002d52:
        addi.w  #0x20,d1                        | +0cc
        dbra    d0,.L002cec                     | +0d0
        tst.b   0x10a2cd.l                      | +0d4
        bne.w   .L002d7c                        | +0da
        move.b  #0xff,0x10a2cd.l                | +0de
        lea     0x1082c8.l,a1                   | +0e6
        move.w  #0x1fe0,d1                      | +0ec
        clr.w   d0                              | +0f0
        bra.w   .L002cec                        | +0f2
.L002d7c:
        move.b  0x10a2c8.l,0x10a2c9.l           | +0f6
        rts                                     | +100
.L002d88:
        cmpi.b  #0xff,d0                        | +102
        beq.w   .L002e62                        | +106
        lea     0x1082c8.l,a1                   | +10a
        moveq   #0,d1                           | +110
        move.w  #0xc3,d0                        | +112
.L002d9c:
        btst    #0x7,(a1,d1.w)                  | +116
        bne.w   .L002e30                        | +11c
        btst    #0x1,(a1,d1.w)                  | +120
        beq.w   .L002db6                        | +126
        jsr     0x13408.l                       | +12a
.L002db6:
        btst    #0x1,(a1,d1.w)                  | +130
        bne.w   .L002e30                        | +136
        movea.l 0x2(a1,d1.w),a2                 | +13a
        lea     0x10a2d4.l,a3                   | +13e
        adda.l  d1,a3                           | +144
        btst    #0x6,(a1,d1.w)                  | +146
        beq.w   .L002de0                        | +14c
        jsr     0x133e6.l                       | +150
        bra.w   .L002e30                        | +156
.L002de0:
        btst    #0x3,(a1,d1.w)                  | +15a
        beq.w   .L002dfe                        | +160
        btst    #0x0,(a1,d1.w)                  | +164
        beq.w   .L002e30                        | +16a
        jsr     0x133b0.l                       | +16e
        bra.w   .L002e30                        | +174
.L002dfe:
        movea.l a3,a4                           | +178
        addq.w  #0x2,a3                         | +17a
        addq.l  #0x4,a2                         | +17c
        move.w  #0xe,d5                         | +17e
.L002e08:
        move.b  (a2)+,d2                        | +182
        move.b  (a2)+,d3                        | +184
        move.b  (a2)+,d4                        | +186
        addq.l  #0x1,a2                         | +188
        jsr     0x13624.l                       | +18a
        lea     Sub_00002F30(pc),a0             | +190  -> $002F30 (hueco futuro, defsym forward)
        add.w   d4,d4                           | +194
        adda.l  d4,a0                           | +196
        move.w  (a0),d4                         | +198
        move.w  d4,(a3)+                        | +19a
        dbra    d5,.L002e08                     | +19c
        andi.b  #0xfe,(a1,d1.w)                 | +1a0
        move.w  #0x1,(a4)                       | +1a6
.L002e30:
        addi.w  #0x20,d1                        | +1aa
        dbra    d0,.L002d9c                     | +1ae
        move.b  #0xff,0x10a2ce.l                | +1b2
        tst.b   0x10a2cd.l                      | +1ba
        bne.w   .L002d7c                        | +1c0
        move.b  #0xff,0x10a2cd.l                | +1c4
        lea     0x1082c8.l,a1                   | +1cc
        move.w  #0x1fe0,d1                      | +1d2
        clr.w   d0                              | +1d6
        bra.w   .L002d9c                        | +1d8
.L002e62:
        lea     0x1082c8.l,a1                   | +1dc
        moveq   #0,d1                           | +1e2
        move.w  #0xc3,d0                        | +1e4
.L002e6e:
        btst    #0x7,(a1,d1.w)                  | +1e8
        bne.w   .L002efe                        | +1ee
        btst    #0x1,(a1,d1.w)                  | +1f2
        beq.w   .L002e88                        | +1f8
        jsr     0x13408.l                       | +1fc
.L002e88:
        btst    #0x1,(a1,d1.w)                  | +202
        bne.w   .L002efe                        | +208
        movea.l 0x2(a1,d1.w),a2                 | +20c
        lea     0x10a2d4.l,a3                   | +210
        adda.l  d1,a3                           | +216
        btst    #0x6,(a1,d1.w)                  | +218
        beq.w   .L002eb2                        | +21e
        jsr     0x133e6.l                       | +222
        bra.w   .L002efe                        | +228
.L002eb2:
        btst    #0x3,(a1,d1.w)                  | +22c
        beq.w   .L002ed0                        | +232
        btst    #0x0,(a1,d1.w)                  | +236
        beq.w   .L002efe                        | +23c
        jsr     0x133b0.l                       | +240
        bra.w   .L002efe                        | +246
.L002ed0:
        movea.l a3,a4                           | +24a
        addq.w  #0x2,a3                         | +24c
        addq.l  #0x4,a2                         | +24e
        move.w  #0xe,d5                         | +250
.L002eda:
        move.b  (a2)+,d2                        | +254
        move.b  (a2)+,d3                        | +256
        move.b  (a2)+,d4                        | +258
        addq.l  #0x1,a2                         | +25a
        jsr     0x13694.l                       | +25c
        add.w   d4,d4                           | +262
        move.w  TaskHandler_002c86+0x2aa(pc,d4.l),d4 | +264  -> $002F30 (hueco futuro, defsym forward)
        move.w  d4,(a3)+                        | +268
        dbra    d5,.L002eda                     | +26a
        andi.b  #0xfe,(a1,d1.w)                 | +26e
        move.w  #0x1,(a4)                       | +274
.L002efe:
        addi.w  #0x20,d1                        | +278
        dbra    d0,.L002e6e                     | +27c
        move.b  #0xff,0x10a2ce.l                | +280
        tst.b   0x10a2cd.l                      | +288
        bne.w   .L002d7c                        | +28e
        move.b  #0xff,0x10a2cd.l                | +292
        lea     0x1082c8.l,a1                   | +29a
        move.w  #0x1fe0,d1                      | +2a0
        clr.w   d0                              | +2a4
        bra.w   .L002e6e                        | +2a6
