| ============================================================================
|  Metal Slug 1 - asm/sprite_dispatch_dual_05a9d6.s
|  ----------------------------------------------------------------------------
|  Wave Z (Sprite dispatch backends + probe cluster $027xxx + helpers) - #1
|
|  Sprite_Dispatch_05A9D6  @ $05A9D6  (entry A, 12 B de prologo hasta $05A9E2)
|  Sprite_Dispatch_05A9E2  @ $05A9E2  (entry B, 180 B compartidos, total 192 B)
|
|  Dispatcher de sprite dual-entry. Dos puntos de entrada convergen en $5A9E6:
|
|    - Entry A ($05A9D6): `andi.b #3,d5; ori.b #4,d5; bra $5A9E6`
|      Fuerza bit 2 encendido -> "modo B" (usa el slot descendente $614a).
|    - Entry B ($05A9E2): `andi.b #3,d5`
|      Respeta el bit 2 recibido del caller (modo A o B segun d5).
|
|  Tras convergir, hace `jsr $5B1B2(pc)` (subrutina auxiliar), fija
|  a5 = $108080 (BASE arena global), y toma decisiones de slot:
|
|    - Si a0<0 (sprite invalido)          -> exit path $5AA92
|    - Si $4254(a5) >= $11D0              -> exit path $5AA92  (queue full)
|    - Si $6148(a5) >= $6148(a5)          -> exit path $5AA92  (colision heads)
|    - Si !bit2(d5): USA slot ADD  -> escribe en $5428(a5)+$6148, d6+=4
|    - Si  bit2(d5): USA slot SUB  -> escribe en $5428(a5)-$614a, d7-=4
|
|  Continua con `jsr $5AA96(pc, d7.w)` (jump-table PC-rel indexada por d7*4),
|  verifica coincidencia con head y hace commit o rollback (`subq.w #$4,$6148`
|  o `addq.w #$4,$614a`) segun bit 2 de d5. Finaliza con `move usp,a1; rts`.
|
|  Firma C conceptual:
|
|      /* Dispatch de sprite dual-entry. Entry A fuerza modo descendente,
|       * Entry B respeta el modo pasado por d5. Publica el long-word d1
|       * en el sprite queue (a5+$5428) por uno de dos slots (ADD/SUB) y
|       * salta a la jump-table PC-rel en $5AA96 para procesar la variante
|       * exacta del sprite segun d7 (nibble bajo de flags). */
|      /* void */ void Sprite_Dispatch(uint8 mode_d5, uint32 payload_d1, ...);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Dos puntos de entrada globales que caen en la misma funcion. GCC
|       nunca emite `bra` de 4 B a la propia funcion contigua para
|       homogeneizar el prologo. Idioma "dual entry" ya visto en Wave W#4/#5
|       (Sprite_HexFormat8_05D72C/_05D740).
|    2. `move a1, usp` / `move usp, a1` como save/restore de un registro
|       auxiliar via USP (User Stack Pointer). Es un truco de asm hand-coded
|       en modo supervisor: el 68000 en modo supervisor puede usar USP como
|       "registro extra" cuando la CPU no esta en modo usuario. GCC nunca
|       genera esto.
|    3. `jsr $5AA96(pc, d7.w)` con d7 = (nibble_flags | mode_bit) << 2 es
|       jump-table indexada PC-rel. GCC emite switch con jump-table separada
|       o cadena de if/else, no `jsr` a codigo indexado.
|    4. `56 C7` (`sne.b d7`) usa Set-if-not-equal para materializar $00/$FF
|       en un byte segun CCR previo. Es idioma clasico de bit-masking hand-coded.
|    5. `48 41` (`swap d1`) INTERCALADO entre el `jsr aux` y el uso posterior
|       de $4254(a5). Preserva la parte alta de d1 durante llamadas
|       intermedias sin usar la pila. GCC preserva por pila o registro.
|    6. Todos los offsets de a5 (BASE $108080) son grandes ($4254, $5428,
|       $6148, $614A, $11D0=limit) - patente de una arena estatica gigante
|       con layout fijo, no de un struct C convencional.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Sprite_Dispatch_05A9D6
        .globl  Sprite_Dispatch_05A9E2
        .type   Sprite_Dispatch_05A9D6, @function
        .type   Sprite_Dispatch_05A9E2, @function
        .section .text.Sprite_Dispatch_05A9D6, "ax", @progbits

Sprite_Dispatch_05A9D6:
        andi.b  #0x3, d5                       | +00  d5 &= 0x03
        ori.b   #0x4, d5                       | +04  d5 |= 0x04  (forzar modo B)
        bra.w   .Lmerge                        | +08  goto merge point
                                              |
Sprite_Dispatch_05A9E2:
        andi.b  #0x3, d5                       | +0c  d5 &= 0x03  (respetar bit 2 recibido)
