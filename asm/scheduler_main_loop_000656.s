| ============================================================================
|  Metal Slug 1 - asm/scheduler_main_loop_000656.s
|  ----------------------------------------------------------------------------
|  Wave Y (Scheduler central + arranque + colas + constructores de entities) - #1
|
|  Scheduler_MainLoop_000656  @ $000656  (154 bytes, 2 callers)
|
|  Bucle central del scheduler de tareas del juego. Recorre la lista enlazada
|  de task nodes empezando en $100080, decrementa saturadamente los tres
|  timers del nodo ($44, $59, $45), invoca al handler apuntado por (a6),
|  limpia flags al volver, avanza al siguiente nodo (a6->next en $8(a6)) y
|  repite hasta agotar la lista.
|
|  Firma C conceptual:
|
|      /* Ejecuta el frame de scheduling completo: para cada task node
|       * activo, decrementa sus timers, invoca su handler, y avanza al
|       * siguiente. La variable global CURRENT_TASK_PTR ($106E84) queda
|       * apuntando siempre al nodo activo, y $106E8E guarda un snapshot
|       * de SP para permitir abort tipo longjmp desde el handler. */
|      void Scheduler_MainLoop(void);
|
|  Layout de un task node (offsets confirmados por este helper y por vecinos
|  ya matcheados como Entity_AllocFromFreeList_0006FE, Wave W#16):
|      +$00 : void (*handler)(void)          | invocado por `jsr (a0)`
|      +$08 : struct TaskNode *next          | lista enlazada simple
|      +$12 : uint8 flags_bits[0..7]         | bit 1 = contar en $106E8A
|      +$13 : uint8 flags2                   | bit 6 se limpia post-call
|      +$44 : uint8 timer_a  (satura en 0)
|      +$45 : uint8 timer_b  (satura en 0)
|      +$59 : uint8 timer_c  (satura en 0)
|      +$5A : uint8 flags3   (bits 0, 2..5 se limpian post-call)
|      +$69 : uint8 flags4   (bits 0..2, 4, 6..7 se limpian post-call)
|
|  Globales confirmadas:
|      $100080  = TASK_ARENA_BASE        | primer task node
|      $106E84  = CURRENT_TASK_PTR       | (=a6 al inicio de cada iteracion)
|      $106E8A  = FRAME_TASK_MARKED_CNT  | contador de nodos con flags bit 1
|      $106E8C  = FRAME_TASK_TOTAL_CNT   | contador total de nodos procesados
|      $106E8E  = SCHED_SP_SAVE          | snapshot de SP pre-handler
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. El decremento saturado usa `subq.b #1, d0; clr.b/moveq d1; addx.b d1, d0`
|       con `d1=0` para que X (bit borrow) propague el saturating floor.
|       GCC nunca emite este idioma; usaria `bne`+`subq` explicito.
|    2. `move.l a7, $106e8e.l` seguido de `movea.l $106e8e.l, a7` al retornar
|       es un mecanismo de abort del handler tipo setjmp/longjmp sin
|       stack unwind. GCC no genera SP snapshots asi.
|    3. El bucle no tiene condicion de salida propia: `bra.w $65c` cae por
|       diseno en `Scheduler_CompareField10_0006F0` cuando a6 alcanza el
|       centinela de la lista. Fall-through al vecino como salida.
|    4. Los tres `andi.b #imm, $XX(a6)` usan constantes distintas ($01, $17)
|       antes del handler y ($FA, $68) despues - protocolo de flags para
|       el handler que GCC no puede inferir.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Scheduler_MainLoop_000656
        .type   Scheduler_MainLoop_000656, @function
        .section .text.Scheduler_MainLoop_000656, "ax", @progbits

Scheduler_MainLoop_000656:
        lea.l   0x100080.l, a6                | +00  a6 = TASK_ARENA_BASE
        move.l  a6, 0x106e84.l                | +06  CURRENT_TASK_PTR = a6
                                              |
                                              | ---- header re-entry point ($65c) ----
                                              | El bra.w del final del bucle salta AQUI
                                              | (a la instruccion siguiente al lea), no
                                              | al inicio de la funcion, por eso el lea
                                              | no se re-ejecuta.
        andi.b  #0x1,  0x5a(a6)               | +0c  flags3 &= 0x01
        andi.b  #0x17, 0x69(a6)               | +12  flags4 &= 0x17
                                              |
                                              | ---- timer_a saturating decrement ----
        tst.b   0x44(a6)                      | +18  if (timer_a == 0)
        beq.w   .Lskip_ta                     | +1c    skip
        move.b  0x44(a6), d0                  | +20  d0 = timer_a
        moveq   #0x0,     d1                  | +24  d1 = 0
        subq.b  #0x1,     d0                  | +26  d0 -= 1  (X=1 si borrow)
        addx.b  d1,       d0                  | +28  d0 += X    -> satura en 0
        move.b  d0, 0x44(a6)                  | +2a  timer_a = d0
.Lskip_ta:
                                              | ---- optional counter: flags bit 1 ----
        btst.b  #0x1, 0x12(a6)                | +2e  if (flags & 2)
        beq.w   .Lskip_cnt                    | +34    skip
        addq.w  #0x1, 0x106e8a.l              | +38    FRAME_TASK_MARKED_CNT++
.Lskip_cnt:
                                              | ---- timer_c saturating decrement ----
        tst.b   0x59(a6)                      | +3e  if (timer_c == 0)
        beq.w   .Lskip_tc                     | +42    skip
        move.b  0x59(a6), d0                  | +46  d0 = timer_c
        clr.b   d1                            | +4a  d1 = 0  (variante: clr.b vs moveq)
        subq.b  #0x1, d0                      | +4c  d0 -= 1
        addx.b  d1,   d0                      | +4e  d0 += X    -> satura en 0
        move.b  d0, 0x59(a6)                  | +50  timer_c = d0
.Lskip_tc:
                                              | ---- timer_b saturating decrement ----
        tst.b   0x45(a6)                      | +54  if (timer_b == 0)
        beq.w   .Lskip_tb                     | +58    skip
        move.b  0x45(a6), d0                  | +5c  d0 = timer_b
        moveq   #0x0, d1                      | +60  d1 = 0
        subq.b  #0x1, d0                      | +62  d0 -= 1
        addx.b  d1,   d0                      | +64  d0 += X    -> satura en 0
        move.b  d0, 0x45(a6)                  | +66  timer_b = d0
.Lskip_tb:
                                              | ---- call handler with SP snapshot ----
        movea.l (a6), a0                      | +6a  a0 = task->handler
        move.l  a7, 0x106e8e.l                | +6c  SCHED_SP_SAVE = SP
        jsr     (a0)                          | +72  handler()
                                              |
                                              | ---- post-handler flag cleanup ----
        bclr.b  #0x6, 0x13(a6)                | +74  flags2 &= ~0x40
        andi.b  #0xfa, 0x5a(a6)               | +7a  flags3 &= 0xFA (clear bits 0,2)
        andi.b  #0x68, 0x69(a6)               | +80  flags4 &= 0x68 (clear bits 0..2,4,6..7)
                                              |
                                              | ---- advance to next node & loop ----
        addq.w  #0x1, 0x106e8c.l              | +86  FRAME_TASK_TOTAL_CNT++
        movea.l 0x106e8e.l, a7                | +8c  restore SP (in case handler juggled it)
        movea.l 0x8(a6), a6                   | +92  a6 = task->next
        bra.w   .Lheader_reentry              | +96  goto $65c  (skip lea, keep clearing)
                                              |
                                              | Nota: el offset del bra.w termina
                                              | dando $ff6e (=-146 en signed 16-bit),
                                              | apuntando a $65c justo despues del lea.
                                              | No re-inicializa a6 - eso lo hace la
                                              | llamada inicial. La funcion depende de
                                              | que la lista enlazada tenga un centinela
                                              | que caiga en Scheduler_CompareField10.

        .equ    .Lheader_reentry, Scheduler_MainLoop_000656 + 0x06

        .size   Scheduler_MainLoop_000656, .-Scheduler_MainLoop_000656
