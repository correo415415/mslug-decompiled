| ============================================================================
|  Metal Slug 1 - asm/sprite_hex_format4_05d6c2.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #3
|
|  Sprite_HexFormat4_05D6C2  @ $05D6C2  (90 bytes, 6 callers)
|
|  Formatea un word (d0) como 4 caracteres hex ASCII y los escribe en el
|  Fix Layer (VRAM $3C0000) con base VRAM en a1. Es el primer helper de un
|  cluster de 3 formatteadores hex-a-Fix-Layer que comparten la tabla
|  ASCII "0123456789ABCDEF" en $5D71C:
|
|      $05D6C2  --  4 nibbles (word)   -- este helper (W#3)
|      $05D72C  --  8 nibbles (long)   -- HexFormat8, con supresion de ceros
|      $05D740  --  4 nibbles (word)   -- HexFormat8 entry B (dual entry)
|      $05D71C  --  tabla comun ('0'-'9' 'A'-'F') 16 B
|
|  Los 6 callers son helpers de HUD/debug o de score display: la tabla
|  ASCII lo confirma (no es una tabla de tiles, es literalmente texto que
|  se envia al tile-map del Fix Layer).
|
|  Entrada:
|      d0 : word a formatear (nibbles bits 12,8,4,0 -> chars 0..3)
|      d1 : base VRAM (offset absoluto en $3C0000)
|      a1 : mismo que d1 (usado como offset word por movem.w)
|
|  Salida:
|      $10E21E : buffer de 5 words con los nibbles crudos (0..15)
|      $3C0000 : escrituras movem.w d0/d2 (data + address) x 4
|
|  Algoritmo:
|      Fase 1 (extraccion de nibbles a buffer $10E21E):
|          a2 = $10E21E; d2 = 12; d3 = 4
|          push d3
|          for i in 0..3:  d3 = (d0 >> d2) & 0xF; *(a2)++ = d3; d2 -= 4
|          *(a2) = d0 & 0xF
|          pop d3
|      Fase 2 (traduccion nibble -> ASCII + escritura VRAM):
|          d3--; a3 = $10E21E
|          d0 = a1
|          dbra d3 loop:
|              d4 = *(a3)++
|              d2 = table[d4] & 0xFF   (traduce 0..15 a '0'..'F' via $5D71C)
|              d2 += d1                 (base VRAM)
|              movem.w d0/d2, $3C0000    (data=d0, address=d2, MMIO)
|              d0 += 0x20                (avanza 32 columnas)
|
|  Firma C conceptual:
|
|      void Sprite_HexFormat4(uint16_t value /*d0*/,
|                             uint16_t vram_base /*d1, a1*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. `movem.w d0/d2, $3C0000.l` es un truco 68000 clasico para MMIO:
|       movem con destino MMIO escribe los registros de la lista
|       consecutivamente al mismo puerto en dos ciclos, aprovechando que
|       el Fix Layer decodifica (data, address) como dos writes. GCC nunca
|       emite movem con destino MMIO absoluto.
|    2. La tabla ASCII "0123456789ABCDEF" a 26 B del lea que la referencia
|       via lea $5d71c(pc,d4.w) - dato embebido justo tras el rts.
|       Rederivable como static const char[] pero no con el mismo offset
|       PC-relativo exacto que emitiria GCC.
|    3. Uso del stack (-(a7)/(a7)+) para preservar d2 y d3 sin link/unlk
|       porque en 68000 lsr.l d2,d3 consume d2 como shift-count.
|    4. dbra d3, .Lloop cierra el bucle con contador y signed test tipico
|       de 68000, no de C.
|    5. `andi.l #0xF` (6 B) donde bastaria `andi.b #0xF,d3` (4 B); se hace
|       para limpiar high word antes de `move.w d3,(a2)`.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Sprite_HexFormat4_05D6C2
        .type   Sprite_HexFormat4_05D6C2, @function
        .section .text.Sprite_HexFormat4_05D6C2, "ax", @progbits

Sprite_HexFormat4_05D6C2:
        lea     0x10e21e.l, a2          | +00  a2 = HEX_BUFFER
        moveq   #0xc, d2                | +06  d2 = 12 (shift inicial)
        move.w  #0x4, d3                | +08  d3 = 4 (iters restantes)
        move.w  d3, -(a7)               | +0c  push d3 (preservar durante fase 1)
.Lextract_loop:
        move.w  #0x0, d1                | +0e  d1 = 0 (limpia hi)
        move.l  d2, -(a7)               | +12  push d2 (lsr.l lo consume)
        move.l  d0, d3                  | +14  d3 = d0 (scratch)
        lsr.l   d2, d3                  | +16  d3 >>= d2
        andi.l  #0xf, d3                | +18  d3 &= 0xF (nibble)
        move.w  d3, (a2)                | +1e  *a2 = nibble
        addq.l  #0x2, a2                | +20  a2 += 2
        move.l  (a7)+, d2               | +22  pop d2
        subq.l  #0x4, d2                | +24  d2 -= 4
        bne.b   .Lextract_loop          | +26  d2 != 0 ?
        andi.l  #0xf, d0                | +28  nibble bajo: d0 &= 0xF
        move.w  d0, (a2)                | +2e  *a2 = nibble bajo
        move.w  (a7)+, d3               | +30  pop d3 (restore iter count)
        subq.w  #0x1, d3                | +32  d3-- (para dbra)
        lea     0x10e21e.l, a3          | +34  a3 = HEX_BUFFER (reset cursor)
        move.l  a1, d0                  | +3a  d0 = base VRAM
.Lwrite_loop:
        move.w  (a3)+, d4               | +3c  d4 = *a3++ (nibble)
        move.b  .LhexTable-.-2(pc, d4.w), d2 | +3e  d2 = HEX_TABLE[nibble]  (disp = +26 = tabla justo tras rts)
        andi.w  #0xff, d2               | +42  d2 &= 0xFF
        add.w   d1, d2                  | +46  d2 += base
        movem.w d0/d2, 0x3c0000.l       | +48  MMIO write (data, address)
        addi.w  #0x20, d0               | +50  d0 += 32 (siguiente columna)
        dbra    d3, .Lwrite_loop        | +54  d3--; jump if != -1
        rts                             | +58
.LhexTable:
        | Nota: aunque los 16 B de la tabla ASCII "0123456789ABCDEF" viven
        | fisicamente en $5D71C..$5D72B (justo tras el rts de este helper),
        | NO forman parte de esta funcion: son datos compartidos que otros
        | dos helpers del cluster (HexFormat8_05D72C y HexFormat8_05D740)
        | tambien referencian via lea $5d71c(pc),a0. Se registran como una
        | entidad de datos aparte (Data_HexAsciiTable_5D71C, 16 B) para no
        | duplicar bytes ni asumir propiedad exclusiva. La etiqueta local
        | .LhexTable solo sirve como ancla para el ensamblador; el linker
        | resuelve el displacement PC-relativo al empalmarse con la seccion
        | de datos en la misma direccion absoluta.
        .size   Sprite_HexFormat4_05D6C2, .-Sprite_HexFormat4_05D6C2
