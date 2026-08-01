| ============================================================================
|  Metal Slug 1 - asm/entity_add_timer_0138fe.s
|  ----------------------------------------------------------------------------
|  Wave V (Entity/Sprite helpers) - funcion #7
|
|  Entity_AddTimer0_74_0138FE  @ $0138FE  (8 bytes, 3 callers)
|
|  Suma la constante $74 al timer0 ($1c(a6)) de la entidad apuntada por a6.
|  Los 3 callers son wrappers de scheduling que "arman" el timer a un valor
|  base + $74 antes de encolar el siguiente evento.
|
|  Firma C conceptual:
|
|      /* Suma exactamente 0x74 al campo timer0 (word, +$1c). */
|      void Entity_AddTimer0_74(struct Entity *a6);
|
|  Notas forenses:
|    - addi.w #imm,off(a6) con imm=0x74 (6 B) donde GCC habria emitido
|      `add.w #$74,$1c(a6)` no es identico a addi.w a nivel de opcode:
|      GCC 13/14 al usar la forma con destino memoria puede elegir add.w
|      cuando el imm cabe en un `addq` de +8, o addi.w para el resto. En
|      este caso 0x74 > 8 asi que ambas formas coinciden en tamano pero
|      difieren en opcode (066E vs 0X6E). El .s garantiza addi.w.
|    - Podria ser rederivable en C como `*(volatile uint16_t*)&e->timer0
|      += 0x74;` pero probar en C anadiria una fuente y multiplicaria
|      la superficie de test para 8 B: no compensa. Se documenta aqui.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_AddTimer0_74_0138FE
        .type   Entity_AddTimer0_74_0138FE, @function
        .section .text.Entity_AddTimer0_74_0138FE, "ax", @progbits

Entity_AddTimer0_74_0138FE:
        addi.w  #0x74, 0x1c(a6)        | +00  entity->timer0 += 0x74
        rts                             | +06
        .size   Entity_AddTimer0_74_0138FE, .-Entity_AddTimer0_74_0138FE
