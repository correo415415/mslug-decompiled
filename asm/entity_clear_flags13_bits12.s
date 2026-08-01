| ============================================================================
|  Metal Slug 1 - asm/entity_clear_flags13_bits12.s
|  ----------------------------------------------------------------------------
|  Wave V (Entity flag/probe helpers) - funcion #1
|
|  Entity_ClearFlags13Bits12  @ $0283CA  (14 bytes, 5 callers)
|
|  Limpia los bits 1 y 2 del byte de flags $13(a6) de la entidad apuntada
|  por a6 y retorna. Semantica: cancela dos banderas de estado (tipicamente
|  "pending update" + "handler in progress") antes de que el llamador
|  invoque otra rutina que las volveria a evaluar.
|
|  Firma C conceptual:
|
|      /* Borra dos bits del campo de flags de bajo nivel de la entidad. */
|      void Entity_ClearFlags13Bits12(struct Entity *a6);
|
|  Notas forenses:
|    - Dos bclr.b consecutivos con literal en la misma direccion memoria
|      (mismo (a6) + mismo offset $13) no son rederivables por GCC: el
|      compilador colapsaria las dos operaciones en un unico
|      andi.b #0xF9,$13(a6) (6 B) en vez de emitir 6+6+2 = 14 B.
|    - $13(a6) es un campo de flags de la entidad ya observado en otros
|      contextos (patrones move.b/andi.b/or.b clasicos del interprete
|      de scripts). Se documenta en include/mslug.h como flags13.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ClearFlags13Bits12
        .type   Entity_ClearFlags13Bits12, @function
        .section .text.Entity_ClearFlags13Bits12, "ax", @progbits

Entity_ClearFlags13Bits12:
        bclr.b  #1, 0x13(a6)           | +00  flags13 &= ~0x02
        bclr.b  #2, 0x13(a6)           | +06  flags13 &= ~0x04
        rts                             | +0c
        .size   Entity_ClearFlags13Bits12, .-Entity_ClearFlags13Bits12
