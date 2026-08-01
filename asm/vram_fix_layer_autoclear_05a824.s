| ============================================================================
|  Metal Slug 1 - asm/vram_fix_layer_autoclear_05a824.s
|  ----------------------------------------------------------------------------
|  Wave DD - #3
|
|  VRAM_FixLayerAutoclear_05A824  @ $05A824  (150 B, 1 caller)
|
|  Rutina de reset masivo de VRAM Fix Layer + reset de flags/slots globales.
|  Se llama en transiciones (fin de nivel, boot, cambio de escenario).
|
|  Estructura:
|
|    1. FASE A: escritura secuencial de $0 a un rango VRAM via autoinc.
|         d2 = 0
|         *$3C0004 = 1          (autoinc mode = 1 word)
|         *$3C0000 = $8200      (base VRAM addr para autoinc)
|         a0 = $3C0002          (autoinc data port)
|         d0 = $17C = 380       (contador -1)
|       loop_A:
|         *a0 = d2              (write 0)
|         d0.b = d0.b           (redundant, no-op sobre CCR ya set)
|         dbra d0, loop_A       (380+1 = 381 iter)
|
|    2. FASE B: escritura secuencial de $0 a otro rango VRAM (con NOPs
|       intercalados para stretch de timing MMIO).
|         d1 = 0
|         d2 = 0
|         *$3C0004 = 1
|         *$3C0000 = $0
|         a0 = $3C0002
|         d0 = $2F9F = 12191   (contador -1)
|       loop_B:
|         nop
|         nop
|         *a0 = d1              (write 0)
|         nop
|         nop
|         *a0 = d2              (write 0)
|         dbra d0, loop_B       (12191+1 = 12192 iter x 2 writes = 24384 words)
|
|    3. Reset de flags/counters globales:
|         *$10E1FA = 0
|         *$10E1F6 = 0
|         *$10E1F8 = 0
|
|    4. Reset de slots de sprite en $108080..$108080+$616C:
|         a5 = $108080
|         a5[$616C] = 0
|         a5[$6168].w = 0
|         a5[$616A].w = 0
|
|         a0 = a5 + $614C
|         a0[$8] = 0
|         a0[$4].w = $FFFF     (sentinel "unused slot")
|
|         a0 = a5 + $6158
|         a0[$8] = 0
|         a0[$4].w = $FFFF
|
|    5. rts.
|
|  Idiomas hand-coded:
|    - Doble bucle MMIO con puerto autoinc separado (`$3C0000` para addr,
|      `$3C0002` para data). Puro protocolo hardware Neo Geo, no rederivable
|      por GCC.
|    - NOPs intercalados en el bucle B: sincronizacion critica con el
|      timing del hardware (probablemente para VDP sync durante VBlank).
|    - `move.b d0, d0` no-op en el bucle A (equivalente a un `tst.b d0`
|      con menos bytes). GCC nunca emite auto-move como no-op.
|    - `lea.l $614C(a5), a0` con desplazamiento grande via long displacement.
|
|  Firma C conceptual:
|
|      /* Reset masivo de VRAM Fix Layer via autoinc:
|       *   - 381 words en rango $8200 (banda estatica)
|       *   - 24 384 words en rango $0000 (banda de sprites)
|       * y reset de flags/slots globales en $10E1F6/F8/FA y $108080. */
|      void VRAM_FixLayerAutoclear(void);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  VRAM_FixLayerAutoclear_05A824
        .type   VRAM_FixLayerAutoclear_05A824, @function
        .section .text.VRAM_FixLayerAutoclear_05A824, "ax", @progbits

VRAM_FixLayerAutoclear_05A824:
                                              | ---- FASE A: 381 words ----
        moveq   #0x0, d2                       | +00  d2 = 0
        move.w  #0x1, 0x3c0004.l               | +02  autoinc mode = 1
        move.w  #0x8200, 0x3c0000.l            | +0a  VRAM addr = $8200
        movea.l #0x3c0002, a0                  | +12  a0 = data port
        move.w  #0x17c, d0                     | +18  d0 = 380
.LA_loop:
        move.w  d2, (a0)                       | +1c  *port = 0
        move.b  d0, d0                         | +1e  no-op (byte move to self)
        dbra    d0, .LA_loop                   | +20  (380+1 iter)
                                              |
                                              | ---- FASE B: 12192 iter x 2 writes ----
        move.w  #0x0, d1                       | +24  d1 = 0
        moveq   #0x0, d2                       | +28  d2 = 0
        move.w  #0x1, 0x3c0004.l               | +2a  autoinc mode = 1
        move.w  #0x0, 0x3c0000.l               | +32  VRAM addr = 0
        movea.l #0x3c0002, a0                  | +3a  a0 = data port
        move.w  #0x2f9f, d0                    | +40  d0 = 12191
.LB_loop:
        nop                                    | +44
        nop                                    | +46
        move.w  d1, (a0)                       | +48  *port = 0 (write #1)
        nop                                    | +4a
        nop                                    | +4c
        move.w  d2, (a0)                       | +4e  *port = 0 (write #2)
        dbra    d0, .LB_loop                   | +50
                                              |
                                              | ---- reset flags globales ----
        clr.w   0x10e1fa.l                     | +54
        clr.w   0x10e1f6.l                     | +5a
        clr.w   0x10e1f8.l                     | +60
                                              |
                                              | ---- reset slots $108080 ----
        lea.l   0x108080.l, a5                 | +66  a5 = &slot_base
        clr.b   0x616c(a5)                     | +6c
        clr.w   0x6168(a5)                     | +70
        clr.w   0x616a(a5)                     | +74
                                              |
        lea.l   0x614c(a5), a0                 | +78  a0 = slot A
        clr.b   0x8(a0)                        | +7c
        move.w  #0xffff, 0x4(a0)               | +80  sentinel unused
                                              |
        lea.l   0x6158(a5), a0                 | +86  a0 = slot B
        clr.b   0x8(a0)                        | +8a
        move.w  #0xffff, 0x4(a0)               | +8e  sentinel unused
        rts                                    | +94

        .size   VRAM_FixLayerAutoclear_05A824, .-VRAM_FixLayerAutoclear_05A824
