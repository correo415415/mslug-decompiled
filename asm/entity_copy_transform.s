| ============================================================================
|  Metal Slug 1 - asm/entity_copy_transform.s
|  ----------------------------------------------------------------------------
|  Wave S (Entity/Sprite helpers) - funcion #4
|
|  Entity_CopyTransform  @ $05dd02  (32 bytes, 19 callers)
|
|  Copia 5 campos de la entidad src (a6) a la entidad dst (a0):
|    - pos_x   ($22, word)
|    - pos_y   ($24, word)
|    - flags38 ($38, word) - probablemente flags visuales/animacion
|    - flags3a ($3a, byte) - siguiente byte tras flags38
|    - flags11 ($11, byte) - campo temprano del header, semantica pendiente
|
|  Entrada (registros absolutos):
|      a0 : entidad destino
|      a6 : entidad origen
|
|  Salida: ninguna (retorna via rts sin tocar d0)
|
|  Firma C conceptual:
|      void Entity_CopyTransform(struct Entity *a0 /*dst*/,
|                                struct Entity *a6 /*src*/);
|
|  Notas forenses:
|    - 5 moves consecutivos memory-to-memory con el mismo offset origen
|      y destino son emitibles por GCC solo con __builtin_memcpy sobre
|      un struct bit-packed muy especifico; GCC 13 no emite exactamente
|      esta secuencia sin gimnasia. Se codifica en .s para matchear.
|    - Los offsets $22,$24,$38,$3a,$11 se consolidan en struct Entity
|      en include/mslug.h en esta misma iteracion.
|  ============================================================================

        .text
        .globl  Entity_CopyTransform
        .type   Entity_CopyTransform, @function
        .section .text.Entity_CopyTransform, "ax", @progbits

Entity_CopyTransform:
        move.w  0x22(a6), 0x22(a0)     | +00  dst.pos_x   = src.pos_x
        move.w  0x24(a6), 0x24(a0)     | +06  dst.pos_y   = src.pos_y
        move.w  0x38(a6), 0x38(a0)     | +0c  dst.flags38 = src.flags38
        move.b  0x3a(a6), 0x3a(a0)     | +12  dst.flags3a = src.flags3a
        move.b  0x11(a6), 0x11(a0)     | +18  dst.flags11 = src.flags11
        rts                             | +1e
        .size   Entity_CopyTransform, .-Entity_CopyTransform
