| ============================================================================
|  Metal Slug 1 - asm/global_set_dual_flag_000ffe.s
|  ----------------------------------------------------------------------------
|  Wave Y - #4
|
|  Global_SetDualFlagFrom10FD82_000FFE  @ $000FFE  (34 bytes)
|
|  Publica en dos globales gemelas ($1081BF y $1081C0) el valor derivado
|  del flag $10FD82:
|      $10FD82 == 0  ->  $1081BF = $1081C0 = 4
|      $10FD82 != 0  ->  $1081BF = $1081C0 = 0
|
|  Firma C conceptual:
|
|      /* Sincroniza dos flags gemelas con el estado de $10FD82. */
|      void Global_SetDualFlagFrom10FD82(void);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Rama bne + bra explicita a un merge point para materializar 0 o 4
|       en d0. GCC emitiria un patron con seq/scc + and, no un doble branch.
|    2. Publicacion en dos direcciones absolutas distintas mediante dos
|       move.b d0 consecutivos. GCC coalesceria en un move.w si fueran
|       contiguas, pero $1081BF y $1081C0 SON contiguas y aun asi el codigo
|       usa dos move.b: patente de asm hand-coded.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Global_SetDualFlagFrom10FD82_000FFE
        .type   Global_SetDualFlagFrom10FD82_000FFE, @function
        .section .text.Global_SetDualFlagFrom10FD82_000FFE, "ax", @progbits

Global_SetDualFlagFrom10FD82_000FFE:
        tst.b   0x10fd82.l                     | +00  if ($10fd82 != 0)
        bne.w   .Lzero                         | +06     goto .Lzero
        move.b  #0x4, d0                       | +0a  d0 = 4
        bra.w   .Lstore                        | +0e  goto .Lstore
.Lzero:
        clr.b   d0                             | +12  d0 = 0
.Lstore:
        move.b  d0, 0x1081bf.l                 | +14  flag_a = d0
        move.b  d0, 0x1081c0.l                 | +1a  flag_b = d0
        rts                                    | +20

        .size   Global_SetDualFlagFrom10FD82_000FFE, .-Global_SetDualFlagFrom10FD82_000FFE
