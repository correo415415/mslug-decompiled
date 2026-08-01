| ============================================================================
|  Metal Slug 1 - asm/player_state_dispatch_0519be.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #3
|
|  Player_StateDispatch_0519BE  @ $0519BE  (82 bytes, 1 caller)
|
|  Dispatcher de estado por-jugador con jump-table PC-rel. Selecciona el
|  contexto del jugador segun $6E(a6):
|    - si $6E(a6) < 0 (bit 7)   -> exit inmediato via $51A0E rts
|    - si $6E(a6) == 0          -> a0 = $106E94  (P1_ctx)
|    - si $6E(a6) > 0           -> a0 = $106E9C  (P2_ctx)
|
|  Prerrequisito: bit 1 de $12(a6) DESACTIVADO. Si esta activo, sale.
|
|  Tras seleccionar contexto: `bsr $51862(pc)` -> jump-table `$5188C(pc)`
|  indexada por $58(a6)&$1F (long-word stride +$4) -> BCD helper Z2#4 ->
|  post-hook `$51828(pc)` -> rts.
|
|  Firma C conceptual:
|
|      /* Dispatch de estado por-jugador con jump-table de 32 entradas
|       * long-word, gated por $12 flag y $6E signo. */
|      void Player_StateDispatch(struct Entity *self /*a6*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Player_StateDispatch_0519BE
        .type   Player_StateDispatch_0519BE, @function
        .section .text.Player_StateDispatch_0519BE, "ax", @progbits

Player_StateDispatch_0519BE:
        btst.b  #0x1, 0x12(a6)                 | +00  if (self->flags & 2)
        bne.w   .Lexit                         | +06     goto exit
        move.b  0x6e(a6), d0                   | +0a  d0 = self->player_flag
        bmi.w   .Lexit                         | +0e  if (d0 < 0) goto exit
        bne.w   .Lp2                           | +12  if (d0 != 0) goto P2
        lea.l   0x106e94.l, a0                 | +16  a0 = P1_ctx
        bra.w   .Lloaded                       | +1c  goto loaded
.Lp2:
        lea.l   0x106e9c.l, a0                 | +20  a0 = P2_ctx
.Lloaded:
        lea.l   0x1081b6.l, a1                 | +26  a1 = state_buffer
        bsr.w   .Lprep                         | +2c  bsr Sub_00051862  (prep)
        lea     .Ljt(pc), a2                   | +30  a2 = &StateJT[0]
        move.b  0x58(a6), d1                   | +34  d1 = self->state_idx
        andi.w  #0x1f, d1                      | +38  d1 &= 0x1F
        add.w   d1, d1                         | +3c  d1 *= 2
        add.w   d1, d1                         | +3e  d1 *= 4 (long-word stride)
        addq.w  #0x4, d1                       | +40  d1 += 4 (skip first slot)
        adda.w  d1, a2                         | +42  a2 = &StateJT[idx+1]
        bsr.w   .Lbcd_helper                   | +44  BCD_AddClamp99999999_051A10
        suba.w  #0x8, a0                       | +48  a0 -= 8 (undo shift)
        bsr.w   .Lpost                         | +4c  bsr Sub_00051828  (post-jt)
.Lexit:
        rts                                    | +50

        .equ    .Lprep,       Sub_00051862
        .equ    .Ljt,         StateJumpTable_05188C
        .equ    .Lbcd_helper, BCD_AddClamp99999999_051A10
        .equ    .Lpost,       Sub_00051828

        .size   Player_StateDispatch_0519BE, .-Player_StateDispatch_0519BE
