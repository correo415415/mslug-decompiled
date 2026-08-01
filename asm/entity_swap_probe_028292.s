| ============================================================================
|  Metal Slug 1 - asm/entity_swap_probe_028292.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #12
|
|  Entity_SwapProbeCommit_028292  @ $028292  (70 bytes, 2 callers)
|
|  Trampolin "swap-probe-commit-or-rollback" que:
|    1. Si $60(a6) != $FFFFFFFF, lo pone a $FFFFFFFF (marca "in transit")
|    2. Guarda $34(a6) en el stack
|    3. Copia $62(a6) -> $34(a6)  (swap del campo activo)
|    4. Llama a Sub_027A92 (probe)
|    5. Segun el retorno por CCR:
|         C=0: commit  -- $62(a6) <- viejo $34, $34(a6) <- valor previo,
|                        andi.b #$EE, ccr (clear C, X)
|         C=1: rollback -- $62(a6) <- viejo $34, $34(a6) <- valor previo,
|                        ori.b  #$11, ccr (set C, X para propagar fallo)
|    6. rts
|
|  Firma C conceptual:
|
|      /* Ejecuta un swap temporal de $34(a6) <-> $62(a6), llama a un
|       * probe, y segun el resultado hace commit (ambos campos se
|       * intercambian) o rollback (ambos se restauran). El resultado
|       * se comunica por CCR al caller (bcc/bcs tras el jsr). */
|      int Entity_SwapProbeCommit(struct Entity *a6);
|
|  Notas forenses (multiples idiomas irreproducibles por GCC 1:1):
|    1. cmpi.w #$FFFF, $60(a6) seguido de move.l #$FFFFFFFF a la misma
|       direccion: idioma "test-and-set to sentinel" que aprovecha el
|       hecho de que .w (compare word bajo) y .l (write long) sobre la
|       misma direccion cubren distintos campos parciales.
|    2. Uso de -(a7)/(a7)+ para preservar $34(a6) durante el probe -
|       stack manual sin link/unlk.
|    3. Retorno por CCR con DOS caminos distintos:
|         andi.b #$EE, ccr  -- clear C(0) y X(4), preserva N/Z/V
|         ori.b  #$11, ccr  -- set C(0) y X(4), propagando fallo
|       GCC nunca emite andi/ori directo al CCR porque no forma parte
|       de su ABI m68k.
|    4. Los mismos 8 B de rollback ($3d6e0034 0062 / $3d5f 0034) se
|       repiten en las dos ramas: idioma "commit-or-rollback both use
|       the same swap operation" que GCC habria factorizado.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_SwapProbeCommit_028292
        .type   Entity_SwapProbeCommit_028292, @function
        .section .text.Entity_SwapProbeCommit_028292, "ax", @progbits

Entity_SwapProbeCommit_028292:
        cmpi.w  #0xffff, 0x60(a6)       | +00  ¿ $60(a6).w == $FFFF ? (marcador ya activo?)
        beq.w   .Lskip_mark             | +06  si: no re-marcar
        move.l  #0xffffffff, 0x60(a6)   | +0a  no: marcar $60(a6) = $FFFFFFFF
.Lskip_mark:
        move.w  0x34(a6), -(a7)         | +12  push $34(a6) (guardar)
        move.w  0x62(a6), 0x34(a6)      | +16  $34(a6) = $62(a6)  (swap)
        jsr     ThunkTarget_027a92(pc)  | +1c  probe (retorna Carry)
        bcc.w   .Lcommit                | +20  C=0: commit
        | -- rollback (C=1) --
        move.w  0x34(a6), 0x62(a6)      | +24  $62(a6) = $34(a6) actual
        move.w  (a7)+, 0x34(a6)         | +2a  $34(a6) = valor previo (pop)
        ori.b   #0x11, ccr              | +2e  set C, X (propaga fallo)
        bra.w   .Ldone                  | +32
.Lcommit:
        move.w  0x34(a6), 0x62(a6)      | +36  $62(a6) = $34(a6) actual
        move.w  (a7)+, 0x34(a6)         | +3c  $34(a6) = valor previo (pop)
        andi.b  #0xee, ccr              | +40  clear C, X (senal exito)
.Ldone:
        rts                             | +44
        .size   Entity_SwapProbeCommit_028292, .-Entity_SwapProbeCommit_028292
