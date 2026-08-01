| ============================================================================
|  Metal Slug 1 - asm/fix_blit_rect_05da9c.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #7
|
|  Fix_BlitRect_05DA9C  @ $05DA9C  (60 bytes, 3 callers)
|
|  Blit rectangular al Fix Layer con clipping vertical: rellena un
|  rectangulo de d1 columnas x d2 filas de tiles idénticos (d0) en la
|  base a1. Usa la misma tecnica MMIO `movem.w d3-d4, $3C0000` que
|  W#6 (Fix_BlitStream) y W#3/W#4 (HexFormat).
|
|  Entrada:
|      a1 : offset VRAM base (esquina superior izquierda)
|      d0 : valor de tile a escribir (con atributos)
|      d1 : ancho en columnas (se decrementa 1 para dbra)
|      d2 : alto en filas (se decrementa 1 para dbra)
|
|  Salida:
|      d3-d5 : restaurados via movem.l
|
|  Algoritmo:
|      push d3-d5
|      d1--; d2--                 ; convertir a dbra semantics
|      d4 = d0                    ; guardar tile+attrs
|      for row in 0..d2:
|          d3 = a1                ; cursor de columna
|          d5 = d1                ; contador de columnas
|          for col in 0..d5:
|              if $7000 <= d3 <= $74FF:
|                  movem.w d3-d4, $3C0000    ; MMIO write
|              d3 += 0x20
|              dbra d5
|          a1 += 1                ; siguiente fila (offset +1 no +$20:
|                                 ;  el layout del Fix Layer es column-major
|                                 ;  con paso $20 entre columnas y +1 entre
|                                 ;  filas)
|          dbra d2
|      pop d3-d5
|      rts
|
|  Firma C conceptual:
|
|      /* Rellena un rectangulo de width x height tiles idénticos al
|       * Fix Layer con clipping vertical del rango VRAM. */
|      void Fix_BlitRect(uint16_t tile /*d0*/,
|                        uint16_t width /*d1*/, uint16_t height /*d2*/,
|                        uint16_t vram_base /*a1*/);
|
|  Notas forenses:
|    1. movem.l d3-d5,-(a7) al arranque + movem.l (a7)+,d3-d5 al final
|       es el prologo/epilogo clasico de una funcion leaf que necesita
|       preservar registros scratch. GCC habria emitido link/unlk.
|    2. Doble dbra anidado (d5 para columnas, d2 para filas) con
|       decremento manual de d1 y d2 al inicio (5341 5342) - patron
|       tipico de 68000 hand-written que ajusta contadores para el
|       comportamiento signed de dbra.
|    3. adda.w #$1,a1 para avanzar fila (offset +1 en Fix Layer)
|       demuestra que el tile map del MVS es **column-major**: los
|       tiles vecinos horizontalmente estan a offset +$20, los
|       verticalmente a offset +1.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Fix_BlitRect_05DA9C
        .type   Fix_BlitRect_05DA9C, @function
        .section .text.Fix_BlitRect_05DA9C, "ax", @progbits

Fix_BlitRect_05DA9C:
        movem.l d3-d5, -(a7)            | +00  push d3-d5 (scratch)
        subq.w  #0x1, d1                | +04  d1--
        subq.w  #0x1, d2                | +06  d2--
        move.w  d0, d4                  | +08  d4 = tile+attrs (fijo para todos los slots)
.Lrow_loop:
        move.l  a1, d3                  | +0a  d3 = cursor columna (base fila)
        move.w  d1, d5                  | +0c  d5 = ancho
.Lcol_loop:
        cmpi.w  #0x74ff, d3             | +0e  d3 > $74FF ?
        bgt.w   .Lclip                  | +12  si: skip
        cmpi.w  #0x7000, d3             | +16  d3 < $7000 ?
        blt.w   .Lclip                  | +1a  si: skip
        movem.w d3-d4, 0x3c0000.l       | +1e  MMIO (address=d3, data=d4)
.Lclip:
        addi.w  #0x20, d3               | +26  d3 += 32 (siguiente columna)
        dbra    d5, .Lcol_loop          | +2a  d5--; loop
        adda.w  #0x1, a1                | +2e  a1 += 1 (siguiente fila)
        dbra    d2, .Lrow_loop          | +32  d2--; loop
        movem.l (a7)+, d3-d5            | +36  pop d3-d5
        rts                             | +3a
        .size   Fix_BlitRect_05DA9C, .-Fix_BlitRect_05DA9C
