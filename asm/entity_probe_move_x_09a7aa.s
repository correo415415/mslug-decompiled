| ============================================================================
|  Metal Slug 1 - asm/entity_probe_move_x_09a7aa.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #8
|
|  Entity_ProbeMoveX_09A7AA  @ $09A7AA  (34 bytes, 3 callers)
|
|  Prueba un movimiento horizontal de la entidad (a0) y lo aplica si el
|  probe pasa. Es un trampolin de fisica que combina:
|
|      1. jsr $9A7CC(pc)      -- probe global (retorna condicion por CCR)
|      2. si Carry = 1        -> retorna sin mover (colision)
|      3. si Carry = 0:
|           d0 = 0x10
|           si $3a(a6) bit 0 = 1  ->  d0 = -0x10  (mirror horizontal)
|           $22(a0) += d0                          -- pos_x += ±16
|           andi.b #$EE, ccr                       -- limpia X, Z de CCR
|           rts
|
|  Firma C conceptual:
|
|      /* Intenta mover el entity (a0) horizontalmente 16 px en la
|       * direccion codificada por flags3a(a6).bit0. Si Sub_09A7CC
|       * indica colision (C=1), no mueve. La rutina limpia bits X, Z
|       * del CCR antes de retornar (el caller lee CCR). */
|      void Entity_ProbeMoveX(struct Entity *a0 /*mover*/,
|                             struct Entity *a6 /*context*/);
|
|  Notas forenses:
|    1. Retorno por CCR (los callers usan bcc/bcs/beq inmediatamente
|       tras el jsr). GCC nunca emite `andi.b #imm, ccr` (opcode 023C)
|       porque ese modo esta reservado para operaciones sobre el
|       Status Register en modo supervisor; aqui se usa la forma word
|       de andi para el CCR con el nibble bajo (bits N/Z/V/C/X).
|    2. La constante $EE = 1110 1110 en binario limpia bit 0 (C) y
|       bit 4 (X) del CCR - deja N, Z, V con su valor actual. Este
|       "cleanup" solo tiene sentido si el caller LEE explicitamente
|       C y X - patron clasico de tail-call que aprovecha el CCR como
|       canal de retorno.
|    3. neg.w d0 en vez de un moveq #-16 o un lookup [+16,-16]: el
|       primer moveq deja $10, luego el neg lo convierte a $FFF0.
|       GCC habria factorizado con un select o con dos moveq alternos.
|    4. btst.b #$0, $3a(a6) - GCC solo emite btst con bit inmediato
|       cuando el bit y el offset son constantes; forma coherente.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ProbeMoveX_09A7AA
        .type   Entity_ProbeMoveX_09A7AA, @function
        .section .text.Entity_ProbeMoveX_09A7AA, "ax", @progbits

Entity_ProbeMoveX_09A7AA:
        jsr     Sub_00009A7CC(pc)       | +00  probe global (returns C on collision)
        bcs.w   .Lcollision             | +04  C=1: no mover
        move.w  #0x10, d0               | +08  d0 = +16 (delta por defecto)
        btst.b  #0x0, 0x3a(a6)          | +0c  ¿ flags3a & 1 ?
        beq.w   .Lapply                 | +12  no: +16
        neg.w   d0                      | +16  si: d0 = -16 (mirror)
.Lapply:
        add.w   d0, 0x22(a0)            | +18  a0->pos_x += d0
        andi.b  #0xee, ccr              | +1c  clear C, X del CCR
.Lcollision:
        rts                             | +20
        .size   Entity_ProbeMoveX_09A7AA, .-Entity_ProbeMoveX_09A7AA
