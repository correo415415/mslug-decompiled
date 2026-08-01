| ============================================================================
|  Metal Slug 1 - asm/entity_spawn_publish_d0_at70_05239e.s
|  ----------------------------------------------------------------------------
|  Wave Z - #2 (par contiguo con Y#-clone en $0523B2)
|
|  Entity_SpawnAndPublishD0At70_05239E  @ $05239E  (20 bytes)
|
|  Preserva d0 sobre la llamada a Task_AllocFromFreeList_0004AE (T#4) usando
|  la pila, y publica el word d0 (parametro implicito del caller) en $70(a0)
|  del entity recien reservado. El template PC-rel apunta a $523EE.
|
|  Firma C conceptual:
|
|      /* Reserva un entity desde el template $523EE y publica el word
|       * pasado en d0 (por el caller) en el campo $70 del nuevo entity. */
|      void Entity_SpawnAndPublishD0At70(uint16 payload /*d0*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. `move.l d0, -(a7)` + `move.l (a7)+, d0` sobre un unico `jsr`.
|       Idioma clasico de asm hand-coded, ya visto en Entity_AllocByPlayerSlot
|       (Y#7). GCC preservaria d0 en un callee-saved o lo pasaria por pila
|       segun ABI.
|    2. Convencion de paso por d0 (word entero visible como long en pila).
|       ABI GCC pasaria word en la pila alineado a 2, no en d0.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_SpawnAndPublishD0At70_05239E
        .type   Entity_SpawnAndPublishD0At70_05239E, @function
        .section .text.Entity_SpawnAndPublishD0At70_05239E, "ax", @progbits

Entity_SpawnAndPublishD0At70_05239E:
        move.l  d0, -(a7)                      | +00  push d0 (over the jsr)
        lea     .LTpl(pc), a1                  | +02  a1 = &Template_$523EE
        jsr     0x4ae.l                        | +06  Task_AllocFromFreeList (T#4)
        move.l  (a7)+, d0                      | +0c  pop d0
        move.w  d0, 0x70(a0)                   | +0e  a0->field70 = d0_lo
        rts                                    | +12

        .equ    .LTpl, Template_0523EE

        .size   Entity_SpawnAndPublishD0At70_05239E, .-Entity_SpawnAndPublishD0At70_05239E
