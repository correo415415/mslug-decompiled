| ============================================================================
|  Metal Slug 1 - asm/entity_alloc_sprite_slot_00236e.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #1
|
|  Entity_AllocSpriteSlot_00236E  @ $00236E  (1594 bytes, 8 callers)
|
|  Reserva un slot visual en el sub-rango [$1084748..$1084748+47*32] de
|  SPRITE_SLOT_TABLE ($1082C8) para el entity (a6). Si encuentra un slot
|  cuyo descriptor coincide con el del entity y cuyos flags $84 (HIT|DEAD)
|  estan a cero, lo asigna. Si tras 47 intentos no encuentra ninguno,
|  cae a un fallback round-robin sobre $1082C4 (indice ciclico mod 48).
|
|  Prologo del helper:
|    jsr Sub_00002_9A8_prologo   -- sub-prologo compartido con Entity_ProbeSpriteSlot
|                                   (calcula d1 = &SpriteDesc[idx en $14E00], d5 = idx,
|                                    d3 = $1e(a6), y verifica si el entity ya tiene slot).
|                                   Retorna via rts, salida con d2!=0 si YA hay slot.
|    tst.b d2       -- ¿ya reservado?  ->  bne $29A6 (rts)
|    cmpi.w #3, $1e(a6)  -- ¿el entity ha alcanzado el limite de 3 slots?  ->  bge $29A6
|    tst.w $1082C2.l     -- ¿sistema de sprites activo?                    ->  beq $29A6
|
|  Setup:
|    d1 &= 0xFFFF; d1 <<= 6; d1 += $14E00      -- d1 = &SpriteDesc[idx]
|    d4 = d1                                    -- guarda desc_ptr para epilogo
|    a1 = $1082C8 + 0x200 + 0xC80 + 0x400       -- a1 = $1084748 (base del sub-rango)
|    d2 = 0                                     -- contador de slot
|
|  Bucle desenrollado 47 veces:
|    Cada iteracion:
|      cmp.l   $2(a1), d1              -- slot.desc_ptr == our_desc ?
|      bne.w   .next                   -- no
|      move.b  (a1), d5                -- d5 = slot.flags_b0
|      andi.b  #$84, d5                -- ¿HIT o DEAD ?
|      bne.w   .next                   -- si: sigue buscando
|      jmp     .Lepilogue(pc)          -- encontrado, ir a epilogo con d2 = slot index
|    .next:
|      addq.w  #1, d2                  -- avanzar contador
|      adda.l  #$20, a1                -- avanzar puntero (32 B por slot)
|
|  Fallback (tras 47 intentos sin encontrar):
|    subq.w #1, $1082C2                -- decrementa "slots activos" (compensa fallo)
|    d3 = $1082C4                      -- lee round-robin index
|    $1082C4++;  if $1082C4 >= 48 clr  -- avanza ciclicamente
|    a2 = $1081C2                      -- tabla de mapeo round-robin -> slot idx
|    d2 = a2[d3]                       -- selecciona slot forzosamente
|    -- fall-through al epilogo
|
|  Epilogo comun (con d2 = indice de slot elegido):
|    d1 = d4                           -- recupera desc_ptr
|    d2 += 0x94                        -- desplazamiento a sub-tabla hija (buffer secundario)
|    d4 = d2
|    d2 <<= 5                          -- d2 = byte offset en SPRITE_SLOT_TABLE
|    a1 = $1082C8
|    slot[d4].desc_ptr = d1            -- 2(a1, d2.w)
|    slot[d4].refcount++               -- 6(a1, d2.w)
|    if refcount == 1: slot[d4].active = 1     -- (a1, d2.w)
|    $14(a6) = d4                      -- entity->last_slot = slot index
|    d1 = $1e(a6);  d1 *= 2
|    $16(a6, d1.w) = d4                -- entity->slot_history[timer0] = slot index
|    $1e(a6)++                         -- timer0++
|    rts
|
|  Firma C conceptual:
|      /* Reserva un slot visual del sub-rango [$1084748..) del sprite
|       * slot table para el entity 'a6'. Devuelve implicitamente el
|       * indice en $14(a6) y actualiza el slot_history en $16..$1c(a6).
|       * Si no hay slot libre, se sobre-escribe uno via round-robin. */
|      void Entity_AllocSpriteSlot(struct Entity *a6);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. jsr $29a8(pc) que apunta al inicio de Entity_ProbeSpriteSlot_29A8
|       (una sub-rutina INDEPENDIENTE de 74 B). El compilador nunca
|       comparte prologo entre dos funciones que retornan resultados
|       distintos por CCR/d2; llamaria a una unica funcion o inline.
|    2. Tres `adda.l #imm,a1` consecutivos con distinto imm (0x200, 0xC80,
|       0x400) en lugar de uno consolidado con imm=0x1880. GCC habria
|       usado un unico lea.l $1084748.l,a1 (6 B) en lugar de 3x6 = 18 B.
|    3. 47 iteraciones desenrolladas 100% identicas byte-a-byte. Ningun
|       compilador desenrolla mas alla de 8 sin `#pragma unroll` explicito.
|       Es codigo escrito a mano por un desarrollador que decidio que
|       47*30 B de ROM (~1.4 KB) valian los ~48 ciclos ahorrados por iter.
|    4. El fallback cae por fall-through al epilogo comun (SIN branch),
|       compartiendo el mismo cierre con la ruta "found". GCC habria
|       inlineado el epilogo o factorizado.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_AllocSpriteSlot_00236E
        .type   Entity_AllocSpriteSlot_00236E, @function
        .section .text.Entity_AllocSpriteSlot_00236E, "ax", @progbits

