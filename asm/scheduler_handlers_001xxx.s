| ============================================================================
|  Metal Slug 1 - asm/scheduler_handlers_001xxx.s
|  ----------------------------------------------------------------------------
|  Wave MM batch 3 - handlers de codigo apuntados por la super-tabla dispatch
|  $000B92 (Wave MM batch 2) en la zona `$00109C..$00125E`.
|
|  Contenido (8 funciones, 438 bytes):
|
|      $00109C   AttractHandler_00109C              76 B  handler attract
|      $0010E8   SchedTail_JsrDB8_0010E8             4 B  handler-tail micro
|      $0010F2   AttractHandler_2Task_0010F2        86 B  handler attract (2 tasks)
|      $001148   AttractPhase2_Gate106ED5_001148    34 B  fase 2 con gate $106ED5
|      $001172   AttractHandler_Frame_001172        88 B  handler attract w/ frame_ctr
|      $0011CA   AttractPhase2_Multiway_0011CA      32 B  fase 2 multi-way
|      $0011EA   AttractHandler_Loader_0011EA       68 B  handler w/ jsr $2B58 loader
|      $00122E   AttractPhase2_Probes5D0_00122E     50 B  fase 2 w/ probes $5D09A/$5D0AC
|
|  Todos son handlers invocados por `jmp (a0)` desde SchedulerLoopA_000FC6
|  (Wave MM#1) tras que este haya deref el cursor de la super-tabla. Todos
|  (salvo el ultimo AttractPhase2_Probes5D0) terminan con `bra.w $FE0`
|  (re-entry al bucle B) - patron threaded continuation-passing (MM#1).
|
|  Toolchain: m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  AttractHandler_00109C  @ $00109C  (76 bytes)
| ---------------------------------------------------------------------------
|
        .globl  AttractHandler_00109C
        .type   AttractHandler_00109C, @function
        .section .text.AttractHandler_00109C, "ax", @progbits
AttractHandler_00109C:
        moveq   #-1, d0                         | +00  d0 = $FF (moveq signed)
        jsr     Sub_00024FEC                    | +02  PlayerCtx_ResetTwoBlocks (HH#3)
        jsr     FUN_0000212e                    | +08  hook $212E
        jsr     BIOS_FIX_CLEAR                  | +0e  BIOS VBlank ($C004C2)
        jsr     Pubcleaner_10A2Cx_052712        | +14  Pubcleaner_10A2Cx (LL#1)
        lea.l   TaskTpl_091330, a1              | +1a  a1 = &task_tpl_$91330
        jsr     ThunkTarget_0004ae              | +20  Task_Alloc
        move.b  #0xff, 0x106ece.l               | +26  key latch A = -1
        move.b  #0xff, 0x106ecf.l               | +2e  key latch B = -1
        jsr     PcThunkTarget_001af8(pc)        | +36  hook @ $1AF8 (pc-rel)
        jsr     Sub_00046AC6                    | +3a  FixLayer_QuadBatch (HH#3)
        move.b  #0xff, 0x106ed2.l               | +40  hud_dirty = -1
        bra.w   SchedulerDispatch_LoopB_000FE0  | +48  goto loop B

        .size   AttractHandler_00109C, .-AttractHandler_00109C

|
| ---------------------------------------------------------------------------
|  SchedTail_JsrDB8_0010E8  @ $0010E8  (4 bytes)
| ---------------------------------------------------------------------------
|
        .globl  SchedTail_JsrDB8_0010E8
        .type   SchedTail_JsrDB8_0010E8, @function
        .section .text.SchedTail_JsrDB8_0010E8, "ax", @progbits
SchedTail_JsrDB8_0010E8:
        bsr.w   Sub_00001DB8                    | +00  hook $1DB8 (fallthru $0010EC)

        .size   SchedTail_JsrDB8_0010E8, .-SchedTail_JsrDB8_0010E8

|
| ---------------------------------------------------------------------------
|  AttractHandler_2Task_0010F2  @ $0010F2  (86 bytes)
| ---------------------------------------------------------------------------
|
        .globl  AttractHandler_2Task_0010F2
        .type   AttractHandler_2Task_0010F2, @function
        .section .text.AttractHandler_2Task_0010F2, "ax", @progbits
AttractHandler_2Task_0010F2:
        jsr     FUN_0000212e                    | +00  hook $212E
        jsr     BIOS_FIX_CLEAR                  | +06  BIOS VBlank
        jsr     Pubcleaner_10A2Cx_052712        | +0c  Pubcleaner_10A2Cx (LL#1)
        lea.l   TaskTpl_0913AC, a1              | +12  a1 = &task_tpl_$913AC
        jsr     ThunkTarget_0004ae              | +18  Task_Alloc (task 1)
        lea.l   TaskTpl_099B06, a1              | +1e  a1 = &task_tpl_$99B06
        jsr     ThunkTarget_0004ae              | +24  Task_Alloc (task 2)
        move.w  #0x10e0, d0                     | +2a  d0 = $10E0
        jsr     InputGuardCall219c              | +2e  input guard w/ d0
        move.b  #0xff, 0x106ece.l               | +34  key latch A = -1
        move.b  #0xff, 0x106ecf.l               | +3c  key latch B = -1
        jsr     Sub_00046AC6                    | +44  FixLayer_QuadBatch (HH#3)
        move.b  #0xff, 0x106ed2.l               | +4a  hud_dirty = -1
        bra.w   SchedulerDispatch_LoopB_000FE0  | +52  goto loop B

        .size   AttractHandler_2Task_0010F2, .-AttractHandler_2Task_0010F2

|
| ---------------------------------------------------------------------------
|  AttractPhase2_Gate106ED5_001148  @ $001148  (34 bytes)
| ---------------------------------------------------------------------------
|
        .globl  AttractPhase2_Gate106ED5_001148
        .type   AttractPhase2_Gate106ED5_001148, @function
        .section .text.AttractPhase2_Gate106ED5_001148, "ax", @progbits
AttractPhase2_Gate106ED5_001148:
        bsr.w   Sub_00001DB8                    | +00  hook $1DB8
        tst.b   0x106ed2.l                      | +04  if (hud_dirty)
        bne.w   .Lgate_ed5_fallthru             | +0a   goto fallthru ($1170)
        jsr     FUN_000005B6                    | +0e  refresh handler
        tst.b   0x106ed5.l                      | +14  if (gate_ED5)
        bne.w   .Lgate_ed5_ret                  | +1a   goto ret ($116A)
        bra.w   SchedulerDispatch_LoopB_000FE0  | +1e  goto loop B

        .size   AttractPhase2_Gate106ED5_001148, .-AttractPhase2_Gate106ED5_001148

| Symbolos externos para .bne.w targets (targets absolutos)
        .set    .Lgate_ed5_ret,       0x0000116A
        .set    .Lgate_ed5_fallthru,  0x00001170

|
| ---------------------------------------------------------------------------
|  AttractHandler_Frame_001172  @ $001172  (88 bytes)
| ---------------------------------------------------------------------------
|
        .globl  AttractHandler_Frame_001172
        .type   AttractHandler_Frame_001172, @function
        .section .text.AttractHandler_Frame_001172, "ax", @progbits
AttractHandler_Frame_001172:
        move.w  0x10007c.l, d0                  | +00  d0 = frame_counter
        addq.w  #0x1, d0                        | +06  ++d0
        move.w  d0, 0x10007c.l                  | +08  frame_counter = d0
        jsr     Sub_0005E998                    | +0e  video update hook
        jsr     BIOS_FIX_CLEAR                  | +14  BIOS VBlank
        jsr     Sub_00046AC6                    | +1a  FixLayer_QuadBatch (HH#3)
        jsr     Pubcleaner_10A2Cx_052712        | +20  Pubcleaner_10A2Cx (LL#1)
        lea.l   TaskTpl_0977D6, a1              | +26  a1 = &task_tpl_$977D6
        jsr     ThunkTarget_0004ae              | +2c  Task_Alloc
        move.b  #0xff, 0x106ece.l               | +32  key latch A = -1
        move.b  #0xff, 0x106ecf.l               | +3a  key latch B = -1
        jsr     PcThunkTarget_001af8(pc)        | +42  hook @ $1AF8 (pc-rel)
        jsr     Sub_00046AC6                    | +46  FixLayer_QuadBatch (HH#3)
        move.b  #0xff, 0x106ed2.l               | +4c  hud_dirty = -1
        bra.w   SchedulerDispatch_LoopB_000FE0  | +54  goto loop B

        .size   AttractHandler_Frame_001172, .-AttractHandler_Frame_001172

|
| ---------------------------------------------------------------------------
|  AttractPhase2_Multiway_0011CA  @ $0011CA  (32 bytes)
| ---------------------------------------------------------------------------
|
        .globl  AttractPhase2_Multiway_0011CA
        .type   AttractPhase2_Multiway_0011CA, @function
        .section .text.AttractPhase2_Multiway_0011CA, "ax", @progbits
AttractPhase2_Multiway_0011CA:
        bsr.w   Sub_00001DB8                    | +00  hook $1DB8
        tst.b   0x106ed2.l                      | +04  if (hud_dirty)
        beq.w   .Lmulti_refresh                 | +0a   goto refresh
        bra.w   .Lmulti_tail                    | +0e  else goto tail
.Lmulti_refresh:                                | $0011DC
        jsr     FUN_000005B6                    | +12  refresh handler
        bra.w   SchedulerDispatch_LoopB_000FE0  | +18  goto loop B
.Lmulti_tail:                                   | $0011E6
        bra.w   PcThunkTarget_001af8            | +1c  tail-call to $1AF8

        .size   AttractPhase2_Multiway_0011CA, .-AttractPhase2_Multiway_0011CA

|
| ---------------------------------------------------------------------------
|  AttractHandler_Loader_0011EA  @ $0011EA  (68 bytes)
| ---------------------------------------------------------------------------
|
        .globl  AttractHandler_Loader_0011EA
        .type   AttractHandler_Loader_0011EA, @function
        .section .text.AttractHandler_Loader_0011EA, "ax", @progbits
AttractHandler_Loader_0011EA:
        jsr     BIOS_FIX_CLEAR                  | +00  BIOS VBlank
        jsr     Pubcleaner_10A2Cx_052712        | +06  Pubcleaner_10A2Cx (LL#1)
        lea.l   ScriptSlotPairTable_0009B4(pc), a0            | +0c  a0 = &ctx_$09B4 (pc-rel)
        jsr     Sub_00002B58                    | +10  loader $2B58 w/ a0
        lea.l   TaskTpl_0977EA, a1              | +16  a1 = &task_tpl_$977EA
        jsr     ThunkTarget_0004ae              | +1c  Task_Alloc
        move.b  #0xff, 0x106ece.l               | +22  key latch A = -1
        move.b  #0xff, 0x106ecf.l               | +2a  key latch B = -1
        jsr     Sub_00046AC6                    | +32  FixLayer_QuadBatch (HH#3)
        move.b  #0xff, 0x106ed2.l               | +38  hud_dirty = -1
        bra.w   SchedulerDispatch_LoopB_000FE0  | +40  goto loop B

        .size   AttractHandler_Loader_0011EA, .-AttractHandler_Loader_0011EA

|
| ---------------------------------------------------------------------------
|  AttractPhase2_Probes5D0_00122E  @ $00122E  (50 bytes)
| ---------------------------------------------------------------------------
|
        .globl  AttractPhase2_Probes5D0_00122E
        .type   AttractPhase2_Probes5D0_00122E, @function
        .section .text.AttractPhase2_Probes5D0_00122E, "ax", @progbits
AttractPhase2_Probes5D0_00122E:
        bsr.w   Sub_00001DB8                    | +00  hook $1DB8
        tst.b   0x106ed2.l                      | +04  if (hud_dirty)
        beq.w   .Lprobes_refresh                | +0a   goto refresh
        jsr     Sub_0005D09A                    | +0e  probe #1 CCR-C
        bcs.w   .Lprobes_refresh                | +14   if (C) goto refresh
        jsr     Sub_0005D0AC                    | +18  probe #2 CCR-C
        bcs.w   .Lprobes_refresh                | +1e   if (C) goto refresh
        bra.w   .Lprobes_end                    | +22  else goto end (rts)
.Lprobes_refresh:                               | $001254
        jsr     FUN_000005B6                    | +26  refresh handler
        bra.w   SchedulerDispatch_LoopB_000FE0  | +2c  goto loop B
.Lprobes_end:                                   | $00125E
        rts                                     | +30  UNICO rts del batch

        .size   AttractPhase2_Probes5D0_00122E, .-AttractPhase2_Probes5D0_00122E
