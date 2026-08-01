| ============================================================================
|  Metal Slug 1 - decomp/asm/entity_has_linked.s
|  ----------------------------------------------------------------------------
|  Wave S (Entity/Sprite helpers) - funcion #2
|
|  Entity_HasLinkedSlots  @ $028d70  (30 bytes, 115 callers)
|
|  Comprueba los dos "slots" enlazados de la entidad apuntada por a6:
|      $3c(a6) : slot parent/prev  (ENTITY_NIL = 0xFFFFFFFF si vacio)
|      $40(a6) : slot child/next
|
|  Firma C conceptual (no reproducible por GCC 1:1 porque el resultado
|  se comunica por CCR, no por registro de retorno):
|
|      /* Devuelve implicitamente Z=1 sii "la primera comprobacion util
|       * dio vacia"; Z=0 si el segundo slot esta ocupado. Los llamadores
|       * se ramifican con beq/bne inmediatamente despues del jsr. */
|      void Entity_HasLinkedSlots(struct Entity *a6);
|
|  Curiosidad forense: el bne.w del final NO cae en $028d8e como una
|  "salida alternativa" bien portada, sino que hace fall-through al
|  cuerpo de Script_DispatchOpcode (interprete de scripts, 70 B) que
|  empieza justo un byte despues del rts. Este solape estructural es
|  evidencia dura de que MSLUG1 no es C: ningun compilador emite un
|  branch condicional a la siguiente funcion como "salida" de la actual.
|  Ver asm/script_dispatch.s (Wave T#1) para la funcion destino.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 (bare-metal freestanding)
|              GAS m68k con --register-prefix-optional.
|  ============================================================================

        .text
        .globl  Entity_HasLinkedSlots
        .type   Entity_HasLinkedSlots, @function
        .section .text.Entity_HasLinkedSlots, "ax", @progbits

Entity_HasLinkedSlots:
        movea.l 0x40(a6), a1           | +00  a1 = entity->slot_child
        cmpa.l  #-1, a1                | +04  a1 == ENTITY_NIL ?
        beq.w   .Lret_z1                | +0a  si, salta al rts (Z=1)
        movea.l 0x3c(a6), a1           | +0e  a1 = entity->slot_parent
        cmpa.l  #-1, a1                | +12  a1 == ENTITY_NIL ?
        bne.w   Script_DispatchOpcode   | +18  no, cae en el siguiente helper
.Lret_z1:
        rts                             | +1e
        .size   Entity_HasLinkedSlots, .-Entity_HasLinkedSlots
