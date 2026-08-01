| ============================================================================
|  Metal Slug 1 - asm/player_inc_counter_at81_032afa.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #11 (clon de Z-batch2 #10 con offset distinto)
|
|  Player_IncCounterAt81_032AFA  @ $032AFA  (34 bytes)
|
|  Clon byte-a-byte de Player_IncCounterAt84_032B36 (Z-batch2 #10) salvo el
|  offset del incremento final: $81 en vez de $84.
|
|  Firma C conceptual:
|
|      /* Incrementa el contador byte en $81 del slot P1 (si $6D==1) o P2. */
|      void Player_IncCounterAt81(struct Entity *self /*a6*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Player_IncCounterAt81_032AFA
        .type   Player_IncCounterAt81_032AFA, @function
        .section .text.Player_IncCounterAt81_032AFA, "ax", @progbits

Player_IncCounterAt81_032AFA:
        cmpi.b  #0x1, 0x6d(a6)                 | +00  if (self->flag_6D != 1)
        bne.w   .Lp2                           | +06     goto P2
        lea.l   0x100440.l, a1                 | +0a  a1 = P1_slot
        bra.w   .Ldo                           | +10
.Lp2:
        lea.l   0x1004e0.l, a1                 | +14  a1 = P2_slot
.Ldo:
        addi.b  #0x1, 0x81(a1)                 | +1a  ++slot->field81
        rts                                    | +20

        .size   Player_IncCounterAt81_032AFA, .-Player_IncCounterAt81_032AFA
