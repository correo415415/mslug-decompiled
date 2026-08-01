| ============================================================================
|  Metal Slug 1 - asm/decimal_clamp_99999999_05d8f2.s
|  ----------------------------------------------------------------------------
|  Wave X (post-allocator: HUD debug + comparadores + arranque) - funcion #2
|
|  Decimal_Clamp99999999_05D8F2  @ $05D8F2  (18 bytes, ≥2 callers via X#1)
|
|  Satura d0 a un maximo de 99_999_999 (decimal). Si d0 excede ese valor,
|  lo sustituye por $99999999 (bit pattern "overflow display" que un
|  decoder BCD interpreta como "9999_9999", el maximo visualizable en 8
|  digitos decimales).
|
|  Semantica del compare (con tail-call!):
|      #$05F5E0FF == 99_999_999 exactamente.
|      Si d0 <= 99_999_999 (unsigned bcs = Carry Set = d0 < imm):
|          tail-call a Sub_BinToDecimalDecoder_05D904 (funcion contigua)
|          -- el decoder convierte d0 binario a 8 nibbles decimales.
|      Si d0 > 99_999_999:
|          d0 = $99999999 (ya son 8 nibbles '9' en binario, no requiere
|                          decodificacion) y cae al rts inmediato.
|
|  Firma C conceptual:
|
|      /* Satura d0 a 99_999_999 y decodifica a BCD 8-nibble.
|       * En caso de exceso, retorna directamente el bit pattern
|       * 0x99999999 (ya en formato BCD-parece-hex de 8 nueves). */
|      uint32_t Decimal_Clamp99999999(uint32_t value /*d0*/);
|
|  NOVENO caso identificado en el proyecto de "tail-call a funcion
|  contigua" (tras W#16 EmptyEntity_Init_00076A). Idioma clasico de asm
|  hand-coded para reutilizar el codigo de la funcion siguiente sin
|  gastar espacio en un rts adicional ni en un branch mas largo.
|
|  Contexto arquitectonico:
|    Es el pre-clamp que Debug_DrawHUDVars_096A80 (X#1) llama justo antes
|    de Sprite_HexFormat4_05D6C2 (W#3) para 2 de sus 7 lineas de volcado.
|    La combinacion Sub_5D8F2 + HexFormat4 forma un pipeline
|    "decimal-value display" (no un simple hex dump), donde el valor
|    de la ROM esta en binario pero se muestra como si fuera BCD
|    (interpretacion visual con la tabla ASCII "0123456789ABCDEF").
|    Los digitos 'A'-'F' no aparecen en la salida porque el clamp
|    garantiza que cada nibble es <= 9.
|
|  Notas forenses:
|    1. cmpi.l #$05F5E0FF, d0 con literal decimal exacto (99_999_999) en
|       hexadecimal, no como constante simbolica. Idioma de asm hand-coded
|       donde el desarrollador conocia la conversion 99_999_999 -> $05F5E0FF
|       de cabeza (o via una tabla en su libreta). GCC habria emitido el
|       decimal como #99999999 en el .s fuente, generando el mismo bit
|       pattern pero via constant folding del compilador.
|    2. move.l #$99999999, d0 con literal inline (6 B) donde el valor
|       $99999999 = -0x66666667 signed, no es codificable como moveq
|       (moveq solo cubre -128..127). No hay optimizacion posible.
|    3. bcs.w con destino a 10 B (target $5D904 = final de la funcion antes
|       del rts). Podria haber sido bcs.b (2 B) en vez de bcs.w (4 B),
|       ahorrando 2 B. La eleccion de bcs.w sugiere que el codigo original
|       fue traducido de una version mayor (con destinos > 128 B) o que se
|       reservo espacio para expansion futura. Micro-evidencia de asm
|       hand-coded o de una macroherramienta de ensamblado.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Decimal_Clamp99999999_05D8F2
        .type   Decimal_Clamp99999999_05D8F2, @function
        .section .text.Decimal_Clamp99999999_05D8F2, "ax", @progbits

Decimal_Clamp99999999_05D8F2:
        cmpi.l  #0x05f5e0ff, d0             | +00  ¿ d0 > 99_999_999 ?
        bcs.w   Sub_BinToDecimalDecoder_05D904 | +06  d0 <= max: tail-call al decoder contiguo
        move.l  #0x99999999, d0              | +0a  d0 = cap "9999_9999" (ya en formato BCD)
        rts                                  | +10
        .size   Decimal_Clamp99999999_05D8F2, .-Decimal_Clamp99999999_05D8F2
