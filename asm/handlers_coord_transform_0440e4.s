| ============================================================================
|  Metal Slug 1 - asm/handlers_coord_transform_0440e4.s
|  ----------------------------------------------------------------------------
|  Wave CC batch 2 - #1..#2
|
|  Handlers gemelos "aplicar transformacion de camara con clipping" en
|  $0440E4..$044229 (326 B, 2 funciones casi identicas).
|
|  Ruta larga complementaria a los helpers cortos de CC batch 1:
|    - Gate re-entrancia por $13(a6) bit 6
|    - jsr $999DE = Clipping_Test (retorno CCR-C)
|    - Ruta ACEPTADA (C=0): d0 -= cam.x; d1 += cam.y
|    - Ruta RECHAZADA (C=1) con signo-XOR compacto (sgt/spl/eor/beq)
|    - Chequeo override por $106ECE/$106F5E
|
|  CC2#2 identico a CC2#1 salvo operar sobre coords globales $106F38/$106F3A.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

        .globl  Handler_ApplyCameraSelf_0440E4
        .type   Handler_ApplyCameraSelf_0440E4, @function
        .section .text.Handler_ApplyCameraSelf_0440E4, "ax", @progbits

Handler_ApplyCameraSelf_0440E4:
        btst.b  #0x6, 0x13(a6)
        bne.w   .L1_exit
        move.w  0x22(a6), d0
        move.w  0x24(a6), d1
        bset.b  #0x6, 0x13(a6)
        jsr     0x999de.l
        bcs.w   .L1_rejected
        sub.w   0x106f6c.l, d0
        add.w   0x106f6e.l, d1
        bra.w   .L1_check_override
.L1_rejected:
        btst.b  #0x7, 0x13(a6)
        beq.w   .L1_check_6b
.L1_offset_from_arg:
        add.w   d2, d0
        add.w   d3, d1
        bra.w   .L1_check_override
.L1_check_6b:
        btst.b  #0x6, 0x6b(a6)
        bne.w   .L1_signxor
        btst.b  #0x1, 0x6b(a6)
        bne.w   .L1_signxor
        btst.b  #0x0, 0x6b(a6)
        bne.w   .L1_signxor
        bra.b   .L1_offset_from_arg
.L1_signxor:
        cmpi.w  #0x1e, 0x22(a6)
        sgt.b   d4
        tst.w   d2
        spl.b   d5
        eor.b   d4, d5
        beq.w   .L1_no_offset_x
        add.w   d2, d0
.L1_no_offset_x:
        add.w   d3, d1
.L1_check_override:
        cmpi.b  #0x1, 0x106ece.l
        bne.w   .L1_publish
        tst.w   0x106f5e.l
        beq.w   .L1_publish
        move.w  0x22(a6), d0
.L1_publish:
        move.w  d0, 0x22(a6)
        move.w  d1, 0x24(a6)
.L1_exit:
        rts
        .size   Handler_ApplyCameraSelf_0440E4, .-Handler_ApplyCameraSelf_0440E4


        .globl  Handler_ApplyCameraGlobals_044182
        .type   Handler_ApplyCameraGlobals_044182, @function
        .section .text.Handler_ApplyCameraGlobals_044182, "ax", @progbits

Handler_ApplyCameraGlobals_044182:
        btst.b  #0x6, 0x13(a6)
        bne.w   .L2_exit
        move.w  0x106f38.l, d0
        move.w  0x106f3a.l, d1
        bset.b  #0x6, 0x13(a6)
        jsr     0x999de.l
        bcs.w   .L2_rejected
        sub.w   0x106f6c.l, d0
        add.w   0x106f6e.l, d1
        bra.w   .L2_check_override
.L2_rejected:
        btst.b  #0x7, 0x13(a6)
        beq.w   .L2_check_6b
.L2_offset_from_arg:
        add.w   d2, d0
        add.w   d3, d1
        bra.w   .L2_check_override
.L2_check_6b:
        btst.b  #0x6, 0x6b(a6)
        bne.w   .L2_signxor
        btst.b  #0x1, 0x6b(a6)
        bne.w   .L2_signxor
        btst.b  #0x0, 0x6b(a6)
        bne.w   .L2_signxor
        bra.b   .L2_offset_from_arg
.L2_signxor:
        cmpi.w  #0x1e, 0x22(a6)
        sgt.b   d4
        tst.w   d2
        spl.b   d5
        eor.b   d4, d5
        beq.w   .L2_no_offset_x
        add.w   d2, d0
.L2_no_offset_x:
        add.w   d3, d1
.L2_check_override:
        cmpi.b  #0x1, 0x106ece.l
        bne.w   .L2_publish
        tst.w   0x106f5e.l
        beq.w   .L2_publish
        move.w  0x106f38.l, d0
.L2_publish:
        move.w  d0, 0x106f38.l
        move.w  d1, 0x106f3a.l
.L2_exit:
        rts
        .size   Handler_ApplyCameraGlobals_044182, .-Handler_ApplyCameraGlobals_044182
