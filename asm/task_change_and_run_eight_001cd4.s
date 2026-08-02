| ============================================================================
|  Metal Slug 1 - asm/task_change_and_run_eight_001cd4.s
|  ----------------------------------------------------------------------------
|  Wave SS#6 - batch de re-arme de los 8 TCBs estaticos principales.
|
|  TaskList_ChangeAndRunEight_001CD4  @ $001CD4  (96 bytes, 7 callers)
|
|  ---------- Callers --------------------------------------------------------
|
|      $000F8E  jsr $1cd4(pc)   (SchedulerBootstrap_Boot_000E8E, Wave MM#1)
|      $001020  bsr.w $1cd4     (SchedTail_JsrCD4_001020, Wave MM#3)
|      $0012E6  jsr $1cd4(pc)   (handler attract, no matcheado)
|      $00140C  bsr.w $1cd4     (idem)
|      $0015AE  bsr.w $1cd4     (idem)
|      $00182C  bsr.w $1cd4     (idem)
|      $001B06  bsr.w $1cd4     (idem)
|
|  Es EL "callee $1CD4" que Wave MM#3 dejo documentado como destino del
|  handler-tail SchedTail_JsrCD4_001020 (su gemelo $1D3C sigue pendiente).
|
|  ---------- Firma C conceptual ---------------------------------------------
|
|    /* Invoca Task_ChangeAndRun_0626 (matcheado, scheduler.c) sobre los
|       8 TCBs estaticos "de gameplay" en orden fijo.  Task_ChangeAndRun
|       consume el cambio de handler pendiente del TCB y lo ejecuta.  */
|    void TaskList_ChangeAndRunEight(void)
|    {
|        Task_ChangeAndRun($100260);
|        Task_ChangeAndRun($100800);
|        Task_ChangeAndRun($1008A0);
|        Task_ChangeAndRun($100440);   // player 1
|        Task_ChangeAndRun($1004E0);   // player 2
|        Task_ChangeAndRun($100580);
|        Task_ChangeAndRun($100300);   // partner P1
|        Task_ChangeAndRun($1003A0);   // partner P2
|        // fall-through en JsrAbsThunk_001d34 (jsr FUN_000005B6; rts)
|    }
|
|  Los 8 TCBs son subconjunto de los 12 que instala
|  TaskSlots_BootInstall_000A7C (Wave SS#5) - faltan $1001C0 (el propio
|  scheduler), $100620, $1006C0 y $100760 (tasks "de sistema" que no se
|  re-arman por frame).
|
|  Nota forense: sin rts propio - cae por fall-through en
|  JsrAbsThunk_001d34 (Wave I, 8 B: jsr $5B6.l; rts), de modo que la
|  "novena operacion" implicita es FUN_000005B6 (hook video/audio del
|  scheduler, el mismo que invoca AttractHandler_10002C en MM#3).
|  10a aparicion del idioma fall-through del proyecto.
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text
        .globl  TaskList_ChangeAndRunEight_001CD4
        .type   TaskList_ChangeAndRunEight_001CD4, @function
        .section .text.TaskList_ChangeAndRunEight_001CD4, "ax", @progbits

TaskList_ChangeAndRunEight_001CD4:
        lea     0x100260.l, a0                  | +000
        jsr     Task_ChangeAndRun_0626          | +006
        lea     0x100800.l, a0                  | +00c
        jsr     Task_ChangeAndRun_0626          | +012
        lea     0x1008a0.l, a0                  | +018
        jsr     Task_ChangeAndRun_0626          | +01e
        lea     0x100440.l, a0                  | +024  player 1
        jsr     Task_ChangeAndRun_0626          | +02a
        lea     0x1004e0.l, a0                  | +030  player 2
        jsr     Task_ChangeAndRun_0626          | +036
        lea     0x100580.l, a0                  | +03c
        jsr     Task_ChangeAndRun_0626          | +042
        lea     0x100300.l, a0                  | +048  partner P1
        jsr     Task_ChangeAndRun_0626          | +04e
        lea     0x1003a0.l, a0                  | +054  partner P2
        jsr     Task_ChangeAndRun_0626          | +05a
        | fall-through en JsrAbsThunk_001d34 ($001D34, Wave I)         | +060
