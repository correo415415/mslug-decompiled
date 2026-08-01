| ============================================================================
|  Metal Slug 1 - asm/list_apply_sentinel_04784c.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #8 (clon de Z-batch1 #7 con callback y stride distintos)
|
|  List_ApplyWithSentinelFF_04784C  @ $04784C  (38 bytes)
|
|  Clon byte-a-byte de List_ApplyWithSentinelFF_047888 (Wave Z-batch1 #7)
|  salvo el callback PC-rel:
|    - callback PC-rel:  $047822 (aqui) vs $047872 (en Z-batch1 #7)
|    - stride del cursor: $40 (mismo que Z-batch1 #7)
|
|  Firma C conceptual:
|
|      /* Igual que List_ApplyWithSentinelFF_047888 pero con callback
|       * distinto (Sub_00047822). Mismo stride $40 (task-node aligned). */
|      void List_ApplyWithSentinelFF_04784C(uint16_t *list /*a5*/,
|                                           void *table /*a4=a1_orig*/,
|                                           uint16 d1_saved /*d1*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  List_ApplyWithSentinelFF_04784C
        .type   List_ApplyWithSentinelFF_04784C, @function
        .section .text.List_ApplyWithSentinelFF_04784C, "ax", @progbits

List_ApplyWithSentinelFF_04784C:
        move.w  d1, d4                         | +00  d4 = d1_orig (save)
        movea.l a1, a4                         | +02  a4 = a1_orig (save)
        movea.l a2, a5                         | +04  a5 = list cursor
.Lloop:
        move.b  (a5)+, d0                      | +06  d0 = *cursor++
        cmpi.b  #0xff, d0                      | +08  if (d0 == $FF)
        beq.w   .Lend                          | +0c     goto end
        move.w  d4, d1                         | +10  restore d1 for callback
        movea.l a4, a1                         | +12  restore a1 for callback
        jsr     .Lcallback(pc)                 | +14  Sub_00047822
        move.l  a4, d0                         | +18  d0 = a4 as long
        addi.l  #0x40, d0                      | +1a  d0 += 0x40 (task-node stride)
        movea.l d0, a4                         | +20  a4 = d0 (advance)
        bra.b   .Lloop                         | +22  loop back
.Lend:
        rts                                    | +24

        .equ    .Lcallback, Sub_00047822

        .size   List_ApplyWithSentinelFF_04784C, .-List_ApplyWithSentinelFF_04784C
