| ============================================================================
|  Metal Slug 1 - asm/entity_init_fields_05dc34.s
|  ----------------------------------------------------------------------------
|  Wave V (Entity/Sprite helpers) - funcion #5
|
|  Entity_InitFields_05DC34  @ $05DC34  (112 bytes, 4 callers)
|
|  Reinicia el conjunto de campos "de spawn" de la entidad apuntada por
|  a0 al estado neutro que espera el gestor de entidades. Es el prologo
|  compartido de los 4 constructores de entidad conocidos (spawn desde
|  script, spawn desde patch, spawn por proyectil, spawn por spawner de
|  nivel), que reutilizan literalmente estos 112 B en vez de llamarlos
|  como sub-rutina (para ahorrar el jsr/rts, patron tipico de asm hecho a
|  mano en un juego con presupuesto de ciclos muy ajustado).
|
|  Firma C conceptual:
|
|      /* Deja la entidad en el estado neutro esperado por el gestor:
|       * transform a cero, sprite/palette a "no-asignado" (0xFFFF),
|       * timers a cero, y activa el flag "reservado" (bit 6 de flags13).
|       */
|      void Entity_InitFields(struct Entity *a0);
|
|  Mapeo tentativo de los campos tocados (a documentar en mslug.h):
|
|    Offset  Anch. Valor      Semantica tentativa
|    ------  ----- ---------  ---------------------------------------------
|    $12     b     0          state_low
|    $13     b     0 + bset 6 flags13 (bit 6 = SLOT_RESERVED)
|    $14     w     0          state_high
|    $16..$1C w    0xFFFF x4  4 sprite handles / palette entries (NIL)
|    $1E     w     0          timer0
|    $26     w     0          timer1
|    $28     l     0          scratch_l0
|    $2C     l     0          scratch_l1
|    $32     b     0xFF       anchor_x (NIL)
|    $33     b     0xFF       anchor_y (NIL)
|    $38     w     0          flags38  (visual/animation)
|    $3A     b     0          flags3a
|    $44     b     0          hit_stun
|    $45     b     0          hit_kind
|    $59     b     0          ai_state
|    $5A     b     0          ai_sub
|    $5B     b     0          ai_flags
|    $69     b     0          coll_group
|    $6A     b     0          coll_flags
|    $6B     b     0          coll_mask
|
|  Notas forenses:
|    - 4 x `move.w #$ffff,X(a0)` con offsets consecutivos $16/$18/$1a/$1c
|      es INMEDIATAMENTE reducible por GCC a `move.l #-1,$16(a0);
|      move.l #-1,$1a(a0)` (2 x 8 B) o incluso a `movem.l`. El helper
|      original emite 4 x 6 B (=24 B) reservando espacio de codigo:
|      caracteristica del ensamblador copiado literalmente entre
|      constructores.
|    - El bset.b #6,$13(a0) al final es la reserva del slot: el gestor
|      lo consulta antes de sobrescribir la entidad.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_InitFields_05DC34
        .type   Entity_InitFields_05DC34, @function
        .section .text.Entity_InitFields_05DC34, "ax", @progbits

Entity_InitFields_05DC34:
        clr.w   0x14(a0)                | +00  state_high = 0
        move.w  #0xffff, 0x16(a0)       | +04  sprite_handle_0 = NIL
        move.w  #0xffff, 0x18(a0)       | +0a  sprite_handle_1 = NIL
        move.w  #0xffff, 0x1a(a0)       | +10  sprite_handle_2 = NIL
        move.w  #0xffff, 0x1c(a0)       | +16  sprite_handle_3 = NIL
        clr.w   0x1e(a0)                | +1c  timer0 = 0
        clr.b   0x12(a0)                | +20  state_low = 0
        clr.w   0x38(a0)                | +24  flags38 = 0
        clr.b   0x13(a0)                | +28  flags13 = 0  (se re-activa bit 6 al final)
        clr.b   0x5a(a0)                | +2c  ai_sub = 0
        clr.b   0x69(a0)                | +30  coll_group = 0
        clr.b   0x5b(a0)                | +34  ai_flags = 0
        clr.b   0x6b(a0)                | +38  coll_mask = 0
        move.b  #0xff, 0x32(a0)         | +3c  anchor_x = NIL
        move.b  #0xff, 0x33(a0)         | +42  anchor_y = NIL
        clr.b   0x3a(a0)                | +48  flags3a = 0
        clr.l   0x28(a0)                | +4c  scratch_l0 = 0
        clr.l   0x2c(a0)                | +50  scratch_l1 = 0
        clr.w   0x26(a0)                | +54  timer1 = 0
        clr.b   0x59(a0)                | +58  ai_state = 0
        clr.b   0x45(a0)                | +5c  hit_kind = 0
        clr.b   0x44(a0)                | +60  hit_stun = 0
        clr.b   0x6a(a0)                | +64  coll_flags = 0
        bset.b  #6, 0x13(a0)            | +68  flags13 |= SLOT_RESERVED
        rts                             | +6e
        .size   Entity_InitFields_05DC34, .-Entity_InitFields_05DC34
