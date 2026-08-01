| ============================================================================
|  Metal Slug 1 - asm/player_inc_counter_at84_032b36.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #10
|
|  Player_IncCounterAt84_032B36  @ $032B36  (34 bytes)
|
|  Selecciona slot de jugador segun $6D(a6) e incrementa el byte en $84(a1)
|  del slot elegido.
|    - $6D(a6) == 1  ->  a1 = $100440 (P1)  -> ++$84(a1)
|    - $6D(a6) != 1  ->  a1 = $1004E0 (P2)  -> ++$84(a1)
|
|  Firma C conceptual:
|
|      /* Incrementa el contador byte en $84 del slot P1 (si $6D==1) o P2. */
|      void Player_IncCounterAt84(struct Entity *self /*a6*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Player_IncCounterAt84_032B36
        .type   Player_IncCounterAt84_032B36, @function
        .section .text.Player_IncCounterAt84_032B36, "ax", @progbits

Player_IncCounterAt84_032B36:
        cmpi.b  #0x1, 0x6d(a6)                 | +00  if (self->flag_6D != 1)
        bne.w   .Lp2                           | +06     goto P2
        lea.l   0x100440.l, a1                 | +0a  a1 = P1_slot
        bra.w   .Ldo                           | +10
.Lp2:
        lea.l   0x1004e0.l, a1                 | +14  a1 = P2_slot
.Ldo:
        addi.b  #0x1, 0x84(a1)                 | +1a  ++slot->field84
        rts                                    | +20

        .size   Player_IncCounterAt84_032B36, .-Player_IncCounterAt84_032B36
