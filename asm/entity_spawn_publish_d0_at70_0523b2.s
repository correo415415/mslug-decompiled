| ============================================================================
|  Metal Slug 1 - asm/entity_spawn_publish_d0_at70_0523b2.s
|  ----------------------------------------------------------------------------
|  Wave Z - #3 (clon casi identico del Wave Z #2, template distinto)
|
|  Entity_SpawnAndPublishD0At70_0523B2  @ $0523B2  (20 bytes)
|
|  Clon byte-a-byte de Entity_SpawnAndPublishD0At70_05239E salvo el template
|  PC-rel: $524AA en lugar de $523EE. Los 2 spawners coexisten porque cada
|  uno se llama con un template distinto pero la misma convencion.
|
|  Firma C conceptual:
|
|      /* Reserva un entity desde el template $524AA y publica el word
|       * pasado en d0 (por el caller) en el campo $70 del nuevo entity. */
|      void Entity_SpawnAndPublishD0At70_alt(uint16 payload /*d0*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Duplicado byte-a-byte de la funcion vecina en $05239E excepto el
|       word `43 fa 00 f4` vs `43 fa 00 4c`. El asm original NO factoriza
|       templates en un solo helper con parametro - los duplica.
|       GCC habria inlineado ambos en una macro o los habria consolidado.
|    2. El template esta HACIA DELANTE (offset PC-rel positivo $00F4).
|       Idioma "code delante, datos detras" comun en asm hand-coded del juego.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_SpawnAndPublishD0At70_0523B2
        .type   Entity_SpawnAndPublishD0At70_0523B2, @function
        .section .text.Entity_SpawnAndPublishD0At70_0523B2, "ax", @progbits

Entity_SpawnAndPublishD0At70_0523B2:
        move.l  d0, -(a7)                      | +00  push d0 (over the jsr)
        lea     .LTpl(pc), a1                  | +02  a1 = &Template_$524AA
        jsr     0x4ae.l                        | +06  Task_AllocFromFreeList (T#4)
        move.l  (a7)+, d0                      | +0c  pop d0
        move.w  d0, 0x70(a0)                   | +0e  a0->field70 = d0_lo
        rts                                    | +12

        .equ    .LTpl, Template_0524AA

        .size   Entity_SpawnAndPublishD0At70_0523B2, .-Entity_SpawnAndPublishD0At70_0523B2
