| ============================================================================
|  Metal Slug 1 - asm/copy2bytes_10fdb6_to_10e3a0_0981fc.s
|  ----------------------------------------------------------------------------
|  Wave Y - #3  (helper trivial)
|
|  Copy2Bytes_10FDB6to10E3A0_0981FC  @ $0981FC  (18 bytes, 1 caller)
|
|  Copia 2 bytes contiguos desde $10FDB6/B7 a $10E3A0/A1 usando (a0)+/(a1)+
|  para el primero y (a0)/(a1) planos para el segundo. Idioma clasico para
|  copiar un word de 16 bits sin usar move.w (evitando alineacion o efecto
|  colateral de flags).
|
|  Firma C conceptual:
|
|      /* Copia dos bytes contiguos: $10FDB6..$10FDB7 -> $10E3A0..$10E3A1. */
|      void Copy2Bytes_10FDB6to10E3A0(void);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Dos moves de byte con (a0)+/(a1)+ seguido de (a0)/(a1) planos.
|       GCC emitiria un unico move.w o un memcpy inlined con dos move.b
|       con desplazamiento explicito.
|    2. lea.l de dos direcciones globales absolutas en secuencia como
|       argumentos implicitos de los moves indirectos. Convencion de asm
|       hand-coded, no de ABI GCC.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Copy2Bytes_10FDB6to10E3A0_0981FC
        .type   Copy2Bytes_10FDB6to10E3A0_0981FC, @function
        .section .text.Copy2Bytes_10FDB6to10E3A0_0981FC, "ax", @progbits

Copy2Bytes_10FDB6to10E3A0_0981FC:
        lea.l   0x10fdb6.l, a0                 | +00  a0 = src
        lea.l   0x10e3a0.l, a1                 | +06  a1 = dst
        move.b  (a0)+, (a1)+                   | +0c  dst[0] = src[0]; ++src; ++dst
        move.b  (a0),  (a1)                    | +0e  dst[1] = src[1]
        rts                                    | +10

        .size   Copy2Bytes_10FDB6to10E3A0_0981FC, .-Copy2Bytes_10FDB6to10E3A0_0981FC
