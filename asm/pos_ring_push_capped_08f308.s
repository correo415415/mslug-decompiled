| ============================================================================
|  Metal Slug 1 - asm/pos_ring_push_capped_08f308.s
|  ----------------------------------------------------------------------------
|  Wave Z - #10
|
|  PosRing_PushCapped_08F308  @ $08F308  (60 bytes, 1 caller)
|
|  Push condicional al ring buffer de posiciones ubicado en $10E33A. Publica
|  las coordenadas $22(a6) y $24(a6) del entity en (buffer_base + $8 +
|  head_offset), pero SOLO si el contador de entradas ($6(a0)) es < 4.
|
|  Layout del descriptor en $10E33A:
|    +$00 (word)  contador secundario (no tocado por este helper)
|    +$02 (word)  head_offset (indice byte-offset, paso 4, mask $1F wrap)
|    +$04 (word)  desconocido
|    +$06 (word)  count (cap = 4; solo se publica si count < 4)
|    +$08+...     buffer circular de tuplas (posX word, posY word)
|
|  Firma C conceptual:
|
|      /* Publica (entity->field22, entity->field24) en el ring buffer de
|       * $10E33A si count<4. Avanza head_offset con wrap en $20 (8 tuplas
|       * de 4 B) y ++count. */
|      void PosRing_PushCapped(struct Entity *self /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. `lea.l $10E33A.l, a0; lea.l $8(a0), a1; adda.w $2(a0), a1`
|       construye el cursor de escritura EN TRES pasos: base, +$8 (skip
|       header), + head_offset. GCC habria emitido un solo `lea` con
|       indexado (a0, d0.w) o directamente `movea.l head_offset, a1`.
|    2. `move.w $22(a6),(a1); move.w $24(a6),$2(a1)` publica los dos
|       coords como accesos separados, sin fusionar en `move.l` aunque
|       $22 y $24 son contiguos. Es coherente con la convencion del
|       proyecto: campos coord se tratan como pareja de words, nunca
|       como long, ya visto en probe/revert Z#5/Z#6.
|    3. La actualizacion del head hace `addq.w #4, d7; cmpi.w #$20, d7;
|       bcs; clr.w d7` en vez de `andi.w #$1F, d7`. Genera 12 B en vez
|       de 6 B pero evita un shift si $20 no fuera potencia de 2 en el
|       futuro - patente de reserva defensiva hand-coded.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  PosRing_PushCapped_08F308
        .type   PosRing_PushCapped_08F308, @function
        .section .text.PosRing_PushCapped_08F308, "ax", @progbits

PosRing_PushCapped_08F308:
        lea.l   0x10e33a.l, a0                 | +00  a0 = &pos_ring_desc
        lea.l   0x8(a0), a1                    | +06  a1 = &pos_ring_buffer[0]
        adda.w  0x2(a0), a1                    | +0a  a1 += head_offset
        move.w  0x22(a6), (a1)                 | +0e  ring[head].x = self->field22
        move.w  0x24(a6), 0x2(a1)              | +12  ring[head].y = self->field24
        cmpi.w  #0x4, 0x6(a0)                  | +18  if (count >= 4)
        bcc.w   .Lexit                         | +1e     no publish; done
        move.w  0x2(a0), d7                    | +22  d7 = head_offset
        addq.w  #0x4, d7                       | +26  d7 += 4  (tuple stride)
        cmpi.w  #0x20, d7                      | +28  if (d7 < 0x20)
        bcs.w   .Lno_wrap                      | +2c     skip wrap
        clr.w   d7                             | +30  d7 = 0  (wrap)
.Lno_wrap:
        move.w  d7, 0x2(a0)                    | +32  publish head_offset
        addq.w  #0x1, 0x6(a0)                  | +36  ++count
.Lexit:
        rts                                    | +3a

        .size   PosRing_PushCapped_08F308, .-PosRing_PushCapped_08F308
