| ============================================================================
|  Metal Slug 1 - asm/entity_reserve_and_setpos_05e4b2.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #10
|
|  Entity_ReserveAndSetPos_05E4B2  @ $05E4B2  (24 bytes, 2 callers)
|
|  Reserva el slot del entity marcando bit 6 de flags13, llama a un
|  helper local que devuelve tres valores en d0/d1/d2 (probablemente un
|  RNG o un decoder de parametros), y aplica esos valores a los campos
|  flags38, pos_x y pos_y del entity.
|
|  Algoritmo:
|      entity->flags13 |= 0x40      -- SLOT_RESERVED (bit 6, mismo bit
|                                       que Entity_InitFields_05DC34 activa
|                                       al final de su inicializacion)
|      (d0, d1, d2) = Sub_05E4CA()  -- helper PC-relativo (16 B mas alla)
|      entity->flags38 = d0         -- $38(a6)
|      entity->pos_x   = d1         -- $22(a6)
|      entity->pos_y   = d2         -- $24(a6)
|      rts
|
|  Firma C conceptual:
|
|      /* Reserva el slot del entity y le asigna flags visuales +
|       * posicion (X,Y) a partir de un helper local que retorna 3 valores. */
|      void Entity_ReserveAndSetPos(struct Entity *a6);
|
|  Notas forenses:
|    1. jsr d16(pc) con destino a 16 B - forma corta que solo cabe en
|       PC-rel 16-bit. Los 2 callers deben usarlo como wrapper cercano.
|    2. `move.w d0/d1/d2, offset(a6)` con tres registros de retorno es
|       la "convencion de paso multiple" clasica de asm 68000: el helper
|       Sub_05E4CA emite los 3 valores en d0/d1/d2 y el caller los aplica
|       directamente sin buffer intermedio. GCC habria emitido un struct
|       de retorno con puntero via a1 o similar.
|    3. bset.b #6, $13(a6) es la misma reserva que Entity_InitFields_05DC34
|       hace al final de su init - el bit 6 de flags13 es el "SLOT_RESERVED"
|       consolidado por consistencia entre helpers.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ReserveAndSetPos_05E4B2
        .type   Entity_ReserveAndSetPos_05E4B2, @function
        .section .text.Entity_ReserveAndSetPos_05E4B2, "ax", @progbits

Entity_ReserveAndSetPos_05E4B2:
        bset.b  #0x6, 0x13(a6)          | +00  entity->flags13 |= SLOT_RESERVED
        jsr     Sub_00005E4CA(pc)       | +06  (d0,d1,d2) = helper local
        move.w  d0, 0x38(a6)            | +0a  entity->flags38 = d0
        move.w  d1, 0x22(a6)            | +0e  entity->pos_x   = d1
        move.w  d2, 0x24(a6)            | +12  entity->pos_y   = d2
        rts                             | +16
        .size   Entity_ReserveAndSetPos_05E4B2, .-Entity_ReserveAndSetPos_05E4B2
