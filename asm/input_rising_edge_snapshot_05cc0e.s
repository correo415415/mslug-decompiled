| ============================================================================
|  Metal Slug 1 - asm/input_rising_edge_snapshot_05cc0e.s
|  ----------------------------------------------------------------------------
|  Wave DD - #2
|
|  Input_RisingEdgeSnapshot_05CC0E  @ $05CC0E  (186 B, 1 caller)
|
|  Pieza del pipeline de input del juego. Estructura:
|
|    1. `clr.w $10E20C` + `jmp $24FB8` (early exit via tail-jump).
|       NOTA: esta es una rama alternativa que NO se toma en el flujo normal;
|       la funcion propiamente empieza a partir de $05CC1A (por eso el rts
|       final esta 174 B despues).
|
|    2. `jsr $25066` (helper matcheado en Wave A/B).
|
|    3. Snapshot P1: copia 6 B de $106EB0 -> $10E200 (move.l+move.w).
|    4. Snapshot P2: copia 6 B de $106EB6 -> $10E206 (move.l+move.w).
|
|    5. Rising-edge detection sobre $10FDAC (input latch):
|         d1 = old = *$10E20C
|         d0 = new = *$10FDAC
|         *$10E20C = d0                  (guardar nuevo)
|         d1 ^= d0                       (bits que cambiaron)
|         d1 &= d0                       (bits que subieron: rising edge)
|         *$10E20D = d1                  (publicar edge P1)
|
|    6. Rama por $10FDB6 (mascara START P1):
|         if (mask == 0 || mask == 3):
|             copy $10E200[6B] -> $10E212[6B]  (activar snapshot)
|         if (mask == 3):
|             clear $10E212[6B]                (limpiar tras activar)
|
|    7. Idem para P2 con $10FDB7, $10E206 -> $10E218.
|
|    8. rts. Segunda funcion pequeña $05CCC8 (6B) es un `move.b $6d(a6),
|       $6d(a0); rts` que forma parte del cluster pero es un helper aparte.
|
|  Idiomas hand-coded:
|    - Rising-edge en 3 instrucciones (`eor.b d0,d1; and.b d0,d1; move.b d1,...`)
|      con la publicacion del nuevo valor entre medias. GCC emitiria
|      dos snapshots temporales.
|    - Sequential `move.l (a0)+, (a1)+` + `move.w (a0), (a1)` para copia
|      de 6 B (no 8) sin loop.
|    - `movea.l (a0)+, (a1)+` post-increment en fuente y destino simultaneo.
|    - `bra.w` en el fallthrough del `clr.w + jmp` inicial no es un thunk
|      real: es una rama alternativa que otro caller usa como entry point
|      lateral ($05CC0E strict) pero el "main entry" empieza en $05CC1A.
|
|  Firma C conceptual:
|
|      /* Actualiza snapshots de input P1/P2 en $10E200/$10E206, detecta
|       * rising edges del latch $10FDAC en $10E20D, y activa/limpia
|       * los snapshots secundarios $10E212/$10E218 segun mascaras
|       * $10FDB6/$10FDB7 (0=activar, 3=activar+limpiar). */
|      void Input_RisingEdgeSnapshot(void);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Input_RisingEdgeSnapshot_05CC0E
        .type   Input_RisingEdgeSnapshot_05CC0E, @function
        .section .text.Input_RisingEdgeSnapshot_05CC0E, "ax", @progbits

Input_RisingEdgeSnapshot_05CC0E:
        clr.w   0x10e20c.l                     | +00  early alt: reset latch
        jmp     0x24fb8.l                      | +06  early alt: tail-jump
                                              |
                                              | ---- main entry $05CC1A ----
        jsr     0x25066.l                      | +0c  Sub_00025066
                                              |
                                              | ---- snapshot P1 (6B) ----
        lea.l   0x106eb0.l, a0                 | +12  a0 = &input_p1_current
        lea.l   0x10e200.l, a1                 | +18  a1 = &input_p1_snap
        move.l  (a0)+, (a1)+                   | +1e  copy 4B
        move.w  (a0), (a1)                     | +20  copy 2B
                                              |
                                              | ---- snapshot P2 (6B) ----
        lea.l   0x106eb6.l, a0                 | +22  a0 = &input_p2_current
        lea.l   0x10e206.l, a1                 | +28  a1 = &input_p2_snap
        move.l  (a0)+, (a1)+                   | +2e  copy 4B
        move.w  (a0), (a1)                     | +30  copy 2B
                                              |
                                              | ---- rising-edge sobre latch ----
        lea.l   0x10fdac.l, a0                 | +32  a0 = &input_latch
        lea.l   0x10e20c.l, a1                 | +38  a1 = &input_latch_prev
        move.b  (a1), d1                       | +3e  d1 = old latch
        move.b  (a0), d0                       | +40  d0 = new latch
        move.b  d0, (a1)                       | +42  save new
        eor.b   d0, d1                         | +44  d1 = changed bits
        and.b   d0, d1                         | +46  d1 = rising edge
        move.b  d1, 0x1(a1)                    | +48  publish edge byte
                                              |
                                              | ---- rama por mask P1 ----
        lea.l   0x10fdb6.l, a0                 | +4c  a0 = &mask_p1
        cmpi.b  #0x0, (a0)                     | +52  if (mask == 0)
        beq.w   .Lp2_dispatch                  | +56     skip (IDLE)
        cmpi.b  #0x3, (a0)                     | +5a  if (mask == 3)
        beq.w   .Lclear_p1                     | +5e     clear snap
                                              |
                                              | ---- default: copy p1 ----
        lea.l   0x10e200.l, a2                 | +62  a2 = &input_p1_snap
        lea.l   0x10e212.l, a1                 | +68  a1 = &input_p1_snap_active
        move.l  (a2)+, (a1)+                   | +6e
        move.w  (a2)+, (a1)+                   | +70
        bra.w   .Lp2_dispatch                  | +72
.Lclear_p1:
        lea.l   0x10e212.l, a1                 | +76  a1 = &input_p1_snap_active
        clr.l   (a1)+                          | +7c  clear 4B
        clr.w   (a1)+                          | +7e  clear 2B
                                              |
                                              | ---- rama por mask P2 ----
.Lp2_dispatch:
        lea.l   0x10fdb6.l, a0                 | +80  a0 = &mask_p1 (base)
        cmpi.b  #0x0, 0x1(a0)                  | +86  if (mask_p2 == 0)
        beq.w   .Lexit                         | +8c     skip (IDLE)
        cmpi.b  #0x3, 0x1(a0)                  | +90  if (mask_p2 == 3)
        beq.w   .Lclear_p2                     | +96     clear snap
                                              |
                                              | ---- default: copy p2 ----
        lea.l   0x10e206.l, a2                 | +9a  a2 = &input_p2_snap
        lea.l   0x10e218.l, a1                 | +a0  a1 = &input_p2_snap_active
        move.l  (a2)+, (a1)+                   | +a6
        move.w  (a2)+, (a1)+                   | +a8
        bra.w   .Lexit                         | +aa
.Lclear_p2:
        lea.l   0x10e218.l, a1                 | +ae  a1 = &input_p2_snap_active
        clr.l   (a1)+                          | +b4
        clr.w   (a1)+                          | +b6
.Lexit:
        rts                                    | +b8

        .size   Input_RisingEdgeSnapshot_05CC0E, .-Input_RisingEdgeSnapshot_05CC0E
