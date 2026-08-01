| ============================================================================
|  Metal Slug 1 - asm/clipping_test_0999de.s
|  ----------------------------------------------------------------------------
|  Wave DD - #1
|
|  Clipping_Test_0999DE  @ $0999DE  (24 B, 2 callers)
|
|  Helper de test de clipping con retorno-por-CCR-C. Descubierto en Wave CC
|  como callee de Handler_ApplyCameraSelf_0440E4 y Handler_ApplyCameraGlobals_
|  044182 via `jsr $999DE.l; bcs.w rejected`.
|
|  Estructura minima:
|    1. Lee self->flag6C ($6C(a6)) en d5, copia a d6.
|    2. Aisla el nibble bajo: d5 &= $F.
|    3. Si nibble bajo == $F -> `andi.b #$FE, ccr` (clear C explicito) + rts.
|       = retorno "ACCEPTED con override" (los callers de CC ven la ruta
|         "aceptada" del clipping).
|    4. Si no, `lsr.b #4, d6` (extract nibble alto) y comparar con d5:
|       - si iguales -> `trap #$F` (asercion/bug guard).
|       - si distintos -> continua con logica de clipping externa
|         ($10E3BE/$10E47E/$10E480, no cubierto en esta funcion corta
|         porque el rts es al principio; el resto pertenece a otra
|         funcion que cae por fallthrough).
|
|  Los 24 B exactos cubren solo hasta el primer rts del "camino rapido":
|  el resto del control flow (a partir de $0999F6) es codigo NO cubierto
|  por esta funcion sino por la siguiente que empieza tras el fallthrough.
|
|  Idiomas hand-coded:
|    - `andi.b #$FE, ccr` como manera compacta de dejar C=0 sin tocar
|      X/N/Z/V. GCC no genera esto porque no tiene manejo directo del
|      registro CCR.
|
|  Firma C conceptual:
|
|      /* Test de clipping rapido: si nibble bajo de $6C(a6) == $F,
|       * devuelve "aceptado" (C=0) inmediatamente. En otro caso continua
|       * en el codigo vecino (fallthrough). */
|      bool Clipping_Test(struct Entity *self /*a6*/);   /* return via CCR-C */
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Clipping_Test_0999DE
        .type   Clipping_Test_0999DE, @function
        .section .text.Clipping_Test_0999DE, "ax", @progbits

Clipping_Test_0999DE:
        move.b  0x6c(a6), d5                   | +00  d5 = self->flag6C
        move.b  d5, d6                         | +04  d6 = d5
        andi.b  #0xf, d5                       | +06  d5 &= 0x0F (low nibble)
        cmpi.b  #0xf, d5                       | +0a  if (d5 == 0x0F)
        bne.w   .L_continue_fallthrough        | +0e     goto continue
        andi.b  #0xfe, ccr                     | +12  clear C flag
        rts                                    | +16
.L_continue_fallthrough:
                                              | (fallthrough al codigo
                                              |  vecino $0999F6, que ya no
                                              |  es parte de esta funcion)

        .size   Clipping_Test_0999DE, .-Clipping_Test_0999DE
