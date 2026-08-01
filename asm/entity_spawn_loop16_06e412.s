| ============================================================================
|  Metal Slug 1 - asm/entity_spawn_loop16_06e412.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #6
|
|  Entity_SpawnLoop16_06E412  @ $06E412  (36 bytes)
|
|  Bucle de spawn de 16 entities desde el template PC-rel $6DD5C. Cada
|  iteracion:
|    - jsr $4AE (Task_AllocFromFreeList, T#4)
|    - jsr $5DD02 (Entity_CopyTransform, S#4)
|    - copia $9B(a6) -> $9B(a0)   (propaga campo del padre al hijo)
|
|  Contador en la pila (`move.w d0, -(a7)` + `move.w (a7)+, d0`). Termina
|  cuando d0 decrementa a 0.
|
|  Firma C conceptual:
|
|      /* Spawn 16 hijos identicos desde un mismo template, propagando el
|       * byte $9B del padre a cada uno. */
|      void Entity_SpawnLoop16(struct Entity *parent /*a6*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_SpawnLoop16_06E412
        .type   Entity_SpawnLoop16_06E412, @function
        .section .text.Entity_SpawnLoop16_06E412, "ax", @progbits

Entity_SpawnLoop16_06E412:
        move.w  #0x10, d0                      | +00  d0 = 16 (contador)
.Lloop:
        move.w  d0, -(a7)                      | +04  push d0 (over jsrs)
        lea     .LTpl(pc), a1                  | +06  a1 = &Template_06DD5C
        jsr     0x4ae.l                        | +0a  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +10  Entity_CopyTransform (S#4)
        move.b  0x9b(a6), 0x9b(a0)             | +16  new->field9B = parent->field9B
        move.w  (a7)+, d0                      | +1c  pop d0
        subq.w  #0x1, d0                       | +1e  --d0
        bne.b   .Lloop                         | +20  loop until 0
        rts                                    | +22

        .equ    .LTpl, Template_06DD5C

        .size   Entity_SpawnLoop16_06E412, .-Entity_SpawnLoop16_06E412