Entity_AllocSpriteSlot_00236E:
        jsr     Entity_ProbeSpriteSlot_29A8(pc) | +000  sub-prologo comun (prueba y prepara d1)
        tst.b   d2                              | +004  ¿ya reservado?
        bne.w   Rts_shared_29A6                 | +006  si: retorna
        cmpi.w  #3, 0x1e(a6)                    | +00a  ¿limite de 3 slots alcanzado?
        bge.w   Rts_shared_29A6                 | +010  si: retorna
        tst.w   0x1082c2.l                      | +014  ¿sistema activo?
        beq.w   Rts_shared_29A6                 | +01a  no: retorna
        andi.l  #0xffff, d1                     | +01e  d1 &= 0xFFFF
        lsl.l   #6, d1                          | +024  d1 <<= 6  (idx * 64)
        lea     0x14e00.l, a1                   | +026  a1 = SpriteDesc bank B
        add.l   a1, d1                          | +02c  d1 = &SpriteDesc[idx]
        move.l  d1, d4                          | +02e  d4 = desc_ptr (guardar)
        lea     0x1082c8.l, a1                  | +030  a1 = SPRITE_SLOT_TABLE
        adda.l  #0x200, a1                      | +036  +512
        adda.l  #0xc80, a1                      | +03c  +3200
        adda.l  #0x400, a1                      | +042  +1024   (total +$1880)
        clr.w   d2                              | +048  d2 = 0  (contador de slot)

| ==== Bucle desenrollado 47 iteraciones (30 B cada slot completo) ====

.Lslot_1_body:
        cmp.l   0x2(a1), d1                     | +04a  slot.desc_ptr == our_desc ?
        bne.w   .Lslot_2_bridge                 | +04e  no: siguiente
        move.b  (a1), d5                        | +052  d5 = slot.flags_b0
        andi.b  #0x84, d5                       | +054  ¿HIT | DEAD ?
        bne.w   .Lslot_2_bridge                 | +058  si: siguiente
        jmp     .Lepilogue(pc)                  | +05c  encontrado

.Lslot_2_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_2_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_3_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_3_bridge
        jmp     .Lepilogue(pc)

