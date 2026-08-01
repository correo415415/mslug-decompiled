| ============================================================================
|  Metal Slug 1 - asm/entity_probe_slot4c_0283d8.s
|  ----------------------------------------------------------------------------
|  Wave V (Entity flag/probe helpers) - funcion #2
|
|  Entity_ProbeSlot4c_0283D8  @ $0283D8  (20 bytes, 6 callers)
|
|  Sondea el slot enlazado en $4c(a6) (que hasta ahora hemos visto usarse
|  como "handler slot" / "child entity slot" en otros helpers del cluster
|  $028xxx):
|      d7  = 0xFF              -- valor centinela (se restaura como retorno
|                                 implicito y como "flag pendiente")
|      a0  = *(entity->slot4c)
|      si  entity->slot4c == ENTITY_NIL (0xFFFFFFFF)
|          rts                 -- retorna dejando d7=0xFF y a0=ENTITY_NIL
|      si no
|          fall-through a $283EC (cuerpo del helper siguiente que consume
|          a0 como puntero valido)
|
|  Firma C conceptual (retorno por CCR + fall-through, no rederivable
|  por GCC 1:1):
|
|      /* Retorna 'existe' via Z de la comparacion; si existe cae en el
|       * helper contiguo con a0 ya cargado. d7=0xFF se usa por el
|       * llamador como marcador. */
|      void Entity_ProbeSlot4c(struct Entity *a6);
|
|  Notas forenses:
|    - moveq #-1,d7 seguido de un cmpi.l #-1,$4c(a6) que NO usa d7 es
|      evidencia forense clara de asm hecho a mano: un compilador habria
|      cargado la constante en un scratch (d0/d1) o directamente habria
|      usado el immediate en el compare, no en d7 sin proposito visible
|      dentro de la funcion.
|    - El bne.w cae a $283EC como "salida alternativa": misma tecnica
|      que Entity_HasLinkedSlots (Wave S#2) fall-through a
|      Script_DispatchOpcode.
|    - $4c(a6) documentado como slot_handler en include/mslug.h.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ProbeSlot4c_0283D8
        .type   Entity_ProbeSlot4c_0283D8, @function
        .section .text.Entity_ProbeSlot4c_0283D8, "ax", @progbits

Entity_ProbeSlot4c_0283D8:
        moveq   #-1, d7                 | +00  d7 = 0xFFFFFFFF (sentinel)
        movea.l 0x4c(a6), a0            | +02  a0 = entity->slot4c
        cmpi.l  #-1, 0x4c(a6)           | +06  slot4c == ENTITY_NIL ?
        bne.w   Sub_0002_83EC           | +0e  no, cae en el helper contiguo
        rts                             | +12
        .size   Entity_ProbeSlot4c_0283D8, .-Entity_ProbeSlot4c_0283D8
