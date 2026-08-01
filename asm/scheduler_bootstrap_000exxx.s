| ============================================================================
|  Metal Slug 1 - asm/scheduler_bootstrap_000exxx.s
|  ----------------------------------------------------------------------------
|  Wave MM batch 1 - corazon del scheduler bootstrap del arranque BIOS.
|
|  Contenido (5 funciones, 526 bytes):
|
|      $000E8E   SchedulerBootstrap_Boot_000E8E   338 B  setup + selector +
|                                                       bucle dispatch A
|      $000FE0   SchedulerDispatch_LoopB_000FE0    64 B  bucle dispatch B
|                                                       + Publish_1081Bx tail
|      $001020   SchedTail_JsrCD4_001020            6 B  handler tail cola-in
|      $001026   SchedTail_JsrD3C_001026            6 B  handler tail cola-in
|      $00102C   AttractHandler_10002C            112 B  handler grande attract
|
|  Es el punto de entrada canonico del scheduler-by-script-table del juego,
|  invocado desde la reset vector chain despues de las Waves P (BIOS entry
|  points) y Y (Scheduler_MainLoop). Interpreta la super-tabla dispatch
|  situada en $000B92..$000E8E (760 B, 6 sub-tablas u32 BE con centinelas
|  $FFFFFFFF, 186 entradas totales, 93 handlers unicos) como bytecode
|  virtual: cada entrada u32 es un handler y $FFFFFFFF significa "avanza
|  cursor via el u32 siguiente" (patron continuation-passing).
|
|  ---------- Descubrimiento arquitectonico Wave MM batch 1 ----------------
|
|  1. **Super-tabla dispatch $000B92** documentada por primera vez. 6
|     sub-tablas concatenadas con centinelas $FFFFFFFF en $BC6, $D6E,
|     $D82, $D96, $E3A. Cada sub-tabla corresponde a un modo de arranque
|     seleccionado por los bytes de config BIOS $10FDAE / $10FDAF:
|
|       T1  $000B92..$000BC6  (13 entries)  Boot mode "$10FDAF=2"
|       T2  $000BCA..$000D6E  (81 entries)  Boot script principal
|       T3  $000D72..$000D82  (4 entries)   Boot sub-mode A
|       T4  $000D86..$000D96  (4 entries)   Boot sub-mode B
|       T5  $000D9A..$000E3A  (40 entries)  Boot script attract
|       T6  $000E3E..$000E8E  (20 entries)  Boot script demo/title
|
|     Se registrara como BootDispatchTable_000B92 en Wave MM batch 2 (760 B
|     como .long). Aqui solo consume via lea.l XXX(pc), a0.
|
|  2. **Bytecode virtual continuation-passing**: el bucle dispatch A
|     ($000FC6..$000FDE) es un interprete de 8 instrucciones:
|
|       .loop:
|         a0  = *cursor                (cursor = $70(a6))
|         if (*a0 == $FFFFFFFF)        (sub-table sentinel)
|             a0 = *(a0 + 4)           (jump to next sub-table)
|             *cursor = a0             (persist)
|         a0 = *a0                     (deref handler ptr)
|         jmp (a0)                     (execute; handler will bra $FE0
|                                       to advance cursor and re-enter)
|
|     Es la variante 68000 del "threaded interpreter" clasico. GCC no
|     genera este patron - usaria un switch/table + call convencional.
|
|  3. **Fall-through split**: SchedulerBootstrap_Boot ($000E8E) termina en
|     jmp (a0) en $000FDE. Los HANDLERS APUNTADOS por la super-tabla
|     retornan al bucle B ($000FE0) via `bra.b $FE0` en vez de rts, y B
|     avanza el cursor +4 y salta a $FC6 (loop A). Es la 7a aparicion del
|     idioma fall-through entre funciones vecinas del proyecto, y la 1a
|     como "hub compartido" entre >2 handlers.
|
|  4. **$70(a6) = cursor persistente**: el cursor de la super-tabla se
|     preserva en $70 del contexto scheduler activo. Todos los handlers
|     leen/escriben este offset. Es la variable global de estado del
|     interprete.
|
|  5. **Setup MMIO scheduler**: $10E1E4 / $10E1E6 son los limites del
|     scroll horizontal / vertical del BG layer, publicados a valores
|     iniciales ($F0 / $1E8) antes del primer frame. $10007C es el frame
|     counter global (incrementado en cada iteracion del dispatch).
|
|  ---------- Convencion ABI ------------------------------------------------
|
|  a6 = puntero al Task Control Block del scheduler activo. Todos los
|  handlers y helpers de este cluster preservan a6 (nunca lo modifican).
|  El cursor de super-tabla vive en $70(a6). El puntero al mainloop tail
|  se instala en (a6) para que el scheduler central lo re-entre cada frame.
|
|  Toolchain: m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  SchedulerBootstrap_Boot_000E8E  @ $000E8E  (338 bytes)
|
|  Punto de entrada del scheduler-by-script-table. Estructura:
|
|    1. Setup MMIO inicial (RTC clear + scroll limits + IRQ setup)
|    2. Selector 5-way por $10FDAF / $10FDAE (modo BIOS):
|         $10FDAF == 2                             -> branch mode-2
|         $10FDAF != 2 && $10FDAE == 3             -> branch title
|         $10FDAF != 2 && $10FDAE != 3             -> branch demo
|         Cada branch elige un handler-list en $BE6/$BDA/$BCE/$B92/$BBA/$E6E
|    3. Fall-through al mainloop tail: incrementa frame counter, invoca
|       audio update + video update + BIOS VBlank, instala tail_ptr en (a6)
|    4. Loop dispatch A: interpreta super-tabla via *cursor con
|       auto-avance sobre $FFFFFFFF sentinels y jmp (a0)
| ---------------------------------------------------------------------------
|
        .globl  SchedulerBootstrap_Boot_000E8E
        .type   SchedulerBootstrap_Boot_000E8E, @function
        .section .text.SchedulerBootstrap_Boot_000E8E, "ax", @progbits
