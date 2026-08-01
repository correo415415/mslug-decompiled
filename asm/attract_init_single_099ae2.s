| ============================================================================
|  Metal Slug 1 - asm/attract_init_single_099ae2.s
|  ----------------------------------------------------------------------------
|  Wave CC batch 1 - #14
|
|  AttractInit_Single_099AE2  @ $099AE2  (26 B, 1 caller)
|
|  Inicializador attract mode single-player. Publica canales audio $10FD88=3
|  y $10FD8B=2, resetea frame counter $10E486=0. Contrapartida asimetrica
|  de Global_Clear10E486_099AFC (Wave O#4) que solo hace $10E486=0 en dual.
|
|  Caller: TitleModeInit_024E38 (BB1#1) en rama single-player.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  AttractInit_Single_099AE2
        .type   AttractInit_Single_099AE2, @function
        .section .text.AttractInit_Single_099AE2, "ax", @progbits

AttractInit_Single_099AE2:
        move.b  #0x3, 0x10fd88.l
        move.b  #0x2, 0x10fd8b.l
        move.b  #0x0, 0x10e486.l
        rts
        .size   AttractInit_Single_099AE2, .-AttractInit_Single_099AE2
