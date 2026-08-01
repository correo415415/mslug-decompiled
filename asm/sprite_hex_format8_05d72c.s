| ============================================================================
|  Metal Slug 1 - asm/sprite_hex_format8_05d72c.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #4+#5 (dual entry)
|
|  Sprite_HexFormat8_05D72C  @ $05D72C  (146 bytes totales, dual entry)
|  Sprite_HexFormat8_05D740  @ $05D740  (segundo entry point, +20 B)
|
|  Formatea un valor a 8 o 4 caracteres hex ASCII y los escribe en el
|  Fix Layer (VRAM $3C0000) con supresion de ceros a la izquierda.
|  Segundo par del cluster hex-formatter comparte con W#3 la tabla ASCII
|  "0123456789ABCDEF" en $5D71C.
|
|      Entry A ($5D72C, 20 B) - 8 nibbles (long):
|          d0 = *(--a0)                -- preload del arg desde el stack del caller
|          a2 = $10E21E; d2 = 28; d3 = 8
|          push d3
|          bra.w .Lcommon
|
|      Entry B ($5D740, 14 B) - 4 nibbles (word):
|          a2 = $10E21E; d2 = 12; d3 = 4
|          push d3
|          -- fall-through a .Lcommon --
|
|      Cuerpo comun ($5D74E-$5D7BE, 112 B):
|          Fase 1 (extraccion de nibbles): identica a W#3
|              d1 = 0
|              for i in 0..d3-1:  push d2; d3 = (d0 >> d2) & 0xF;
|                                  *(a2)++ = d3; d2 -= 4; pop d2
|              *(a2) = d0 & 0xF
|          Fase 2 (traduccion + escritura con supresion de ceros):
|              pop d3; d3--; d2 = 0
|              a3 = $10E21E; a0 = HEX_TABLE ($5D71C)
|              d5 = a1 (base VRAM)
|              dbra d3 loop:
|                  d4 = *(a3)++
|                  if (d4 != 0)       -- nibble no-cero
|                      d2 = 0xFFFF    -- marca "encontrado no-cero"
|                      goto .Lprint
|                  if (d2 != 0)       -- ya encontramos no-cero antes
|                      d2 = 0xFFFF
|                      goto .Lprint
|                  if (d3 == 0)       -- es el ultimo digito
|                      d2 = 0xFFFF
|                      goto .Lprint
|                  d4 = 0x10           -- mostrar espacio (indice fuera de tabla)
|                  bra .Lprint2
|                .Lprint:
|                  d2 = 0xFFFF
|                .Lprint2:
|                  d6 = table[d4] & 0xFF
|                  d6 += d1
|                  movem.w d5/d6, $3C0000  -- MMIO (data=d5, addr=d6)
|                  d5 += 0x20
|
|  Nota clave sobre el flag d2: se usa como "sticky bit" para recordar
|  que ya se encontro un digito no-cero (una vez encontrado, todos los
|  restantes se imprimen). $10 fuera del rango [0..15] indexa el byte
|  del padding tras la tabla ($5D72C = 0x20 = ' '), sirviendo como el
|  caracter espacio para leading zeros.
|
|  Firma C conceptual:
|
|      /* Entry A: value viene por -(a0) (stack pop implicito), 8 chars. */
|      void Sprite_HexFormat8_FromStack(uint16_t vram_base /*d1,a1*/);
|      /* Entry B: value ya en d0, 4 chars con leading-zero suppression. */
|      void Sprite_HexFormat4_ZeroSup(uint16_t value /*d0*/,
|                                     uint16_t vram_base /*d1,a1*/);
|
|  Notas forenses:
|    1. Dual entry-point contiguo con bra.w de A a cuerpo comun, y
|       fall-through directo de B al cuerpo comun. Mismo idioma que
|       Sprite_SetupSlotFromTableA/B (V#4).
|    2. move.l -(a0),d0 en $05D72C es un uso de pre-decrement con a0
|       como puntero de stack alternativo: el caller pasa un long por
|       (-(a0)) en vez de por registro o (a7). Idioma tipico de asm
|       hecho a mano cuando a7 esta ocupado.
|    3. Uso de d2=0xFFFF como sticky bit para leading-zero suppression:
|       GCC habria emitido un bool separado o un branch al principio
|       del bucle. Aqui se aprovecha que el valor 0xFFFF no colisiona
|       con ningun nibble (0..15).
|    4. Comparte la tabla ASCII con W#3 via lea $5d71c(pc),a0 con
|       displacement 16-bit negativo (0xFF9E = -0x62).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Sprite_HexFormat8_05D72C
        .globl  Sprite_HexFormat8_05D740
        .type   Sprite_HexFormat8_05D72C, @function
        .type   Sprite_HexFormat8_05D740, @function
        .section .text.Sprite_HexFormat8_05D72C, "ax", @progbits

Sprite_HexFormat8_05D72C:
        move.l  -(a0), d0               | +00  d0 = *(--a0)   pop long del "stack alt"
        lea     0x10e21e.l, a2          | +02  a2 = HEX_BUFFER
        moveq   #0x1c, d2               | +08  d2 = 28 (shift inicial, 7 nibbles altos)
        move.w  #0x8, d3                | +0a  d3 = 8 (iteraciones)
        move.w  d3, -(a7)               | +0e  push d3
        bra.w   .Lextract_loop          | +10  saltar al cuerpo comun (14 B ahead)
Sprite_HexFormat8_05D740:
        lea     0x10e21e.l, a2          | +14  a2 = HEX_BUFFER
        moveq   #0xc, d2                | +1a  d2 = 12 (shift inicial, 3 nibbles altos)
        move.w  #0x4, d3                | +1c  d3 = 4 (iteraciones)
        move.w  d3, -(a7)               | +20  push d3
        | -- fall-through a .Lextract_loop --
.Lextract_loop:
        move.w  #0x0, d1                | +22  d1 = 0 (dentro del bucle: se recarga cada iter)
        move.l  d2, -(a7)               | +26  push d2
        move.l  d0, d3                  | +28  d3 = d0
        lsr.l   d2, d3                  | +2a  d3 >>= d2
        andi.l  #0xf, d3                | +2c  d3 &= 0xF
        move.w  d3, (a2)                | +32  *a2 = nibble
        addq.l  #0x2, a2                | +34  a2 += 2
        move.l  (a7)+, d2               | +36  pop d2
        subq.l  #0x4, d2                | +38  d2 -= 4
        bne.b   .Lextract_loop          | +3a  loop
        andi.l  #0xf, d0                | +3c  d0 &= 0xF (nibble bajo)
        move.w  d0, (a2)                | +42  *a2 = nibble bajo
        move.w  (a7)+, d3               | +44  pop d3 (restore iter count)
        subq.w  #0x1, d3                | +46  d3-- (para dbra)
        clr.w   d2                      | +48  d2 = 0 (sticky bit inicial)
        lea     0x10e21e.l, a3          | +4a  a3 = HEX_BUFFER (reset cursor)
        lea     HEX_TABLE_5D71C(pc), a0 | +50  a0 = HEX_TABLE ($5D71C, disp = -0x62 PC-rel 16-bit)
        move.l  a1, d5                  | +54  d5 = base VRAM
.Lwrite_loop:
        move.w  (a3)+, d4               | +56  d4 = *a3++ (nibble)
        tst.w   d4                      | +58  ¿ nibble no-cero ?
        bne.w   .Lprint_marker          | +5a  si: marcar y imprimir
        tst.w   d2                      | +5e  ¿ sticky bit ya activo ?
        bne.w   .Lprint_marker          | +60  si: imprimir (ya no hay leading zeros)
        tst.w   d3                      | +64  ¿ es el ultimo digito ?
        beq.w   .Lprint_marker          | +66  si: imprimir cero como valor final
        move.w  #0x10, d4               | +6a  no: mostrar espacio (indice 16 = padding tabla)
        bra.w   .Lprint                 | +6e  ir a impresion sin activar sticky
.Lprint_marker:
        move.w  #0xffff, d2             | +72  activar sticky bit
.Lprint:
        move.b  (a0, d4.w), d6          | +76  d6 = HEX_TABLE[d4]
        andi.w  #0xff, d6               | +7a  d6 &= 0xFF
        add.w   d1, d6                  | +7e  d6 += base (VRAM columna)
        movem.w d5/d6, 0x3c0000.l       | +80  MMIO (data=d5, addr=d6)
        addi.w  #0x20, d5               | +88  d5 += 32 (siguiente columna)
        dbra    d3, .Lwrite_loop        | +8c  d3--; loop if != -1
        rts                             | +90
        | Nota: los 16 B de la tabla ASCII "0123456789ABCDEF" viven fisicamente
        | 0x62 B ANTES de esta funcion (en $5D71C, entre HexFormat4 y HexFormat8).
        | El lea de arriba se resuelve como PC-rel negativo via --defsym en
        | tools/symbols.py. Ver W#3 (Sprite_HexFormat4_05D6C2) para la primera
        | funcion del cluster que la usa con displacement +0x1A.
        .size   Sprite_HexFormat8_05D72C, .-Sprite_HexFormat8_05D72C
        .size   Sprite_HexFormat8_05D740, .-Sprite_HexFormat8_05D740