SchedulerBootstrap_Boot_000E8E:
        jsr     0x5cace.l                       | +000 external init hook
        move.w  #0xf0, 0x10e1e4.l               | +006 BG scroll H limit
        move.w  #0x1e8, 0x10e1e6.l              | +00e BG scroll V limit
        move.w  #0x0, d0                        | +016 d0 = 0
        jsr     0x5e998.l                       | +01a external tick w/ d0=0
        clr.b   0x12(a6)                        | +020 tcb.flag_12 = 0
        clr.b   0x13(a6)                        | +024 tcb.flag_13 = 0
        clr.b   0x5b(a6)                        | +028 tcb.flag_5b = 0
        clr.b   0x5a(a6)                        | +02c tcb.flag_5a = 0
        move.b  0x10fdaf.l, d0                  | +030 d0 = BIOS_mode_hi
        cmpi.b  #0x2, d0                        | +036 if (d0 == 2)
        beq.w   .Lsched_boot_mode2              | +03a   goto mode2
        clr.l   0x10fdb6.l                      | +03e reset dbl-flag block
        clr.b   0x106ecc.l                      | +044 reset key latch A
        clr.b   0x106ecd.l                      | +04a reset key latch B
        move.b  0x10fdae.l, d0                  | +050 d0 = BIOS_mode_lo
        cmpi.b  #0x3, d0                        | +056 if (d0 == 3)
        beq.w   .Lsched_boot_hardstart_10FEC5   | +05a   goto hardstart ($F52)
        bra.w   .Lsched_boot_demo               | +05e else goto demo ($F36)
