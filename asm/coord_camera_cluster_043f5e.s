| ============================================================================
|  Metal Slug 1 - asm/coord_camera_cluster_043f5e.s
|  ----------------------------------------------------------------------------
|  Wave CC batch 1 - #1..#12
|
|  Subsistema completo del sistema de coordenadas camara<->pantalla + blitter
|  de tile individual en $043EDA..$0440E3 (206 B netos, 12 helpers).
|
|  Descubrimiento arquitectonico: el juego mantiene CUATRO pares de coord
|  de camara en RAM:
|
|      $106F6C/$106F6E   camara principal        (X/Y del viewport)
|      $106F70/$106F74   scroll rapido           (world->screen offset)
|      $106F50/$106F54   camara secundaria       (paralaje background)
|      $108064/$108066   camara terciaria        (paralaje foreground)
|
|  Idioma "Y-flip Neo Geo" clasico: `neg.w d1; addi.w #$200, d1` (o su
|  inverso) para convertir entre coord logica del juego y coord de sprite
|  VRAM (el hardware Neo Geo tiene Y invertida en la VRAM).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

        .globl  FixBlit_TileByCoord_043F5E
        .type   FixBlit_TileByCoord_043F5E, @function
        .section .text.FixBlit_TileByCoord_043F5E, "ax", @progbits

FixBlit_TileByCoord_043F5E:
        lea.l   0x106f6c.l, a0
        add.w   0x4(a0), d0
        neg.w   d1
        add.w   0x8(a0), d1
        move.w  0x22(a0), d2
        lsl.w   #0x4, d2
        add.w   d2, d0
        move.w  d0, d3
        andi.w  #0x7, d3
        move.w  0x24(a0), d2
        lsl.w   #0x4, d2
        add.w   d2, d1
        move.w  d1, d4
        andi.w  #0x7, d4
        move.w  d0, d2
        andi.w  #0x1f0, d0
        andi.w  #0x8, d2
        andi.w  #0x1f8, d1
        ror.w   #0x2, d1
        ror.w   #0x3, d2
        rol.w   #0x3, d0
        add.w   d1, d0
        add.w   d2, d0
        lea.l   0x7c(a0), a0
        move.b  d7, (a0, d0.w)
        rts
        .size   FixBlit_TileByCoord_043F5E, .-FixBlit_TileByCoord_043F5E


        .globl  Coord_LocalToScreen_04400E
        .type   Coord_LocalToScreen_04400E, @function
        .section .text.Coord_LocalToScreen_04400E, "ax", @progbits

Coord_LocalToScreen_04400E:
        add.w   0x106f70.l, d0
        neg.w   d1
        addi.w  #0x200, d1
        add.w   0x106f74.l, d1
        rts
        .size   Coord_LocalToScreen_04400E, .-Coord_LocalToScreen_04400E


        .globl  Coord_ScreenToLocal_044022
        .type   Coord_ScreenToLocal_044022, @function
        .section .text.Coord_ScreenToLocal_044022, "ax", @progbits

Coord_ScreenToLocal_044022:
        sub.w   0x106f70.l, d0
        sub.w   0x106f74.l, d1
        subi.w  #0x200, d1
        neg.w   d1
        rts
        .size   Coord_ScreenToLocal_044022, .-Coord_ScreenToLocal_044022


        .globl  Coord_ResetAndClearBuf_044036
        .type   Coord_ResetAndClearBuf_044036, @function
        .section .text.Coord_ResetAndClearBuf_044036, "ax", @progbits

Coord_ResetAndClearBuf_044036:
        lea.l   0x106f6c.l, a0
        bsr.w   Buffer_ClearBlock1024L_043EDA
        clr.w   (a0)
        clr.w   0x2(a0)
        rts
        .size   Coord_ResetAndClearBuf_044036, .-Coord_ResetAndClearBuf_044036


        .globl  Coord_ApplyCameraDeltaToSelf_044048
        .type   Coord_ApplyCameraDeltaToSelf_044048, @function
        .section .text.Coord_ApplyCameraDeltaToSelf_044048, "ax", @progbits

