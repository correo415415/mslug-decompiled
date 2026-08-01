| ============================================================================
|  Metal Slug 1 - asm/entity_clear_ptr_slots_05dc1c.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #15
|
|  Entity_ClearPtrSlots_05DC1C  @ $05DC1C  (24 bytes)
|
|  Limpia 35 slots de 4 bytes (long words) del entity apuntado por a0,
|  desde offset $10(a0) hasta $9C(a0) inclusive, escribiendo el valor
|  sentinela $FFFFFFFF (ENTITY_NIL) en cada uno. Se llama desde el
|  allocator de entities $0006FE (Entity_AllocFromFreeList) inmediatamente
|  antes de invocar Entity_InitFields_05DC34 (V#5).
|
|  Semantica: los offsets $10..$9C son la "linked-list/reference table"
|  del entity struct - punteros a parent, child, target, hitbox_owner,
|  etc. Cada uno de los 35 slots almacena un puntero long (4 B) o
|  $FFFFFFFF si esta vacio.
|
|  Algoritmo:
|      d0 = 0x9C
|      do:
|          *(long*)(a0 + d0) = 0xFFFFFFFF   -- clear NIL
|          d0 -= 4
|      while (d0 >= 0x10)                    -- unsigned bcc (Carry Clear)
|      rts
|
|  Firma C conceptual:
|
|      /* Limpia 35 slots de referencia a $FFFFFFFF (ENTITY_NIL) en el
|       * rango [$10..$9C] del entity struct. Usado por el allocator
|       * antes de re-inicializar el resto de campos. */
|      void Entity_ClearPtrSlots(struct Entity *a0);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. move.l #$FFFFFFFF con literal inline (10 B por instruccion, la
|       maxima duracion de opcode de un move.l imediato a memoria
|       indexada). GCC habria emitido `moveq #-1, d1; move.l d1, X(a0)`
|       (6 B por iter, aunque con overhead inicial) o un `movem.l` para
|       escribir varias palabras a la vez.
|    2. Bucle backward con `subi.w #4, d0` + `cmpi.w #$10, d0` + `bcc.b`
|       (unsigned compare). El limite $10 y el paso $4 sobre un rango
|       [$10..$9C] (140 B) dan 35 iteraciones. `dbra` no funcionaria
|       porque el contador debe usarse como offset de indexado, no como
|       simple counter.
|    3. bcc.b con disp -0x12 (`64ee`) reentra en el `move.l` NO en el
|       arranque de la funcion - patron do-while clasico de asm 68000.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ClearPtrSlots_05DC1C
        .type   Entity_ClearPtrSlots_05DC1C, @function
        .section .text.Entity_ClearPtrSlots_05DC1C, "ax", @progbits

Entity_ClearPtrSlots_05DC1C:
        move.w  #0x9c, d0                   | +00  d0 = 0x9C (offset inicial)
.Lloop:
        move.l  #0xffffffff, (a0, d0.w)     | +04  a0[d0].long = ENTITY_NIL
        subi.w  #0x4, d0                    | +0c  d0 -= 4
        cmpi.w  #0x10, d0                   | +10  ¿ d0 >= 0x10 ?
        bcc.b   .Lloop                       | +14  si (Carry Clear): otra iter
        rts                                  | +16
        .size   Entity_ClearPtrSlots_05DC1C, .-Entity_ClearPtrSlots_05DC1C