.Lsched_boot_mode2:                             | $000EF0
        moveq   #-1, d0                         | +062 d0 = $FF (moveq signed)
        jsr     Sub_00024FEC                    | +064 PlayerCtx_ResetTwoBlocks (HH#3)
        tst.b   0x10fd82.l                      | +06a
        beq.w   .Lsched_boot_mode2_alt          | +070   if(!flag) mode2_alt
        lea.l   BootTblEntry_BE6(pc), a0        | +074 a0 = &tbl_$BE6
        move.l  a0, 0x70(a6)                    | +078 cursor = a0
        bra.w   .Lsched_boot_enter_mainloop     | +07c
.Lsched_boot_mode2_alt:                         | $000F0E
        lea.l   BootTblEntry_BDA(pc), a0        | +080 a0 = &tbl_$BDA
        move.l  a0, 0x70(a6)                    | +084 cursor = a0
        bra.w   .Lsched_boot_enter_mainloop     | +088
.Lsched_boot_title:                             | $000F1A
        move.b  #0xff, 0x106ece.l               | +08c key latch A = -1
        move.b  #0xff, 0x106ecf.l               | +094 key latch B = -1
        lea.l   BootTblEntry_BCE(pc), a0        | +09c a0 = &tbl_$BCE
        move.l  a0, 0x70(a6)                    | +0a0 cursor = a0
        bra.w   .Lsched_boot_enter_mainloop     | +0a4
.Lsched_boot_demo:                              | $000F36
        move.b  #0xff, 0x106ece.l               | +0a8 key latch A = -1
        move.b  #0xff, 0x106ecf.l               | +0b0 key latch B = -1
        lea.l   BootTblEntry_B92(pc), a0        | +0b8 a0 = &tbl_$B92 (T1)
        move.l  a0, 0x70(a6)                    | +0bc cursor = a0
        bra.w   .Lsched_boot_enter_mainloop     | +0c0
.Lsched_boot_hardstart_10FEC5:                  | $000F52
        move.b  #0x1, 0x10fec5.l                | +0c4 hardstart flag set
        move.b  #0xff, 0x106ece.l               | +0cc key latch A = -1
        move.b  #0xff, 0x106ecf.l               | +0d4 key latch B = -1
        lea.l   BootTblEntry_BBA(pc), a0        | +0dc a0 = &tbl_$BBA
        move.l  a0, 0x70(a6)                    | +0e0 cursor = a0
        bra.w   .Lsched_boot_enter_mainloop     | +0e4
| Nota: no hay caller directo a esta 5a rama; los tres 'beq.w $f52' del
| dispatcher externo caeran aqui cuando el usuario pulse Start durante
| attract. Layout preservado byte-a-byte.
.Lsched_boot_start_pressed:                     | $000F76 (unreachable-here)
        move.b  #0xff, 0x106ece.l               | +0e8
        move.b  #0xff, 0x106ecf.l               | +0f0
        lea.l   BootTblEntry_E6E(pc), a0        | +0f8 a0 = &tbl_$E6E (T6)
        move.l  a0, 0x70(a6)                    | +0fc
        jsr     PcThunkTarget_001CD4(pc)        | +100 pre-boot hook (pc-rel to $1CD4)
        bra.w   .Lsched_boot_enter_mainloop     | +104
        nop                                     | +108 padding
| ---- Mainloop tail (shared entry for all 5 branches) -----------------
.Lsched_boot_enter_mainloop:                    | $000F98
        move.w  0x10007c.l, d0                  | +10a d0 = frame_counter
        addq.w  #0x1, d0                        | +110 ++d0
        move.w  d0, 0x10007c.l                  | +112 frame_counter = d0
        jsr     0x5e998.l                       | +118 video update hook
        jsr     PcThunkTarget_001E1C(pc)        | +11e audio update hook (pc-rel to $1E1C)
        jsr     0x52712.l                       | +122 Pubcleaner_10A2Cx (LL#1)
        bsr.w   Sub_00001DA4                    | +128 tail hook (symbols.py)
        jsr     0xc004c2.l                      | +12c BIOS VBlank
        lea.l   SchedulerLoopA_000FC6(pc), a1   | +132 a1 = &loop_a
        move.l  a1, (a6)                        | +136 tcb.entry = &loop_a
| ---- Loop dispatch A ------------------------------------------------
| Etiqueta promovida a global para que bra.b cross-section (desde
| SchedulerDispatch_LoopB_000FE0 en $000FFC) se resuelva via R_68K_PC8.
        .globl  SchedulerLoopA_000FC6
SchedulerLoopA_000FC6:
.Lsched_boot_loop_a:                            | $000FC6
        movea.l 0x70(a6), a0                    | +138 a0 = cursor
        cmpi.l  #0xffffffff, (a0)               | +13c if (*cursor==-1)
        bne.w   .Lsched_boot_loop_a_deref       | +142   goto deref
        movea.l 0x4(a0), a0                     | +146   a0 = *(cursor+4)
        move.l  a0, 0x70(a6)                    | +14a   cursor = a0
.Lsched_boot_loop_a_deref:                      | $000FDC
        movea.l (a0), a0                        | +14e a0 = *a0 (handler)
        jmp     (a0)                            | +150 execute handler

        .size   SchedulerBootstrap_Boot_000E8E, .-SchedulerBootstrap_Boot_000E8E

|
|
| ---------------------------------------------------------------------------
|  SchedulerDispatch_LoopB_000FE0  @ $000FE0  (64 bytes)
|
|  Es el punto de re-entrada de todos los handlers de la super-tabla:
|  cada handler termina en `bra.w $FE0` (fall-through explicito), y este
|  bloque avanza el cursor de +4 (o salta a *(cursor+4) si encuentra
|  $FFFFFFFF), lo re-publica en $70(a6), y salta a $FC6 (loop A) para
|  interpretar la siguiente entrada.
|
|  Es "fall-through split hub" (7a aparicion, 1a como hub multi-handler).
|
|  Tras el fall-through natural entra el publisher Publish_1081Bx que
|  cierra el arranque con la publicacion condicional del gate $10FD82 en
|  $1081BF/$1081C0 (byte 4 si flag=0, byte 0 si flag!=0). Termina en rts.
| ---------------------------------------------------------------------------
|
        .globl  SchedulerDispatch_LoopB_000FE0
        .type   SchedulerDispatch_LoopB_000FE0, @function
        .section .text.SchedulerDispatch_LoopB_000FE0, "ax", @progbits
SchedulerDispatch_LoopB_000FE0:
        movea.l 0x70(a6), a0                    | +00 a0 = cursor
        adda.l  #0x4, a0                        | +04 a0 += 4 (advance)
        cmpi.l  #0xffffffff, (a0)               | +0a if (*a0 == -1)
        bne.w   .Lsched_loopb_publish           | +10   goto publish
        movea.l 0x4(a0), a0                     | +14   a0 = *(a0+4)
.Lsched_loopb_publish:                          | $000FF8
        move.l  a0, 0x70(a6)                    | +18 cursor = a0
        bra.b   SchedulerLoopA_000FC6           | +1c goto loop_a (cross-section)
| $000FFE en adelante es Global_SetDualFlagFrom10FD82_000FFE (34 B, ya
| matcheado en Wave B/R). NO forma parte de este bucle - la fase
| Publish_1081Bx que segia aqui era en realidad una funcion independiente
| con su propio rts.

        .size   SchedulerDispatch_LoopB_000FE0, .-SchedulerDispatch_LoopB_000FE0

|
| ---------------------------------------------------------------------------
|  SchedTail_JsrCD4_001020  @ $001020  (6 bytes)
|
|  Handler-tail micro: invoca $1CD4 (pre-boot hook) y salta al bucle B
|  para avanzar el cursor. Un handler entero en 6 bytes.
| ---------------------------------------------------------------------------
|
        .globl  SchedTail_JsrCD4_001020
        .type   SchedTail_JsrCD4_001020, @function
        .section .text.SchedTail_JsrCD4_001020, "ax", @progbits
SchedTail_JsrCD4_001020:
        bsr.w   PcThunkTarget_001CD4            | +00 hook $1CD4 (symbols.py)
        bra.b   SchedulerDispatch_LoopB_000FE0  | +04 goto loop B

        .size   SchedTail_JsrCD4_001020, .-SchedTail_JsrCD4_001020


|
| ---------------------------------------------------------------------------
|  SchedTail_JsrD3C_001026  @ $001026  (6 bytes)
|
|  Gemelo estructural del anterior con destino $1D3C. Otro handler
|  entero en 6 bytes. **8o par de clones no factorizados del proyecto**.
| ---------------------------------------------------------------------------
|
        .globl  SchedTail_JsrD3C_001026
        .type   SchedTail_JsrD3C_001026, @function
        .section .text.SchedTail_JsrD3C_001026, "ax", @progbits
SchedTail_JsrD3C_001026:
        bsr.w   PcThunkTarget_001D3C            | +00 hook $1D3C (symbols.py, Wave MM#1)
        bra.b   SchedulerDispatch_LoopB_000FE0  | +04 goto loop B

        .size   SchedTail_JsrD3C_001026, .-SchedTail_JsrD3C_001026


|
| ---------------------------------------------------------------------------
|  AttractHandler_10002C  @ $00102C  (112 bytes)
|
|  Handler grande de attract: incrementa frame counter, refresca video/BIOS,
|  invoca Pubcleaner_10A2Cx (Wave LL#1) y pinta el marco HUD Fix Layer
|  (FixLayer_QuadBatch_046AC6 de Wave HH#3). Guarda el marco recien pintado
|  con $106ED2 = -1 y salta al bucle B via bra.w $FE0.
|
|  Post-fall-through: un segundo handler mas corto (fase 2) que testea
|  $106ED2 y o bien cae en `JsrPcThunk_001096` (thunk `jsr $1af8(pc); rts`
|  ya matcheado por Wave J), o bien llama a $5B6 y salta al bucle B.
|  Idioma **"fall-through a thunk matcheado"**, 8a aparicion del fall-
|  through en el proyecto, y **1a hacia una funcion ya matcheada por una
|  wave anterior**. Tamano real: 106 B ($00102C..$001096), no 112 B; los
|  ultimos 6 B pertenecen al thunk vecino.
| ---------------------------------------------------------------------------
|
        .globl  AttractHandler_10002C
        .type   AttractHandler_10002C, @function
        .section .text.AttractHandler_10002C, "ax", @progbits
AttractHandler_10002C:
        move.w  0x10007c.l, d0                  | +00 d0 = frame_counter
        addq.w  #0x1, d0                        | +06 ++d0
        move.w  d0, 0x10007c.l                  | +08 frame_counter = d0
        jsr     Sub_0005E998                    | +0e video update hook
        jsr     BIOS_FIX_CLEAR                  | +14 BIOS VBlank (BIOS $C004C2)
        jsr     Pubcleaner_10A2Cx_052712        | +1a Pubcleaner_10A2Cx (LL#1)
        lea.l   TaskTpl_098720, a1              | +20 a1 = &task_tpl_$98720
        jsr     ThunkTarget_0004ae              | +26 Task_Alloc
        move.b  #0xff, 0x106ece.l               | +2c key latch A = -1
        move.b  #0xff, 0x106ecf.l               | +34 key latch B = -1
        jsr     PcThunkTarget_001af8(pc)        | +3c hook @ $1af8 (pc-rel)
        jsr     Sub_00046AC6                    | +40 FixLayer_QuadBatch (HH#3)
        move.b  #0xff, 0x106ed2.l               | +46 hud_dirty = -1
        bra.w   SchedulerDispatch_LoopB_000FE0  | +4e goto loop B
| ---- Fase 2 del handler: gate $106ED2 ------------------------------
.Lattr_handler_phase2:                          | $00107E
        bsr.w   Sub_00001DB8                    | +52 hook @ $1DB8 (symbols.py)
        tst.b   0x106ed2.l                      | +56 if (hud_dirty)
        bne.w   .Lattr_handler_falls_to_thunk   | +5c   goto fall-through
        jsr     FUN_000005B6                    | +60 refresh handler
        bra.w   SchedulerDispatch_LoopB_000FE0  | +66 goto loop B
.Lattr_handler_falls_to_thunk:                  | $001096 (byte just past)
| ---- Fall-through natural a JsrPcThunk_001096 (Wave J, ya matcheado):
|      $001096 : jsr $1af8(pc)  ; +0  (opcode 4EBA 0A60)
|      $00109A : rts            ; +4
| Los 6 B siguientes son cuerpo integro del thunk ya matcheado. NO se
| incluyen en esta funcion (evita overlap del linker con JsrPcThunk_001096).
| Idioma "fall-through a thunk matcheado" documentado por primera vez.

        .size   AttractHandler_10002C, .-AttractHandler_10002C

| PC-rel targets: $1af8 from two different callers (displacement varies).
| GAS resolves the (pc) form using the absolute address of the label.
