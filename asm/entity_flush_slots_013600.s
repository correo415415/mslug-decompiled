| ============================================================================
|  Metal Slug 1 - asm/entity_flush_slots_013600.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #9
|
|  Entity_FlushSlotHistory_013600  @ $013600  (36 bytes, 3 callers)
|
|  Vacia la cola de slots pendientes del entity (a6) llamando a Sub_2BC4
|  para cada slot registrado en slot_history[$16..$1c], y limpia el slot
|  tras usarlo. El contador `timer0` en $1e(a6) actua como puntero de
|  cima de pila LIFO: se decrementa antes de leer cada slot, y el bucle
|  termina cuando llega a 0.
|
|  Algoritmo:
|      while (entity->timer0 > 0):
|          entity->timer0--                       -- consume slot
|          d2 = entity->timer0 * 2                -- word offset en slot_history
|          d1 = entity->slot_history[d2]          -- lee slot idx
|          entity->slot_history[d2] = 0           -- limpia entrada
|          jsr Sub_00002BC4                       -- procesa slot (probablemente
|                                                    "release/deactivate slot idx")
|          -- loop back al tst.w $1e(a6) --
|      rts
|
|  Firma C conceptual:
|
|      /* Libera todos los slots pendientes registrados en el entity
|       * llamando a Sub_2BC4 para cada uno. Es el drain de emergencia
|       * usado por el destructor y por eventos de nivel. */
|      void Entity_FlushSlotHistory(struct Entity *a6);
|
|  Notas forenses:
|    1. `bra.b $13600` (self-loop al PRINCIPIO de la funcion, no a una
|       etiqueta interna) es idioma clasico de asm 68000 para bucles
|       LIFO drain. GCC habria emitido `while(cond) { body; }` con una
|       etiqueta al inicio del body y bra al test.
|    2. subq.w #1 sobre memoria directa `$1e(a6)` en vez de load/mod/
|       store: GCC solo emite RMW-memory en -Os con -mmemcpy-with-decr;
|       en -m68000 -Os regular tiende a `move.w $1e(a6),d0 ; subq.w
|       #1,d0 ; move.w d0,$1e(a6)`.
|    3. clr.w $16(a6, d2.w) borra la entrada leida - es el idioma
|       "consume-and-clear" tipico de free-lists de asm hand-coded.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_FlushSlotHistory_013600
        .type   Entity_FlushSlotHistory_013600, @function
        .section .text.Entity_FlushSlotHistory_013600, "ax", @progbits

Entity_FlushSlotHistory_013600:
        tst.w   0x1e(a6)                | +00  ¿ quedan slots ?
        beq.w   .Ldone                  | +04  no: salir
        subq.w  #0x1, 0x1e(a6)          | +08  timer0--
        move.w  0x1e(a6), d2            | +0c  d2 = timer0
        add.w   d2, d2                  | +10  d2 *= 2 (word offset)
        move.w  0x16(a6, d2.w), d1      | +12  d1 = slot_history[timer0]
        clr.w   0x16(a6, d2.w)          | +16  slot_history[timer0] = 0
        jsr     Sub_00002BC4            | +1a  procesa slot (release)
        bra.b   Entity_FlushSlotHistory_013600 | +20  loop back al tst
.Ldone:
        rts                             | +22
        .size   Entity_FlushSlotHistory_013600, .-Entity_FlushSlotHistory_013600
