| ============================================================================
|  Metal Slug 1 - asm/fix_blit_batch_046b20.s
|  ----------------------------------------------------------------------------
|  Wave CC batch 2 - #3..#4
|
|  Cluster de 2 blitters MMIO al Fix Layer inter-conectados por bucle
|  mutuo en $046B20..$046C47 (296 B).
|
|  #3  $046B20  FixBlit_BatchRow4x2_046B20             (186 B)
|                Blit unroll fisico 4x2 tiles + bucle exterior 16 filas.
|                Termina con `bls.w #4` (tail-call condicional).
|
|  #4  $046BDA  FixBlit_BatchRow4x1_ColorInc_046BDA    (110 B)
|                Blit fila 4 tiles con incremento atributo +$10 por tile.
|                Termina con `bra.b` backward al medio de #3.
|
|  Primer bucle mutuo entre funciones vecinas del proyecto.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

        .globl  FixBlit_BatchRow4x2_046B20
        .type   FixBlit_BatchRow4x2_046B20, @function
        .section .text.FixBlit_BatchRow4x2_046B20, "ax", @progbits

FixBlit_BatchRow4x2_046B20:
        move.w  #0x7000, d0
        asl.w   #0x5, d2
        add.w   d2, d0
        add.w   d3, d0
        subi.w  #0x20, d0
        cmpi.w  #0x0, d4
        bne.w   .L3_palette_hi
        move.w  #0x0, d4
        bra.w   .L3_check_upper
.L3_palette_hi:
        move.w  #0x1000, d4
.L3_check_upper:
        cmpi.w  #0x74ff, d0
        bls.w   .L3_check_lower
        addi.w  #0x20, d0
        bra.w   .L3_tile_loop
.L3_check_lower:
        cmpi.w  #0x7000, d0
        bcc.w   .L3_do_blit
        addi.w  #0x20, d0
        bra.w   .L3_tile_loop
.L3_do_blit:
        move.w  #0x2320, d1
        movem.w d0-d1, 0x3c0000.l
        addq.w  #0x1, d0
        movem.w d0-d1, 0x3c0000.l
        addq.w  #0x1, d0
        movem.w d0-d1, 0x3c0000.l
        addq.w  #0x1, d0
        movem.w d0-d1, 0x3c0000.l
        addi.w  #0x1d, d0
        movem.w d0-d1, 0x3c0000.l
        addq.w  #0x1, d0
        movem.w d0-d1, 0x3c0000.l
        addq.w  #0x1, d0
        movem.w d0-d1, 0x3c0000.l
        addq.w  #0x1, d0
        movem.w d0-d1, 0x3c0000.l
        addi.w  #0x1d, d0
.L3_tile_loop:
        move.w  #0x3e00, d1
        add.w   d4, d1
        move.w  #0x0, d6
        bra.b   .L3_check_d6
.L3_iter_next:
        addq.w  #0x1, d6
.L3_check_d6:
        cmpi.w  #0xf, d6
        bgt.w   FixBlit_BatchRow4x2_046B20 + 0x108
        cmpi.w  #0x74ff, d0
        bls.w   FixBlit_BatchRow4x1_ColorInc_046BDA
        rts
        .size   FixBlit_BatchRow4x2_046B20, .-FixBlit_BatchRow4x2_046B20


        .globl  FixBlit_BatchRow4x1_ColorInc_046BDA
        .type   FixBlit_BatchRow4x1_ColorInc_046BDA, @function
        .section .text.FixBlit_BatchRow4x1_ColorInc_046BDA, "ax", @progbits

FixBlit_BatchRow4x1_ColorInc_046BDA:
        cmpi.w  #0x7000, d0
        bcc.w   .L4_do_blit
        addi.w  #0x20, d0
        addq.w  #0x1, d1
        bra.w   .L4_epilogue
.L4_do_blit:
        movem.w d0-d1, 0x3c0000.l
        addq.w  #0x1, d0
        addi.w  #0x10, d1
        movem.w d0-d1, 0x3c0000.l
        addq.w  #0x1, d0
        addi.w  #0x10, d1
        movem.w d0-d1, 0x3c0000.l
        addq.w  #0x1, d0
        addi.w  #0x10, d1
        movem.w d0-d1, 0x3c0000.l
        addi.w  #0x1d, d0
        subi.w  #0x2f, d1
.L4_epilogue:
        bra.b   FixBlit_BatchRow4x2_046B20 + 0xa6
        move.w  #0x3f00, d1
        add.w   d4, d1
        move.w  #0x0, d6
        bra.b   .L4_check_d6
.L4_iter_next:
        addq.w  #0x1, d6
.L4_check_d6:
        cmpi.w  #0x3, d6
        bgt.w   FixBlit_BatchRow4x1_ColorInc_046BDA + 0xbc
        cmpi.w  #0x74ff, d0
        bls.w   FixBlit_BatchRow4x1_ColorInc_046BDA + 0x6e
        rts
        .size   FixBlit_BatchRow4x1_ColorInc_046BDA, .-FixBlit_BatchRow4x1_ColorInc_046BDA
