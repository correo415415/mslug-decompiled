| ============================================================================
|  Metal Slug 1 - asm/debug_hex_counter_cluster_047482.s
|  ----------------------------------------------------------------------------
|  Wave AA batch 2 - #1..#5
|
|  Cluster de 5 helpers contiguos del subsistema "debug HUD counter" en el
|  rango $047482..$047675 (500 B). Todos los helpers de blit al Fix Layer
|  utilizan el idioma MMIO hand-coded `movem.w d0-d1, $3C0000.l` (protocolo
|  address+data en dos writes consecutivos, ya validado en W#3, W#4, W#6,
|  W#7). El divisor por 10 iterativo #5 complementa el pipeline decimal
|  descubierto en Wave X (X#3 decoder, X#4 divisor shift-and-subtract 32-iter)
|  con la variante "divisor por 10 con cociente en d1 y resto en d0".
|
|  Layout del cluster:
|
|     $047482  Debug_DrawHexCounter_047482             (156 B, AA2 #1)
|     $04751E  Debug_DrawHexCounter_ClampBranch_04751E (138 B, AA2 #2)
|     $0475A8  Debug_DrawHexCounter_Fallback_0475A8    (166 B, AA2 #3)
|     $04764E  Debug_SetCounter_04764E                 (  8 B, AA2 #4)
|     $047656  Sub_Divide10_047656                     ( 32 B, AA2 #5)
|
|  Gate global: la funcion top-level (#1) esta protegida por
|  `cmpi.b #$1, $10FDAF.l` (mismo puerto de habilitacion de debug ya usado
|  por Debug_DrawHUDVars X#1). Si el flag no vale 1, sale inmediatamente
|  sin tocar el Fix Layer.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|  ----------------------------------------------------------------------------
|  #1  Debug_DrawHexCounter_047482  @ $047482  (156 B)
|
|  Top-level del cluster. Estructura:
|
|    1. Gate: si $10FDAF != 1 -> sale.
|    2. Si $106E92 <= $63 (=99): pinta 8 tiles al Fix Layer con dos
|       cuartetos de par (col=$7243,fila=$63B6) y (col=$7283,fila=$63B8).
|       Cada cuarteto es un bloque 2x2 de tiles vecinos, escritos con el
|       protocolo MMIO `movem.w d0-d1, $3C0000.l` y desplazamientos de
|       coordenada +$20/+1 (Fix Layer column-major stride, ya visto en W#7).
|    3. Si $106E92 > $63: tail-call a la rama #2 (fall-through natural
|       hacia $04751E, sin `bra` explicito, esta al inicio de la funcion
|       vecina).
|
|  El `bls.w $4751E` cae por diseño a la siguiente funcion como salida
|  alternativa (10\u00b0 tail-call a funcion contigua del proyecto).
|
|  Firma C conceptual:
|
|      /* Debug HUD del contador principal: dibuja el valor de $106E92
|       * al Fix Layer si el gate global $10FDAF esta activo y el valor
|       * cabe en 2 digitos hex. Si excede, delega en la rama larga (#2). */
|      void Debug_DrawHexCounter(void);
|  ----------------------------------------------------------------------------

        .globl  Debug_DrawHexCounter_047482
        .type   Debug_DrawHexCounter_047482, @function
        .section .text.Debug_DrawHexCounter_047482, "ax", @progbits

Debug_DrawHexCounter_047482:
        cmpi.b  #0x1, 0x10fdaf.l               | +00  gate: $10FDAF == 1 ?
        bne.w   .L1_exit                       | +06  no -> exit
        rts                                    | +0a  gate pass pero
                                              |         (branch cae aqui)
.L1_exit:
        cmpi.w  #0x63, 0x106e92.l              | +0c  counter <= 99 ?
        bls.w   .L1_fallback_04751E            | +14  no -> delega en #2
        move.w  #0x7243, d0                    | +18  d0 = $7243 (col A)
        move.w  #0x63b6, d1                    | +1c  d1 = $63B6 (fila A)
        movem.w d0-d1, 0x3c0000.l              | +20  MMIO tile write #1
        addi.w  #0x20, d0                      | +28  +$20 columna
        addq.w  #0x1, d1                       | +2c  +1 fila
        movem.w d0-d1, 0x3c0000.l              | +2e  MMIO tile write #2
        subi.w  #0x1f, d0                      | +36  -$1F retroceso col
        addi.w  #0xf, d1                       | +3a  +$F fila (nueva linea)
        movem.w d0-d1, 0x3c0000.l              | +3e  MMIO tile write #3
        addi.w  #0x20, d0                      | +46  +$20 columna
        addq.w  #0x1, d1                       | +4a  +1 fila
        movem.w d0-d1, 0x3c0000.l              | +4c  MMIO tile write #4
        subi.w  #0x21, d0                      | +54  -$21 (reset para bloque B)
                                              |
        move.w  #0x7283, d0                    | +58  d0 = $7283 (col B)
        move.w  #0x63b8, d1                    | +5c  d1 = $63B8 (fila B)
        movem.w d0-d1, 0x3c0000.l              | +60  MMIO tile write #5
        addi.w  #0x20, d0                      | +68  +$20 columna
        addq.w  #0x1, d1                       | +6c  +1 fila
        movem.w d0-d1, 0x3c0000.l              | +6e  MMIO tile write #6
        subi.w  #0x1f, d0                      | +76  -$1F retroceso col
        addi.w  #0xf, d1                       | +7a  +$F fila (nueva linea)
        movem.w d0-d1, 0x3c0000.l              | +7e  MMIO tile write #7
        addi.w  #0x20, d0                      | +86  +$20 columna
        addq.w  #0x1, d1                       | +8a  +1 fila
        movem.w d0-d1, 0x3c0000.l              | +8c  MMIO tile write #8
        subi.w  #0x21, d0                      | +94  -$21 (reset final)
        rts                                    | +98

        .equ    .L1_fallback_04751E, Debug_DrawHexCounter_ClampBranch_04751E

        .size   Debug_DrawHexCounter_047482, .-Debug_DrawHexCounter_047482


|  ----------------------------------------------------------------------------
|  #2  Debug_DrawHexCounter_ClampBranch_04751E  @ $04751E  (138 B)
|
|  Segunda rama del debug HUD. Estructura simetrica a #1 pero con dos
|  cambios sistematicos:
|
|    - Guard por-registro: `cmpi.b #$0, d1; bne.w #3` -> si d1 (byte pasado
|      por caller externo, semantica "modo") no es cero, delega en la rama
|      #3 (fallback via tabla).
|    - Coordenadas de fila fijas: en lugar de `$63B6/$63B8`, usa `$4B40` en
|      ambos bloques (con mismos deltas +$20/+1/-$1F/+$F/-$21).
|
|  El `bne.w $475A8` es fall-through directo a la funcion vecina #3 (11\u00b0
|  tail-call a funcion contigua del proyecto).
|
|  Firma C conceptual:
|
|      /* Rama "clamp branch": si d1 != 0, delega en la fallback #3.
|       * Si d1 == 0, pinta los dos bloques 2x2 con fila fija $4B40. */
|      void Debug_DrawHexCounter_ClampBranch(uint8_t mode /*d1*/);
|  ----------------------------------------------------------------------------

        .globl  Debug_DrawHexCounter_ClampBranch_04751E
        .type   Debug_DrawHexCounter_ClampBranch_04751E, @function
        .section .text.Debug_DrawHexCounter_ClampBranch_04751E, "ax", @progbits

Debug_DrawHexCounter_ClampBranch_04751E:
        cmpi.b  #0x0, d1                       | +00  mode == 0 ?
        bne.w   .L2_fallback_0475A8            | +04  no -> delega en #3
        move.w  #0x7243, d0                    | +08  d0 = $7243 (col A)
        move.w  #0x4b40, d1                    | +0c  d1 = $4B40 (fila fija)
        movem.w d0-d1, 0x3c0000.l              | +10  MMIO tile write #1
        addi.w  #0x20, d0                      | +18  +$20 columna
        addq.w  #0x1, d1                       | +1c  +1 fila
        movem.w d0-d1, 0x3c0000.l              | +1e  MMIO tile write #2
        subi.w  #0x1f, d0                      | +26  -$1F retroceso col
        addi.w  #0xf, d1                       | +2a  +$F fila
        movem.w d0-d1, 0x3c0000.l              | +2e  MMIO tile write #3
        addi.w  #0x20, d0                      | +36  +$20 columna
        addq.w  #0x1, d1                       | +3a  +1 fila
        movem.w d0-d1, 0x3c0000.l              | +3c  MMIO tile write #4
        subi.w  #0x21, d0                      | +44  -$21 (reset bloque B)
                                              |
        move.w  #0x7283, d0                    | +48  d0 = $7283 (col B)
        move.w  #0x4b40, d1                    | +4c  d1 = $4B40 (fila fija)
        movem.w d0-d1, 0x3c0000.l              | +50  MMIO tile write #5
        addi.w  #0x20, d0                      | +58  +$20 columna
        addq.w  #0x1, d1                       | +5c  +1 fila
        movem.w d0-d1, 0x3c0000.l              | +5e  MMIO tile write #6
        subi.w  #0x1f, d0                      | +66  -$1F retroceso col
        addi.w  #0xf, d1                       | +6a  +$F fila
        movem.w d0-d1, 0x3c0000.l              | +6e  MMIO tile write #7
        addi.w  #0x20, d0                      | +76  +$20 columna
        addq.w  #0x1, d1                       | +7a  +1 fila
        movem.w d0-d1, 0x3c0000.l              | +7c  MMIO tile write #8
        subi.w  #0x21, d0                      | +84  -$21 (reset final)
        rts                                    | +88

        .equ    .L2_fallback_0475A8, Debug_DrawHexCounter_Fallback_0475A8

        .size   Debug_DrawHexCounter_ClampBranch_04751E, .-Debug_DrawHexCounter_ClampBranch_04751E


|  ----------------------------------------------------------------------------
|  #3  Debug_DrawHexCounter_Fallback_0475A8  @ $0475A8  (166 B)
|
|  Rama "fallback via tabla decimal": el contador excede $63 (via #1) y
|  el modo es != 0 (via #2). Estructura:
|
|    1. Carga $106E92 en d0 con clamp a $63.
|    2. `jsr $47656(pc)` = #5 = Sub_Divide10: divide d0 por 10, deja
|       cociente en d1 y resto en d0.
|    3. Lookup en tabla long `$28DECC` con:
|         - d2 = table[d0 * 2]  (word: tile-id del digito resto)
|         - d3 = table[d1 * 2]  (word: tile-id del digito cociente)
|    4. Pinta el bloque A (col $7243) con d3 como d1 (cociente arriba),
|       bloque B (col $7283) con d2 como d1 (resto arriba). Solo 8 tiles
|       en total (4+4), a diferencia de #1 y #2 que hacen 8 con
|       aritmetica.
|
|  Firma C conceptual:
|
|      /* Fallback via tabla: convierte el contador a dos digitos hex
|       * usando divisor por 10 (#5) y una tabla word en $28DECC, y
|       * los pinta como bloques 2x2 en el Fix Layer. */
|      void Debug_DrawHexCounter_Fallback(void);
|  ----------------------------------------------------------------------------

        .globl  Debug_DrawHexCounter_Fallback_0475A8
        .type   Debug_DrawHexCounter_Fallback_0475A8, @function
        .section .text.Debug_DrawHexCounter_Fallback_0475A8, "ax", @progbits

Debug_DrawHexCounter_Fallback_0475A8:
        move.w  0x106e92.l, d0                 | +00  d0 = counter
        cmpi.w  #0x63, d0                      | +06  if (d0 <= 99)
        bls.w   .L3_no_clamp                   | +0a     skip clamp
        move.w  #0x63, d0                      | +0e  clamp d0 = 99
.L3_no_clamp:
        jsr     .L3_div10(pc)                  | +12  d0 /= 10, d1 = cociente,
                                              |         d0 = resto
        lea.l   0x28decc.l, a1                 | +16  a1 = &DigitTable[0]
        lsl.w   #0x1, d0                       | +1c  d0 *= 2 (indice word)
        move.w  (a1, d0.w), d2                 | +1e  d2 = DigitTable[d0]
        lsl.w   #0x1, d1                       | +22  d1 *= 2
        move.w  (a1, d1.w), d3                 | +24  d3 = DigitTable[d1]
                                              |
        move.w  #0x7243, d0                    | +28  d0 = $7243 (col A)
        move.w  d3, d1                         | +2c  d1 = cociente-tile
        movem.w d0-d1, 0x3c0000.l              | +2e  MMIO tile write #1
        addi.w  #0x20, d0                      | +36  +$20 columna
        addq.w  #0x1, d1                       | +3a  +1 (tile vecino)
        movem.w d0-d1, 0x3c0000.l              | +3c  MMIO tile write #2
        subi.w  #0x1f, d0                      | +44  -$1F retroceso col
        addi.w  #0xf, d1                       | +48  +$F fila
        movem.w d0-d1, 0x3c0000.l              | +4c  MMIO tile write #3
        addi.w  #0x20, d0                      | +54  +$20 columna
        addq.w  #0x1, d1                       | +58  +1 tile vecino
        movem.w d0-d1, 0x3c0000.l              | +5a  MMIO tile write #4
        subi.w  #0x21, d0                      | +62  -$21 (reset bloque B)
                                              |
        move.w  #0x7283, d0                    | +66  d0 = $7283 (col B)
        move.w  d2, d1                         | +6a  d1 = resto-tile
        movem.w d0-d1, 0x3c0000.l              | +6c  MMIO tile write #5
        addi.w  #0x20, d0                      | +74  +$20 columna
        addq.w  #0x1, d1                       | +78  +1
        movem.w d0-d1, 0x3c0000.l              | +7a  MMIO tile write #6
        subi.w  #0x1f, d0                      | +82  -$1F retroceso col
        addi.w  #0xf, d1                       | +86  +$F fila
        movem.w d0-d1, 0x3c0000.l              | +8a  MMIO tile write #7
        addi.w  #0x20, d0                      | +92  +$20 columna
        addq.w  #0x1, d1                       | +96  +1
        movem.w d0-d1, 0x3c0000.l              | +98  MMIO tile write #8
        subi.w  #0x21, d0                      | +a0  -$21 (reset final)
        rts                                    | +a4

        .equ    .L3_div10, Sub_Divide10_047656

        .size   Debug_DrawHexCounter_Fallback_0475A8, .-Debug_DrawHexCounter_Fallback_0475A8


|  ----------------------------------------------------------------------------
|  #4  Debug_SetCounter_04764E  @ $04764E  (8 B)
|
|  Setter del contador global $106E92. Publica el valor pasado en d0.
|  No es un stub trivial: el epilogo esta contiguo al inicio de #5 sin
|  padding, y el propio helper solo tiene el `move.w` + `rts` (8 B).
|
|  Firma C conceptual:
|
|      /* Publica d0 en el contador global $106E92. */
|      void Debug_SetCounter(uint16_t value /*d0*/);
|  ----------------------------------------------------------------------------

        .globl  Debug_SetCounter_04764E
        .type   Debug_SetCounter_04764E, @function
        .section .text.Debug_SetCounter_04764E, "ax", @progbits

Debug_SetCounter_04764E:
        move.w  d0, 0x106e92.l                 | +00  $106E92 = d0
        rts                                    | +06

        .size   Debug_SetCounter_04764E, .-Debug_SetCounter_04764E


|  ----------------------------------------------------------------------------
|  #5  Sub_Divide10_047656  @ $047656  (32 B)
|
|  Divisor por 10 iterativo con clamp de entrada a $63.
|
|  Algoritmo:
|
|      d0 = min(d0, 99);
|      d2 = 10;
|      d1 = 0;
|      while ((d0 -= 10) >= 0)
|          d1 += 1;
|      d0 += 10;   // undo el ultimo sub que causo bmi
|      /* d1 = cociente (0..9), d0 = resto (0..9) */
|
|  Salida "por bucle sub-add hasta bmi": el `bmi.w` sale del bucle cuando
|  d0 se hace negativo, y el `add.w d2,d0` de despues restaura el
|  ultimo valor positivo como resto. Idioma clasico de asm defensivo
|  hand-coded que evita la `divu` de 68000 (mas lenta para operandos
|  chicos, y con manejo distinto de div-by-zero).
|
|  Complementa el pipeline decimal ya identificado en Wave X:
|    - X#3 `Sub_BinToDecimalDecoder`   (extractor BCD via asl.l variable)
|    - X#4 `Sub_LongDivide`            (divisor 32-iter shift-and-subtract,
|                                       cualquier divisor con TRAP #15 en
|                                       div-by-zero)
|    - AA2 #5 `Sub_Divide10_047656`    (divisor por 10 iterativo,
|                                       optimizado para operandos <100)
|
|  Firma C conceptual:
|
|      /* Divide d0 (0..$63 tras clamp) entre 10; devuelve cociente
|       * en d1 y resto en d0. */
|      void Sub_Divide10(uint16_t *d0 /*inout*/, uint16_t *d1 /*out*/);
|  ----------------------------------------------------------------------------

        .globl  Sub_Divide10_047656
        .type   Sub_Divide10_047656, @function
        .section .text.Sub_Divide10_047656, "ax", @progbits

Sub_Divide10_047656:
        cmpi.w  #0x63, d0                      | +00  if (d0 <= 99)
        bls.w   .L5_no_clamp                   | +04     skip clamp
        move.w  #0x63, d0                      | +08  clamp d0 = 99
.L5_no_clamp:
        move.w  #0xa, d2                       | +0c  d2 = 10
        clr.w   d1                             | +10  d1 = 0
.L5_loop:
        sub.w   d2, d0                         | +12  d0 -= 10
        bmi.w   .L5_out                        | +14  if (d0 < 0) out
        addq.w  #0x1, d1                       | +18  ++d1
        bra.b   .L5_loop                       | +1a  loop
.L5_out:
        add.w   d2, d0                         | +1c  d0 += 10 (undo)
        rts                                    | +1e

        .size   Sub_Divide10_047656, .-Sub_Divide10_047656
