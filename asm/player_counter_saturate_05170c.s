| ============================================================================
|  Metal Slug 1 - asm/player_counter_saturate_05170c.s
|  ----------------------------------------------------------------------------
|  Wave Z - #11
|
|  Player_CounterSaturateByte_05170C  @ $05170C  (64 bytes, 1 caller)
|
|  Incremento saturado (cap = $FF) de un byte cuyo offset dentro del bloque
|  global $10E3A2 depende del slot de jugador identificado por a1:
|
|    a1 == $106E94  (P1)  ->  ++($10E3A2 + $18)  saturado en $FF
|    a1 == $106E9C  (P2)  ->  ++($10E3A2 + $19)  saturado en $FF
|    a1 == otro           ->  rts sin cambios
|
|  Firma C conceptual:
|
|      /* Incrementa saturadamente el contador byte-per-player en $10E3A2
|       * segun el slot recibido en a1. P1 y P2 tienen offsets adyacentes
|       * (+$18, +$19); otros valores de a1 no hacen nada. */
|      void Player_CounterSaturateByte(struct PlayerCtx *slot /*a1*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Dos `cmpa.l #imm, a1` con `bne.w` a saltos DIFERENTES + `adda.l
|       #$18/$19` en cada rama, en vez de un switch con jump table.
|       GCC habria emitido un array de 2 elementos con indexado.
|    2. La cadena de branches converge en $5173C (`.Ldo_inc`), donde el
|       incremento saturado usa un pattern clasico:
|         move.b (a4), d0   ; d0 = *ctr
|         addq.b #1, d0     ; ++d0, sets X=1 if overflow (0xFF+1=0x00, X=1)
|         bcc.w .Lstore     ; if no carry, store d0
|         move.b #$FF, d0   ; overflow -> saturate
|       .Lstore: move.b d0, (a4)
|       GCC usaria `cmp.b #$FF; beq skip; addq.b #1`.
|    3. $106E94 y $106E9C (dif $8) coinciden EXACTAMENTE con el offset
|       de "next" en task node ($8 respecto a base). Esto refuerza la
|       hipotesis "task node = player context" para P1 y P2.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Player_CounterSaturateByte_05170C
        .type   Player_CounterSaturateByte_05170C, @function
        .section .text.Player_CounterSaturateByte_05170C, "ax", @progbits

Player_CounterSaturateByte_05170C:
        lea.l   0x10e3a2.l, a4                 | +00  a4 = &counters_base
        cmpa.l  #0x106e94, a1                  | +06  if (a1 != P1_slot)
        bne.w   .Lcheck_p2                     | +0c     goto .Lcheck_p2
        adda.l  #0x18, a4                      | +10  a4 = &counters_base[$18]
        bra.w   .Ldo_inc                       | +16  goto .Ldo_inc
.Lcheck_p2:
        cmpa.l  #0x106e9c, a1                  | +1a  if (a1 != P2_slot)
        bne.w   .Lexit                         | +20     goto .Lexit
        adda.l  #0x19, a4                      | +24  a4 = &counters_base[$19]
        bra.w   .Ldo_inc                       | +2a  goto .Ldo_inc
.Lexit:
        rts                                    | +2e
.Ldo_inc:
        move.b  (a4), d0                       | +30  d0 = *ctr
        addq.b  #0x1, d0                       | +32  ++d0 (X=1 si overflow)
        bcc.w   .Lstore                        | +34  if (!C) skip saturate
        move.b  #0xff, d0                      | +38  d0 = 0xFF (saturate)
.Lstore:
        move.b  d0, (a4)                       | +3c  *ctr = d0
        rts                                    | +3e

        .size   Player_CounterSaturateByte_05170C, .-Player_CounterSaturateByte_05170C