.Lslot_3_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_3_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_4_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_4_bridge
        jmp     .Lepilogue(pc)

.Lslot_4_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_4_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_5_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_5_bridge
        jmp     .Lepilogue(pc)

.Lslot_5_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_5_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_6_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_6_bridge
        jmp     .Lepilogue(pc)

.Lslot_6_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_6_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_7_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_7_bridge
        jmp     .Lepilogue(pc)

.Lslot_7_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_7_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_8_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_8_bridge
        jmp     .Lepilogue(pc)

.Lslot_8_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_8_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_9_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_9_bridge
        jmp     .Lepilogue(pc)

.Lslot_9_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_9_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_10_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_10_bridge
        jmp     .Lepilogue(pc)

.Lslot_10_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_10_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_11_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_11_bridge
        jmp     .Lepilogue(pc)

.Lslot_11_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_11_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_12_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_12_bridge
        jmp     .Lepilogue(pc)

.Lslot_12_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_12_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_13_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_13_bridge
        jmp     .Lepilogue(pc)

.Lslot_13_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_13_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_14_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_14_bridge
        jmp     .Lepilogue(pc)

.Lslot_14_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_14_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_15_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_15_bridge
        jmp     .Lepilogue(pc)

.Lslot_15_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_15_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_16_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_16_bridge
        jmp     .Lepilogue(pc)

.Lslot_16_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_16_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_17_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_17_bridge
        jmp     .Lepilogue(pc)

.Lslot_17_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_17_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_18_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_18_bridge
        jmp     .Lepilogue(pc)

.Lslot_18_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_18_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_19_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_19_bridge
        jmp     .Lepilogue(pc)

.Lslot_19_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_19_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_20_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_20_bridge
        jmp     .Lepilogue(pc)

.Lslot_20_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_20_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_21_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_21_bridge
        jmp     .Lepilogue(pc)

.Lslot_21_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_21_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_22_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_22_bridge
        jmp     .Lepilogue(pc)

.Lslot_22_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_22_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_23_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_23_bridge
        jmp     .Lepilogue(pc)

.Lslot_23_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_23_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_24_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_24_bridge
        jmp     .Lepilogue(pc)

.Lslot_24_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_24_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_25_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_25_bridge
        jmp     .Lepilogue(pc)

.Lslot_25_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_25_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_26_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_26_bridge
        jmp     .Lepilogue(pc)

.Lslot_26_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_26_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_27_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_27_bridge
        jmp     .Lepilogue(pc)

.Lslot_27_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_27_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_28_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_28_bridge
        jmp     .Lepilogue(pc)

.Lslot_28_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_28_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_29_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_29_bridge
        jmp     .Lepilogue(pc)

.Lslot_29_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_29_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_30_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_30_bridge
        jmp     .Lepilogue(pc)

.Lslot_30_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_30_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_31_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_31_bridge
        jmp     .Lepilogue(pc)

.Lslot_31_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_31_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_32_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_32_bridge
        jmp     .Lepilogue(pc)

.Lslot_32_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_32_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_33_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_33_bridge
        jmp     .Lepilogue(pc)

.Lslot_33_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_33_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_34_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_34_bridge
        jmp     .Lepilogue(pc)

.Lslot_34_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_34_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_35_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_35_bridge
        jmp     .Lepilogue(pc)

.Lslot_35_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_35_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_36_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_36_bridge
        jmp     .Lepilogue(pc)

.Lslot_36_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_36_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_37_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_37_bridge
        jmp     .Lepilogue(pc)

.Lslot_37_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_37_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_38_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_38_bridge
        jmp     .Lepilogue(pc)

.Lslot_38_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_38_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_39_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_39_bridge
        jmp     .Lepilogue(pc)

.Lslot_39_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_39_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_40_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_40_bridge
        jmp     .Lepilogue(pc)

.Lslot_40_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_40_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_41_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_41_bridge
        jmp     .Lepilogue(pc)