.Lmerge:
        jsr     .Laux_5b1b2(pc)                | +10  aux hook (unknown)
        move    a1, usp                        | +14  save a1 via USP (trick)
        lea.l   0x108080.l, a5                 | +16  a5 = BASE arena global
                                              |
        move.l  a0, d7                         | +1c  d7 = sprite ptr as long
        bmi.w   .Lexit_bad                     | +1e  if (sprite < 0) exit
        swap    d1                             | +22  preserve high word of d1
        move.w  0x4254(a5), d1                 | +24  d1_lo = queue_head (word)
        cmpi.w  #0x11d0, d1                    | +28  if (queue_head >= 0x11D0)
        bcc.w   .Lexit_bad                     | +2c     exit (queue full)
        move.w  0x6148(a5), d6                 | +30  d6 = slot_add
        move.w  0x614a(a5), d7                 | +34  d7 = slot_sub
        cmp.w   d7, d6                         | +38  if (slot_add >= slot_sub)
        bcc.w   .Lexit_bad                     | +3a     exit (heads colliding)
                                              |
        btst.b  #0x2, d5                       | +3e  test mode bit
        bne.w   .Luse_sub                      | +42
                                              |
                                              | ---- ADD path: slot creciente ----
        lea.l   0x5428(a5), a4                 | +46  a4 = &queue[0]
        move.l  d1, (a4, d6.w)                 | +4a  queue[slot_add] = d1_lo
        addq.w  #0x4, d6                       | +4e  slot_add += 4
        move.w  d6, 0x6148(a5)                 | +50  publish slot_add
        bra.w   .Ldispatch                     | +54  goto dispatch
                                              |
.Luse_sub:
                                              | ---- SUB path: slot decreciente ----
        subq.w  #0x4, d7                       | +58  slot_sub -= 4
        lea.l   0x5428(a5), a4                 | +5a  a4 = &queue[0]
        move.l  d1, (a4, d7.w)                 | +5e  queue[slot_sub] = d1_lo
        move.w  d7, 0x614a(a5)                 | +62  publish slot_sub
                                              |
.Ldispatch:
        move.w  d5, -(a7)                      | +66  push d5 (mode + flags)
        lsl.w   #0x8, d2                       | +68  d2 <<= 8  (prep variant)
        lea.l   0x4258(a5), a3                 | +6a  a3 = &sprite_table[0]
        lea.l   0x11d0(a3), a2                 | +6e  a2 = &sprite_table[$11D0/4]  (limit)
        adda.w  d1, a3                         | +72  a3 += head_offset
        move.b  d3, d7                         | +74  d7 = sprite_id (byte)
        and.b   d4, d7                         | +76  d7 &= mask
        addq.b  #0x1, d7                       | +78  d7 += 1
        sne.b   d7                             | +7a  d7 = (d7 != 0) ? 0xFF : 0x00
        andi.w  #0x4, d7                       | +7c  d7 &= 0x04  (isolate bit)
        andi.b  #0x3, d5                       | +80  d5 &= 0x03  (re-mask, safety)
        or.b    d5, d7                         | +84  d7 |= d5    (fuse mode+variant)
        add.w   d7, d7                         | +86  d7 *= 2
        add.w   d7, d7                         | +88  d7 *= 4  (index * sizeof(long))
        .byte   0x4e, 0xbb, 0x70, 0x34          | +8a  4E BB 70 34 = jsr (pc, d7.w, $34)
                                              |     PC=$5AA64 + $34 = $5AA98 - 2, target = SpriteDispatchJT_05AA96
                                              |     GAS no puede codificar disp8 pc-rel a
                                              |     un simbolo externo `--defsym`; se emiten
                                              |     los 4 bytes directamente.
                                              |
        move.w  (a7)+, d5                      | +8e  pop d5
        lea.l   0x4258(a5), a2                 | +90  a2 = &sprite_table[0]
        suba.l  a2, a3                         | +94  a3 = head_offset (relative)
        move.w  0x4254(a5), d0                 | +96  d0 = queue_head
        cmp.w   a3, d0                         | +9a  if (queue_head == a3)
        beq.w   .Lcheck_side                   | +9c     goto check_side
        move.w  a3, 0x4254(a5)                 | +a0  publish new queue_head
        bra.w   .Lexit_bad                     | +a4  goto exit
                                              |
.Lcheck_side:
        btst.b  #0x2, d5                       | +a8  test mode bit
        bne.w   .Lrollback_sub                 | +ac
        subq.w  #0x4, 0x6148(a5)               | +b0  rollback slot_add -= 4
        bra.w   .Lexit_bad                     | +b4
.Lrollback_sub:
        addq.w  #0x4, 0x614a(a5)               | +b8  rollback slot_sub += 4
                                              |
.Lexit_bad:
        move    usp, a1                        | +bc  restore a1 from USP
        rts                                    | +be

        .equ    .Laux_5b1b2,   Sub_0005B1B2
        .equ    .Ljump_table,  SpriteDispatchJT_05AA96

        .size   Sprite_Dispatch_05A9D6, .-Sprite_Dispatch_05A9D6
