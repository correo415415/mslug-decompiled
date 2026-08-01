| ============================================================================
|  Metal Slug 1 - asm/list_apply_sentinel_0477fc.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #9 (clon de Z-batch1 #7 con stride distinto)
|
|  List_ApplyWithSentinelFF_0477FC  @ $0477FC  (38 bytes)
|
|  Segundo clon del helper list-apply. Difiere de Z-batch1 #7 en dos
|  parametros:
|    - callback PC-rel:  $0477D4
|    - stride del cursor: $20 (== mitad del stride $40 de los otros dos)
|
|  Firma C conceptual:
|
|      /* Recorre lista de bytes desde a2 hasta centinela $FF invocando
|       * Sub_000477D4 con cada elemento. Avanza a4 en 0x20 bytes por
|       * iteracion (half-stride respecto a los clones de stride $40). */
|      void List_ApplyWithSentinelFF_0477FC(uint16_t *list /*a2*/,
|                                           void *table /*a1*/,
|                                           uint16 d1_saved /*d1*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  List_ApplyWithSentinelFF_0477FC
        .type   List_ApplyWithSentinelFF_0477FC, @function
        .section .text.List_ApplyWithSentinelFF_0477FC, "ax", @progbits

List_ApplyWithSentinelFF_0477FC:
        move.w  d1, d4                         | +00  d4 = d1_orig (save)
        movea.l a1, a4                         | +02  a4 = a1_orig (save)
        movea.l a2, a5                         | +04  a5 = list cursor
.Lloop:
        move.b  (a5)+, d0                      | +06  d0 = *cursor++
        cmpi.b  #0xff, d0                      | +08  if (d0 == $FF)
        beq.w   .Lend                          | +0c     goto end
        move.w  d4, d1                         | +10  restore d1 for callback
        movea.l a4, a1                         | +12  restore a1 for callback
        jsr     .Lcallback(pc)                 | +14  Sub_000477D4
        move.l  a4, d0                         | +18  d0 = a4 as long
        addi.l  #0x20, d0                      | +1a  d0 += 0x20 (half-stride)
        movea.l d0, a4                         | +20  a4 = d0 (advance)
        bra.b   .Lloop                         | +22  loop back
.Lend:
        rts                                    | +24

        .equ    .Lcallback, Sub_000477D4

        .size   List_ApplyWithSentinelFF_0477FC, .-List_ApplyWithSentinelFF_0477FC
