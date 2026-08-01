| ============================================================================
|  Metal Slug 1 - asm/entity_spawn_and_tag_06e224.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #7
|
|  Entity_SpawnAndTag_06E224  @ $06E224  (38 bytes)
|
|  Spawner con "tag" derivado del bit 0 del parametro d0. Reserva un entity
|  desde el template $6DF32, copia transform, y publica bit0(d0) en $3A
|  (byte) del nuevo entity. Preserva d0 sobre los dos jsr con movem.w. Al
|  final invoca $6E2BC(pc) como post-hook.
|
|  Absorbe JsrPcThunk_06e244 (Wave J): los ultimos 6 B de la funcion
|  (`jsr $6E2BC(pc); rts`) fueron catalogados como thunk independiente
|  por el escaner Wave J. Es el 19 falso positivo del proyecto.
|
|  Firma C conceptual:
|
|      /* Reserva entity desde template fijo, copia transform del padre,
|       * escribe bit0(d0) en el campo $3A del hijo, invoca hook y retorna. */
|      void Entity_SpawnAndTag(uint8 tag_bit /*d0*/,
|                              struct Entity *parent /*a6*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_SpawnAndTag_06E224
        .type   Entity_SpawnAndTag_06E224, @function
        .section .text.Entity_SpawnAndTag_06E224, "ax", @progbits

Entity_SpawnAndTag_06E224:
        movem.w d0, -(a7)                      | +00  push d0 (idioma canonico)
        lea     .LTpl(pc), a1                  | +04  a1 = &Template_06DF32
        jsr     0x4ae.l                        | +08  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +0e  Entity_CopyTransform (S#4)
        movem.w (a7)+, d0                      | +14  pop d0
        andi.b  #0x1, d0                       | +18  d0 &= 0x01 (tag bit)
        move.b  d0, 0x3a(a0)                   | +1c  new->field3A = tag
        jsr     .Lpost(pc)                     | +20  post-hook en $06E2BC
        rts                                    | +24

        .equ    .LTpl,  Template_06DF32
        .equ    .Lpost, PcThunkTarget_06e2bc     | nombre canonico historico

        .size   Entity_SpawnAndTag_06E224, .-Entity_SpawnAndTag_06E224
