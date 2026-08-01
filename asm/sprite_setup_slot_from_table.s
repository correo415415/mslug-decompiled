| ============================================================================
|  Metal Slug 1 - asm/sprite_setup_slot_from_table.s
|  ----------------------------------------------------------------------------
|  Wave V (Entity/Sprite helpers) - funcion #4
|
|  Sprite_SetupSlotFromTableA  @ $002C26  (66 bytes cabecera-a-rts, 6 callers)
|  Sprite_SetupSlotFromTableB  @ $002C30  (dual entry-point contiguo, 6 callers)
|
|  Configura una entrada de la "slot table de sprites" en $1082C8 a partir
|  de un indice y una base de tabla de descriptores. Los llamadores eligen
|  el banco de descriptores llamando a una u otra etiqueta:
|
|      $002C26 : banco A  ->  a2 = $1CE00  (tabla A, +72 KiB desde $14E00)
|      $002C30 : banco B  ->  a2 = $14E00  (tabla B, base)
|
|  Ambas entradas convergen en $002C36 donde:
|      d2 = (uint16_t)d2 * 64  + a2   ; puntero al descriptor de sprite
|      d1 <<= 5                       ; d1 -> byte offset en slot table (32 B/slot)
|      slot = $1082C8[d1]
|      slot.desc_ptr ($0A) = d2
|      slot.w08 = slot.w10 = d3       ; probablemente pos_x / anchor_x
|      slot.b0e = 0
|      slot.b0f = d4                  ; flip/prio/palette
|      slot.b00 |= 0x02               ; SLOT_FLAG_ACTIVE
|
|  Firma C conceptual (dual entry con banco fijo, no rederivable por GCC):
|
|      /* Ambas devuelven slot preparado; el llamador nunca lee el retorno.
|       * a1 apunta a la slot table (fija en $1082C8). */
|      void Sprite_SetupSlotFromTableA(uint16_t idx  /*d2*/,
|                                      uint16_t slot /*d1*/,
|                                      uint16_t pos  /*d3*/,
|                                      uint8_t  attr /*d4*/);
|      void Sprite_SetupSlotFromTableB(uint16_t idx, uint16_t slot,
|                                      uint16_t pos, uint8_t attr);
|
|  Notas forenses:
|    - Dos entradas contiguas en la misma seccion, la primera saltando 8 B
|      sobre la segunda mediante bra.w para "seleccionar el banco A y
|      caer en el codigo comun". GCC no emite esto: dividiria el codigo
|      en helper + wrapper y lo alinearia.
|    - move.b #$0 seguido de move.b d4 al offset contiguo ($0e, $0f) es
|      un idioma de ensamblador que GCC colapsaria en un unico
|      move.w #(0<<8|d4),$0e(...) con arithmetica previa.
|    - $1082C8 es la base de la SlotTable global (32 B por slot, escala
|      d1<<5); se documenta en include/mslug.h como SPRITE_SLOT_TABLE.
|    - Los bancos $14E00 y $1CE00 tienen exactamente 0x8000 (32 KiB) de
|      separacion: probablemente dos paginas de descriptores 64B*512.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Sprite_SetupSlotFromTableA
        .globl  Sprite_SetupSlotFromTableB
        .type   Sprite_SetupSlotFromTableA, @function
        .type   Sprite_SetupSlotFromTableB, @function
        .section .text.Sprite_SetupSlotFromTableA, "ax", @progbits

Sprite_SetupSlotFromTableA:
        lea     0x1ce00.l, a2           | +00  banco A: descriptores en $1CE00
        bra.w   .Lcommon                | +06  salta a codigo comun (skip 8 B)
Sprite_SetupSlotFromTableB:
        lea     0x14e00.l, a2           | +0a  banco B: descriptores en $14E00
.Lcommon:
        andi.l  #0xffff, d2             | +10  d2 &= 0xFFFF (zero-extend idx)
        lsl.l   #6, d2                  | +16  d2 *= 64   (tamano de descriptor)
        add.l   a2, d2                  | +18  d2 += base -> puntero absoluto
        lsl.w   #5, d1                  | +1a  d1 *= 32   (tamano de slot)
        lea     0x1082c8.l, a1          | +1c  a1 = SPRITE_SLOT_TABLE
        move.l  d2, 0x0a(a1, d1.w)      | +22  slot.desc_ptr = d2
        move.w  d3, 0x08(a1, d1.w)      | +26  slot.w08 = d3
        move.w  d3, 0x10(a1, d1.w)      | +2a  slot.w10 = d3
        move.b  #0, 0x0e(a1, d1.w)      | +2e  slot.b0e = 0
        move.b  d4, 0x0f(a1, d1.w)      | +34  slot.b0f = d4
        ori.b   #2, (a1, d1.w)          | +38  slot.b00 |= SLOT_FLAG_ACTIVE
        rts                             | +3e
        .size   Sprite_SetupSlotFromTableA, .-Sprite_SetupSlotFromTableA
        .size   Sprite_SetupSlotFromTableB, .-Sprite_SetupSlotFromTableB