.Lslot_41_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_41_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_42_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_42_bridge
        jmp     .Lepilogue(pc)

.Lslot_42_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_42_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_43_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_43_bridge
        jmp     .Lepilogue(pc)

.Lslot_43_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_43_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_44_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_44_bridge
        jmp     .Lepilogue(pc)

.Lslot_44_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_44_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_45_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_45_bridge
        jmp     .Lepilogue(pc)

.Lslot_45_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_45_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_46_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_46_bridge
        jmp     .Lepilogue(pc)

.Lslot_46_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_46_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_47_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_47_bridge
        jmp     .Lepilogue(pc)

.Lslot_47_bridge:
        addq.w  #1, d2
        adda.l  #0x20, a1
.Lslot_47_body:
        cmp.l   0x2(a1), d1
        bne.w   .Lslot_48_bridge
        move.b  (a1), d5
        andi.b  #0x84, d5
        bne.w   .Lslot_48_bridge
        jmp     .Lepilogue(pc)

| ==== Fallback: 47 slots probados sin exito (round-robin degradado) ====

.Lslot_48_bridge:
        addq.w  #1, d2                          | +0bc4  cierra la iter 47
        adda.l  #0x20, a1                       | +0bc6
        subq.w  #1, 0x1082c2.l                  | +0bcc  decrementa "slots activos"
        move.w  0x1082c4.l, d3                  | +0bd2  d3 = round-robin index
        addq.w  #1, 0x1082c4.l                  | +0bd8  round-robin++
        cmpi.w  #0x30, 0x1082c4.l               | +0bde  ¿ alcanzo 48 ?
        blt.w   .Lrr_no_wrap                    | +0be6  no: mantener
        clr.w   0x1082c4.l                      | +0bea  si: wrap a 0
.Lrr_no_wrap:
        lea     0x1081c2.l, a2                  | +0bf0  a2 = tabla round-robin -> slot idx
        moveq   #0, d2                          | +0bf6  d2 = 0 (limpiar high)
        move.b  (a2, d3.w), d2                  | +0bf8  d2 = a2[d3]
        | -- fall-through al epilogo --

| ==== Epilogo comun (con d2 = indice de slot elegido) ====

.Lepilogue:
        move.l  d4, d1                          | +0bfc  d1 = desc_ptr (recuperado)
        addi.w  #0x94, d2                       | +0bfe  d2 += 0x94  (offset a sub-tabla hija)
        move.w  d2, d4                          | +0c02  d4 = final slot index
        lsl.w   #5, d2                          | +0c04  d2 = byte offset (idx * 32)
        lea     0x1082c8.l, a1                  | +0c06  a1 = SPRITE_SLOT_TABLE
        move.l  d1, 0x2(a1, d2.w)               | +0c0c  slot[d4].desc_ptr = d1
        addq.w  #1, 0x6(a1, d2.w)               | +0c10  slot[d4].refcount++
        cmpi.w  #1, 0x6(a1, d2.w)               | +0c14  ¿ era 0 ? (ahora 1)
        bne.w   .Lskip_activate                 | +0c1a  no: ya activo
        move.b  #1, (a1, d2.w)                  | +0c1e  si: marcar active
.Lskip_activate:
        move.w  d4, d2                          | +0c24  d2 = slot idx
        move.w  d2, 0x14(a6)                    | +0c26  entity->last_slot = d2
        move.w  0x1e(a6), d1                    | +0c2a  d1 = timer0
        add.w   d1, d1                          | +0c2e  d1 *= 2  (word index)
        move.w  d2, 0x16(a6, d1.w)              | +0c30  entity->slot_history[timer0] = slot idx
        addq.w  #1, 0x1e(a6)                    | +0c34  timer0++
Rts_shared_29A6:
        rts                                     | +0c38
        .size   Entity_AllocSpriteSlot_00236E, .-Entity_AllocSpriteSlot_00236E

