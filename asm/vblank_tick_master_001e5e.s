| ============================================================================
|  Metal Slug 1 - asm/vblank_tick_master_001e5e.s
|  ----------------------------------------------------------------------------
|  Wave DD - #4
|
|  VBlankTick_Master_001E5E  @ $001E5E  (152 B, 1 caller)
|
|  Handler maestro del tick de VBlank. Se invoca cada frame (60 Hz PAL/NTSC
|  segun modo) desde el vector VBlank del 68000. Complementa el cluster
|  IRQ ya matcheado en Wave Q.
|
|  Estructura:
|
|    1. Publica $4 en $3C000C (VDP auto-anim register, avance timer).
|    2. Incrementa $106EDD (frame counter Modular byte).
|    3. Si $106EDD > $A (=10): salta el poke a hardware.
|    4. Sino: escribe d0 a $300001 (I/O output, probablemente hw watchdog).
|    5. Incrementa $106ED9 (frame counter global byte).
|    6. `jsr $226A` (Input_QueuePush_00212E, ya matcheado en Wave A/S).
|    7. Lee $106EDA (frame delay counter):
|       - Si != 0: decrementa y salta a fin (skip logica pesada).
|       - Si == 0: continua...
|    8. Chequeos combinados:
|         if ($106EDE == 0 || $106ED9 > 1) && $106ED8 == 0:
|             if ($106EDC == 0 && $106EDB != 0):
|                 $106EDB = 0
|                 jsr $137C6
|             jsr $1EFE(pc)       (helper PC-rel a $001EFE)
|             jsr $5C9D6.l        (helper vecino sprite dispatch)
|             $106ED8 = 1         (marca "tick heavy done")
|             rts
|    9. Fin ligero: incrementa $106EE2 (heavy skip counter) + rts.
|
|  Idiomas hand-coded:
|    - `move.w #$4, $3C000C.l` — poke al VDP autoanim en cada tick.
|    - `move.b d0, $300001.l` — poke al I/O output (bank register hw).
|    - `jsr $1EFE(pc)` — llamada PC-relativa a la funcion vecina inmediata
|      (que ya no cae dentro de esta funcion). Idioma que GCC no genera
|      porque no puede prever ubicaciones absolutas de otras funciones.
|    - Cadena de tres `tst.b/cmpi.b` en cascada con branches, todos hacia
|      un mismo destino ($1EF6, fin ligero). GCC combinaria en un `mov`
|      logico o serie de tests con seteo de flag booleano.
|
|  Firma C conceptual:
|
|      /* Handler maestro del tick VBlank. Publica autoanim, incrementa
|       * los frame counters, ejecuta el input queue push, y en frames
|       * "elegibles" hace el batch de heavy work (jsr $137C6 + $1EFE +
|       * $5C9D6) marcando $106ED8=1 como flag "done"; los demas frames
|       * solo incrementan $106EE2 como counter de heavy-skips. */
|      void VBlankTick_Master(uint8_t io_out /*d0*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  VBlankTick_Master_001E5E
        .type   VBlankTick_Master_001E5E, @function
        .section .text.VBlankTick_Master_001E5E, "ax", @progbits

VBlankTick_Master_001E5E:
        move.w  #0x4, 0x3c000c.l               | +00  VDP autoanim += 4
        addq.b  #0x1, 0x106edd.l               | +08  frame_mod_ctr++
        cmpi.b  #0xa, 0x106edd.l               | +0e  if (ctr > 10)
        bhi.w   .Lskip_iopoke                  | +16     skip hw poke
        move.b  d0, 0x300001.l                 | +1a  I/O output = d0
.Lskip_iopoke:
        addq.b  #0x1, 0x106ed9.l               | +20  frame_ctr++
        jsr     0x226a.l                       | +26  Input_QueuePush
                                              |
        move.b  0x106eda.l, d0                 | +2c  d0 = frame_delay_ctr
        beq.w   .Lcheck_gate                   | +32  if (d0 == 0) goto gate
        subq.b  #0x1, d0                       | +36  --d0
        move.b  d0, 0x106eda.l                 | +38  store
        bra.w   .Lskip_heavy                   | +3e  goto skip_heavy
                                              |
.Lcheck_gate:
        cmpi.b  #0x0, 0x106ede.l               | +42  if ($106EDE == 0)
        beq.w   .Lcheck_step2                  | +4a     goto step2
        cmpi.b  #0x1, 0x106ed9.l               | +4e  if (frame_ctr <= 1)
        bls.w   .Lskip_heavy                   | +56     goto skip
.Lcheck_step2:
        tst.b   0x106ed8.l                     | +5a  if ($106ED8 != 0)
        bne.w   .Lskip_heavy                   | +60     goto skip
        tst.b   0x106edc.l                     | +64  if ($106EDC != 0)
        bne.w   .Ldo_heavy                     | +6a     skip clear
        tst.b   0x106edb.l                     | +6e  if ($106EDB == 0)
        beq.w   .Ldo_heavy                     | +74     skip clear
        clr.b   0x106edb.l                     | +78  $106EDB = 0
        jsr     0x137c6.l                      | +7e  Sub_000137C6
.Ldo_heavy:
        jsr     .Ltick_helper(pc)              | +84  jsr $001EFE (pc-rel)
        jsr     0x5c9d6.l                      | +88  Sub_0005C9D6
        move.b  #0x1, 0x106ed8.l               | +8e  $106ED8 = 1 (done)
        rts                                    | +96
.Lskip_heavy:
        addq.w  #0x1, 0x106ee2.l               | +98  heavy_skip_ctr++
        rts                                    | +9e

        .equ    .Ltick_helper, 0x001EFE

        .size   VBlankTick_Master_001E5E, .-VBlankTick_Master_001E5E
