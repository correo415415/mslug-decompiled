| ============================================================================
|  Metal Slug 1 - asm/bcd_add_clamp_99999999_051a10.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #4
|
|  BCD_AddClamp99999999_051A10  @ $051A10  (24 bytes)
|
|  Suma BCD de 4 nibbles (=8 digitos decimales) desde el buffer apuntado por
|  a2 al buffer apuntado por a1, ambos con predecrement, con clamp final a
|  $99999999 si hay overflow. Operador aritmetico del pipeline decimal del
|  juego, complementario del pipeline "display decimal" reconstruido en
|  Wave X (X#2 Decimal_Clamp99999999_05D8F2, X#3 Sub_BinToDecimalDecoder,
|  X#4 Sub_LongDivide).
|
|  Estrategia BCD:
|    - `andi.b #$EF, ccr` limpia X (extend) para arrancar sin acarreo
|    - 4 x `abcd.b -(a2), -(a1)` : suma cada par de digitos BCD, propagando
|      X entre iteraciones (canonico de multi-precision BCD add)
|    - Si tras las 4 iteraciones X sigue seteado (bcc.w no toma la rama),
|      publica `$99999999` en (a1) como clamp de saturacion decimal
|
|  Firma C conceptual:
|
|      /* Suma BCD de 8 digitos (4 bytes BCD-packed) desde a2 hacia a1 con
|       * predecrement en ambos, y satura a $99999999 si hay overflow. */
|      void BCD_AddClamp99999999(uint8_t *dst_end /*a1*/,
|                                const uint8_t *src_end /*a2*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. `abcd.b -(a2), -(a1)` unrolled 4x en vez de dbra/loop. Ningun
|       backend GCC emite `abcd` (instruccion BCD del 68000 sin equivalente
|       C directo). Es asm hand-coded puro.
|    2. `andi.b #$EF, ccr` limpia el bit X ANTES de arrancar la cadena de
|       `abcd`. GCC habria tenido que emitir un `moveq #0, dX; asl.b #0, dX`
|       o similar para forzar X=0 - forma prohibitivamente artificial.
|    3. Clamp por saturacion con `move.l #$99999999, (a1)` (constante
|       inmediata de 6 B) como respuesta a overflow. GCC habria emitido
|       un branch condicional con set-if-overflow.
|    4. GAS con --register-prefix-optional rechaza el modo predecrement
|       `-(aX)` en `abcd` (lo parsea como aritmetica). Emitimos los 4
|       bytes literales (`c3 0a` cada uno = abcd.b -(a2), -(a1)).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  BCD_AddClamp99999999_051A10
        .type   BCD_AddClamp99999999_051A10, @function
        .section .text.BCD_AddClamp99999999_051A10, "ax", @progbits

BCD_AddClamp99999999_051A10:
        andi.b  #0xef, ccr                     | +00  CCR &= 0xEF (clear X)
                                              |
                                              | ---- 4x abcd.b -(a2), -(a1) ----
                                              | GAS con --register-prefix-optional parsea
                                              | `-(a2)` como aritmetica y rechaza el modo
                                              | predecrement. Emitimos los 4 bytes literales
                                              | (`c3 0a` cada uno = abcd.b -(a2), -(a1)).
        .byte   0xc3, 0x0a                     | +04  abcd.b -(a2), -(a1)
        .byte   0xc3, 0x0a                     | +06  abcd.b -(a2), -(a1)
        .byte   0xc3, 0x0a                     | +08  abcd.b -(a2), -(a1)
        .byte   0xc3, 0x0a                     | +0a  abcd.b -(a2), -(a1)
        bcc.w   .Lno_overflow                  | +0c  if (!C) skip clamp
        move.l  #0x99999999, (a1)              | +10  saturate to max BCD
.Lno_overflow:
        rts                                    | +16

        .size   BCD_AddClamp99999999_051A10, .-BCD_AddClamp99999999_051A10
