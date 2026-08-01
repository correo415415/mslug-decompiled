| ============================================================================
|  Metal Slug 1 - asm/global_clear_10e486_099afc.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #11
|
|  Global_Clear10E486_099AFC  @ $099AFC  (10 bytes, 2 callers)
|
|  Limpia el byte de flag global en $10E486 (adyacente al bloque
|  TRAIL_ID_COUNTER $10E484 documentado en W#... digo Wave V#6:
|  Entity_TrailRecord_099812). Probablemente es un "trail flush flag"
|  o un lock del sistema de trails - los 2 callers son wrappers del
|  cluster de trails/particulas.
|
|  Firma C conceptual:
|
|      /* Limpia el byte global $10E486 (semantica: 'trail flush pending'
|       * o similar, coherente con el bloque TRAIL_* en $10E47E..$10E486). */
|      void Global_Clear10E486(void);
|
|  Notas forenses:
|    1. `move.b #0, abs.l` (10 B) es la forma "move immediate to abs
|       long address". GCC emitiria `clr.b abs.l` (8 B) que es 2 B mas
|       corto. La forma larga de MSLUG1 es una micro-anti-optimizacion
|       o intencion explicita: `clr.b` en 68000 hace un read-modify-write
|       innecesario a la direccion (bug conocido del 68000), asi que
|       el codigo original usa `move.b #0` para EVITAR el read spurio.
|       Esto es evidencia dura de asm hand-coded consciente de MMIO.
|    2. Los 6 bytes de la instruccion codifican: 13fc (move.b #imm, abs.l)
|       + 0000 (immediate 8-bit encoded como word) + 0010e486 (abs.l).
|       Coherente con el opcode canonico.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Global_Clear10E486_099AFC
        .type   Global_Clear10E486_099AFC, @function
        .section .text.Global_Clear10E486_099AFC, "ax", @progbits

Global_Clear10E486_099AFC:
        move.b  #0x0, 0x10e486.l        | +00  $10E486 = 0  (forma explicita, no clr.b)
        rts                             | +08
        .size   Global_Clear10E486_099AFC, .-Global_Clear10E486_099AFC
