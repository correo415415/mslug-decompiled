| ============================================================================
|  Metal Slug 1 - asm/init_mode_toggle_001260.s
|  ----------------------------------------------------------------------------
|  Wave EE batch 1 - #1
|
|  Init_ModeToggle_001260  @ $001260  (148 bytes)
|
|  Handler de un slot del scheduler (registrado como puntero long en la
|  tabla de descriptores de estado en $000BA2). Alterna la ejecucion entre
|  dos submodos de la fase de arranque (odd/even sobre $10007B), con una
|  rama fallback para el caso "trabajo pendiente" (gate por $106ED2).
|
|  Firma C conceptual:
|
|      /* Handler de tarea invocado desde el scheduler ($1006E80 arena).
|       * Publica cero en el flag $106F28, arranca varios subsistemas
|       * (init de audio/video, reset de I/O, warmup del bloque $46AC6) y
|       * dos scheduler_add() consecutivos (TaskHandler_00091630 y
|       * PcThunk_001C88). Luego incrementa saturadamente el contador
|       * $10007B (mod-2) y despacha:
|       *   - bit0 == 0 -> jsr Sub_00024FEC(0); jmp Label_001940 (submodo A)
|       *   - bit0 == 1 -> jsr Sub_00024FEC(1); jmp Label_00199A (submodo B)
|       *   - fallback  -> bsr Sub_00001DB8; si $106ED2 != 0 => tail-call
|       *                   PcThunk_001AF8 + rts; si == 0 => reset con
|       *                   $FFFF + jsr PcThunk_001CD4 + tail al scheduler
|       *                   ($FE0). */
|      void Init_ModeToggle(void);
|
|  Etiquetas internas alcanzadas por bra.w cortos:
|      $12B6  -> bit0 == 1 path (submodo B)
|      $12CA  -> fallback / probe-and-branch path
|      $12DC  -> "no work pending" reset path
|      $12EE  -> "work pending" tail-call path (PcThunkTarget_001AF8 + rts)
|
|  Globales tocadas:
|      $10007B  = ModeTick counter (bit-0 controla submodo)
|      $106F28  = Init flag, siempre 0 al entrar
|      $106ED2  = "pending work" flag consumido por rama fallback
|
|  Punteros pasados a scheduler_add ($4AE):
|      $91630  (TaskHandler_00091630)  \  ambos aparecen tambien en Wave R
|      $1C88   (PcThunkTarget_001C88)  /  como epilogo compartido de otros
|                                          handlers de la zona baja.
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. `moveq #$0, d0` inmediatamente antes de `jsr $5E998.l` para
|       forzar arg=0 sobre d0 - GCC habria usado `clr.l d0` (patron mas
|       corto habitual). Igual con el par $12AA (moveq #0) y $12BE
|       (moveq #1) usados como bandera antes del mismo jsr, patron
|       simetrico hand-coded.
|    2. Doble `jsr $4AE` consecutivo con `lea.l imm, a1` y `lea.l pc+d, a1`
|       en instrucciones distintas (una absoluta, otra PC-rel). GCC
|       preferiria emitir ambas absolutas o ambas PC-rel segun -mpcrel.
|    3. `jmp $1940(pc)` y `jmp $199A(pc)` son jumps PC-rel de 4 bytes
|       (4EFA + disp16) usados como *continuaciones* dentro del mismo
|       area, sin rts propio del path bit-0. Es tail-jump idiomatico de
|       ensamblador escrito a mano.
|    4. `move.w #$FFFF, d0` como argumento sentinela para $24FEC en la
|       rama fallback - GCC habria usado `moveq #-1, d0` (2 bytes, mismo
|       resultado 32-bit signed). Aqui se emiten 4 bytes ($303C FFFF)
|       para dejar d0.b == $FF sin tocar los bits altos de d0.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Init_ModeToggle_001260
        .type   Init_ModeToggle_001260, @function
        .section .text.Init_ModeToggle_001260, "ax", @progbits

Init_ModeToggle_001260:
        clr.b   0x106f28.l                     | +00  publica flag = 0
        moveq   #0x0, d0                       | +06  d0 = 0  (arg para $5E998)
        jsr     0x5e998.l                      | +08  Sub_0005E998(0)
        jsr     0xc004c2.l                     | +0e  BIOS_ClearMainAudioBank
        jsr     0x46ac6.l                      | +14  Sub_00046AC6 (init pesado)
        lea.l   0x91630.l, a1                  | +1a  a1 = TaskHandler_00091630
        jsr     0x4ae.l                        | +20  scheduler_add(a1)
        lea.l   .Lpc_thunk_1c88(pc), a1        | +26  a1 = PcThunkTarget_001C88  (PC-rel)
        jsr     0x4ae.l                        | +2a  scheduler_add(a1)
                                              |
                                              | ---- mode-tick advance ($10007B++ mod 2) ----
        move.b  0x10007b.l, d0                 | +30  d0 = tick
        addq.b  #0x1, d0                       | +36  d0 += 1
        move.b  d0, 0x10007b.l                 | +38  tick = d0
        andi.b  #0x1, d0                       | +3e  d0 &= 1
        cmpi.b  #0x0, d0                       | +42  if (d0 != 0)
        bne.w   .Lbit_one                      | +46    goto submodo B
                                              |
                                              | ---- submodo A (bit0 == 0) ----
        moveq   #0x0, d0                       | +4a  d0 = 0
        jsr     0x24fec.l                      | +4c  Sub_00024FEC(0)
        jmp     .Ltail_a(pc)                   | +52  jmp $1940 (continuacion externa)

.Lbit_one:                                     | $12B6
        cmpi.b  #0x1, d0                       | +56  if (d0 != 1)
        bne.w   .Lfallback                     | +5a    goto fallback
        moveq   #0x1, d0                       | +5e  d0 = 1
        jsr     0x24fec.l                      | +60  Sub_00024FEC(1)
        jmp     .Ltail_b(pc)                   | +66  jmp $199A (continuacion externa)

.Lfallback:                                    | $12CA
        bsr.w   Sub_00001DB8                   | +6a  Sub_00001DB8() (probe)
        tst.b   0x106ed2.l                     | +6e  if (pending_work == 0)
        beq.w   .Lreset_path                   | +74    goto reset
        bra.w   .Ltail_pending                 | +78  else tail_pending

.Lreset_path:                                  | $12DC
        move.w  #0xffff, d0                    | +7c  d0 = -1 (sentinel)
        jsr     0x24fec.l                      | +80  Sub_00024FEC(-1)
        jsr     .Lpc_reset(pc)                 | +86  jsr $1CD4 (reset ext)
        bra.w   Sub_00000FE0                   | +8a  tail al scheduler

.Ltail_pending:                                | $12EE
        jsr     .Lpc_tail_thunk(pc)            | +8e  jsr $1AF8 (thunk PC-rel)
        rts                                    | +92

        | ------------------------------------------------------------------
        | Symbols alcanzados por PC-rel corto: se resuelven al enlazar.
        | ------------------------------------------------------------------
        .equ    .Lpc_thunk_1c88,   PcThunkTarget_001C88
        .equ    .Ltail_a,          Label_001940
        .equ    .Ltail_b,          Label_00199A
        .equ    .Lpc_reset,        PcThunkTarget_001CD4
        .equ    .Lpc_tail_thunk,   PcThunkTarget_001af8

        .size   Init_ModeToggle_001260, .-Init_ModeToggle_001260
