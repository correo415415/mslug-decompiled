| ============================================================================
|  Metal Slug 1 - asm/bin_to_decimal_decoder_05d904.s
|  ----------------------------------------------------------------------------
|  Wave X (post-allocator: HUD debug + comparadores + arranque) - funcion #3
|
|  Sub_BinToDecimalDecoder_05D904  @ $05D904  (28 bytes, ≥2 callers via X#2)
|
|  Convierte d0 (binario long, 0..99_999_999) a 8 nibbles BCD packed en d0.
|  Cada iteracion extrae un digito decimal (via division por 10 delegada a
|  Sub_LongDivide_05D920) y lo mete en el nibble correspondiente de d4.
|  d5 es el contador de shift (0, 4, 8, 12, ..., 28) usado con asl.l.
|
|  Algoritmo:
|      d4 = 0                          -- acumulador BCD
|      d5 = 0                          -- shift count (bits)
|      while (d0 != 0):
|          d1 = 10
|          (d0, d2) = LongDivide(d0, 10)   -- d0 = d0/10, d2 = d0 mod 10
|          d2 <<= d5                        -- posicionar digito en el nibble
|          d4 |= d2                         -- acumular en BCD
|          d5 += 4                          -- avanzar al siguiente nibble
|      d0 = d4                              -- devolver BCD
|      rts
|
|  Firma C conceptual:
|
|      /* Convierte value (binario, 0..99_999_999) a 8 nibbles BCD packed.
|       * Devuelve el BCD en d0 (mismo registro que la entrada). */
|      uint32_t Sub_BinToDecimalDecoder(uint32_t value /*d0*/);
|
|  Es el paso central del pipeline "decimal display" identificado en X#1:
|      Decimal_Clamp99999999 (X#2) -> Sub_BinToDecimalDecoder (X#3, este) ->
|      Sprite_HexFormat4 (W#3) -> tile map del Fix Layer.
|
|  Notas forenses:
|    1. tst.l d0 + beq como test del "quedan digitos" en vez de dbra o
|       contador fijo. Idioma clasico de "extract-while-nonzero" que ahorra
|       codigo si d0 tiene ceros trailing (muchos valores de score reales
|       terminan en 000..0 cuando el jugador aun no juega).
|    2. asl.l d5, d2 con d5 como shift count VARIABLE. GCC habria emitido
|       una tabla de shifts explicita o el compiler runtime __ashlsi3.
|       Aqui se aprovecha que asl.l con Dn como count es una sola instr
|       (2 B) en 68000.
|    3. bra.b hacia arriba al inicio del loop: patron do-while con test
|       al principio.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Sub_BinToDecimalDecoder_05D904
        .type   Sub_BinToDecimalDecoder_05D904, @function
        .section .text.Sub_BinToDecimalDecoder_05D904, "ax", @progbits

Sub_BinToDecimalDecoder_05D904:
        moveq   #0x0, d4                    | +00  d4 = 0 (acumulador BCD)
        moveq   #0x0, d5                    | +02  d5 = 0 (shift count)
.Lloop:
        tst.l   d0                          | +04  ¿ quedan digitos ?
        beq.w   .Ldone                      | +06  no: salir
        moveq   #0xa, d1                    | +0a  d1 = 10 (divisor)
        jsr     Sub_LongDivide_05D920(pc)   | +0c  (d0, d2) = LongDivide(d0, 10)
        asl.l   d5, d2                      | +10  d2 <<= d5 (posicionar nibble)
        or.l    d2, d4                      | +12  d4 |= d2 (acumular BCD)
        addq.l  #0x4, d5                    | +14  d5 += 4 (siguiente nibble)
        bra.b   .Lloop                      | +16  otra iter
.Ldone:
        move.l  d4, d0                      | +18  d0 = BCD final
        rts                                  | +1a
        .size   Sub_BinToDecimalDecoder_05D904, .-Sub_BinToDecimalDecoder_05D904
