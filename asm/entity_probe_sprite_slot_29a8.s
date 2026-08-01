| ============================================================================
|  Metal Slug 1 - asm/entity_probe_sprite_slot_29a8.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #2
|
|  Entity_ProbeSpriteSlot_29A8  @ $0029A8  (74 bytes)
|
|  Comprueba si el entity (a6) ya tiene un slot visual asignado en su
|  slot_history ($16..$1c) cuyo descriptor coincida con el que se le
|  pasa en d1. Es el "prologo compartido" que Entity_AllocSpriteSlot
|  (W#1) llama con `jsr $29a8(pc)` para saber si debe abortar la
|  reserva (el entity ya tiene un slot valido).
|
|  Entrada:
|      d1 : idx del descriptor de sprite (word, se zero-extend a long)
|      a6 : entity
|
|  Salida:
|      d1 : preservado (recuperado desde d5 al final)
|      d2 : $00 si NO tiene slot valido, $FF si SI lo tiene
|      $14(a6) : actualizado al slot_history[d3] encontrado, si d2=$FF
|
|  Control-flow real (bucle backward por slot_history):
|
|      .Lloop_head ($29C0):
|          tst.w d3
|          beq.w  epilogue          -- d3 == 0: salir con d2 = valor previo
|          subq.w #1, d3            -- d3--
|          d4 = d3 * 2              -- word index
|          d0 = slot_history[d4]
|          d0 <<= 5                 -- byte offset
|          cmp.l  slot[d0].desc_ptr, d1
|          bne.b  .Lloop_head       -- no match: probar slot anterior
|          if (entity.flag_bit2)    -- entity->flags0 & 0x04 ?
|              bne.b .Lloop_head    -- si: skip este slot
|          $14(a6) = slot_history[d4]
|          d2 = 0xFF                -- marcar "encontrado"
|          -- fall-through al epilogue --
|      epilogue ($29EE):
|          d1 = d5
|          rts
|
|  Firma C conceptual:
|
|      /* Devuelve $FF en d2 si el entity ya tiene un slot valido para
|       * este descriptor en su slot_history, actualizando $14(a6) con el
|       * indice de slot encontrado. Devuelve $00 en d2 si no. */
|      uint8_t Entity_ProbeSpriteSlot(uint16_t idx /*d1*/,
|                                     struct Entity *a6);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Guarda d1 en d5 como scratch de salida para poder machacar d1
|       con el puntero calculado. GCC habria hecho el calculo en un
|       registro distinto (d0) preservando d1.
|    2. Bucle backward con `bne.b .Lloop_head` que RE-ENTRA por la
|       cabecera `tst.w d3` (que evalua el decremento reciente). El
|       decremento se hace UNA SOLA VEZ por iteracion, DESPUES del
|       tst.w d3, no antes. GCC habria emitido dbra.w d3 o un loop con
|       decremento antes del test.
|    3. Fall-through directo entre el brazo "encontrado" (move.b #0xff,d2)
|       y el epilogo (move.l d5,d1) sin bra.b intermedio. Los dos brazos
|       (beq.w salida temprana y bne.b reentrada) convergen en el mismo
|       epilogo con d2 = valor calculado dentro del bucle.
|    4. Comparte el prologo (d1 = &SpriteDesc[idx en $14E00]) con
|       Entity_AllocSpriteSlot (W#1) y Sprite_SetupSlotFromTableB (V#4).
|       Los tres helpers arrancan con las mismas 6 instrucciones que
|       calculan la direccion absoluta del descriptor: evidencia dura de
|       que $14E00 es la BASE OFICIAL del banco B de descriptores de
|       sprites en MSLUG1.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_ProbeSpriteSlot_29A8
        .type   Entity_ProbeSpriteSlot_29A8, @function
        .section .text.Entity_ProbeSpriteSlot_29A8, "ax", @progbits

Entity_ProbeSpriteSlot_29A8:
        move.l  d1, d5                  | +00  d5 = d1 (guardar idx original)
        clr.b   d2                      | +02  d2 = 0 (retorno = "no encontrado")
        andi.l  #0xffff, d1             | +04  d1 &= 0xFFFF (zero-extend idx)
        lsl.l   #6, d1                  | +0a  d1 *= 64
        lea     0x14e00.l, a1           | +0c  a1 = SpriteDesc bank B
        add.l   a1, d1                  | +12  d1 = &SpriteDesc[idx]
        move.w  0x1e(a6), d3            | +14  d3 = entity->timer0 (# slots)
.Lloop_head:
        tst.w   d3                      | +18  ¿ quedan slots por probar ?
        beq.w   .Lepilogue              | +1a  no: salir
        subq.w  #1, d3                  | +1e  d3--
        move.w  d3, d4                  | +20  d4 = d3
        add.w   d4, d4                  | +22  d4 *= 2 (word index)
        move.w  0x16(a6, d4.w), d0      | +24  d0 = slot_history[d3]
        lsl.w   #5, d0                  | +28  d0 *= 32 (byte offset)
        lea     0x1082c8.l, a1          | +2a  a1 = SPRITE_SLOT_TABLE
        cmp.l   0x2(a1, d0.w), d1       | +30  slot.desc_ptr == our_desc ?
        bne.b   .Lloop_head             | +34  no: probar slot anterior
        btst.b  #2, (a6)                | +36  ¿ entity.flags0 & 0x04 ?
        bne.b   .Lloop_head             | +3a  si: skip (transitioning)
        move.w  0x16(a6, d4.w), 0x14(a6) | +3c  $14(a6) = slot_history[d3]
        move.b  #0xff, d2               | +42  d2 = 0xFF (retorno = "encontrado")
        | -- fall-through al epilogo, sin bra.b intermedio --
.Lepilogue:
        move.l  d5, d1                  | +46  d1 = d5 (restaurar)
        rts                             | +48
        .size   Entity_ProbeSpriteSlot_29A8, .-Entity_ProbeSpriteSlot_29A8