Coord_ApplyCameraDeltaToSelf_044048:
        move.w  0x106f6c.l, d0
        sub.w   d0, 0x22(a6)
        move.w  0x106f6e.l, d0
        add.w   d0, 0x24(a6)
        rts
        .size   Coord_ApplyCameraDeltaToSelf_044048, .-Coord_ApplyCameraDeltaToSelf_044048


        .globl  Coord_ApplyCameraDeltaToA0Mark_04405E
        .type   Coord_ApplyCameraDeltaToA0Mark_04405E, @function
        .section .text.Coord_ApplyCameraDeltaToA0Mark_04405E, "ax", @progbits

Coord_ApplyCameraDeltaToA0Mark_04405E:
        move.w  0x106f6c.l, d0
        sub.w   d0, 0x22(a0)
        move.w  0x106f6e.l, d0
        add.w   d0, 0x24(a0)
        bset.b  #0x6, 0x13(a0)
        rts
        .size   Coord_ApplyCameraDeltaToA0Mark_04405E, .-Coord_ApplyCameraDeltaToA0Mark_04405E


        .globl  Coord_ApplyCameraTerciaryToSelf_04407A
        .type   Coord_ApplyCameraTerciaryToSelf_04407A, @function
        .section .text.Coord_ApplyCameraTerciaryToSelf_04407A, "ax", @progbits

Coord_ApplyCameraTerciaryToSelf_04407A:
        move.w  0x108064.l, d0
        sub.w   d0, 0x22(a6)
        move.w  0x108066.l, d0
        add.w   d0, 0x24(a6)
        rts
        .size   Coord_ApplyCameraTerciaryToSelf_04407A, .-Coord_ApplyCameraTerciaryToSelf_04407A


        .globl  Coord_ApplyCameraDeltaToGlobals_044090
        .type   Coord_ApplyCameraDeltaToGlobals_044090, @function
        .section .text.Coord_ApplyCameraDeltaToGlobals_044090, "ax", @progbits

Coord_ApplyCameraDeltaToGlobals_044090:
        move.w  0x106f6c.l, d0
        sub.w   d0, 0x106f38.l
        move.w  0x106f6e.l, d0
        add.w   d0, 0x106f3a.l
        rts
        .size   Coord_ApplyCameraDeltaToGlobals_044090, .-Coord_ApplyCameraDeltaToGlobals_044090


        .globl  Coord_ApplyCameraDeltaToD1D2_0440AA
        .type   Coord_ApplyCameraDeltaToD1D2_0440AA, @function
        .section .text.Coord_ApplyCameraDeltaToD1D2_0440AA, "ax", @progbits

Coord_ApplyCameraDeltaToD1D2_0440AA:
        move.w  0x106f6c.l, d0
        add.w   d0, d1
        move.w  0x106f6e.l, d0
        sub.w   d0, d2
        rts
        .size   Coord_ApplyCameraDeltaToD1D2_0440AA, .-Coord_ApplyCameraDeltaToD1D2_0440AA


        .globl  Coord_LocalToScreenSecondary_0440BC
        .type   Coord_LocalToScreenSecondary_0440BC, @function
        .section .text.Coord_LocalToScreenSecondary_0440BC, "ax", @progbits

Coord_LocalToScreenSecondary_0440BC:
        add.w   0x106f50.l, d0
        subi.w  #0x200, d1
        neg.w   d1
        add.w   0x106f54.l, d1
        rts
        .size   Coord_LocalToScreenSecondary_0440BC, .-Coord_LocalToScreenSecondary_0440BC


        .globl  Coord_ScreenToLocalSecondary_0440D0
        .type   Coord_ScreenToLocalSecondary_0440D0, @function
        .section .text.Coord_ScreenToLocalSecondary_0440D0, "ax", @progbits

Coord_ScreenToLocalSecondary_0440D0:
        sub.w   0x106f50.l, d0
        sub.w   0x106f54.l, d1
        neg.w   d1
        addi.w  #0x200, d1
        rts
        .size   Coord_ScreenToLocalSecondary_0440D0, .-Coord_ScreenToLocalSecondary_0440D0


        .globl  Buffer_ClearBlock1024L_043EDA
        .type   Buffer_ClearBlock1024L_043EDA, @function
        .section .text.Buffer_ClearBlock1024L_043EDA, "ax", @progbits

Buffer_ClearBlock1024L_043EDA:
        lea.l   0x7c(a0), a1
        move.l  #0x3ff, d7
.L12_loop:
        clr.l   (a1)+
        dbra    d7, .L12_loop
        rts
        .size   Buffer_ClearBlock1024L_043EDA, .-Buffer_ClearBlock1024L_043EDA
